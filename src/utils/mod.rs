use std::collections::HashSet;
use std::hash::Hash;
use std::iter::Sum;
use std::ops::{Div, Sub};

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

pub fn mean<'a, T: 'a + Sum<&'a T> + Div<Output = T> + Copy>(slices: &'a [T], length: &T) -> T {
    slices.iter().sum::<T>() / *length
}

pub fn median<T: Ord + Copy>(slices: &mut [T]) -> T {
    slices.sort();
    let middle = slices.len() / 2;
    slices[middle]
}

// Clone vs Cloned?
// HashSet?
