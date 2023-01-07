use crate::api::{wallpaper_api_config::Resolution, Purity};
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::{error::Error, fs, io};

#[derive(Deserialize, Debug, Clone)]
pub struct Query {
    pub query: String,
    pub categories: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
//#[serde(rename_all="camelCase")] // this could be used if all your keys have the wrong case (from an API for example)
pub struct Config {
    pub api_key: Option<String>,
    pub purity: Purity,
    pub new_picture_delay: usize,
    pub min_resolution: Resolution,
    pub query_data: Vec<Query>,
}

pub async fn load_config() -> Result<Config, Box<dyn Error>> {
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let file_path = exe_dir.join("config.yaml");

    let config_file_string = fs::read_to_string(file_path).map_err(|_| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "Cannot parse config.yaml. Place it in the same directory as the .exe",
        )
    })?;
    let config: Config = serde_yaml::from_str(&config_file_string)?;
    Ok(config)
}

pub fn get_random_query(config: &Config) -> Query {
    let Some(rand_query_data) = config.query_data.choose(&mut rand::thread_rng()) else {
        return Query {
            query: "".to_string(),
            categories: Some("".to_string()),
        };
    };

    rand_query_data.clone()
}
