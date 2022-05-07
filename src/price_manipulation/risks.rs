use rust_decimal::{prelude::{Decimal, FromPrimitive}, MathematicalOps};
use crate::price_manipulation::returns::{get_average_return};

pub async fn get_variance(returns: Vec<Decimal>) -> Decimal {
    let avg_return = get_average_return(returns.clone()).await;
    let squared_deviation: Decimal = returns
        .iter()
        .map(|x| (x - avg_return).powu(2))
        .sum();
    let length = Decimal::from_usize(returns.len()).unwrap();
    
    squared_deviation / length
}

// TODO: Should add more method except from standard deviation

pub async fn get_volatility(returns: Vec<Decimal>) -> Decimal {
    let variance = get_variance(returns).await;
    if let Some(volatility) = variance.sqrt() {
        volatility
    } else {
        Decimal::ZERO
    }
}