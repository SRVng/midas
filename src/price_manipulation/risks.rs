use std::borrow::BorrowMut;

use crate::get_return;
use crate::utils::mean;
use rust_decimal::{prelude::FromPrimitive, Decimal};

pub async fn get_variance(mut prices: &'_ [Decimal]) -> Decimal {
    let mut prices_clone: &'_ [Decimal] = &prices.borrow_mut();
    let returns = get_return(prices.clone()).await;

    mean(
        prices_clone.borrow_mut(),
        &Decimal::from_usize(prices.len()).unwrap(),
    )
}
