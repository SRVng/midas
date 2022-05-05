#[derive(Debug)]
pub struct RiskReward {
    optimal_quantity: f64,
    risk_reward_ratio: f64,
    value: f64,
    estimate_profit: f64,
}

pub fn main_risk_reward(risk_per_trade: f64, entry: f64, sl: f64, tp: f64) -> RiskReward {
    let optimal_quantity = calculate_optimal_quantity(risk_per_trade, entry, sl).unwrap();

    println!("Optimal Quantity: {}", optimal_quantity);
    println!("RRR: {}", calculate_risk_reward_ratio(sl, tp).unwrap());
    println!("Value: {}", optimal_quantity * entry);
    println!("Est. Profit: {}", optimal_quantity* (tp - entry));
    RiskReward { 
        optimal_quantity, 
        risk_reward_ratio: calculate_risk_reward_ratio(sl, tp)
            .expect("Wrong parameter for stop loss and take profit"), 
        value: optimal_quantity * entry, 
        estimate_profit: optimal_quantity * (tp - entry) 
    }
}

pub fn calculate_optimal_quantity(risk_per_trade: f64, entry: f64, sl: f64) -> Result<f64,String> {

    let result: f64 = risk_per_trade / (entry - sl);

    match result {
        0.0.. => Ok(result),
        x => Err(String::from("N/A"))
    }
}

pub fn calculate_risk_reward_ratio(sl: f64, tp: f64) -> Result<f64, String> {
    let result = tp / sl;

    match result {
        0.1.. => Ok(result),
        x => Err(String::from("N/A"))
    }
}