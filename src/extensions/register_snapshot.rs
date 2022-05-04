use crate::{source::ISource, ConfigurationSnapshot};

use anthill_di::{DependencyContext, DependencyLifeCycle, types::AddDependencyResult, DependencyBuilder};

#[async_trait_with_sync::async_trait(Sync)]
pub trait RegisterSnapshotExtension {
    async fn register_snapshot<TConfiguration: Sync + Send + 'static, TSource: ISource<TConfiguration> + 'static>(&mut self, lifecycle: DependencyLifeCycle) -> AddDependencyResult<DependencyBuilder<ConfigurationSnapshot<TConfiguration, TSource>>>;
}

#[async_trait_with_sync::async_trait(Sync)]
impl RegisterSnapshotExtension for DependencyContext {
    async fn register_snapshot<TConfiguration: Sync + Send + 'static, TSource: ISource<TConfiguration> + 'static>(&mut self, lifecycle: DependencyLifeCycle) -> AddDependencyResult<DependencyBuilder<ConfigurationSnapshot<TConfiguration, TSource>>> {
        self.register_type(lifecycle).await
    }
}

