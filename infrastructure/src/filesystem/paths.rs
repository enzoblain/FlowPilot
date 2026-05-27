use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;
use tracing::info;

use crate::config::Config;
use crate::filesystem::error::PathsError;

pub struct Paths {
    root: PathBuf,
    config: PathBuf,
    logs: PathBuf,
}

impl Paths {
    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    pub fn config(&self) -> &PathBuf {
        &self.config
    }

    pub fn logs(&self) -> &PathBuf {
        &self.logs
    }

    pub fn load() -> Result<Self, PathsError> {
        let project_dirs = ProjectDirs::from("com", "enzoblain", "FlowPilot")
            .ok_or(PathsError::ProjectDirsNotFound)?;

        let root = project_dirs.data_dir().to_path_buf();
        let config = root.join("config.json");
        let logs = root.join("logs");

        info!("Creating FlowPilot directories");

        fs::create_dir_all(&root).map_err(|_| PathsError::RootDirectoryCreationFailed)?;
        fs::create_dir_all(&logs).map_err(|_| PathsError::RootDirectoryCreationFailed)?;

        Ok(Self { root, config, logs })
    }

    pub fn config_exists(&self) -> bool {
        self.config.exists()
    }

    pub fn is_first_launch(&self) -> bool {
        !self.config_exists()
    }

    pub fn load_config(&self) -> Result<Config, PathsError> {
        if !self.config_exists() {
            return Err(PathsError::ConfigNotFound);
        }

        info!("Loading FlowPilot configuration");
        let content = fs::read_to_string(&self.config).map_err(|_| PathsError::ConfigReadFailed)?;

        serde_json::from_str(&content).map_err(|_| PathsError::InvalidConfig)
    }

    pub fn save_config(&self, config: &Config) -> Result<(), PathsError> {
        info!("Saving FlowPilot configuration");

        let content = serde_json::to_string_pretty(config)
            .map_err(|_| PathsError::ConfigSerializationFailed)?;

        fs::write(&self.config, content).map_err(|_| PathsError::ConfigWriteFailed)?;

        Ok(())
    }

    pub fn delete_config(&self) -> Result<(), PathsError> {
        if !self.config_exists() {
            return Err(PathsError::ConfigNotFound);
        }

        info!("Deleting FlowPilot configuration");
        fs::remove_file(&self.config).map_err(|_| PathsError::ConfigDeleteFailed)?;

        Ok(())
    }

    pub fn delete_all(&self) -> Result<(), PathsError> {
        if !self.root.exists() {
            return Err(PathsError::ProjectDirsNotFound);
        }

        info!("Deleting FlowPilot root directory");
        fs::remove_dir_all(&self.root).map_err(|_| PathsError::RootDirectoryDeletionFailed)?;

        Ok(())
    }
}
