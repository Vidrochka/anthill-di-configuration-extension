use std::marker::PhantomData;

use async_trait::async_trait;
use derive_new::new;
use serde::Deserialize;

use crate::types::{LoadConfigurationResult, LoadConfigurationError};

use super::ISource;

#[derive(new, Clone)]
pub struct JsonFileConfiguration<TConfiguration> where for<'de> TConfiguration: Deserialize<'de> + Sync + Send + 'static {
    path: String,
    #[new(default)] pd: PhantomData<TConfiguration>,
}

#[async_trait]
impl<TConfiguration> ISource<TConfiguration> for JsonFileConfiguration<TConfiguration> where for<'de> TConfiguration: Deserialize<'de> + Sync + Send + 'static {
    async fn get(&mut self) -> LoadConfigurationResult<TConfiguration> {
        let data = tokio::fs::read_to_string(self.path.clone()).await
            .map_err(|err| LoadConfigurationError::IOError(err))?;

        let configuration: TConfiguration = serde_json::from_str(&*data)
            .map_err(|err| LoadConfigurationError::TokioError(err))?;

        return Ok(configuration)
    }
}