use std::path::PathBuf;

pub fn home_dir() -> tokio::io::Result<PathBuf> {
    std::env::home_dir().ok_or_else(|| {
        tokio::io::Error::new(std::io::ErrorKind::NotFound, "Unable to get home directory")
    })
}

pub fn pulse_directory() -> tokio::io::Result<PathBuf> {
    Ok(home_dir()?.join(".config/pulse-trader"))
}

pub fn pulse_config_directory() -> tokio::io::Result<PathBuf> {
    Ok(home_dir()?.join(".config/pulse-trader/config.toml"))
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct WatchList {
    symbols: Vec<String>,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Config {
    watchlist: WatchList,
}

impl Config {
    pub async fn new() -> tokio::io::Result<Self> {
        let path = pulse_config_directory()?;

        if !path.exists() {
            let default = Self::default();

            tokio::fs::create_dir_all(path.parent().unwrap()).await?;
            tokio::fs::write(&path, default.to_string()?).await?;

            return Ok(default);
        }

        let output = tokio::fs::read_to_string(path).await?;

        Self::from_str(&output)
    }

    pub fn from_str(s: &str) -> tokio::io::Result<Self> {
        toml::from_str(s)
            .map_err(|v| tokio::io::Error::new(std::io::ErrorKind::InvalidInput, v.to_string()))
    }

    pub fn to_string(&self) -> tokio::io::Result<String> {
        toml::to_string(self)
            .map_err(|v| tokio::io::Error::new(std::io::ErrorKind::Other, v.to_string()))
    }
}
