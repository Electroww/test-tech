use reqwest::Error;
use serde_json::Value;

#[tokio::main]
pub async fn get_opendatasoft_data() -> Result<Vec<Value>, Error> {
    let url = "https://public.opendatasoft.com/api/records/1.0/search/?dataset=economicref-france-sirene-v3&q=&rows=5000&geofilter.distance=43.60426186809618%2C1.44195556640625%2C1000";
    let request_response = reqwest::get(url).await?.json::<Value>().await?;
    let records = request_response["records"].as_array().unwrap().to_vec(); 

    Ok(records)
} 