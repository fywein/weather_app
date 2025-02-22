use reqwest;
use crate::models::WeatherData;
use crate::error::WeatherError;

pub async fn get_weather(city: &str, api_key: &str) -> Result<WeatherData, WeatherError> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city, api_key
    );

    let response = reqwest::get(&url).await?;
    if response.status().is_success() {
        let weather_data: WeatherData = response.json().await?;
        Ok(weather_data)
    } else {
        Err(WeatherError::ApiError(response.status().to_string()))
    }
}
