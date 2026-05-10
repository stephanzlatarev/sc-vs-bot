#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::sync::Arc;
use egui_async::Bind;

use eframe::egui;
use anyhow::{anyhow, Result};

pub async fn run_gui() -> Result<()> {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "SC2 Human vs Bot",
        options,
        Box::new(|_cc| {
            Ok(Box::<SC2VsHumanApp>::default())
        }),
    ).expect("failed at running the UI");
    Ok(())
}


struct SC2VsHumanApp {
    is_playing: bool,
    bind: Bind<(), anyhow::Error>
}

impl Default for SC2VsHumanApp {
    fn default() -> Self {
        Self {
            is_playing: false,
            bind: Bind::default(),
        }
    }
}

impl eframe::App for SC2VsHumanApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 👇 Crucial: Call this once per frame!
        ctx.plugin_or_default::<egui_async::EguiAsyncPlugin>();
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // We use `show_inside` because `ui` already provides a root UI context.
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.label("Click play to start the game");

            if self.is_playing{
                ui.add(egui::Button::new("Starting..."));
                ui.spinner();
                self.bind.read_or_request(|| async {
                    play().await
                });
            } else {
                if ui.button("Start Playing").clicked(){
                    self.is_playing = true
                }
            }
            
        });
    }

}



async fn play() -> Result<()> {
    let mut config = crate::config::Config::new();
    crate::lobby::prepare(&mut config).await?;

    let config = Arc::new(config);
    let mut game = crate::game::Game::new(Arc::clone(&config));
    let mut client = crate::client::Client::new(Arc::clone(&config));

    let result: Result<()> = async {
        game.start().await?;
        client.connect().await?;

        client.create_game().await?;

        let (mut tcp_handle, mut udp_handle) = crate::network::start(Arc::clone(&config));

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
