use std::{iter::Sum, fmt::Debug};
use std::ops::{Add, Sub, Mul, Div};
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use crate::utils::mean;

pub struct IMAParams<'a, T: 'a + FromPrimitive + Sum<&'a T> + Div<Output = T> + Copy + Debug> {
    pub prices: &'a [T],
    pub period: u32
}

// TODO: Custom Error

fn calculate_simple_moving_average<'a, T: 'a + FromPrimitive + Sum<&'a T> + Div<Output = T> + Copy + Debug>(prices: &'a [T], period: u32) -> Option<T> {
    Some(mean(prices, &T::from_u32(period).unwrap()))
}

pub fn simple_moving_average<'a, T: 'a + FromPrimitive + Sum<&'a T> + Div<Output = T> + Copy + Debug>(params: IMAParams<'a, T>) -> Result<Vec<T>, String> {
    let IMAParams { prices, period } = params;
    let max_length = prices.len();

    let mut sma: Vec<T> = Vec::new();

    for index in 0..max_length {
        if index + period.to_usize().unwrap() > max_length {
            break;
        }
        if let Some(value) = calculate_simple_moving_average(&prices[index..index + period as usize], period) {
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

fn calculate_exponential_moving_average<'a, 
    T: 'a + 
        FromPrimitive + 
        Sum<&'a T> +
        Add<Output = T> +
        Sub<Output = T> +
        Mul<Output = T> +
        Div<Output = T> + 
        Copy + 
        Debug>
    (
        price: T, 
        smoothing_factor: T,
        prev_ema: T
    ) -> Option<T> {
        Some(
            (smoothing_factor * price) + (T::from_i16(1).unwrap() - smoothing_factor) * prev_ema 
        )
}

pub fn exponential_moving_average<'a, 
    T: 'a + 
        FromPrimitive + 
        Sum<&'a T> + 
        Add<Output = T> +
        Sub<Output = T> +
        Mul<Output = T> +
        Div<Output = T> + 
        Copy + 
        Debug>
    (
        params: IMAParams<'a, T>
    ) -> Result<Vec<T>, String> {
        let IMAParams { prices, period } = params;

        let max_length = prices.len();
        let first_ema = calculate_simple_moving_average(&prices[0..period as usize], period).expect("Failed to calculate SMA");
        let smoothing_factor = T::from_u32(2).unwrap() / T::from_u32(1 + period).unwrap();

        let mut ema: Vec<T> = vec![first_ema];

        for index in 1..max_length {
            let prev_ema = ema[index - 1];
            if let Some(value) = calculate_exponential_moving_average(prices[index], smoothing_factor, prev_ema) {
                ema.push(value);
            } else {
                panic!("Error in calculate EMA")
            }
        }

        Ok(ema)
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

        if let Ok(value) = simple_moving_average(IMAParams { prices: &PRICES, period: PERIOD }) {
            assert!(value == result);            
        } else {
            panic!("Failed")
        }
    }

    #[test]
    fn test_exponential_moving_average() {
        let result = [
            dec!(3),
            dec!(3.6666666666666666666666666667),
            dec!(5.2222222222222222222222222223),
            dec!(7.0740740740740740740740740742),
            dec!(10.358024691358024691358024691),
            dec!(12.786008230452674897119341564),
            dec!(14.928669410150891632373113855),
            dec!(16.976223136716963877457704619),
            dec!(18.992074378905654625819234873),
        ];

        if let Ok(value) = exponential_moving_average(IMAParams { prices: &PRICES, period: PERIOD }) {
            println!("{:?}", value);
            assert!(value == result);
        } else {
            panic!("Failed")
        }
    }
}