use crate::{integrations::GHHeatmapConfig, kindle::get_config_filepath};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadConfigError {
    #[error("failed to load config file in {filepath}: {cause}")]
    LoadFile {
        filepath: PathBuf,
        cause: std::io::Error,
    },
    #[error("failed to parse config file in {filepath}: {cause}")]
    ParseFile {
        filepath: PathBuf,
        cause: toml::de::Error,
    },
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    pub gh_heatmap: Option<GHHeatmapConfig>,
}

impl Config {
    fn load_from_file(filepath: PathBuf) -> Result<Self, LoadConfigError> {
        let file_content =
            std::fs::read_to_string(&filepath).map_err(|cause| LoadConfigError::LoadFile {
                filepath: filepath.clone(),
                cause,
            })?;

        toml::de::from_str(&file_content)
            .map_err(|cause| LoadConfigError::ParseFile { filepath, cause })
    }

    pub fn load() -> Result<Self, LoadConfigError> {
        Self::load_from_file(get_config_filepath())
    }
}
