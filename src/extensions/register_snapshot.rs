use crate::{source::ISource, ConfigurationSnapshot};

use anthill_di::{DependencyContext, DependencyLifeCycle, types::AddDependencyResult, DependencyBuilder};
use async_trait::async_trait;

#[async_trait]
pub trait RegisterSnapshotExtension {
    async fn register_snapshot<TConfiguration: Sync + Send + 'static, TSource: ISource<TConfiguration> + 'static>(&mut self, lifecycle: DependencyLifeCycle) -> AddDependencyResult<DependencyBuilder<ConfigurationSnapshot<TConfiguration, TSource>>>;
}

#[async_trait]
impl RegisterSnapshotExtension for DependencyContext {
    async fn register_snapshot<TConfiguration: Sync + Send + 'static, TSource: ISource<TConfiguration> + 'static>(&mut self, lifecycle: DependencyLifeCycle) -> AddDependencyResult<DependencyBuilder<ConfigurationSnapshot<TConfiguration, TSource>>> {
        self.register_type(lifecycle).await
    }
}

