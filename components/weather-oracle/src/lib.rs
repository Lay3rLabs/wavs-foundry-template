mod trigger;
use trigger::{decode_trigger_event, encode_trigger_output, Destination};
use wavs_wasi_chain::http::{fetch_json, http_request_get};
pub mod bindings;
use crate::bindings::{export, Guest, TriggerAction};
use serde::{Deserialize, Serialize};
use wstd::{http::HeaderValue, runtime::block_on};

struct Component;
export!(Component with_types_in bindings);

impl Guest for Component {
    fn run(action: TriggerAction) -> std::result::Result<Option<Vec<u8>>, String> {
        let (trigger_id, req, dest) =
            decode_trigger_event(action.data).map_err(|e| e.to_string())?;

        // Convert bytes to string and trim null bytes that come from format-bytes32-string
        let zipcode = String::from_utf8(req.clone())
            .map_err(|e| format!("Failed to parse zipcode: {}", e))?
            .trim_end_matches('\0')
            .to_string();

        println!("Received zipcode: {}", zipcode);

        // Validate zip code format (basic check)
        if zipcode.len() < 5 || !zipcode.chars().all(|c| c.is_digit(10)) {
            return Err(format!("Invalid zipcode format: {}", zipcode));
        }

        let res = block_on(async move {
            let weather_data = get_weather_data(&zipcode).await?;
            println!("Weather data: {:?}", weather_data);
            serde_json::to_vec(&weather_data).map_err(|e| e.to_string())
        })?;

        let output = match dest {
            Destination::Ethereum => Some(encode_trigger_output(trigger_id, &res)),
            Destination::CliOutput => Some(res),
        };
        Ok(output)
    }
}

async fn get_weather_data(zipcode: &str) -> Result<WeatherData, String> {
    // Get API key from environment variable
    let api_key = std::env::var("WAVS_ENV_OPENWEATHER_API_KEY")
        .map_err(|e| format!("Failed to get API key: {}", e))?;

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?zip={},us&appid={}&units=imperial",
        zipcode, api_key
    );

    let req = http_request_get(&url).map_err(|e| e.to_string())?;

    // Parse the JSON response
    let response: OpenWeatherResponse = fetch_json(req).await.map_err(|e| e.to_string())?;

    // Convert to our simplified data structure
    Ok(WeatherData {
        location: response.name,
        temperature: response.main.temp,
        description: response.weather.first().map_or("".to_string(), |w| w.description.clone()),
        humidity: response.main.humidity,
        wind_speed: response.wind.speed,
        timestamp: response.dt,
    })
}

// Our simplified weather data structure for output
#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherData {
    location: String,
    temperature: f64,
    description: String,
    humidity: u32,
    wind_speed: f64,
    timestamp: u64,
}

// OpenWeatherMap API response structures
#[derive(Debug, Deserialize)]
struct OpenWeatherResponse {
    name: String,
    main: Main,
    weather: Vec<Weather>,
    wind: Wind,
    dt: u64,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f64,
    humidity: u32,
}

#[derive(Debug, Deserialize)]
struct Weather {
    description: String,
}

#[derive(Debug, Deserialize)]
struct Wind {
    speed: f64,
}
