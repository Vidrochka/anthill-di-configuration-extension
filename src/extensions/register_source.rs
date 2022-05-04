use std::future::Future;

use crate::source::ISource;

use anthill_di::{DependencyContext, DependencyLifeCycle, types::AddDependencyResult, DependencyBuilder, types::BuildDependencyResult};

#[async_trait_with_sync::async_trait(Sync)]
pub trait RegisterSourceExtension {
    async fn register_source<TConfiguration: Sync + Send + 'static, TSource: ISource<TConfiguration> + 'static, TClosure: Fn(DependencyContext) -> BuildDependencyResult<TSource> + Sync + Send + 'static>(&mut self, closure: TClosure) -> AddDependencyResult<DependencyBuilder<TSource>>;
    async fn register_source_async<TConfiguration, TSource, TFuture, TClosure>(&mut self, closure: TClosure) -> AddDependencyResult<DependencyBuilder<TSource>>
    where
        TConfiguration: Sync + Send + 'static,
        TSource: ISource<TConfiguration> + 'static,
        TFuture: Future<Output = BuildDependencyResult<TSource>>,
        TFuture: Sync + Send + 'static,
        TClosure: Fn(DependencyContext) -> TFuture,
        TClosure: Sync + Send + 'static;
}


#[async_trait_with_sync::async_trait(Sync)]
impl RegisterSourceExtension for DependencyContext {
    async fn register_source<TConfiguration: Sync + Send + 'static, TSource: ISource<TConfiguration> + 'static, TClosure: Fn(DependencyContext) -> BuildDependencyResult<TSource> + Sync + Send + 'static>(&mut self, closure: TClosure) -> AddDependencyResult<DependencyBuilder<TSource>> {
        self.register_closure(closure, DependencyLifeCycle::Transient).await
    }

    async fn register_source_async<TConfiguration, TSource, TFuture, TClosure>(&mut self, closure: TClosure) -> AddDependencyResult<DependencyBuilder<TSource>>
    where
        TConfiguration: Sync + Send + 'static,
        TSource: ISource<TConfiguration> + 'static,
        TFuture: Future<Output = BuildDependencyResult<TSource>>,
        TFuture: Sync + Send + 'static,
        TClosure: Fn(DependencyContext) -> TFuture,
        TClosure: Sync + Send + 'static,
    {
        self.register_async_closure(closure, DependencyLifeCycle::Transient).await
    }
}
