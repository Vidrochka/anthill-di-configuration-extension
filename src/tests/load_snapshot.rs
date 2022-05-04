use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
struct Test {
    pub test_text: String,
}

#[tokio::test]
async fn load_snapshot() {
    use anthill_di::{DependencyContext, DependencyLifeCycle};
    use crate::{
        extensions::{RegisterSnapshotExtension, RegisterSourceExtension},
        source::JsonFileConfiguration,
        ConfigurationSnapshot
    };
    use std::{fs::File, io::Write};
    
    let mut output = File::create("load_snapshot.json").unwrap();
    write!(output, "{{ \"test_text\": \"test_value\" }}").unwrap();


    let mut root_context = DependencyContext::new_root();
    root_context.register_source(|_| Ok(JsonFileConfiguration::<Test>::new("load_snapshot.json".to_string()))).await.unwrap();
    root_context.register_snapshot::<Test, JsonFileConfiguration::<Test>>(DependencyLifeCycle::Transient).await.unwrap();
    
    let configuration_snapshot = root_context.resolve::<ConfigurationSnapshot<Test, JsonFileConfiguration::<Test>>>().await.unwrap();

    std::fs::remove_file("load_snapshot.json").unwrap();
    assert_eq!(configuration_snapshot.value.test_text, "test_value".to_string());
}