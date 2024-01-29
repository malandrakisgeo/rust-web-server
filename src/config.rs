use std::collections::HashMap;

#[derive(Clone)]
pub struct Config {
    pub address: String,
    pub port: String,
    pub auth: HashMap<String, String>,
    pub file_dir: String,
    pub tls: Option<TlsConfig>,
    pub max_cache_files: usize,
    pub largest_cacheable_file_size: usize,
    pub active_threads: usize,
}



#[derive(Clone)]
pub struct TlsConfig {
    pub cert_path: String,
    pub key_path: String,
}


impl Config {
    pub fn default_config() -> Config {

        return  Config{
            address: "0.0.0.0".to_string(), //127.0.0.1:7151 , 0.0.0.0 for LAN visibility
            port: "7151".to_string(),
            active_threads: 5,
            auth: Default::default(),
            file_dir: ".".to_string(),
            tls: None,
            max_cache_files: 0,
            largest_cacheable_file_size: 0 //100MB in bytes
        }
    }
}
