use dotenv::dotenv;
use lazy_static::lazy_static;
use log::info;
use reqwest::Client;
use serde::Deserialize;

extern crate serde;
extern crate serde_json;

lazy_static! {
    static ref SW_API_BASE_URL: &'static str = "https://swapi.dev/api/";
}

#[derive(Deserialize, Debug)]
pub struct StarshipResponse {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "model")]
    model: String,

    #[serde(rename = "manufacturer")]
    manufacturer: String,

    #[serde(rename = "cost_in_credits")]
    cost_in_credits: String,

    #[serde(rename = "length")]
    length: String,

    #[serde(rename = "max_atmosphering_speed")]
    max_atmosphering_speed: String,

    #[serde(rename = "crew")]
    crew: String,

    #[serde(rename = "passengers")]
    passengers: String,

    #[serde(rename = "cargo_capacity")]
    cargo_capacity: String,

    #[serde(rename = "consumables")]
    consumables: String,

    #[serde(rename = "hyperdrive_rating")]
    hyperdrive_rating: String,

    #[serde(rename = "MGLT")]
    mglt: String,

    #[serde(rename = "starship_class")]
    starship_class: String,

    #[serde(rename = "pilots")]
    pilots: Vec<Option<serde_json::Value>>,

    #[serde(rename = "films")]
    films: Vec<String>,

    #[serde(rename = "created")]
    created: String,

    #[serde(rename = "edited")]
    edited: String,

    #[serde(rename = "url")]
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    env_logger::init();

    info!("Starting up");

    let client: Client = reqwest::Client::new();

    info!("Sending request");

    let starship_endpoint: String = format!("{}/starships/9/", *SW_API_BASE_URL);

    let query_params = vec![("id", "9")];

    let starship: StarshipResponse = client
        .get(starship_endpoint)
        .query(&query_params)
        .send()
        .await?
        .json::<StarshipResponse>()
        .await?;

    info!("Starship: {:?}", starship);

    Ok(())
}
