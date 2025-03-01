use reqwest;
use crate::models::{CurrentWeather, Forecast};
use crate::error::WeatherError;

const API_BASE_URL: &str = "https://api.openweathermap.org/data/2.5";

pub async fn get_current_weather(city: &str, country: &str, api_key: &str) -> Result<CurrentWeather, WeatherError> {
    let url = format!(
        "{}/weather?q={},{}&appid={}&units=metric",
        API_BASE_URL, city, country, api_key
    );
    fetch_data(&url).await
}

pub async fn get_forecast(city: &str, country: &str, api_key: &str) -> Result<Forecast, WeatherError> {
    let url = format!(
        "{}/forecast?q={},{}&appid={}&units=metric",
        API_BASE_URL, city, country, api_key
    );
    fetch_data(&url).await
}

async fn fetch_data<T: serde::de::DeserializeOwned>(url: &str) -> Result<T, WeatherError> {
    let response = reqwest::get(url).await?;
    if response.status().is_success() {
        let data: T = response.json().await?;
        Ok(data)
    } else {
        let status = response.status();
        let error_message = response.text().await?;
        Err(WeatherError::ApiError(format!("API Error: {} - {}", status, error_message)))
    }
}
