use color_eyre::eyre::Result;
use tano_database::song::CreateSong;
use tano_providers::local::LocalProvider;
use tokio::task::JoinSet;

pub async fn get_all_songs(local_providers: Vec<LocalProvider>) -> Result<Vec<CreateSong>> {
    let mut set = JoinSet::new();

    for provider in local_providers {
        set.spawn(async move { provider.get_songs().await });
    }

    let mut songs: Vec<CreateSong> = vec![];

    while let Some(songs_result) = set.join_next().await {
        songs.extend(songs_result??);
    }

    Ok(songs)
}
