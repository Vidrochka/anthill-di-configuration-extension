use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
struct Test {
    pub test_text: String,
}

impl Default for Test {
    fn default() -> Self {
        Self { test_text: "test_value".to_string() }
    }
}

#[tokio::test]
async fn create_if_not_exist_file() {
    use anthill_di::{DependencyContext, DependencyLifeCycle};
    use crate::{
        extensions::{RegisterConfigurationExtension, RegisterSourceExtension},
        source::JsonFileConfiguration,
    };
    use std::{fs::File, io::Write};


    let mut root_context = DependencyContext::new_root();
    root_context.register_source(|_| Ok(JsonFileConfiguration::<Test>::new("create_if_not_exist_file.json".to_string(), true))).await.unwrap();
    root_context.register_configuration::<Test, JsonFileConfiguration::<Test>>(DependencyLifeCycle::Transient).await.unwrap();
    
    let configuration = root_context.resolve::<Test>().await.unwrap();

    std::fs::remove_file("create_if_not_exist_file.json").unwrap();
    assert_eq!(configuration.test_text, "test_value".to_string());
}