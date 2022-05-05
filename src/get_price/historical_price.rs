use reqwest;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct IHistoricalResponse {
    pub prices: Box<[[f32; 2]]>,
    pub market_caps: Box<[[f32; 2]]>,
    pub total_volumes: Box<[[f32; 2]]>
}

pub async fn get_historical_price(coin_id: &str) -> IHistoricalResponse {
    let endpoint: String = format!("https://api.coingecko.com/api/v3/coins/{coin_id}/market_chart?vs_currency=usd&days=360&interval=daily");
    
    let client = reqwest::Client::new();

    let response = client.get(endpoint)
        .send()
        .await
        .expect("Error at response")
        .json::<IHistoricalResponse>()
        .await
        .expect("Error on parsing json");

    response
}