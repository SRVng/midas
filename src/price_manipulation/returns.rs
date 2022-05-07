use crate::utils::remove_with_lag_in_slice;
use rust_decimal::Decimal;

pub async fn get_return(prices: Box<[Decimal]>) -> Box<[Decimal]> {
    let prices_clone = prices.clone();
    let prices_one_lag = prices_clone[1..].to_vec();
    let prices_len = &prices_one_lag.len();

    match remove_with_lag_in_slice(&prices_clone, &prices_one_lag, *prices_len) {
        Ok(x) => x,
        Err(_) => Box::new([]),
    }
}
