mod trigger;
use trigger::{decode_trigger_event, encode_trigger_output, Destination};
pub mod bindings;
use crate::bindings::{export, Guest, TriggerAction};
use serde::{Deserialize, Serialize};
use wavs_wasi_chain::http::{fetch_json, http_request_get};
use wstd::{http::HeaderValue, runtime::block_on};

struct Component;
export!(Component with_types_in bindings);

// API key for OpenWeather
const API_KEY: &str = "d031c89489947a1fdc85577bfe698cd7";

// Data structures for the weather API response
#[derive(Debug, Serialize, Deserialize)]
struct WeatherResponse {
    weather: Option<Vec<WeatherInfo>>,
    main: Option<MainInfo>,
    name: Option<String>,
    cod: Option<i32>,
    message: Option<String>,
    timezone: Option<i32>,
    #[serde(rename = "dt")]
    timestamp: Option<i64>,
    wind: Option<WindInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct WeatherInfo {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MainInfo {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct WindInfo {
    speed: f64,
    deg: i32,
}

// Simplified weather response to return to the user
#[derive(Debug, Serialize, Deserialize)]
struct WeatherResult {
    location: String,
    temperature: f64,
    feels_like: f64,
    humidity: i32,
    description: String,
    wind_speed: f64,
    timestamp: i64,
}

impl Guest for Component {
    fn run(action: TriggerAction) -> std::result::Result<Option<Vec<u8>>, String> {
        // Decode the incoming trigger data
        let (trigger_id, req, dest) =
            decode_trigger_event(action.data).map_err(|e| e.to_string())?;

        // Convert bytes to string and parse as a zip code
        let zip_code = std::str::from_utf8(&req).map_err(|e| e.to_string())?;
        println!("Zip code received: {}", zip_code);

        // Check if the zip code is valid (simple check for 5 digits)
        if !zip_code.trim().chars().all(|c| c.is_digit(10)) || zip_code.trim().len() != 5 {
            return Err(format!("Invalid zip code: {}. Must be 5 digits.", zip_code));
        }

        // Get weather data using the OpenWeather API
        let weather_data = block_on(async move { get_weather_by_zip(zip_code.trim()).await })?;

        // Serialize the result to JSON
        let result_json = serde_json::to_vec(&weather_data).map_err(|e| e.to_string())?;

        // Return the result based on destination
        let output = match dest {
            Destination::Ethereum => Some(encode_trigger_output(trigger_id, &result_json)),
            Destination::CliOutput => Some(result_json),
        };

        Ok(output)
    }
}

// Function to fetch weather data from OpenWeather API
async fn get_weather_by_zip(zip_code: &str) -> Result<WeatherResult, String> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?zip={},us&appid={}&units=imperial",
        zip_code, API_KEY
    );

    println!("Fetching weather data from: {}", url);

    // Create the HTTP request
    let mut req = http_request_get(&url).map_err(|e| e.to_string())?;
    req.headers_mut().insert("Accept", HeaderValue::from_static("application/json"));

    // Make the request and fetch the JSON response
    let weather: WeatherResponse = fetch_json(req).await.map_err(|e| e.to_string())?;

    // Check if the API returned an error
    if let Some(code) = weather.cod {
        if code != 200 {
            return Err(format!(
                "API Error: {} (Code: {})",
                weather.message.unwrap_or_else(|| "Unknown error".to_string()),
                code
            ));
        }
    }

    // Create our simplified weather result
    let result = WeatherResult {
        location: weather.name.unwrap_or_else(|| "Unknown".to_string()),
        temperature: weather.main.as_ref().map_or(0.0, |m| m.temp),
        feels_like: weather.main.as_ref().map_or(0.0, |m| m.feels_like),
        humidity: weather.main.as_ref().map_or(0, |m| m.humidity),
        description: weather
            .weather
            .as_ref()
            .and_then(|w| w.first().map(|info| info.description.clone()))
            .unwrap_or_else(|| "Unknown".to_string()),
        wind_speed: weather.wind.as_ref().map_or(0.0, |w| w.speed),
        timestamp: weather.timestamp.unwrap_or(0),
    };

    Ok(result)
}
