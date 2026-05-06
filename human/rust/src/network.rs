use anyhow::{Context, Result};
use std::io::ErrorKind;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{copy_bidirectional, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

const REMOTE_HOST: &str = "host.docker.internal";
const LOCAL_TCP_HOST: &str = "127.0.0.1";
const LOCAL_TCP_PORT: u16 = 10004;
const REMOTE_TCP_PORT: u16 = 10044;
const REMOTE_UDP_PORT: u16 = 10055;
const UDP_PORT: u16 = 10005;

pub fn start() {
    println!("Networking created");

    tokio::spawn(async {
        if let Err(err) = run_tcp_tunnel().await {
            eprintln!("ERROR: {err:?}");
        }
    });

    tokio::spawn(async {
        if let Err(err) = run_udp_bridge().await {
            eprintln!("ERROR: {err:?}");
        }
    });
}

async fn run_tcp_tunnel() -> Result<()> {
    let mut remote = TcpStream::connect((REMOTE_HOST, REMOTE_TCP_PORT))
        .await
        .with_context(|| format!("connect TCP tunnel {}:{}", REMOTE_HOST, REMOTE_TCP_PORT))?;
    remote.set_nodelay(true)?;
    println!("TCP tunnel connected ({}:{})", REMOTE_HOST, REMOTE_TCP_PORT);

    let mut local = connect_local_game().await?;
    copy_bidirectional(&mut local, &mut remote).await?;

    println!("TCP tunnel disconnected");
    Ok(())
}

async fn connect_local_game() -> Result<TcpStream> {
    loop {
        match TcpStream::connect((LOCAL_TCP_HOST, LOCAL_TCP_PORT)).await {
            Ok(stream) => {
                stream.set_nodelay(true)?;
                println!("Local game connected ({}:{})", LOCAL_TCP_HOST, LOCAL_TCP_PORT);
                return Ok(stream);
            }
            Err(err) if err.kind() == ErrorKind::ConnectionRefused || err.kind() == ErrorKind::ConnectionReset => {
                sleep(Duration::from_secs(1)).await;
            }
            Err(err) => {
                return Err(err).with_context(|| {
                    format!("connect local game {}:{}", LOCAL_TCP_HOST, LOCAL_TCP_PORT)
                });
            }
        }
    }
}

async fn run_udp_bridge() -> Result<()> {
    let tcp = TcpStream::connect((REMOTE_HOST, REMOTE_UDP_PORT))
        .await
        .with_context(|| format!("connect UDP tunnel {}:{}", REMOTE_HOST, REMOTE_UDP_PORT))?;
    tcp.set_nodelay(true)?;
    println!("UDP tunnel connected");

    let udp = Arc::new(UdpSocket::bind(("0.0.0.0", UDP_PORT)).await?);
    let local = udp.local_addr()?;
    println!("Listening on: {}:{} UDP bridge", local.ip(), local.port());

    let peer = Arc::new(Mutex::new(None::<SocketAddr>));
    let (mut tcp_reader, mut tcp_writer) = tcp.into_split();

    let udp_from_tcp = Arc::clone(&udp);
    let peer_from_tcp = Arc::clone(&peer);
    tokio::spawn(async move {
        let mut buffer = vec![0_u8; 65_536];
        loop {
            match tcp_reader.read(&mut buffer).await {
                Ok(0) => {
                    println!("UDP tunnel disconnected");
                    return;
                }
                Ok(n) => {
                    let peer_addr = *peer_from_tcp.lock().await;
                    if let Some(target) = peer_addr {
                        if let Err(err) = udp_from_tcp.send_to(&buffer[..n], target).await {
                            eprintln!("ERROR: {err:?}");
                            return;
                        }
                    }
                }
                Err(err) => {
                    eprintln!("ERROR: {err:?}");
                    return;
                }
            }
        }
    });

    let mut buffer = vec![0_u8; 65_536];
    loop {
        let (n, source) = udp.recv_from(&mut buffer).await?;
        *peer.lock().await = Some(source);

        tcp_writer.write_all(&buffer[..n]).await?;
    }
}
