use self::wallpaper::Wallpaper;
pub mod wallpaper_api_config;
pub mod wallpaper;



pub fn get_wallpaper_url(args: wallpaper_api_config::WallpaperAPIConf) -> String {
    let base_url = "https://wallhaven.cc/api/v1/";

    let mut search = String::from("search?");
    
    if args.categories != "" {
        search.push_str(&format!("categories={}&", args.categories));
    }

    search.push_str(&format!("q={}&", args.query));
    
    search.push_str(&format!("atleast={}&", format!("{}x{}", args.min_resolution.w, args.min_resolution.h)));
 
    let purity = match args.purity {
        wallpaper_api_config::Purity::SFW => {
            "100"
        },
        wallpaper_api_config::Purity::SKETCHY => {
            "010"
        },
        wallpaper_api_config::Purity::SKETCHY_AND_SFW => {
            "110"
        },
        wallpaper_api_config::Purity::NSFW => {
            "001"
        },
        wallpaper_api_config::Purity::ANY => {
            "111"
        },
    };

    search.push_str(&format!("purity={}&", purity));

    search.push_str(&format!("apikey={}&", args.api_key));

 
    //Make it random so we dont always get the same wallpaper
    search.push_str("sorting=random");

    let mut url: String = base_url.to_owned();
    url.push_str(search.as_str());
    return url;
}

pub async fn get_wallpaper_url_from_request_url(request_url : &String) -> Option<Wallpaper>{
    let client = reqwest::Client::new();
    let response = client.get(request_url)
    .header(reqwest::header::CONTENT_TYPE, "application/json")
    .header(reqwest::header::ACCEPT, "application/json")
    .send()
    .await
    .unwrap();

    let api_response = match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<wallpaper::ApiResponse>().await {
                Ok(resp) => resp,
                Err(err) => panic!("Cannot parse json: {:?}", err),
            }
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            panic!("API key Needed");
        },
        other => {
            panic!("Something unexpected happened: {:?}", other);
        }
    };


    if let Some(wallpaper) = api_response.data.get(0) {
        return Some(wallpaper.to_owned());
    }

    return None;
}
