use std::{vec::Vec, env};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestInfo {
    pub success: bool
}

/// This is the response that gets sent back when you call the location API
#[derive(Deserialize)]
pub struct LocationResp {
    pub request_info: RequestInfo,
    pub locations_total: i32,
    pub locations_total_current_page: i32,
    pub page: i32,
    pub limit: i32,
    pub locations: Vec<Location>,
}

#[derive(Deserialize)]
pub struct Location {
    pub id: i32,
    pub name: String,
    pub r#type: String,
    pub full_name: String,
    pub parent_id: i32,
    pub country_code: String,
    pub reach: u32,
    pub gps_coordinates: GpsCoordinates,
}

#[derive(Deserialize)]
pub struct GpsCoordinates {
    pub latitude: f64,
    pub longitude: f64,
}

/// This struct is the configuration for a request to the location API
pub struct LocReqConfig {
    pub api_key: String, // your scale SERP access key
    pub q: String,   // the query string to send
    pub r#type: Option<String>, // i.e. 'city' for a city 
    pub country_code: Option<String>,
}


impl LocReqConfig {

    /// create a new config using the environment variable SCALE_SERP_KEY
    pub fn new_from_env(q: &str) -> Self {
        let api_key: String = env::var("SCALE_SERP_KEY").unwrap();
        LocReqConfig::new(&api_key, q)
    }

    /// create a new config
    pub fn new(api_key: &str, q: &str) -> Self {
        LocReqConfig{
            api_key: api_key.to_string(),
            q: q.to_string(),
            r#type: None,
            country_code: None
        }
    }
    
    /// generate the url you want to call
    pub fn to_url(&self) -> String {
        let mut url = format!("https://api.scaleserp.com/locations?api_key={}&q={}", self.api_key, self.q);
        match &self.r#type {
            None => {},
            Some(tpe) => {
                url.push('&');
                url.push_str(tpe);
            }
        }
        match &self.country_code {
            None => {},
            Some(cc) => {
                url.push('&');
                url.push_str(cc);
            }
        }
        url 
    }
}