mod client;
mod config;
mod game;
mod network;

use anyhow::Result;
use sc2_proto::common::Race;

use client::Client;
use game::Game;

#[tokio::main]
async fn main() -> Result<()> {
    play().await
}

async fn play() -> Result<()> {
    let mut game = Game::new();
    let mut client = Client::new();

    let result: Result<()> = async {
        network::start();

        game.start().await?;
        client.connect().await?;

        client.create_game().await?;
        client.join_game(Race::Protoss, "Human").await?;

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
