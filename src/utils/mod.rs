use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::Sum;
use std::ops::{Add, Div, Sub};
pub mod mock;

use rust_decimal::prelude::FromPrimitive;

pub fn remove_same_values_in_slice<T: Clone + Eq + Hash>(v1: &[T], v2: &[T]) -> Box<[T]> {
    let hs1: HashSet<T> = v1.iter().cloned().collect();
    let hs2: HashSet<T> = v2.iter().cloned().collect();
    (&hs1 - &hs2).iter().cloned().collect()
}

pub fn calculate_return<T: Clone + Sub<Output = T> + Div<Output = T> + Copy + core::fmt::Debug>(
    v: &[T],
    v_lag: &[T],
    lag_length: usize,
) -> Option<Vec<T>> {
    let mut store: Vec<T> = Vec::new();

    for index in 0..lag_length {
        store.push((v_lag[index] - v[index]) / v[index])
    }

    Some(store)
}

pub fn create_indicator_cross_vec<T: Sub<Output = T> + Copy + Debug>(v1: &[T], v2: &[T]) -> Vec<T> {
    let max_length = v2.len();
    let mut crossed: Vec<T> = Vec::new();
    let v1_start_point = &v1[v1.len() - v2.len()..];
    for index in 0..max_length {
        crossed.push(v1_start_point[index] - v2[index])
    }
    crossed
}

pub fn mean<'a, T: 'a + Sum<&'a T> + Div<Output = T> + Copy>(slices: &'a [T], length: &T) -> T {
    slices.iter().sum::<T>() / *length
}

pub fn median<T: Ord + Copy + Add<Output = T> + Div<Output = T> + FromPrimitive + Debug>(
    slices: &mut [T],
) -> T {
    slices.sort();

    match (slices.len() % 2) == 0 {
        true => {
            let middle: i16 = (slices.len() as i16 / 2) - 1; // Index start from 0

            (slices[middle as usize] + slices[(middle + 1) as usize]) / T::from_i16(2).unwrap()
        }
        false => {
            let middle: usize = slices.len() / 2;
            slices[middle]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::prelude::{Decimal, FromPrimitive};
    use rust_decimal_macros::dec;

    #[test]
    fn test_mean() {
        let slices_of_dec: [Decimal; 3] = [dec!(10), dec!(20), dec!(30)];

        let average = mean(
            &slices_of_dec,
            &Decimal::from_usize(slices_of_dec.len()).unwrap(),
        );

        assert!(average == dec!(20));
    }

    #[test]
    fn test_median() {
        let mut slices_of_dec: [Decimal; 4] = [dec!(10), dec!(20), dec!(30), dec!(40)];

        assert!(median(&mut slices_of_dec) == dec!(25));
    }
}

// TODO: Clone vs Cloned ?
// TODO: wtf is HashSet ?
