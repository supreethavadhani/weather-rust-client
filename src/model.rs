//
// Check out `quicktype`.
//   On GitHub: https://github.com/quicktype/quicktype
//   In action: https://app.quicktype.io
//

use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct HelloResponse {
    response: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    coord: Coord,
    weather: Vec<WeatherElement>,
    base: String,
    main: Main,
    visibility: i64,
    wind: Wind,
    clouds: Clouds,
    dt: i64,
    sys: Sys,
    timezone: i64,
    id: i64,
    name: String,
    cod: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Clouds {
    all: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i64,
    humidity: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sys {
    #[serde(rename = "type")]
    sys_type: i64,
    id: i64,
    country: String,
    sunrise: i64,
    sunset: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherElement {
    id: i64,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Wind {
    speed: f64,
    deg: i64,
    gust: f64
}