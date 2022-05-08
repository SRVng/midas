use std::ops::{Div, Mul, Sub};

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

pub fn calculate_risk_reward<T: Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy>(
    params: IRiskRewardParams<T>,
) -> IRiskRewardResult<T> {
    let IRiskRewardParams {
        risk_per_trade,
        entry,
        stop_loss,
        take_profit,
    } = params;

    let optimal_quantity = calculate_optimal_quantity(risk_per_trade, entry, stop_loss);

    IRiskRewardResult {
        optimal_quantity,
        risk_reward_ratio: calculate_reward_per_risk(stop_loss, take_profit),
        value: optimal_quantity * entry,
        estimate_profit: optimal_quantity * (take_profit - entry),
    }
}

fn calculate_optimal_quantity<T: Sub<Output = T> + Mul<Output = T> + Div<Output = T>>(
    risk_per_trade: T,
    entry: T,
    stop_loss: T,
) -> T {
    let result: T = risk_per_trade / (entry - stop_loss);

    result
}

fn calculate_reward_per_risk<T: Sub<Output = T> + Mul<Output = T> + Div<Output = T>>(
    stop_loss: T,
    take_profit: T,
) -> T {
    take_profit / stop_loss
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    #[test]
    fn test_calculate_risk_reward() {
        let result: IRiskRewardResult<Decimal> = IRiskRewardResult {
            optimal_quantity: dec!(333.33333333333333333333333333),
            risk_reward_ratio: dec!(7.4444444444444444444444444444),
            value: dec!(40),
            estimate_profit: dec!(183.33333333333333333333333333),
        };

        let calc = calculate_risk_reward(IRiskRewardParams {
            risk_per_trade: dec!(10),
            entry: dec!(0.12),
            stop_loss: dec!(0.09),
            take_profit: dec!(0.67),
        });

        let IRiskRewardResult {
            optimal_quantity,
            risk_reward_ratio,
            value,
            estimate_profit,
        } = calc;

        assert!(optimal_quantity == result.optimal_quantity);
        assert!(risk_reward_ratio == result.risk_reward_ratio);
        assert!(value == result.value);
        assert!(estimate_profit == result.estimate_profit);
    }
}
