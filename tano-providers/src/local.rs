use color_eyre::eyre::{Result, eyre};
use tano_config::providers::local::LocalConfig;
use tano_database::song::CreateSong;
use tokio::fs;

#[derive(Debug, Clone)]
pub struct LocalProvider {
    pub config: LocalConfig,
}

impl LocalProvider {
    pub fn new(config: LocalConfig) -> Self {
        Self { config }
    }

    pub async fn get_songs(&self) -> Result<Vec<CreateSong>> {
        let mut entries = fs::read_dir(&self.config.path).await?;

        let mut songs: Vec<CreateSong> = vec![];

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let metadata = entry.metadata().await?;

            let title = path
                .file_stem()
                .ok_or(eyre!("Unable to get file stem of: {:?}", path))?
                .to_string_lossy()
                .to_string();

            if metadata.is_file() {
                songs.push(CreateSong {
                    title,
                    provider_id: "local".to_string(),
                    path: path.to_string_lossy().to_string(),
                });
            }
        }

        Ok(songs)
    }
}
