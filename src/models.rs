use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WeatherData {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: i32,
    pub wind: Wind,
    pub clouds: Clouds,
    pub dt: i64,
    pub sys: Sys,
    pub timezone: i32,
    pub id: i32,
    pub name: String,
    pub cod: i32,
}

#[derive(Deserialize, Debug)]
pub struct Coord {
    pub lon: f32,
    pub lat: f32,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub id: i32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize, Debug)]
pub struct Main {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: i32,
    pub humidity: i32,
}

#[derive(Deserialize, Debug)]
pub struct Wind {
    pub speed: f32,
    pub deg: i32,
}

#[derive(Deserialize, Debug)]
pub struct Clouds {
    pub all: i32,
}

#[derive(Deserialize, Debug)]
pub struct Sys {
    pub r#type: i32,
    pub id: i32,
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}
