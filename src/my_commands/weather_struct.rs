use serde_derive::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherData {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: u32,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: i64,
    pub sys: Sys,
    pub timezone: i32,
    pub id: i32,
    pub name: String,
    pub cod: i32,
}
#[derive(Serialize, Deserialize, Debug)]

struct Coord {
    pub lon: f64,
    pub lat: f64,
}
#[derive(Serialize, Deserialize, Debug)]

struct Weather {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String,
}
#[derive(Serialize, Deserialize, Debug)]

struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i32,
    pub humidity: i32,
    pub sea_level: i32,
    pub grnd_level: i32,
}
#[derive(Serialize, Deserialize, Debug)]

struct Wind {
    pub speed: f64,
    pub deg: i32,
    pub gust: f64,
}
#[derive(Serialize, Deserialize, Debug)]

struct Clouds {
    pub all: i32,
}
#[derive(Serialize, Deserialize, Debug)]

struct Sys {
    pub r#type: i32,
    pub id: i32,
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}
