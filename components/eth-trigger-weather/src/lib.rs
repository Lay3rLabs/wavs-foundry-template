mod trigger;
use layer_wasi::{
    bindings::world::{Guest, TriggerAction},
    export_layer_trigger_world,
    wasi::{Request, WasiPollable},
};
use serde::{Deserialize, Serialize};
use trigger::{decode_trigger_event, encode_trigger_output};
use wstd::runtime::{block_on, Reactor};

struct Component;

impl Guest for Component {
    fn run(trigger_action: TriggerAction) -> std::result::Result<Vec<u8>, String> {
        let (trigger_id, req) =
            decode_trigger_event(trigger_action.data).map_err(|e| e.to_string())?;

        let input = std::str::from_utf8(&req)
            .map_err(|_| "Invalid UTF-8 input".to_string())?;

        let coordinates: Vec<&str> = input.split(',').collect();
        if coordinates.len() != 2 {
            return Err("Input must be in the format of latitude,longitude".to_string());
        }

        let (latitude, longitude) = (coordinates[0], coordinates[1]);

        let res = block_on(move |reactor| async move {
            let weather_data = get_weather(&reactor, latitude, longitude).await?;
            let v = CurrentTemperatureResponse{
                latitude: weather_data.latitude,
                longitude: weather_data.longitude,
                temperature: weather_data.current.temperature_2m,
                time: weather_data.current.time,
            };
            Ok(Vec::<u8>::from(v))
        });

        match res {
            Ok(data) => Ok(encode_trigger_output(trigger_id, &data)),
            Err(e) => Err(e),
        }
    }
}

async fn get_weather(
    reactor: &Reactor,
    lat: impl Into<String>,
    lon: impl Into<String>,
) -> Result<WeatherResponse, String> {
    let url: &str = "https://api.open-meteo.com/v1/forecast";
    let params = [
        ("latitude", lat.into()),
        ("longitude", lon.into()),
        ("current", "temperature_2m".to_string()),
    ];

    let url_with_params = reqwest::Url::parse_with_params(url, &params).unwrap();
    let mut req = Request::get(url_with_params.as_str())?;
    req.headers = vec![
        ("Accept".to_string(), "application/json".to_string()),
        ("Content-Type".to_string(), "application/json".to_string()),
    ];

    let response = reactor.send(req).await?;
    Ok(response.json::<WeatherResponse>().map_err(|e| {
        let resp_str = String::from_utf8_lossy(&response.body);
        format!(
            "Failed to parse weather response: {:?}. Response body: {:?}. URL: {:?}",
            e, resp_str, url_with_params
        )
    })?)
}



// Found by doing a UI search, then press f12 in the browser. network tab. Showcased this query url.
// https://api.open-meteo.com/v1/forecast?latitude=36.1622767&longitude=-86.7742984&current=temperature_2m
// Then you can use use a tool like https://transform.tools/json-to-rust-serde to convert the JSON to a struct.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherResponse {
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "generationtime_ms")]
    pub generationtime_ms: f64,
    #[serde(rename = "utc_offset_seconds")]
    pub utc_offset_seconds: i64,
    pub timezone: String,
    #[serde(rename = "timezone_abbreviation")]
    pub timezone_abbreviation: String,
    // pub elevation: i64,
    #[serde(rename = "current_units")]
    pub current_units: CurrentUnits,
    pub current: Current,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentUnits {
    pub time: String,
    pub interval: String,
    #[serde(rename = "temperature_2m")]
    pub temperature_2m: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Current {
    pub time: String,
    pub interval: i64,
    #[serde(rename = "temperature_2m")]
    pub temperature_2m: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentTemperatureResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub time: String,
    #[serde(rename = "temperature_2m")]
    pub temperature: f64,
}

impl From<CurrentTemperatureResponse> for Vec<u8> {
    fn from(response: CurrentTemperatureResponse) -> Vec<u8> {
        let output = serde_json::to_string(&response).unwrap();
        output.into_bytes()
    }
}

export_layer_trigger_world!(Component);
