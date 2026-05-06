use anyhow::Result;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::{Child, Command};

use crate::config::{PATH, PORT, VERSION};

pub struct Game {
    process: Option<Child>,
}

impl Game {
    pub fn new() -> Self {
        Self { process: None }
    }

    pub async fn start(&mut self) -> Result<()> {
        println!("Starting StarCraft II...");

        let cwd = PathBuf::from(PATH).join("Support64");
        let exe = PathBuf::from(PATH)
            .join("Versions")
            .join(VERSION)
            .join("SC2_x64.exe");

        let child = Command::new(&exe)
            .args([
                "-dataVersion",
                "B89B5D6FA7CBF6452E721311BFBC6CB2",
                "-displaymode",
                "1",
                "-listen",
                "127.0.0.1",
                "-port",
                &PORT.to_string(),
            ])
            .current_dir(&cwd)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;

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
