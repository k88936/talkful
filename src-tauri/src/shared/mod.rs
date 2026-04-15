use std::path::PathBuf;

pub fn get_base_path() -> PathBuf {
    #[cfg(not(windows))]
    let base = std::env::var("HOME").unwrap();
    #[cfg(windows)]
    let base = std::env::var("USERPROFILE").unwrap();

    #[cfg(feature = "local_data_dir")]
    let base = ".";

    PathBuf::from(base).join("talkful")
}