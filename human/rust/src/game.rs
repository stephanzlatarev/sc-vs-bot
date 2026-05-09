use anyhow::Result;
use std::process::Stdio;
use std::sync::Arc;
use tokio::process::{Child, Command};

use crate::{config::Config, lobby::get_sc2_version_path};

pub struct Game {
    config: Arc<Config>,
    process: Option<Child>,
}

impl Game {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            process: None,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        println!("Starting StarCraft II...");

        let cwd = self.config.sc2_path.join("Support64");
        let exe = get_sc2_version_path(&self.config);
        // Note this doesn't change from linux to windows
        // But later config may prove that wine is required
        let child =  if cfg!(target_os = "windows"){
            Command::new(&exe)
                .args([
                    "-dataVersion",
                    "B89B5D6FA7CBF6452E721311BFBC6CB2",
                    "-displaymode",
                    "1",
                    "-listen",
                    &self.config.local_host,
                    "-port",
                    &self.config.sc2_port.to_string(),
                ])
                .current_dir(&cwd)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()?
        } else if cfg!(target_os = "linux"){
             Command::new(&exe)
                .args([
                    "-dataVersion",
                    "B89B5D6FA7CBF6452E721311BFBC6CB2",
                    "-displaymode",
                    "1",
                    "-listen",
                    &self.config.local_host,
                    "-port",
                    &self.config.sc2_port.to_string(),
                ])
                .current_dir(&cwd)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()?
        } else {
            unreachable!("Unsupported OS")
        };
        self.process = Some(child);
            Ok(())
    }

    pub async fn stop(&mut self) {
        if let Some(child) = &mut self.process {
            let _ = child.kill().await;
            let _ = child.wait().await;
        }
        self.process = None;
    }
}
