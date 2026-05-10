#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{fs::File, io::Write, sync::Arc};
use directories::ProjectDirs;
use egui_async::Bind;

use eframe::egui;
use anyhow::{anyhow, Result};

pub async fn run_gui() -> Result<()> {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    let appdata = SC2VsHumanApp{
        is_playing: false,
        bind: Bind::default(),
        app_conf: get_local_saved_data_or_default(),
    };
    save_local_save_data(&appdata.app_conf);
    eframe::run_native(
        "SC2 Human vs Bot",
        options,
        Box::new(|_cc| {
            Ok(Box::new(appdata))
        }),
    ).expect("failed at running the UI");
    Ok(())
}


struct SC2VsHumanApp {
    is_playing: bool,
    bind: Bind<(), anyhow::Error>,
    app_conf: AppConfig,
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
            let sc2path = ui.add(egui::TextEdit::singleline(&mut self.app_conf.sc2path));
            if sc2path.changed(){
                save_local_save_data(&self.app_conf);
            }
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

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
struct AppConfig{
    sc2path: String
}

fn get_local_saved_data_or_default() -> AppConfig{
    if let Some(proj_dir) = ProjectDirs::from("com", "vs_AI", "human_vs_ai"){
        let config_path = proj_dir.cache_dir().join("config.json");
        match File::open(&config_path) {
            Ok(x) => {
                return serde_json::from_reader::<File, AppConfig>(x).expect("Failed to parse config file as JSON")
            },
            Err(err) => {
                eprintln!("Failed to open config file {:?}, reason: {} (Skipping)", config_path, err)
            },
        }
    };

    return AppConfig::default();
}

fn save_local_save_data(data: &AppConfig){
    if let Some(proj_dir) = ProjectDirs::from("com", "vs_AI", "human_vs_ai"){
        let config_dir = proj_dir.cache_dir();
        match std::fs::create_dir_all(&config_dir){
            Ok(_) => {

            },
            Err(err) => {
                eprintln!("Failed to create parent directories of the config file: {:?}, reason: {err}", &config_dir)
            },
        };
        let config_path = config_dir.join("config.json");
        
        match File::create(&config_path){
            Ok(mut file) => {
                match serde_json::to_string_pretty(data){
                    Ok(x) => {
                        match file.write_all(x.as_bytes()){
                            Ok(_) => return,
                            Err(err) => {
                                eprintln!("Failed to write config file to {config_path:?}, reason: {err}")
                            },
                        }
                    },
                    Err(err) => {
                        eprintln!("Failed to stringify the config file, reason: ${err}")
                    },
                }
            },
            Err(err) => {
                eprintln!("Failed to create config file: {config_path:?}, reason: {err}")
            },
        }
    };
    eprintln!("Failed to create config file: skipping");
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
