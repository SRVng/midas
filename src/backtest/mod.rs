use std::ops::{Div, Sub};
use std::process::Output;

use crate::get_price::historical_price::{get_historical_chart_data, IHistoricalResponse};
use crate::price_manipulation::returns::{get_average_return, get_return};
use crate::ta_rs::cdc_action_zone;
use crate::ta_rs::ma::get_ma_cross;
use rust_decimal::prelude::Decimal;
// use crate::ta_rs::ma::{simple_moving_average, exponential_moving_average, IMAParams};
use crate::ta_rs::cdc_action_zone::get_cdc_action_zone;

pub struct IBackTestingParams<'a> {
    pub coin_id: &'a str,
    pub period: u32,
}

#[derive(Debug)]
pub struct IBackTestingIndicator<T> {
    pub cdc: T,
    pub ma_cross: T,
}
impl IBackTestingIndicator<Decimal> {
    pub fn get_positions(
        indicator: Vec<Decimal>,
        prices: &[Decimal],
        gap: usize,
    ) -> IBackTestingResult<Decimal> {
        let mut positions: IBackTestingResult<Decimal> = IBackTestingResult { result: Vec::new() };
        for index in 1..indicator.len() {
            if indicator[index] > Decimal::ZERO && indicator[index - 1] < Decimal::ZERO {
                positions.result.push(IBackTestingSingleResult {
                    action: POSITION::Long,
                    price: prices[index + gap],
                })
            } else if indicator[index] < Decimal::ZERO
                && indicator[index - 1] > Decimal::ZERO
                && !positions.result.is_empty()
            {
                if let POSITION::Long = positions.result[positions.result.len() - 1].action {
                    positions.result.push(IBackTestingSingleResult {
                        action: POSITION::Close,
                        price: prices[index + gap],
                    })
                }
            }
        }
        positions
    }
}

pub struct IPreBacktesting<T, K> {
    pub prices: Box<[T]>,
    pub returns: Vec<T>,
    pub indicators: IBackTestingIndicator<K>,
}

#[derive(Debug)]
pub struct IBackTestingSingleResult<T> {
    pub action: POSITION,
    pub price: T,
}

#[derive(Debug)]
pub struct IBackTestingResult<T> {
    pub result: Vec<IBackTestingSingleResult<T>>,
}

impl<T: Sized + core::fmt::Debug + Copy + Sub<Output = T> + Div<Output = T>> IBackTestingResult<T> {
    pub fn get_price(&self) -> Box<[T]> {
        self.result
            .iter()
            .map(|IBackTestingSingleResult { action: _, price }| *price)
            .collect::<Box<[T]>>()
    }
    pub fn get_return(&self) -> Vec<IBacktestingReturn<T>> {
        let price = self.get_price();
        let mut store: Vec<IBacktestingReturn<T>> = Vec::new();
        let mut long_position: T = price[0];
        for index in 0..price.len() {
            if index % 2 == 0 {
                long_position = price[index]
            } else {
                store.push(IBacktestingReturn {
                    entry: long_position,
                    closing: price[index],
                    returns: (price[index] - long_position) / long_position,
                })
            }
        }

        store
    }
}

#[derive(Debug)]
pub struct IBacktestingReturn<T> {
    pub entry: T,
    pub closing: T,
    pub returns: T,
}

#[derive(Debug)]
pub enum POSITION {
    Long,
    Close,
}

// TODO: SMA, EMA Cross ??
// TODO: Dynamic trading strategies testing

// TODO: Average return of backtest should be limited risk

async fn pre_backtest(
    IBackTestingParams { coin_id, .. }: IBackTestingParams<'_>,
) -> Result<IPreBacktesting<Decimal, Vec<Decimal>>, String> {
    let prices: Box<[Decimal]> = get_historical_chart_data(coin_id).await.extract_prices();
    let returns: Vec<Decimal> = get_return(&prices).await;

    let indicators: IBackTestingIndicator<Vec<Decimal>> = IBackTestingIndicator {
        // sma: simple_moving_average(IMAParams { prices: &prices, period: 12 }).unwrap(),
        // ema: exponential_moving_average(IMAParams { prices: &prices, period: 12 }).unwrap(),
        cdc: get_cdc_action_zone(&prices).await.unwrap(),
        ma_cross: get_ma_cross(&prices, 14, 26).unwrap(),
    };

    Ok(IPreBacktesting {
        prices,
        returns,
        indicators,
    })
}

pub async fn start_backtesting(
    params: IBackTestingParams<'_>,
) -> IBackTestingIndicator<IBackTestingResult<Decimal>> {
    let IBackTestingParams { coin_id, period } = params;
    let IPreBacktesting {
        prices,
        returns,
        indicators,
    } = if let Ok(value) = pre_backtest(IBackTestingParams { coin_id, period }).await {
        value
    } else {
        panic!("Fail to run pre backtest")
    };

    let IBackTestingIndicator { cdc, ma_cross } = indicators;

    let gap = prices.len() - cdc.len();

    let cdc = IBackTestingIndicator::get_positions(cdc, &prices, gap);
    let ma_cross = IBackTestingIndicator::get_positions(ma_cross, &prices, gap);

    IBackTestingIndicator { cdc, ma_cross }
}
