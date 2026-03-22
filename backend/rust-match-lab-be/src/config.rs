use std::env;

pub struct Config {
    pub log_level: String,
    pub http_port: u16
}

impl Config {
    fn new(log_level: String, http_port: u16) -> Self {
        Config { log_level, http_port }
    }

    pub fn builder() -> Self {
        let log_level = match env::var("LOG_LEVEL") {
            Ok(v) => v,
            Err(_) => "info".into(),
        };
        let http_port = match env::var("HTTP_PORT") {
            Ok(v) => v,
            Err(_) => "3000".into(),
        };

        Config::new(log_level, http_port.parse().unwrap())
    }
}