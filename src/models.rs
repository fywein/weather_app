use serde::Deserialize;

// 현재 날씨 데이터
#[derive(Deserialize, Debug, Clone)]
pub struct CurrentWeather {
    pub main: Main,
    pub weather: Vec<WeatherDescription>,
    pub wind: Wind,
    pub sys: Sys,
    pub name: String, // 도시 이름 추가
}

#[derive(Deserialize, Debug, Clone)]
pub struct Main {
    pub temp: f32, // 섭씨 온도
    pub feels_like: f32,
    pub humidity: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WeatherDescription {
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Wind {
    pub speed: f32, // m/s
    pub deg: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Sys {
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}

// 5일 예보 데이터
#[derive(Deserialize, Debug, Clone)]
pub struct Forecast {
    pub list: Vec<ForecastItem>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ForecastItem {
    pub dt: i64,
    pub main: Main,
    pub weather: Vec<WeatherDescription>,
    pub wind: Wind,
}
