use std::ops::{Sub, Mul, Div};

pub struct IRiskRewardParams<T: Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy> {
    pub risk_per_trade: T,
    pub entry: T,
    pub stop_loss: T,
    pub take_profit: T,
}
#[derive(Debug)]
pub struct IRiskRewardResult<T: Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy> {
    pub optimal_quantity: T,
    pub risk_reward_ratio: T,
    pub value: T,
    pub estimate_profit: T,
}

pub fn calculate_risk_reward<T: 
    Sub<Output = T> + 
    Mul<Output = T> + 
    Div<Output = T> +
    Copy
    >(params: IRiskRewardParams<T>) -> IRiskRewardResult<T> {
    // let risk_per_trade = T::from_f32_retain(risk_per_trade).unwrap();
    // let entry = T::from_f32_retain(entry).unwrap();
    // let stop_loss = T::from_f32_retain(stop_loss).unwrap();
    // let take_profit = T::from_f32_retain(take_profit).unwrap();
        let IRiskRewardParams { risk_per_trade, entry, stop_loss, take_profit } = params;

        let optimal_quantity = calculate_optimal_quantity(risk_per_trade, entry, stop_loss);

        IRiskRewardResult {
            optimal_quantity,
            risk_reward_ratio: calculate_reward_per_risk(stop_loss, take_profit),
            value: optimal_quantity * entry,
            estimate_profit: optimal_quantity * (take_profit - entry),
    }
}

fn calculate_optimal_quantity<T: Sub<Output = T> + Mul<Output = T> + Div<Output = T>>(risk_per_trade: T, entry: T, stop_loss: T) -> T {
    let result: T = risk_per_trade / (entry - stop_loss);

    result
}

fn calculate_reward_per_risk<T: Sub<Output = T> + Mul<Output = T> + Div<Output = T>>(stop_loss: T, take_profit: T) -> T {
    take_profit / stop_loss
}
