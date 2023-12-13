use std::collections::HashMap;

#[derive(Clone)]
pub struct Config {
    pub address: String,
    pub port: String,
    pub auth: HashMap<String, String>,
    pub file_dir: String,
    pub tls: Option<TlsConfig>,
}



#[derive(Clone)]
pub struct TlsConfig {
    pub cert_path: String,
    pub key_path: String,
}


impl Config {
    pub fn default_config() -> Config {

        return  Config{
            address: "127.0.0.1".to_string(), //127.0.0.1, 0.0.0.0   127.0.0.1:7151
            port: "7151".to_string(),
            auth: Default::default(),
            file_dir: ".".to_string(),
            tls: None,
        }
    }
}