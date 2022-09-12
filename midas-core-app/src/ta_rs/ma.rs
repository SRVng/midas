use crate::utils::{create_indicator_cross_vec, mean};
use num::{FromPrimitive, ToPrimitive};
use std::ops::{Add, Div, Mul, Sub};
use std::{fmt::Debug, iter::Sum};
pub struct IMAParams<'a, T: 'a + FromPrimitive + Sum<&'a T> + Div<Output = T> + Copy + Debug> {
    pub prices: &'a [T],
    pub period: u32,
}

// TODO: Custom Error

fn calculate_simple_moving_average<
    'a,
    T: 'a + FromPrimitive + Sum<&'a T> + Div<Output = T> + Copy + Debug,
>(
    prices: &'a [T],
    period: u32,
) -> Option<T> {
    Some(mean(prices, &T::from_u32(period).unwrap()))
}

pub fn simple_moving_average<
    'a,
    T: 'a + FromPrimitive + Sum<&'a T> + Div<Output = T> + Copy + Debug,
>(
    params: IMAParams<'a, T>,
) -> Result<Vec<T>, String> {
    let IMAParams { prices, period } = params;
    let max_length = prices.len();

    let mut sma: Vec<T> = Vec::new();

    for index in 0..max_length {
        if index + period.to_usize().unwrap() > max_length {
            break;
        }
        if let Some(value) =
            calculate_simple_moving_average(&prices[index..index + period as usize], period)
        {
            sma.push(value);
        } else {
            panic!("Error in calculate SMA")
        }
    }

    Ok(sma)
}

// * αx₍t₎ + (1 - α)EMA₍t-1₎; t > 0
// * α (smoothing factor) most opt for value of 2 (from tradingview)
// * then α = 2 / (1 + t)

fn calculate_exponential_moving_average<
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
    price: T,
    smoothing_factor: T,
    prev_ema: T,
) -> Option<T> {
    Some((smoothing_factor * price) + (T::from_i16(1).unwrap() - smoothing_factor) * prev_ema)
}

pub fn exponential_moving_average<
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
    params: IMAParams<'a, T>,
) -> Result<Vec<T>, String> {
    let IMAParams { prices, period } = params;

    let max_length = prices.len();
    let first_ema = calculate_simple_moving_average(&prices[0..period as usize], period)
        .expect("Failed to calculate SMA");
    let smoothing_factor = T::from_u32(2).unwrap() / T::from_u32(1 + period).unwrap();

    let mut ema: Vec<T> = vec![first_ema];

    for price in prices.iter().take(max_length).skip(period as usize + 1) {
        let prev_ema = ema.last().expect("No previous EMA");
        if let Some(value) =
            calculate_exponential_moving_average(*price, smoothing_factor, *prev_ema)
        {
            ema.push(value);
        } else {
            panic!("Error in calculate EMA")
        }
    }

    Ok(ema)
}

pub fn get_ma_cross<
    'a,
    T: 'a + FromPrimitive + Sum<&'a T> + Sub<Output = T> + Div<Output = T> + Copy + Debug,
>(
    prices: &'a [T],
    fast: u32,
    slow: u32,
) -> Result<Vec<T>, String> {
    let fast_ma: Vec<T> = if let Ok(value) = simple_moving_average(IMAParams {
        prices,
        period: fast,
    }) {
        value
    } else {
        panic!("Error in calculate Fast SMA")
    };
    let slow_ma: Vec<T> = if let Ok(value) = simple_moving_average(IMAParams {
        prices,
        period: slow,
    }) {
        value
    } else {
        panic!("Error in calculate Slow SMA")
    };
    let max_length = slow_ma.len();
    let fast_ma_cutoff = fast_ma.len() - max_length;
    Ok(create_indicator_cross_vec(
        &fast_ma[fast_ma_cutoff..],
        &slow_ma,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    const PRICES: [rust_decimal::Decimal; 9] = [
        dec!(2),
        dec!(4),
        dec!(6),
        dec!(8),
        dec!(12),
        dec!(14),
        dec!(16),
        dec!(18),
        dec!(20),
    ];
    const PERIOD: u32 = 2;

    #[test]
    fn test_simple_moving_average() {
        let result = [
            dec!(3),
            dec!(5),
            dec!(7),
            dec!(10),
            dec!(13),
            dec!(15),
            dec!(17),
            dec!(19),
        ];

        if let Ok(value) = simple_moving_average(IMAParams {
            prices: &PRICES,
            period: PERIOD,
        }) {
            assert!(value == result);
        } else {
            panic!("Failed")
        }
    }

    #[test]
    fn test_exponential_moving_average() {
        let result = [
            dec![3],
            dec![6.3333333333333333333333333335],
            dec![10.111111111111111111111111111],
            dec![12.703703703703703703703703704],
            dec![14.901234567901234567901234568],
            dec![16.967078189300411522633744856],
            dec![18.989026063100137174211248285],
        ];

        if let Ok(value) = exponential_moving_average(IMAParams {
            prices: &PRICES,
            period: PERIOD,
        }) {
            println!("{:#?}", value);
            assert!(value == result);
        } else {
            panic!("Failed")
        }
    }
}
