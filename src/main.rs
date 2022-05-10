mod get_price;
mod price_manipulation;
mod risk_management;
mod ta_rs;
mod utils;

// GET Price
use crate::get_price::historical_price::get_historical_chart_data;
use crate::get_price::simple_price::get_current_price;

// Risk Management
use crate::risk_management::risk_reward_ratio::calculate_risk_reward;

// Price Mutation
use crate::price_manipulation::returns::{get_average_return, get_return};
use crate::price_manipulation::risks::{get_variance, get_volatility};

// Technical analysis
use crate::ta_rs::cdc_action_zone::get_cdc_action_zone;
use crate::ta_rs::ma::{exponential_moving_average, simple_moving_average, IMAParams};

#[tokio::main]
async fn main() {}
