mod get_price;
mod risk_management;

use crate::get_price::simple_price::get_current_price;

#[tokio::main]
async fn main() {
    println!("{:?}", get_current_price("avalanche-2").await);
}
