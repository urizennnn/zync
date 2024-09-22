pub mod app_config_variables {
    use serde::{Deserialize, Serialize};
    use std::collections::BTreeMap;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct App {
        #[serde(rename = "APP_NAME")]
        pub name: String,
        #[serde(rename = "APP_VERSION")]
        pub version: String,
        #[serde(rename = "APP_KEY")]
        pub key: String,
        #[serde(rename = "APP_SECRET")]
        pub secret: String,
        #[serde(rename = "CLIENT_ID")]
        pub client_id: String,
        #[serde(rename = "CLIENT_SUMMARY")]
        pub summary: Files,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Files {
        pub data: BTreeMap<String, String>,
    }

    pub trait NewTrait {
        fn new() -> Self;

        fn insert(&mut self, key: String, value: String);

        fn get(&self, key: &str) -> Option<&String>;
    }

    impl NewTrait for Files {
        fn new() -> Self {
            Files {
                data: BTreeMap::new(),
            }
        }

        fn insert(&mut self, key: String, value: String) {
            self.data.insert(key, value);
        }

        fn get(&self, key: &str) -> Option<&String> {
            self.data.get(key)
        }
    }
}
