use reqwest;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IHistoricalResponse {
    prices: Box<[[Decimal; 2]]>,
    market_caps: Box<[[Decimal; 2]]>,
    total_volumes: Box<[[Decimal; 2]]>,
}

impl IHistoricalResponse {
    pub fn extract_prices(&self) -> Box<[Decimal]> {
        self.prices
            .iter()
            .map(|[_timestamp, price]| *price)
            .collect::<Box<[Decimal]>>()
    }
    pub fn extract_market_caps(&self) -> Box<[Decimal]> {
        self.market_caps
            .iter()
            .map(|[_timestamp, market_caps]| *market_caps)
            .collect::<Box<[Decimal]>>()
    }
    pub fn extract_volumes(&self) -> Box<[Decimal]> {
        self.total_volumes
            .iter()
            .map(|[_timestamp, volume]| *volume)
            .collect::<Box<[Decimal]>>()
    }
}

pub async fn get_historical_chart_data(coin_id: &str) -> IHistoricalResponse {
    let endpoint: String = format!("https://api.coingecko.com/api/v3/coins/{coin_id}/market_chart?vs_currency=usd&days=360&interval=daily");

    let client = reqwest::Client::new();

    let response = client
        .get(endpoint)
        .send()
        .await
        .expect("Error at response")
        .json::<IHistoricalResponse>()
        .await
        .expect("Error on parsing json");

    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_historical_data() {
        let data = get_historical_chart_data("avalanche-2").await;

        assert!(data.extract_prices().len() == 361 as usize);
    }
}
