use strum_macros::EnumString;
// use WallpaperAPI;

// pub mod WallpaperAPIConf {

pub struct Resolution {
    pub w: i32,
    pub h: i32,
}

#[derive(EnumString)]
pub enum Purity {
    SFW,
    SKETCHY,
    SKETCHY_AND_SFW,
    NSFW,
    ANY
}
pub struct WallpaperAPIConf {
    pub query: String,
    pub min_resolution: Resolution,
    pub categories: String,
    pub purity: Purity,
    pub api_key: String,
}

pub struct WallpaperAPIConfBuilder {
    pub query: String,
    pub min_resolution: Resolution,
    pub categories: String,
    pub purity: Purity,
    pub api_key: String,
}


impl WallpaperAPIConfBuilder {
    pub fn new() -> WallpaperAPIConfBuilder {
        WallpaperAPIConfBuilder {
            query: "".to_string(),
            min_resolution: Resolution { w: 1920, h: 1080 },
            categories: "".to_string(),
            purity: Purity::SFW,
            api_key: "".to_string(),
        }
    }

    pub fn query(self, p: String) -> Self {
        return Self { query: p, ..self };
    }

    pub fn min_resolution(self, p: Resolution) -> Self {
        return Self { min_resolution: p, ..self };
    }

    pub fn categories(self, p: String) -> Self {
        return Self { categories: p, ..self };
    }

    pub fn purity(self, p: Purity) -> Self {
        return Self { purity: p, ..self };
    }

    pub fn api_key(self, p: String) -> Self {
        return Self { api_key: p, ..self };
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
// }
