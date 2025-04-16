#[allow(warnings)]
mod bindings;
mod trigger;

use bindings::{export, Guest, TriggerAction};
use serde::Deserialize;
use std::env;
use trigger::{decode_trigger_event, encode_data_for_destination, WeatherResult};
use wavs_wasi_chain::http::{fetch_json, http_request_get};
use wstd::runtime::block_on;

// Define OpenWeather API response structure
#[derive(Debug, Deserialize)]
struct WeatherResponse {
    name: String,
    main: Main,
    weather: Vec<Weather>,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f32,
    humidity: u16,
    pressure: u16,
}

#[derive(Debug, Deserialize)]
struct Weather {
    description: String,
}

struct Component;
export!(Component with_types_in bindings);

impl Guest for Component {
    fn run(action: TriggerAction) -> Result<Option<Vec<u8>>, String> {
        // Decode the trigger event to get the zip code
        let (trigger_id, trigger_data, destination) = decode_trigger_event(action.data)?;

        // Debug the actual bytes first
        println!("Raw trigger data bytes: {:?}", &trigger_data);

        // Convert bytes to string and trim null bytes
        let raw_string = String::from_utf8(trigger_data.clone())
            .map_err(|e| format!("Failed to parse zip code: {}", e))?;

        // Get only the actual zip code by trimming the null bytes (0x00)
        let zip_code = raw_string.trim_end_matches('\0');
        println!("Zip code string: '{}'", zip_code);
        println!("Zip code length: {}", zip_code.len());
        println!("Fetching weather for zip code: {}", zip_code);

        // Get API key from environment variable
        let api_key = env::var("WAVS_ENV_OPENWEATHER_API_KEY")
            .map_err(|e| format!("Failed to get API key: {}", e))?;

        // Get the weather data
        let weather_data = block_on(async { fetch_weather(&zip_code, &api_key).await })?;

        // Encode the response based on destination
        let encoded = encode_data_for_destination(trigger_id, weather_data, destination)?;

        Ok(Some(encoded))
    }
}

// Function to fetch weather data from OpenWeather API
async fn fetch_weather(zip_code: &str, api_key: &str) -> Result<WeatherResult, String> {
    // Build URL with zip code and API key - URL encoding the parameters
    let zip_formatted = zip_code.trim();
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?zip={},us&units=metric&appid={}",
        zip_formatted, api_key
    );

    println!("Making request to: {}", url);

    // Create the request
    let req = http_request_get(&url).map_err(|e| format!("Failed to create request: {}", e))?;

    // Make the request and parse JSON response
    let response: WeatherResponse =
        fetch_json(req).await.map_err(|e| format!("Failed to fetch weather data: {}", e))?;

    println!("Received weather data for: {}", response.name);

    // Convert temperature to fixed-point integer (Celsius * 10)
    let temp_fixed = (response.main.temp * 10.0) as i16;

    // Create the weather result
    let result = WeatherResult {
        triggerId: 0, // Will be set correctly by encode_data_for_destination
        location: response.name,
        description: response
            .weather
            .first()
            .map(|w| w.description.clone())
            .unwrap_or_else(|| "Unknown".to_string()),
        temperature: temp_fixed,
        humidity: response.main.humidity,
        pressure: response.main.pressure,
    };

    Ok(result)
}
