use std::iter::Sum;
use std::ops::Div;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use crate::utils::mean;

// TODO: Custom Error

pub fn simple_moving_average<'a, T: 'a + FromPrimitive + Sum<&'a T> + Div<Output = T> + Copy>(prices: &'a [T], period: u32) -> Result<Vec<T>, String> {
    let max_length = prices.len();

    let mut sma: Vec<T> = Vec::new();

    for index in 0..max_length {
        if index + period.to_usize().unwrap() > max_length {
            break;
        }
        let calc: T = mean(&prices[index..(index + period as usize)], &T::from_u32(period).unwrap());
        sma.push(calc);
    }

    Ok(sma)
}