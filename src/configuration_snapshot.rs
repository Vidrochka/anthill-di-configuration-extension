use crate::{source::ISource, types::SyncConfigurationResult};
use anthill_di::{Constructor, DependencyContext, types::{BuildDependencyResult, BuildDependencyError}};
use async_trait::async_trait;

pub struct ConfigurationSnapshot<TConfiguration, IConfigurationSource> where TConfiguration: Sync + Send + 'static, IConfigurationSource: ISource<TConfiguration> + 'static {
    pub value: TConfiguration,
    source: IConfigurationSource,
}

#[async_trait]
impl<TConfiguration, IConfigurationSource> Constructor for ConfigurationSnapshot<TConfiguration, IConfigurationSource> where TConfiguration: Sync + Send + 'static, IConfigurationSource: ISource<TConfiguration> + 'static {
    async fn ctor(ctx: DependencyContext) ->  BuildDependencyResult<Self> {
        let mut source = ctx.resolve::<IConfigurationSource>().await?;
        let value = source.get().await.map_err(|e| BuildDependencyError::Custom { message: e.to_string() })?;
        Ok(Self{
            value,
            source
        })
    }
}

impl<TConfiguration, IConfigurationSource> ConfigurationSnapshot<TConfiguration, IConfigurationSource> where TConfiguration: Sync + Send + 'static, IConfigurationSource: ISource<TConfiguration> + 'static {
    pub async fn sync(&mut self) -> SyncConfigurationResult {
        let mut value = self.source.get().await?;
        std::mem::swap(&mut self.value, &mut value);
        Ok(())
    }
}