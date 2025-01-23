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

        if !req.contains(&b',') {
            return Err("Input must be in the format of City,State".to_string());
        }
        let input = std::str::from_utf8(&req).unwrap(); // TODO:

        // open weather API, not wavs specific
        let api_key = std::env::var("WAVS_ENV_OPEN_WEATHER_API_KEY")
            .or(Err("missing env var `WAVS_ENV_OPEN_WEATHER_API_KEY`".to_string()))?;

        let res = block_on(move |reactor| async move {
            let loc: Result<LocDataNested, String> =
                get_location(&reactor, api_key.clone(), input).await;

            let location = match loc {
                Ok(data) => data,
                Err(e) => return Err(e),
            };

            let weather_data = get_weather(&reactor, location, api_key).await;

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

async fn get_location(
    reactor: &Reactor,
    app_key: String,
    loc_input: &str,
) -> Result<LocDataNested, String> {
    let url: &str = "http://api.openweathermap.org/geo/1.0/direct";
    let loc_input_formatted = format!("{},US", loc_input);
    let params = [("q", loc_input_formatted.as_str()), ("appid", app_key.as_str())];

    let url_with_params = reqwest::Url::parse_with_params(url, &params).unwrap();
    let mut req = Request::get(url_with_params.as_str())?;
    req.headers = vec![
        ("Accept".to_string(), "application/json".to_string()),
        ("Content-Type".to_string(), "application/json".to_string()),
    ];

    let response = reactor.send(req).await;

    match response {
        Ok(response) => {
            let finalresp = response.json::<LocationData>().map_err(|e| {
                let resp_body = response.body;
                let resp_str = String::from_utf8_lossy(&resp_body);
                format!(
                    "Error debugging location response to JSON. Error: {:?}. had response: {:?} | using URL: {:?}",
                    e, resp_str, url_with_params,
                )
            })?;
            return Ok(finalresp[0].clone());
        }
        Err(e) => {
            return Err(e.to_string());
        }
    }
}

async fn get_weather(
    reactor: &Reactor,
    location: LocDataNested,
    app_key: String,
) -> Result<WeatherResponse, String> {
    let url: &str = "https://api.openweathermap.org/data/2.5/weather";
    let params = [
        ("lat", location.lat.to_string()),
        ("lon", location.lon.to_string()),
        ("appid", app_key),
        ("units", "imperial".to_string()),
    ];

    let url_with_params = reqwest::Url::parse_with_params(url, &params).unwrap();
    let mut req = Request::get(url_with_params.as_str())?;
    req.headers = vec![
        ("Accept".to_string(), "application/json".to_string()),
        ("Content-Type".to_string(), "application/json".to_string()),
    ];

    let response = reactor.send(req).await;

    match response {
        Ok(response) => {
            let finalresp = response.json::<WeatherResponse>().map_err(|e| {
                let resp_body = response.body;
                let resp_str = String::from_utf8_lossy(&resp_body);
                format!(
                    "Error debugging weather response to JSON. Error: {:?}. had response: {:?} | using URL: {:?}",
                    e, resp_str, url_with_params,
                )
            })?;
            return Ok(finalresp);
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

export_layer_trigger_world!(Component);
