mod get_price;
mod price_mutation;
mod risk_management;
mod utils;

// GET Price
use crate::get_price::historical_price::get_historical_chart_data;
use crate::get_price::simple_price::get_current_price;

// Risk Management
use crate::risk_management::risk_reward_ratio::calculate_risk_reward;

// Price Mutation
use crate::price_mutation::returns::get_return;
use crate::price_mutation::risks::get_variance;

#[tokio::main]
async fn main() {}
