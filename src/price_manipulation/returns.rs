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

#[cfg(test)]
mod tests {

    use rust_decimal_macros::dec;
    use super::*;

    #[tokio::test]
    async fn test_calculate_return() {
        let prices: [Decimal; 6] = [
            dec!(59.65),
            dec!(67.08),
            dec!(59.16),
            dec!(57.05),
            dec!(55.22),
            dec!(54.11),
        ];

        assert!(
            get_return(&prices).await ==
            vec![
                dec!(0.124559932942162615255658005),
                dec!(-0.1180679785330948121645796064),
                dec!(-0.0356659905341446923597025017),
                dec!(-0.0320771253286590709903593339),
                dec!(-0.020101412531691416153567548),
            ]
        )
    }

    #[tokio::test]
    async fn test_get_average_return() {
        let prices: [Decimal; 6] = [
            dec!(59.65),
            dec!(67.08),
            dec!(59.16),
            dec!(57.05),
            dec!(55.22),
            dec!(54.11),
        ];

        let returns = get_return(&prices).await;

        assert!(get_average_return(returns).await == dec!(-0.0162705147970854752825101970));
    }
}