use std::{env, sync::OnceLock};

pub struct Config {
    pub log_level: String,
    pub http_port: u16,
    pub sofifa_api_token: String,
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn config() -> &'static Config {
    CONFIG.get_or_init(|| {
        let api_token = match env::var("SOFIFA_API_TOKEN") {
            Ok(v) => v,
            Err(e) => {
                eprintln!("SOFIFA_API_TOKEN: {}", e);
                std::process::exit(1);
            },
        };
        Config { 
            log_level: env::var("LOG_LEVEL").unwrap_or("info".into()),
            http_port: env::var("HTTP_PORT").unwrap_or("3000".into()).parse().unwrap(),
            sofifa_api_token: api_token,
        }
    }) 
}
