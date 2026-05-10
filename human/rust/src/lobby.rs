use anyhow::{bail, Context, Result};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use sc2_proto::common::Race;

use crate::config::Config;

const MAP_URL_BASE: &str = "https://match.superskill.me/maps";

pub async fn prepare(config: &mut Config) -> Result<()> {
    select_race(config)?;

    let path = resolve_sc2_path(&config.sc2_path)?;
    config.sc2_path = path;
    ensure_sc2_version_or_exit(config);
    ensure_map(&config.sc2_path, &config.map_name).await?;
    Ok(())
}

fn ensure_sc2_version_or_exit(config: &Config) {
    let exe = config
        .sc2_path
        .join("Versions")
        .join(&config.sc2_version)
        .join("SC2_x64.exe");

    if !exe.exists() {
        println!(
            "StarCraft II version {} not found. Watch any replay from AI Arena to get it.",
            config.sc2_version
        );
        pause_before_exit();
        std::process::exit(1);
    }
}

pub fn pause_before_exit() {
    print!("\nPress Enter to exit...");
    let _ = io::stdout().flush();
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
}

fn select_race(config: &mut Config) -> Result<()> {
    loop {
        println!("\nSelect your race:");
        println!("1 - Terran");
        println!("2 - Zerg");
        println!("3 - Protoss");
        println!("4 - Random");

        print!("Enter your choice (1-4): ");
        io::stdout().flush().context("failed to flush stdout")?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .context("failed to read input")?;

        match input.trim() {
            "1" => {
                config.player_race = Race::Terran;
                println!("Player race: Terran");
                return Ok(());
            }
            "2" => {
                config.player_race = Race::Zerg;
                println!("Player race: Zerg");
                return Ok(());
            }
            "3" => {
                config.player_race = Race::Protoss;
                println!("Player race: Protoss");
                return Ok(());
            }
            "4" => {
                config.player_race = Race::Random;
                println!("Player race: Random");
                return Ok(());
            }
            _ => {
                println!("Invalid choice. Please enter 1, 2, 3, or 4.");
                continue;
            }
        }
    }
}

fn resolve_sc2_path(configured_path: &Path) -> Result<PathBuf> {
    if configured_path.exists() {
        return Ok(configured_path.to_path_buf());
    }

    println!("Configured SC2 path does not exist: {}", configured_path.display());

    loop {
        print!("Enter your StarCraft II installation path: ");
        io::stdout().flush().context("failed to flush stdout")?;

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .context("failed to read input")?;

        let trimmed = input.trim().trim_matches('"');
        if trimmed.is_empty() {
            println!("Path cannot be empty.");
            continue;
        }

        let candidate = PathBuf::from(trimmed);
        if candidate.exists() {
            return Ok(candidate);
        }

        println!("Path does not exist: {}", candidate.display());
    }
}

async fn ensure_map(path: &Path, map_name: &str) -> Result<()> {
    let maps_dir = path.join("Maps");
    let map_file = format!("{}.SC2Map", map_name);
    let map_path = maps_dir.join(&map_file);

    if map_path.exists() {
        return Ok(());
    }

    println!("Map not found at {}", map_path.display());
    let map_url = format!("{MAP_URL_BASE}/{}.SC2Map", map_name);
    println!("Downloading map from {map_url}...");

    tokio::fs::create_dir_all(&maps_dir)
        .await
        .with_context(|| format!("failed to create maps directory: {}", maps_dir.display()))?;

    let response = reqwest::get(&map_url)
        .await
        .context("failed to request map download")?;
    let response = response
        .error_for_status()
        .context("map download returned error status")?;
    let bytes = response
        .bytes()
        .await
        .context("failed to read map download body")?;

    if bytes.is_empty() {
        bail!("downloaded map file is empty");
    }

    tokio::fs::write(&map_path, &bytes)
        .await
        .with_context(|| format!("failed to save map to {}", map_path.display()))?;

    println!("Map downloaded to {}", map_path.display());
    Ok(())
}
