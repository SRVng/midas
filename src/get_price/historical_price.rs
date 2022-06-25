use reqwest;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IHistoricalResponse {
    prices: Box<[[Option<Decimal>; 2]]>,
    market_caps: Box<[[Option<Decimal>; 2]]>,
    total_volumes: Box<[[Option<Decimal>; 2]]>,
}

impl IHistoricalResponse {
    pub fn extract_prices(&self) -> Box<[Decimal]> {
        self.prices
            .iter()
            .enumerate()
            .map(|(index, [_timestamp, price])| {
                price.unwrap_or(if let [_, Some(x)] = self.prices[index - 1] {
                    x
                } else {
                    // Worst case, replace null with zero is acceptable if there is no alternative
                    Decimal::ZERO
                })
            })
            .collect::<Box<[Decimal]>>()
    }
    pub fn extract_market_caps(&self) -> Box<[Decimal]> {
        self.market_caps
            .iter()
            .enumerate()
            .map(|(index, [_timestamp, market_caps])| {
                market_caps.unwrap_or(if let [_, Some(x)] = self.market_caps[index - 1] {
                    x
                } else {
                    Decimal::ZERO
                })
            })
            .collect::<Box<[Decimal]>>()
    }
    pub fn extract_volumes(&self) -> Box<[Decimal]> {
        self.total_volumes
            .iter()
            .enumerate()
            .map(|(index, [_timestamp, volume])| {
                volume.unwrap_or(if let [_, Some(x)] = self.total_volumes[index - 1] {
                    x
                } else {
                    Decimal::ZERO
                })
            })
            .collect::<Box<[Decimal]>>()
    }
}

pub async fn get_historical_chart_data(coin_id: &str) -> IHistoricalResponse {
    let url: &str = "https://api.coingecko.com/api/v3/coins";
    let usd_query: &str = "market_chart?vs_currency=usd";
    let day: &str = "9999";
    let endpoint: String = format!("{url}/{coin_id}/{usd_query}&days={day}&interval=daily");

    let client = reqwest::Client::new();

    let raw_response = client
        .get(endpoint)
        .send()
        .await
        .expect("Error at response");

    match raw_response.json::<IHistoricalResponse>().await {
        Ok(x) => x,
        Err(e) => panic!("Error message here: {}", e),
    }
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
