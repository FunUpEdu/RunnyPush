use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Info {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SunnyData {
    pub count: String,
    pub am_count: String,
    pub average_speed: String,
}

#[derive(Serialize, Deserialize)]
pub struct SunnyList {
    data: Vec<Sunny>,
}

#[derive(Serialize, Deserialize)]
pub struct Sunny {
    pub time: String,
    pub period: String,
    pub meters: String,
    pub speed: String,
    pub ok: bool,
}
