use std::{thread, time::Duration, fs, error::Error};
use yaml_rust::{YamlLoader};
use std::str::FromStr;
use rand::seq::SliceRandom;

use crate::configManager::get_random_query;
mod configManager;
mod api;



// fn load_config() -> Result<yaml_rust::Yaml, Box<dyn Error>> {
//     let config_file_string = fs::read_to_string("config.yaml")?;
//     let docs = YamlLoader::load_from_str(config_file_string.as_str()).unwrap();
//     // Get the first document
//     let doc = &docs[0];
//     // Debug support
//     // println!("{:?}", doc);



//     Ok(doc.to_owned())
// }

//TODO: Move all this query stuff to its own file
// struct Query {
//     query: String, 
//     categories: String, 
// }

// fn get_random_query(config : &yaml_rust::Yaml) -> Query {
//     if let Some(rand_query_data) = config["query_data"].as_vec().unwrap().choose(&mut rand::thread_rng()) {
//         if let Some(map) = rand_query_data.as_hash() {
//             let query = map.get(&yaml_rust::Yaml::String("query".to_string()));
//             let categories = map.get(&yaml_rust::Yaml::String("categories".to_string()));

//             //A query is always required so dont worry about checking that
//             return Query {
//                 query:  query.to_owned().unwrap().as_str().unwrap().to_string(), 
//                 categories: if categories.to_owned().is_none() || categories.to_owned().unwrap().as_str().is_none() {
//                     "".to_owned()
//                 } else{
//                     categories.to_owned().unwrap().as_str().unwrap().to_string()
//                 },
//             };
//         }
//     }
    
//     Query {
//         query: "".to_string(),
//         categories: "".to_string(),
//     }
// }


#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Running");
    let config = if let Ok(config) = configManager::load_config().await {
        config
    }else{
        panic!("Unable to read config.yaml");
    };
    

    loop {
        //For choosing a random query and tags
        let query = get_random_query(&config);

        let wallpaper_url = 
        api::get_wallpaper_url(api::wallpaper_api_config::WallpaperAPIConfBuilder::new()
        .query(query.query.to_owned())
        .categories(query.categories.to_owned())
        .min_resolution(api::wallpaper_api_config::Resolution {
            w: config.min_resolution.w as i32,
            h: config.min_resolution.h as i32,
        })
        .api_key(config.api_key.clone())
        .purity(config.purity)
        .build());


        //Get the wallpaper json and get just the wallpaper element
        let wallpaper = api::get_wallpaper_url_from_request_url(&wallpaper_url).await;

        //Make sure it found a wallpaper, download it if it did, then set it
        if let Some(wallpaper) = wallpaper {
            if let Err(err) = wallpaper.download_file().await {
                //This can occur if the search query you have returns nothing. Or perhaps if you are unlucky
                println!("{}", err)
            }else{
                wallpaper.set_wallpaper();
            }
        }
        
        thread::sleep(Duration::new((config.new_picture_delay as i32).try_into().unwrap(), 0));
    }
    
}
