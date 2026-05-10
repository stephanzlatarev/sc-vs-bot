mod client;
mod config;
mod game;
mod lobby;
mod network;
mod gui;

use anyhow::{anyhow, Result};
use std::sync::Arc;

use client::Client;
use config::Config;
use game::Game;

#[tokio::main]
async fn main() -> Result<()> {
    let gui = true;
    if gui{
        gui::run_gui().await
    } else {
        play().await
    }
}

async fn play() -> Result<()> {
    let mut config = Config::new();
    lobby::prepare(&mut config).await?;

    let config = Arc::new(config);
    let mut game = Game::new(Arc::clone(&config));
    let mut client = Client::new(Arc::clone(&config));

    let result: Result<()> = async {
        game.start().await?;
        client.connect().await?;

        client.create_game().await?;

        let (mut tcp_handle, mut udp_handle) = network::start(Arc::clone(&config));

        client.join_game(config.player_race, &config.player_name).await?;

        loop {
            tokio::select! {
                step_result = client.step() => {
                    step_result?;
                }
                tcp_result = &mut tcp_handle => {
                    match tcp_result {
                        Ok(Err(e)) => return Err(e.context("TCP tunnel disconnected")),
                        _ => return Err(anyhow!("TCP tunnel disconnected")),
                    }
                }
                udp_result = &mut udp_handle => {
                    match udp_result {
                        Ok(Err(e)) => return Err(e.context("UDP bridge disconnected")),
                        _ => return Err(anyhow!("UDP bridge disconnected")),
                    }
                }
            }
        }
    }
    .await;

    if let Err(err) = &result {
        println!("{err}");
    }

    let _ = client.disconnect().await;
    game.stop().await;

    result
}
