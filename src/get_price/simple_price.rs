use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Price {
    usd: f32
}

#[derive(Deserialize, Serialize, Debug)]
struct Avax {
    #[serde(alias = "avalanche-2")]
    avalanche: Price
}

pub async fn get_current_price(coin_id: &str) -> f32 {
    let endpoint: String = format!("https://api.coingecko.com/api/v3/simple/price?ids={coin_id}&vs_currencies=usd");

    let client: reqwest::Client = reqwest::Client::new();

    let response: f32 = client.get(endpoint)
        .send()
        .await
        .expect("Error at response")
        .json::<Avax>()
        .await
        .expect("Error on parsing to json")
        .avalanche
        .usd;
    
    response
}