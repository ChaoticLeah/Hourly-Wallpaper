use std::{thread, time::Duration};

use crate::config_manager::get_random_query;
mod api;
mod config_manager;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Running");
    let config = if let Ok(config) = config_manager::load_config().await {
        config
    } else {
        panic!("Unable to read config.yaml");
    };

    loop {
        //For choosing a random query and tags
        let query = get_random_query(&config);

        let wallpaper_url = api::get_wallpaper_url(
            api::wallpaper_api_config::WallpaperAPIConfBuilder::new()
                .query(query.query.to_owned())
                .categories(query.categories.to_owned())
                .min_resolution(api::wallpaper_api_config::Resolution {
                    w: config.min_resolution.w as i32,
                    h: config.min_resolution.h as i32,
                })
                .api_key(config.api_key.clone())
                .purity(config.purity)
                .build(),
        );

        //Get the wallpaper json and get just the wallpaper element
        let wallpaper = api::get_wallpaper_url_from_request_url(&wallpaper_url).await;

        //Make sure it found a wallpaper, download it if it did, then set it
        if let Some(wallpaper) = wallpaper {
            if let Err(err) = wallpaper.download_file().await {
                //This can occur if the search query you have returns nothing. Or perhaps if you are unlucky
                println!("{}", err)
            } else {
                wallpaper.set_wallpaper();
            }
        }

        thread::sleep(Duration::new(
            (config.new_picture_delay as i32).try_into().unwrap(),
            0,
        ));
    }
}
