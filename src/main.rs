use dotenv::dotenv;
use std::env;
use clap::Parser;
use chrono::{DateTime, TimeZone, Utc};

mod api;
mod models;
mod cli;
mod error;

use cli::Cli;
use error::WeatherError;

#[tokio::main]
async fn main() -> Result<(), WeatherError> {
    dotenv().ok();
    let api_key = env::var("OPENWEATHERMAP_API_KEY").map_err(|_| WeatherError::ApiKeyNotFound)?;

    let args = Cli::parse();
    let city = args.city;
    let country = args.country;

    // API에서 현재 날씨 정보 가져오기
    let weather = api::get_current_weather(&city, &country, &api_key).await?;

    // 5일 예보 가져오기
    let forecast = api::get_forecast(&city, &country, &api_key).await?;

    // 현재 날씨 정보 출력
    println!("현재 날씨 ({}):", weather.name);
    println!("  날씨: {}", weather.weather[0].description);
    println!("  기온: {:.1}°C (체감: {:.1}°C)", weather.main.temp, weather.main.feels_like);
    println!("  습도: {}%", weather.main.humidity);
    println!("  바람: {:.1} m/s, {}°", weather.wind.speed, weather.wind.deg);
    println!("  국가: {}", weather.sys.country);

    // 5일 예보 정보 출력
    println!("\n5일 예보:");
    for item in forecast.list.iter().take(5) {
        let date = DateTime::<Utc>::from_timestamp(item.dt, 0).unwrap();
        println!("  날짜: {}", date.format("%Y-%m-%d %H:%M:%S"));
        println!("    날씨: {}", item.weather[0].description);
        println!("    기온: {:.1}°C (체감: {:.1}°C)", item.main.temp, item.main.feels_like);
        println!("    바람: {:.1} m/s, {}°", item.wind.speed, item.wind.deg);
        println!("---");
    }

    Ok(())
}
