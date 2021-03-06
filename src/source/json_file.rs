use std::marker::PhantomData;

use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::types::{LoadConfigurationResult, LoadConfigurationError, SyncConfigurationResult};

use super::ISource;

#[derive(new, Clone)]
pub struct JsonFileConfiguration<TConfiguration> where for<'de> TConfiguration: Deserialize<'de> + Serialize + Default + Sync + Send + 'static {
    path: String,
    create_if_not_exist: bool,
    #[new(default)] pd: PhantomData<TConfiguration>,
}


#[async_trait_with_sync::async_trait(Sync)]
impl<TConfiguration> ISource<TConfiguration> for JsonFileConfiguration<TConfiguration> where for<'de> TConfiguration: Deserialize<'de> + Serialize + Default + Sync + Send + 'static {
    async fn get(&mut self) -> LoadConfigurationResult<TConfiguration> {
        if self.create_if_not_exist && std::fs::metadata(self.path.clone()).is_err() {
            _ = tokio::fs::OpenOptions::new().write(true).create(true).open(self.path.clone()).await.map_err(|e| LoadConfigurationError::IOError(e))?;
            let configuration = serde_json::to_string(&TConfiguration::default()).unwrap();
            tokio::fs::write(self.path.clone(), configuration).await.unwrap();
        }

        let data = tokio::fs::read_to_string(self.path.clone()).await
            .map_err(|err| LoadConfigurationError::IOError(err))?;

        let configuration: TConfiguration = serde_json::from_str(&*data)
            .map_err(|err| LoadConfigurationError::TokioError(err))?;

        return Ok(configuration)
    }

    async fn set(&self, configuration: &TConfiguration) -> SyncConfigurationResult {
        let new_configuration = serde_json::to_string(configuration).map_err(|err| LoadConfigurationError::TokioError(err))?;
        tokio::fs::write(self.path.clone(), new_configuration).await.map_err(|err| LoadConfigurationError::IOError(err))?;
        Ok(())
    }
}