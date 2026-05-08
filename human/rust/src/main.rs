mod client;
mod config;
mod game;
mod lobby;
mod network;

use anyhow::Result;
use std::sync::Arc;

use client::Client;
use config::Config;
use game::Game;

#[tokio::main]
async fn main() -> Result<()> {
    play().await
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

        network::start(Arc::clone(&config));

        client.join_game(config.player_race, &config.player_name).await?;

        loop {
            client.step().await?;
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
