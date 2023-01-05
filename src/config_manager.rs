use std::{error::Error, fs};

use rand::seq::SliceRandom;
use serde::Deserialize;

use crate::{api::Purity};

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct Resolution {
    pub w: usize,
    pub h: usize,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Query {
    pub query: String, 
    pub categories: String, 
}


#[derive(Deserialize, Debug)]
//#[serde(rename_all="camelCase")] // this could be used if all your keys have the wrong case (from an API for example)
pub struct Config {
    #[serde(rename="apiKey")]
    pub api_key: String,
    pub purity: Purity, // your enum
    pub new_picture_delay: usize, // you can mark keys as optional using an `Option` if you want
    pub min_resolution: Resolution,
    pub query_data: Vec<Query>, // you already have a Query struct defined, just need to derive Deserialize on it
}

pub async fn load_config() -> Result<Config, Box<dyn Error>> {
    let config_file_string = fs::read_to_string("config.yaml")?;
    let config: Config = serde_yaml::from_str(&config_file_string)?; // you need to add serde_yaml as a dependency
    Ok(config)
}

pub fn get_random_query(config: &Config) -> Query {
    
    let Some(rand_query_data) = config.query_data.choose(&mut rand::thread_rng()) else { // neat let-else syntax (available since rust 1.65)
        return Query {
            query: "".to_string(),
            categories: "".to_string(),
        };
    };

    rand_query_data.clone()
}