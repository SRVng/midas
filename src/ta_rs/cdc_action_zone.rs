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
            dec!(-2.779935897435897435897435898),
            dec!(-3.319950142450142450142450143),
            dec!(-3.526514577803751592925382099),
            dec!(-3.525315607464798623961293094),
            dec!(-3.101295723495934632619499225),
            dec!(-2.640410677953423703418726259),
            dec!(-2.079313725714095497310784747),
            dec!(-1.456467808994233278880793139),
            dec!(-0.768409409182441133058084122),
            dec!(-0.148780641134086134102308293),
            dec!(0.193193814073418051792983975),
            dec!(0.509975010443133091434104882),
            dec!(0.779476232551107745381602971),
            dec!(0.947438999358767571176576084),
            dec!(1.023564369372076351258944466),
            dec!(0.963849894375414268972948816),
            dec!(0.569442828274036121666565344),
            dec!(0.352064724311143999171061878),
            dec!(0.260705073350916336256680965),
            dec!(0.364047739141610716181235167),
            dec!(0.520637602776580834741142475),
            dec!(0.677274816703961060567381619),
            dec!(0.766750983550723789513097398),
            dec!(0.879967342378805998350759648),
            dec!(0.968214149524695979652623124),
            dec!(1.088541828633017805690805356),
            dec!(1.251778137579617324031936102),
            dec!(1.470703700001259484281745820),
            dec!(1.658172973123614345055451487),
            dec!(1.797322141830603317512851067),
        ];

        if let Ok(value) = get_cdc_action_zone(&mock_prices()).await {
            assert!(value == result);
        } else {
            panic!("Failed");
        }
    }
}
