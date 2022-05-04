use crate::source::ISource;

use anthill_di::{DependencyContext, DependencyLifeCycle, types::AddDependencyResult, DependencyBuilder, types::BuildDependencyError};

#[async_trait_with_sync::async_trait(Sync)]
pub trait RegisterConfigurationExtension {
    async fn register_configuration<TConfiguration, TSource>(&mut self, lifecycle: DependencyLifeCycle) -> AddDependencyResult<DependencyBuilder<TConfiguration>>
    where
        TConfiguration: Sync + Send + 'static,
        TSource: ISource<TConfiguration> + 'static + Sync + Send;
}

#[async_trait_with_sync::async_trait(Sync)]
impl RegisterConfigurationExtension for DependencyContext {
    async fn register_configuration<TConfiguration, TSource>(&mut self, lifecycle: DependencyLifeCycle) -> AddDependencyResult<DependencyBuilder<TConfiguration>>
    where
        TConfiguration: Sync + Send + 'static,
        TSource: ISource<TConfiguration> + 'static + Sync + Send
    {
        self.register_async_closure::<TConfiguration, _, _>(move |ctx: DependencyContext| {
            async move {
                let mut source = ctx.resolve::<TSource>().await?;
                let configuration: TConfiguration = source.get().await.map_err(|e| BuildDependencyError::Custom { message: e.to_string() })?;
                return Result::<TConfiguration, BuildDependencyError>::Ok(configuration);
            }
        },lifecycle).await
    }
}
