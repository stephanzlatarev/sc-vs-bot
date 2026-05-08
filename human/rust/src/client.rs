use anyhow::{anyhow, Context, Result};
use futures_util::{SinkExt, StreamExt};
use protobuf::Message as PbMessage;
use sc2_proto::common::Race;
use sc2_proto::sc2api::{
    InterfaceOptions, LocalMap, PlayerSetup, PlayerType, PortSet, Request, RequestCreateGame,
    RequestJoinGame, RequestStep, Response,
};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration, Instant};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

use crate::config::Config;

const MILLIS_PER_LOOP: f64 = 1000.0 / 22.4;

type Ws = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct Client {
    config: Arc<Config>,
    ws: Option<Ws>,
    loop_count: u64,
    time: Option<Instant>,
}

impl Client {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            ws: None,
            loop_count: 0,
            time: None,
        }
    }

    pub async fn connect(&mut self) -> Result<()> {
        let deadline = Instant::now() + Duration::from_secs(60);
        let url = format!(
            "ws://{}:{}/sc2api",
            self.config.local_host, self.config.sc2_port
        );

        while Instant::now() < deadline {
            println!("Connecting to StarCraft II...");
            match connect_async(&url).await {
                Ok((ws, _)) => {
                    println!("Connected");
                    self.ws = Some(ws);
                    return Ok(());
                }
                Err(err) => {
                    println!("Error on attempt to connect to StarCraft II: {err}");
                    sleep(Duration::from_secs(3)).await;
                }
            }
        }

        println!("Unable to connect to StarCraft II");
        Err(anyhow!("unable to connect to StarCraft II"))
    }

    pub async fn create_game(&mut self) -> Result<()> {
        println!("Creating game");

        let mut local_map = LocalMap::new();
        local_map.set_map_path(
            self.config
                .sc2_path
                .join("Maps")
                .join(format!("{}.SC2Map", &self.config.map_name))
                .to_string_lossy()
                .to_string(),
        );

        let mut p1 = PlayerSetup::new();
        p1.set_type(PlayerType::Participant);
        let mut p2 = PlayerSetup::new();
        p2.set_type(PlayerType::Participant);

        let mut create = RequestCreateGame::new();
        create.set_local_map(local_map);
        create.set_player_setup(vec![p1, p2]);
        create.set_realtime(false);

        let mut request = Request::new();
        request.set_create_game(create);

        let response = self.call(request).await?;
        println!("Game created: {:?}", response.create_game());
        Ok(())
    }

    pub async fn join_game(&mut self, race: Race, name: &str) -> Result<()> {
        println!("Joining game...");

        let mut options = InterfaceOptions::new();
        options.set_raw(true);
        options.set_score(true);

        let mut server_ports = PortSet::new();
        server_ports.set_game_port(i32::from(self.config.local_server_port));
        server_ports.set_base_port(i32::from(self.config.local_server_port));

        let mut client_ports = PortSet::new();
        client_ports.set_game_port(i32::from(self.config.local_client_port));
        client_ports.set_base_port(i32::from(self.config.local_client_port));

        let mut join = RequestJoinGame::new();
        join.set_player_name(name.to_string());
        join.set_race(race);
        join.set_options(options);
        join.set_server_ports(server_ports);
        join.set_client_ports(vec![client_ports]);

        let mut request = Request::new();
        request.set_join_game(join);

        let response = self.call(request).await?;
        println!("Game joined: {:?}", response.join_game());
        Ok(())
    }

    pub async fn step(&mut self) -> Result<()> {
        let mut step = RequestStep::new();
        step.set_count(1);

        let mut request = Request::new();
        request.set_step(step);
        self.call(request).await?;

        if self.time.is_none() {
            self.time = Some(Instant::now());
        }

        self.loop_count += 1;

        let elapsed = self
            .time
            .expect("time should be set")
            .elapsed()
            .as_secs_f64()
            * 1000.0;
        let expected = (self.loop_count as f64) * MILLIS_PER_LOOP;

        if elapsed < expected {
            sleep(Duration::from_millis((expected - elapsed) as u64)).await;
        }

        Ok(())
    }

    pub async fn disconnect(&mut self) -> Result<()> {
        println!("Disconnecting...");
        if let Some(ws) = &mut self.ws {
            ws.close(None).await?;
        }
        self.ws = None;
        Ok(())
    }

    async fn call(&mut self, request: Request) -> Result<Response> {
        let ws = self.ws.as_mut().context("SC2 client is not connected")?;
        let payload = request.write_to_bytes()?;

        ws.send(Message::Binary(payload.into())).await?;

        while let Some(message) = ws.next().await {
            let message = message?;
            match message {
                Message::Binary(bytes) => {
                    let response = Response::parse_from_bytes(bytes.as_ref())?;
                    return Ok(response);
                }
                Message::Close(_) => return Err(anyhow!("SC2 websocket closed")),
                Message::Ping(data) => {
                    ws.send(Message::Pong(data)).await?;
                }
                _ => {}
            }
        }

        Err(anyhow!("SC2 websocket ended"))
    }
}
