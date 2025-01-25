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


        let v = input.split(',').collect::<Vec<&str>>();
        if v.len() != 2 {
            return Err("Input must be in the format of City,State".to_string());
        }

        let (city, state) = (v[0], v[1]);

        let res = block_on(move |reactor| async move {
            let loc: Result<Location, String> = get_location(&reactor, city, state).await;

            let location = match loc {
                Ok(data) => data,
                Err(e) => return Err(e),
            };

            let weather_data = get_weather(&reactor, location.lat, location.lon).await;

            match weather_data {
                Ok(data) => {
                    let v = CurrentTemperatureResponse{
                        latitude: data.latitude,
                        longitude: data.longitude,
                        name: location.name,
                        tempature: data.current.temperature_2m,
                        time: data.current.time,
                    };

                    let output: Vec<u8> = v.into();
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
    city: &str,
    state: &str,
) -> Result<Location, String> {
    let url: &str = "https://nominatim.openstreetmap.org/search.php";
    let params = [("city", city), ("state", state), ("format", "jsonv2")];
    let url_with_params = reqwest::Url::parse_with_params(url, &params).unwrap();
    let mut req = Request::get(url_with_params.as_str())?;
    req.headers = vec![
        ("Accept".to_string(), "application/json".to_string()),
        ("Content-Type".to_string(), "application/json".to_string()),
    ];
    req.headers.push(("User-Agent".to_string(), "Mozilla/5.0 (Linux; WAVS; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Mobile Safari/537.36".to_string()));

    let response = reactor.send(req).await;

    match response {
        Ok(response) => {
            let finalresp = response.json::<Vec<Location>>().map_err(|e| {
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
    lat: String,
    lon: String,
) -> Result<WeatherResponse, String> {
    let url: &str = "https://api.open-meteo.com/v1/forecast";
    let params = [
        ("latitude", lat),
        ("longitude", lon),
        ("current", "temperature_2m".to_string()),
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



// Found by doing a UI search, then press f12 in the browser. network tab. Showcased this query url.
// https://nominatim.openstreetmap.org/search.php?city=nashville&state=TN&polygon_geojson=1&format=jsonv2
// Then you can use use a tool like https://transform.tools/json-to-rust-serde to convert the JSON to a struct.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    #[serde(rename = "place_id")]
    pub place_id: i64,
    pub licence: String,
    #[serde(rename = "osm_type")]
    pub osm_type: String,
    #[serde(rename = "osm_id")]
    pub osm_id: i64,
    pub lat: String,
    pub lon: String,
    pub category: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "place_rank")]
    pub place_rank: i64,
    pub importance: f64,
    pub addresstype: String,
    pub name: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    pub boundingbox: Vec<String>,
}

// https://api.open-meteo.com/v1/forecast?latitude=36.1622767&longitude=-86.7742984&current=temperature_2m\
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
    pub elevation: i64,
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
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub time: String,
    #[serde(rename = "temperature_2m")]
    pub tempature: f64,
}

impl From<CurrentTemperatureResponse> for Vec<u8> {
    fn from(response: CurrentTemperatureResponse) -> Vec<u8> {
        let output = serde_json::to_string(&response).unwrap();
        output.into_bytes()
    }
}

export_layer_trigger_world!(Component);
