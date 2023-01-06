use self::{status_error::StatusError, wallpaper::Wallpaper};
use reqwest::Url;
use std::error::Error;
pub mod status_error;
pub mod wallpaper;
pub mod wallpaper_api_config;
pub use self::wallpaper_api_config::Purity;

pub fn get_wallpaper_url(
    args: wallpaper_api_config::WallpaperAPIConf,
) -> Result<String, Box<dyn Error>> {
    let base_url = "https://wallhaven.cc/api/v1/search?";

    let purity = match args.purity {
        Purity::Sfw => "100",
        Purity::Sketchy => "010",
        Purity::SketchyAndSfw => "110",
        Purity::Nsfw => "001",
        Purity::Any => "111",
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
        Ok(url.to_string())
    } else {
        Err("Cannot construct URL".into())
    };
}

pub async fn get_wallpaper_url_from_request_url(
    request_url: &String,
) -> Result<Option<Wallpaper>, Box<dyn Error>> {
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
            Ok(resp) => Ok(resp),
            Err(err) => Err(StatusError::new(&err.to_string())),
        },
        reqwest::StatusCode::UNAUTHORIZED => Err(StatusError::new("API key needed")),
        other => Err(StatusError::new(&format!(
            "Something unexpected happened: {:?}",
            other
        ))),
    };

    if let Some(wallpaper) = api_response?.data.get(0) {
        return Ok(Some(wallpaper.to_owned()));
    }

    Ok(None)
}
