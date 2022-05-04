use crate::types::{LoadConfigurationResult, SyncConfigurationResult};



#[async_trait_with_sync::async_trait(Sync)]
pub trait ISource<TConfiguration> where Self: Sync + Send {
    async fn get(&mut self) -> LoadConfigurationResult<TConfiguration>;
    async fn set(&self, configuration: &TConfiguration) -> SyncConfigurationResult;
}