use self::wallpaper::Wallpaper;
use reqwest::Url;
pub mod wallpaper;
pub mod wallpaper_api_config;

pub fn get_wallpaper_url(args: wallpaper_api_config::WallpaperAPIConf) -> String {
    let base_url = "https://wallhaven.cc/api/v1/search?";

    let purity = match args.purity {
        wallpaper_api_config::Purity::Sfw => "100",
        wallpaper_api_config::Purity::Sketchy => "010",
        wallpaper_api_config::Purity::SketchyAndSfw => "110",
        wallpaper_api_config::Purity::Nsfw => "001",
        wallpaper_api_config::Purity::Any => "111",
    };

    let mut url_parrams = vec![];
    url_parrams.push(("q", args.query.as_str()));
    //Make sure that there is something in categories
    if !args.categories.is_empty() {
        url_parrams.push(("categories", &args.categories));
    }
    let resolution = format!("{}x{}", &args.min_resolution.w, &args.min_resolution.h);
    url_parrams.push(("atleast", &resolution));
    url_parrams.push(("purity", purity));
    url_parrams.push(("apikey", &args.api_key));
    //Make it random so we dont always get the same wallpaper

    url_parrams.push(("sorting", "random"));

    let url = Url::parse_with_params(base_url, url_parrams);


    return if let Ok(url) = url {
        url.to_string()
    } else {
        panic!("Cannot construct URL")
    };
}

pub async fn get_wallpaper_url_from_request_url(request_url: &String) -> Option<Wallpaper> {
    let client = reqwest::Client::new();
    let response = client
        .get(request_url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await
        .unwrap();

    let api_response = match response.status() {
        reqwest::StatusCode::OK => match response.json::<wallpaper::ApiResponse>().await {
            Ok(resp) => resp,
            Err(err) => panic!("Cannot parse json: {:?}", err),
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            panic!("API key Needed");
        }
        other => {
            panic!("Something unexpected happened: {:?}", other);
        }
    };


    if let Some(wallpaper) = api_response.data.get(0) {
        return Some(wallpaper.to_owned());
    }

    None
}
