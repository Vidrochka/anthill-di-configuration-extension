use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoadConfigurationError {
    #[error("Configuration load error: [{0}]")]
    IOError(std::io::Error),
    #[error("Configuration deserialize error: [{0}]")]
    TokioError(serde_json::Error)
}

pub type LoadConfigurationResult<T> = Result<T, LoadConfigurationError>;

pub type SyncConfigurationResult = Result<(), LoadConfigurationError>;