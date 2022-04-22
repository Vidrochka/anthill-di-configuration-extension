[![build & tests](https://github.com/Vidrochka/anthill-di-configuration-extension/actions/workflows/rust.yml/badge.svg)](https://github.com/Vidrochka/anthill-di-configuration-extension/actions/workflows/rust.yml)

# anthill-di-configuration-extension
Rust configuration integrated in anthill-di

---

## Warning

Library required Rust nightly for trait as interface (Unsize)

---

## Basic concepts

Always register the configuration source first

At this stage, json loading via serde_json is implemented, but you can extend the functionality by implementing trait IService

``` rust
fn _() {
    let root_context = DependencyContext::new_root()
    root_context.register_source(|_| Ok(JsonFileConfiguration::<Test>::new("configuration_test.json".to_string()))).await.unwrap();
}
```

You can then register a config type or a custom snapshot wrapper

``` rust
fn _() {
    //let root_context = DependencyContext::new_root()
    //root_context.register_source(|_| Ok(JsonFileConfiguration::<Test>::new("configuration_test.json".to_string()))).await.unwrap();

    root_context.register_configuration::<Test, JsonFileConfiguration::<Test>>(DependencyLifeCycle::Transient).await.unwrap();

    let configuration: Test = root_context.resolve().await.unwrap();
}
```

``` rust
fn _() {
    //let root_context = DependencyContext::new_root()
    //root_context.register_source(|_| Ok(JsonFileConfiguration::<Test>::new("configuration_test.json".to_string()))).await.unwrap();

    root_context.register_snapshot::<Test, JsonFileConfiguration::<Test>>(DependencyLifeCycle::Transient).await.unwrap();

    let configuration_snapshot: ConfigurationSnapshot<Test, JsonFileConfiguration::<Test>> = root_context.resolve().await.unwrap();
}
```

To synchronize data with the source, ConfigurationSnapshot has a sync method

``` rust
fn _() {
    //let root_context = DependencyContext::new_root()
    //root_context.register_source(|_| Ok(JsonFileConfiguration::<Test>::new("configuration_test.json".to_string()))).await.unwrap();

    root_context.register_snapshot::<Test, JsonFileConfiguration::<Test>>(DependencyLifeCycle::Transient).await.unwrap();

    let configuration_snapshot: ConfigurationSnapshot<Test, JsonFileConfiguration::<Test>> = root_context.resolve().await.unwrap();

    // we have some configuration_snapshot.value from file
    // now change file
    // configuration_snapshot.value stay the same

    configuration_snapshot.sync().await.unwrap();

    // we have configuration_snapshot.value from changed file
}
```

### See examples present in src/tests folder
