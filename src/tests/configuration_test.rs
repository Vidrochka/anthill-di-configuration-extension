use serde::Deserialize;


#[derive(Deserialize)]
struct Test {
    pub test_text: String,
}

#[tokio::test]
async fn snapshot_test() {
    use anthill_di::{DependencyContext, DependencyLifeCycle};
    use crate::{
        extensions::{RegisterConfigurationExtension, RegisterSourceExtension},
        source::JsonFileConfiguration,
    };
    use std::{fs::File, io::Write};
    
    let mut output = File::create("configuration_test.json").unwrap();
    write!(output, "{{ \"test_text\": \"test_value\" }}").unwrap();


    let mut root_context = DependencyContext::new_root();
    root_context.register_source(|_| Ok(JsonFileConfiguration::<Test>::new("configuration_test.json".to_string()))).await.unwrap();
    root_context.register_configuration::<Test, JsonFileConfiguration::<Test>>(DependencyLifeCycle::Transient).await.unwrap();
    
    let configuration = root_context.resolve::<Test>().await.unwrap();

    std::fs::remove_file("configuration_test.json").unwrap();
    assert_eq!(configuration.test_text, "test_value".to_string());
}