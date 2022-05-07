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