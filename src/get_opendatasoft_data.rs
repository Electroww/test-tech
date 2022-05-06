use reqwest::Error;
use serde_json::Value;

pub async fn get_opendatasoft_data(lat: String, long: String, distance: String) -> Result<Vec<Value>, Error> {
    let url = &format!("https://public.opendatasoft.com/api/records/1.0/search/?dataset=economicref-france-sirene-v3&q=&rows=5000&geofilter.distance={}%2C+{}%2C+{}", lat, long, distance);
    let request_response = reqwest::get(url).await?.json::<Value>().await?;

    if request_response["records"].is_array() {
        let records = request_response["records"].as_array().unwrap().to_vec();
        Ok(records) 
    } else {
        Ok(vec![])
    }
}

