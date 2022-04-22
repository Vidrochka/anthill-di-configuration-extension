use crate::{source::ISource, ConfigurationSnapshot};

use anthill_di::{DependencyContext, DependencyLifeCycle, types::AddDependencyResult, DependencyBuilder, types::{AsyncCallback, BuildDependencyResult, BuildDependencyError}};
use async_trait::async_trait;

#[async_trait]
pub trait RegisterConfigurationExtension {
    async fn register_configuration<TConfiguration: Sync + Send + 'static, TSource: ISource<TConfiguration> + 'static>(&mut self, lifecycle: DependencyLifeCycle) -> AddDependencyResult<DependencyBuilder<TConfiguration>>;
}

#[async_trait]
impl RegisterConfigurationExtension for DependencyContext {
    async fn register_configuration<TConfiguration: Sync + Send + 'static, TSource: ISource<TConfiguration> + 'static>(&mut self, lifecycle: DependencyLifeCycle) -> AddDependencyResult<DependencyBuilder<TConfiguration>> {
        self.register_async_closure(Box::new(move |ctx: DependencyContext| {
            Box::pin (async move {
                let mut source = ctx.resolve::<TSource>().await?;
                let configuration = source.get().await.map_err(|e| BuildDependencyError::Custom { message: e.to_string() })?;
                return Ok(configuration);
            })
        }),lifecycle).await
    }
}
