use std::fs;

use color_eyre::eyre::Result;
use tano_shared::get_config_file::get_config_file;
use tokio::sync::mpsc;

use crate::{actor::cmd::ConfigCmd, config::Config, utils::get_config_dir::get_config_dir};

pub mod cmd;
pub mod handle;

pub struct ConfigActor {
    receiver: mpsc::Receiver<ConfigCmd>,
}

impl ConfigActor {
    pub fn new(receiver: mpsc::Receiver<ConfigCmd>) -> Self {
        Self { receiver }
    }

    fn handle_command(&mut self, cmd: ConfigCmd) {
        match cmd {
            ConfigCmd::LoadConfig { respond_to } => {
                let _ = respond_to.send(self.load_config());
            }
        }
    }

    pub fn load_config(&self) -> Result<Config> {
        let config_dir = get_config_dir()?;
        let config_path = get_config_file(&config_dir);

        if !config_path.exists() {
            return Ok(Config::default());
        }

        let contents = fs::read_to_string(config_path)?;

        let config: Config = toml::from_str(&contents)?;

        Ok(config)
    }
}

pub async fn run_config_actor(mut actor: ConfigActor) {
    while let Some(cmd) = actor.receiver.recv().await {
        actor.handle_command(cmd);
    }
}
