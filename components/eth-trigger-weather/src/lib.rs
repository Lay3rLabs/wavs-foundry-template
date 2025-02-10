mod trigger;
use trigger::{decode_trigger_event, encode_trigger_output};
pub mod bindings;
use crate::bindings::{export, Guest, TriggerAction};
use serde::{Deserialize, Serialize};
use wstd::{
    http::{Client, Request},
    io::{empty, AsyncRead},
    runtime::block_on,
};

struct Component;

const API_KEY_KEY: &str = "WAVS_ENV_OPEN_WEATHER_API_KEY";

impl Guest for Component {
    fn run(trigger_action: TriggerAction) -> std::result::Result<Vec<u8>, String> {
        let (trigger_id, req) =
            decode_trigger_event(trigger_action.data).map_err(|e| e.to_string())?;

        if !req.contains(&b',') {
            return Err("Input must be in the format of City,State".to_string());
        }
        let input = std::str::from_utf8(&req).unwrap();

        println!("input: {}", input);

        // open weather API, not wavs specific
        let api_key = std::env::var(API_KEY_KEY)
            .or(Err(format!("missing env var `{}`", API_KEY_KEY)))?;

        if api_key.chars().all(|c| c == '0') {
            return Err(format!("missing env var `{}`, it is set as the default placeholder in .env (all 0s)", API_KEY_KEY));
        }

        let res = block_on(async move {
            let loc: Result<LocDataNested, String> = get_location(api_key.clone(), input).await;
            println!("loc: {:?}", loc);

            let location = match loc {
                Ok(data) => data,
                Err(e) => return Err(e),
            };

            let weather_data = get_weather(location, api_key).await;
            println!("weather_data: {:?}", weather_data);

            match weather_data {
                Ok(data) => {
                    let output: Vec<u8> = data.into();
                    Ok(output)
                }
                Err(e) => Err(e),
            }
        });

        match res {
            Ok(data) => Ok(encode_trigger_output(trigger_id, &data)),
            Err(e) => Err(e),
        }
    }
}

async fn get_location(app_key: String, loc_input: &str) -> Result<LocDataNested, String> {
    let url: &str = "http://api.openweathermap.org/geo/1.0/direct";
    let loc_input_formatted = format!("{},US", loc_input);
    let params = [("q", loc_input_formatted.as_str()), ("appid", app_key.as_str())];

    let url_with_params = reqwest::Url::parse_with_params(url, &params).unwrap();
    let req = Request::get(url_with_params.as_str())
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .body(empty())
        .unwrap();

    let response = Client::new().send(req).await;

    match response {
        Ok(mut response) => {
            let mut body_buf = Vec::new();
            response.body_mut().read_to_end(&mut body_buf).await.unwrap();

            let resp = String::from_utf8_lossy(&body_buf);
            let json: LocationData = serde_json::from_str(&resp).unwrap();

            return Ok(json[0].clone());
        }
        Err(e) => {
            return Err(e.to_string());
        }
    }
}

async fn get_weather(location: LocDataNested, app_key: String) -> Result<WeatherResponse, String> {
    let url: &str = "https://api.openweathermap.org/data/2.5/weather";
    let params = [
        ("lat", location.lat.to_string()),
        ("lon", location.lon.to_string()),
        ("appid", app_key),
        ("units", "imperial".to_string()),
    ];

    let url_with_params = reqwest::Url::parse_with_params(url, &params).unwrap();
    let req = Request::get(url_with_params.as_str())
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .body(empty())
        .unwrap();

    let response = Client::new().send(req).await;

    match response {
        Ok(mut response) => {
            let mut body_buf = Vec::new();
            response.body_mut().read_to_end(&mut body_buf).await.unwrap();

            let resp = String::from_utf8_lossy(&body_buf);
            let json: WeatherResponse = serde_json::from_str(&resp).unwrap();

            return Ok(json);
        }
        Err(e) => {
            return Err(e.to_string());
        }
    }
}

/// -----
/// Given the JSON response, use an unescape tool like: https://jsonformatter.org/json-unescape
/// {\"coord\":{\"lon\":-86.7743,\"lat\":36.1623},\"weather\":[{\"id\":804,\"main\":\"Clouds\",\"description\":\"overcast clouds\",\"icon\":\"04d\"}],\"base\":\"stations\",\"main\":{\"temp\":28.13,\"feels_like\":16.21,\"temp_min\":26.17,\"temp_max\":29.34,\"pressure\":1018,\"humidity\":76,\"sea_level\":1018,\"grnd_level\":995},\"visibility\":10000,\"clouds\":{\"all\":100},\"dt\":1736193327,\"sys\":{\"type\":1,\"id\":3477,\"country\":\"US\",\"sunrise\":1736168310,\"sunset\":1736203634},\"timezone\":-21600,\"id\":4644585,\"name\":\"Nashville\",\"cod\":200}
///
/// Then put that into a JSON to struct converter: https://transform.tools/json-to-rust-serde
/// some types will not entirely convert as expected, so you can just ignore them if you get issues or properly convert to the types.
/// -----

// location based
pub type LocationData = Vec<LocDataNested>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocDataNested {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub country: String,
    pub state: String,
}

//

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherResponse {
    coord: Coord,
    weather: Vec<Weather>,
    base: String,
    main: Main,
    visibility: i64,
    // wind: Wind, // this needs to be a floating point / string
    clouds: Clouds,
    dt: i64, // the unix time this was taken, in UTC.
    sys: Sys,
    timezone: i64,
    id: i64,
    name: String,
    cod: i64,
}

// convert WeatherResponse to bytes
impl Into<Vec<u8>> for WeatherResponse {
    fn into(self) -> Vec<u8> {
        let s = serde_json::to_string(&self).unwrap();
        s.into_bytes()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clouds {
    all: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i64,
    humidity: i64,
    sea_level: i64,
    grnd_level: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sys {
    #[serde(rename = "type")]
    sys_type: i64,
    id: i64,
    country: String,
    sunrise: i64,
    sunset: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather {
    id: i64,
    main: String,
    description: String,
    icon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wind {
    speed: i64,
    deg: i64,
}

export!(Component with_types_in bindings);
