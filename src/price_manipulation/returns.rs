use crate::utils::calculate_return;
use crate::utils::mean;
use rust_decimal::prelude::{Decimal, FromPrimitive};

pub async fn get_return(prices: &[Decimal]) -> Vec<Decimal> {
    let prices_one_lag = prices[1..].to_vec();
    let prices_len = &prices_one_lag.len();

    let result = calculate_return(prices, &prices_one_lag, *prices_len);

    if let Some(x) = result {
        x
    } else {
        Vec::new()
    }
}

pub async fn get_average_return(returns: Vec<Decimal>) -> Decimal {
    mean(&returns, &Decimal::from_usize(returns.len()).unwrap())
}
