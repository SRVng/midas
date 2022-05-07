use rust_decimal::Decimal;

#[derive(Debug)]
pub struct RiskReward {
    pub optimal_quantity: Decimal,
    pub risk_reward_ratio: Decimal,
    pub value: Decimal,
    pub estimate_profit: Decimal,
}

pub fn calculate_risk_reward(risk_per_trade: f32, entry: f32, sl: f32, tp: f32) -> RiskReward {
    let risk_per_trade = Decimal::from_f32_retain(risk_per_trade).unwrap();
    let entry = Decimal::from_f32_retain(entry).unwrap();
    let sl = Decimal::from_f32_retain(sl).unwrap();
    let tp = Decimal::from_f32_retain(tp).unwrap();

    let optimal_quantity = calculate_optimal_quantity(risk_per_trade, entry, sl);

    // println!("Optimal Quantity: {}", optimal_quantity);
    // println!("RRR: {}", calculate_reward_per_risk(sl, tp));
    // println!("Value: {}", optimal_quantity * entry);
    // println!("Est. Profit: {}", optimal_quantity* (tp - entry));
    RiskReward {
        optimal_quantity,
        risk_reward_ratio: calculate_reward_per_risk(sl, tp),
        value: optimal_quantity * entry,
        estimate_profit: optimal_quantity * (tp - entry),
    }
}

pub fn calculate_optimal_quantity(risk_per_trade: Decimal, entry: Decimal, sl: Decimal) -> Decimal {
    let result: Decimal = risk_per_trade / (entry - sl);

    result
}

pub fn calculate_reward_per_risk(sl: Decimal, tp: Decimal) -> Decimal {
    let result = tp / sl;

    result
}
