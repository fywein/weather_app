use thiserror::Error;

#[derive(Error, Debug)]
pub enum WeatherError {
    #[error("API request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    
    #[error("API error: {0}")]
    ApiError(String),
    
    #[error("Environment variable not found: {0}")]
    EnvVarError(#[from] std::env::VarError),
}

