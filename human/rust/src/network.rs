use anyhow::{Context, Result};
use std::io::ErrorKind;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{copy_bidirectional, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

use crate::config::Config;

pub fn start(config: Arc<Config>) {
    println!("Networking created");

    let tcp_config = Arc::clone(&config);
    tokio::spawn(async {
        if let Err(err) = run_tcp_tunnel(tcp_config).await {
            eprintln!("ERROR: {err:?}");
        }
    });

    tokio::spawn(async {
        if let Err(err) = run_udp_bridge(config).await {
            eprintln!("ERROR: {err:?}");
        }
    });
}

async fn run_tcp_tunnel(config: Arc<Config>) -> Result<()> {
    let mut remote = TcpStream::connect((config.remote_host.as_str(), config.remote_server_port))
        .await
        .with_context(|| {
            format!(
                "connect TCP tunnel {}:{}",
                config.remote_host, config.remote_server_port
            )
        })?;
    remote.set_nodelay(true)?;
    println!(
        "TCP tunnel connected ({}:{})",
        config.remote_host, config.remote_server_port
    );

    let mut local = connect_local_game(&config).await?;
    copy_bidirectional(&mut local, &mut remote).await?;

    println!("TCP tunnel disconnected");
    Ok(())
}

async fn connect_local_game(config: &Config) -> Result<TcpStream> {
    loop {
        match TcpStream::connect((config.local_host.as_str(), config.local_server_port)).await {
            Ok(stream) => {
                stream.set_nodelay(true)?;
                println!(
                    "Local game connected ({}:{})",
                    config.local_host, config.local_server_port
                );
                return Ok(stream);
            }
            Err(err) if err.kind() == ErrorKind::ConnectionRefused || err.kind() == ErrorKind::ConnectionReset => {
                sleep(Duration::from_secs(1)).await;
            }
            Err(err) => {
                return Err(err).with_context(|| {
                    format!(
                        "connect local game {}:{}",
                        config.local_host, config.local_server_port
                    )
                });
            }
        }
    }
}

async fn run_udp_bridge(config: Arc<Config>) -> Result<()> {
    let tcp = TcpStream::connect((config.remote_host.as_str(), config.remote_client_port))
        .await
        .with_context(|| {
            format!(
                "connect UDP tunnel {}:{}",
                config.remote_host, config.remote_client_port
            )
        })?;
    tcp.set_nodelay(true)?;
    println!("UDP tunnel connected");

    let udp = Arc::new(UdpSocket::bind(("0.0.0.0", config.local_client_port)).await?);
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
