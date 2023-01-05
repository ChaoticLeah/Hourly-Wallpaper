use std::{env, error::Error};
use tokio::{fs::File, io::AsyncWriteExt};
use reqwest::Client;
use serde::{Serialize, Deserialize};
use wallpaper;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Thumb {
    large: String,
    original: String,
    small: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde (rename = "")]
pub struct Wallpaper {
    id: String,
    url: String,
    short_url: String,
    views: i32,
    favorites: i32,
    source: String,
    purity: String,
    category: String,
    dimension_x: i32,
    dimension_y: i32,
    resolution: String,
    ratio: String,
    file_size: i32,
    file_type: String,
    created_at: String,
    colors: Vec<String>,
    pub path: String,
    thumbs: Thumb,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    #[serde (rename = "data")]
    pub data: Vec<Wallpaper>
}


impl Wallpaper {


    pub async fn download_file(&self) -> Result<(), Box<dyn Error>> {
        let client = Client::new();
        let mut file : File = if let Some(temp_dir) = env::temp_dir().as_os_str().to_str() {
            File::create(&format!("{}wallpaper.png", temp_dir)).await?
        }else{
            return Err("Failed to create file in temp dir".into());
        };

        let resp = client.get(&self.path).send().await?;
        let bytes = resp.bytes().await?;
        file.write_all(&bytes).await?;
        

        Ok(())
    }
    
    pub fn set_wallpaper(&self) {
        if let Some(temp_dir) = env::temp_dir().as_os_str().to_str() {
            wallpaper::set_from_path(&format!("{}wallpaper.png", temp_dir)).unwrap();
        }
        
        wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
    }


}