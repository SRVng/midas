use crate::price_manipulation::returns::get_average_return;
use rust_decimal::{
    prelude::{Decimal, FromPrimitive},
    MathematicalOps,
};

pub async fn get_variance(returns: Vec<Decimal>) -> Decimal {
    let avg_return = get_average_return(returns.clone()).await;
    let squared_deviation: Decimal = returns.iter().map(|x| (x - avg_return).powu(2)).sum();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::price_manipulation::returns::get_return;
    use rust_decimal_macros::dec;

    #[tokio::test]
    async fn test_get_variance() {
        let prices: [Decimal; 6] = [
            dec!(59.65),
            dec!(67.08),
            dec!(59.16),
            dec!(57.05),
            dec!(55.22),
            dec!(54.11),
        ];

        let returns = get_return(&prices).await;

        assert!(get_variance(returns).await == dec!(0.0061673295653075877308743126));
    }

    #[tokio::test]
    async fn test_get_volatility() {
        let prices: [Decimal; 6] = [
            dec!(59.65),
            dec!(67.08),
            dec!(59.16),
            dec!(57.05),
            dec!(55.22),
            dec!(54.11),
        ];

        let returns = get_return(&prices).await;

        assert!(get_volatility(returns).await == dec!(0.0061673295653075877308743126).sqrt().unwrap());
    }
}
