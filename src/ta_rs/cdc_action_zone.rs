use std::{
    fmt::Debug,
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
};

use rust_decimal::prelude::FromPrimitive;

use crate::{
    ta_rs::ma::{exponential_moving_average, IMAParams},
    utils::create_indicator_cross_vec,
};

pub async fn get_cdc_action_zone<
    'a,
    T: 'a
        + FromPrimitive
        + Sum<&'a T>
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Copy
        + Debug,
>(
    prices: &'a [T],
) -> Result<Vec<T>, String> {
    let ema_12: Vec<T> =
        if let Ok(value) = exponential_moving_average(IMAParams { prices, period: 12 }) {
            value
        } else {
            panic!("Error at EMA12 calculation")
        };

    let ema_26: Vec<T> =
        if let Ok(value) = exponential_moving_average(IMAParams { prices, period: 26 }) {
            value
        } else {
            panic!("Error at EMA26 calculation")
        };
    Ok(create_indicator_cross_vec(&ema_12, &ema_26))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::mock::mock_prices;
    use rust_decimal_macros::dec;

    #[tokio::test]
    async fn test_cdc_action_zone() {
        let result = [
            dec![2.862388889492811017275605213],
            dec![2.989937212871300315078043333],
            dec![3.088501429616597624580488659],
            dec![3.141695600233379304394877577],
        ];

        if let Ok(value) = get_cdc_action_zone(&mock_prices()).await {
            assert!(value == result);
        } else {
            panic!("Failed");
        }
    }
}
