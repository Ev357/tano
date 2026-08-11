use color_eyre::eyre::{Result, eyre};
use lofty::file::FileType;

pub fn get_format(file_type: &FileType) -> Result<&str> {
    let format = match file_type {
        FileType::Aac => "aac",
        FileType::Aiff => "aiff",
        FileType::Ape => "ape",
        FileType::Flac => "flac",
        FileType::Mpeg => "mpeg",
        FileType::Mp4 => "mp4",
        FileType::Mpc => "mpc",
        FileType::Opus => "opus",
        FileType::Vorbis => "vorbis",
        FileType::Speex => "speex",
        FileType::Wav => "wav",
        FileType::WavPack => "wavpack",
        FileType::Custom(file_name) => file_name,
        _ => return Err(eyre!("Unsupported file type")),
    };

    Ok(format)
}
