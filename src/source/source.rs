use async_trait::async_trait;
// use anthill_di::Constructor;

use crate::types::LoadConfigurationResult;


#[async_trait]
pub trait ISource<TConfiguration> where Self: Sync + Send {
    async fn get(&mut self) -> LoadConfigurationResult<TConfiguration>;
}