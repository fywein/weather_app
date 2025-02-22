use dotenv::dotenv;
use std::env;
use clap::Parser;

mod api;
mod models;
mod cli;
mod error;

use cli::Cli;
use error::WeatherError;

#[tokio::main]
async fn main() -> Result<(), WeatherError> {
    dotenv().ok();
    let api_key = env::var("OPENWEATHERMAP_API_KEY").expect("API key not found");
    
    let args = Cli::parse();
    let weather = api::get_weather(&args.city, &api_key).await?;

    println!("Weather in {}: ", args.city);
    println!("Coordinates: {:.2}°N, {:.2}°E", weather.coord.lat, weather.coord.lon);
    println!("Weather: {}", weather.weather[0].main);
    println!("Description: {}", weather.weather[0].description);
    println!("Temperature: {:.1}°C", weather.main.temp);
    println!("Feels like: {:.1}°C", weather.main.feels_like);
    println!("Min temperature: {:.1}°C", weather.main.temp_min);
    println!("Max temperature: {:.1}°C", weather.main.temp_max);
    println!("Pressure: {} hPa", weather.main.pressure);
    println!("Humidity: {}%", weather.main.humidity);
    println!("Visibility: {} meters", weather.visibility);
    println!("Wind speed: {:.1} m/s", weather.wind.speed);
    println!("Wind direction: {}°", weather.wind.deg);
    println!("Cloudiness: {}%", weather.clouds.all);
    println!("Country: {}", weather.sys.country);
    println!("Sunrise: {}", weather.sys.sunrise);
    println!("Sunset: {}", weather.sys.sunset);

    Ok(())
}
