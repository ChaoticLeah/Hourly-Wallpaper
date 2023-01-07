use serde::Deserialize;
use strum_macros::EnumString;

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct Resolution {
    pub w: i32,
    pub h: i32,
}

#[derive(EnumString, Deserialize, Debug, Copy, Clone)]
// #[strum(ascii_case_insensitive)]
pub enum Purity {
    Sfw,
    Sketchy,
    SketchyAndSfw,
    Nsfw,
    Any,
}
#[derive(Deserialize, Debug)]
pub struct WallpaperAPIConf {
    pub query: String,
    pub min_resolution: Resolution,
    pub categories: Option<String>,
    pub purity: Purity,
    pub api_key: Option<String>,
}

pub struct WallpaperAPIConfBuilder {
    pub query: String,
    pub min_resolution: Resolution,
    pub categories: Option<String>,
    pub purity: Purity,
    pub api_key: Option<String>,
}

impl WallpaperAPIConfBuilder {
    pub fn new() -> WallpaperAPIConfBuilder {
        WallpaperAPIConfBuilder {
            query: "".to_string(),
            min_resolution: Resolution { w: 1920, h: 1080 },
            categories: None,
            purity: Purity::Sfw,
            api_key: None,
        }
    }

    pub fn query(self, p: String) -> Self {
        Self { query: p, ..self }
    }

    pub fn min_resolution(self, p: Resolution) -> Self {
        Self {
            min_resolution: p,
            ..self
        }
    }

    pub fn categories(self, c: Option<String>) -> Self {
        Self {
            categories: c,
            ..self
        }
    }

    pub fn purity(self, p: Purity) -> Self {
        Self { purity: p, ..self }
    }

    pub fn api_key(self, api_key: Option<String>) -> Self {
        Self {
            api_key: api_key,
            ..self
        }
    }

    pub fn build(self) -> WallpaperAPIConf {
        WallpaperAPIConf {
            query: self.query,
            min_resolution: self.min_resolution,
            categories: self.categories,
            purity: self.purity,
            api_key: self.api_key,
        }
    }
}
