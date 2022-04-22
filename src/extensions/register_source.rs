use crate::{source::ISource, ConfigurationSnapshot};

use anthill_di::{DependencyContext, DependencyLifeCycle, types::AddDependencyResult, DependencyBuilder, types::{AsyncCallback, BuildDependencyResult}};
use async_trait::async_trait;

#[async_trait]
pub trait RegisterSourceExtension {
    async fn register_source<TConfiguration: Sync + Send + 'static, TSource: ISource<TConfiguration> + 'static, TClosure: Fn(DependencyContext) -> BuildDependencyResult<TSource> + Sync + Send + 'static>(&mut self, closure: TClosure) -> AddDependencyResult<DependencyBuilder<TSource>>;
    async fn register_source_async<TConfiguration: Sync + Send + 'static, TSource: ISource<TConfiguration> + 'static>(&mut self, closure: AsyncCallback<DependencyContext, BuildDependencyResult<TSource>>) -> AddDependencyResult<DependencyBuilder<TSource>>;
}

#[async_trait]
impl RegisterSourceExtension for DependencyContext {
    async fn register_source<TConfiguration: Sync + Send + 'static, TSource: ISource<TConfiguration> + 'static, TClosure: Fn(DependencyContext) -> BuildDependencyResult<TSource> + Sync + Send + 'static>(&mut self, closure: TClosure) -> AddDependencyResult<DependencyBuilder<TSource>> {
        self.register_closure(closure, DependencyLifeCycle::Transient).await
    }

    async fn register_source_async<TConfiguration: Sync + Send + 'static, TSource: ISource<TConfiguration> + 'static>(&mut self, closure: AsyncCallback<DependencyContext, BuildDependencyResult<TSource>>) -> AddDependencyResult<DependencyBuilder<TSource>> {
        self.register_async_closure(closure, DependencyLifeCycle::Transient).await
    }
}
