use std::cmp::Ordering;

use rust_decimal::Decimal;

use num::{FromPrimitive, ToPrimitive};

use rust_decimal_macros::dec;

use crate::utils::mean;

use super::IBackTestingResult;

#[derive(Debug)]
pub struct IBackTestingSummary {
    pub total_trade: u16,
    pub winning_trade: u16,
    pub losing_trade: u16,
    pub percent_profitable: Decimal,
    pub largest_win: Decimal,
    pub largest_losing: Decimal,
    pub average_win: Decimal,
    pub average_losing: Decimal,
    pub avg_win_loss_ratio: Decimal,
    pub max_consec_win: u16,
    pub max_consec_lose: u16,
    pub profit_factor: Decimal,
    pub maximum_drawdown: Decimal,
    pub returns: Decimal,
}

impl IBackTestingSummary {
    pub fn new() -> IBackTestingSummary {
        IBackTestingSummary {
            total_trade: 0,
            winning_trade: 0,
            losing_trade: 0,
            percent_profitable: dec!(0),
            largest_win: dec!(0),
            largest_losing: dec!(0),
            average_win: dec!(0),
            average_losing: dec!(0),
            avg_win_loss_ratio: dec!(0),
            max_consec_win: 0,
            max_consec_lose: 0,
            profit_factor: dec!(0),
            maximum_drawdown: dec!(0),
            returns: dec!(0),
        }
    }
    pub fn calculate(result: &IBackTestingResult<Decimal>) -> IBackTestingSummary {
        let returns: Vec<Decimal> = result.get_return();

        let mut summary: IBackTestingSummary = IBackTestingSummary::new();

        // * Total Trade
        summary.total_trade = IBackTestingSummary::get_total_trade(&returns);

        // * Winning Trade & Losing Trade & Biggest Win & Biggest Loss
        returns
            .iter()
            .map(|value| {
                if *value >= dec!(0) {
                    summary.winning_trade += 1
                } else {
                    summary.losing_trade += 1
                }

                if *value > summary.largest_win {
                    summary.largest_win = *value
                } else if *value < summary.largest_losing {
                    summary.largest_losing = *value
                }
            })
            .for_each(drop);

        // * Percent Profitable
        summary.percent_profitable = IBackTestingSummary::get_percent_profitable(&summary);

        // * Average Win & Average Loss & Avg.Win / Avg.Loss Ratio
        let positive_return: Vec<Decimal> = IBackTestingResult::filter_positive_return(result);
        let negative_return: Vec<Decimal> = IBackTestingResult::filter_negative_return(result);
        summary.average_win = mean(
            positive_return.as_slice(),
            &Decimal::from_usize(positive_return.len()).expect("Failed to parse usize to dec"),
        );
        summary.average_losing = mean(
            negative_return.as_slice(),
            &Decimal::from_usize(negative_return.len()).expect("Failed to parse usize to dec"),
        );
        summary.avg_win_loss_ratio =
            summary.average_win / (Decimal::NEGATIVE_ONE * summary.average_losing);

        // * Consecutive win and loss
        summary.max_consec_win = IBackTestingSummary::get_max_consective_win(&returns);
        summary.max_consec_lose = IBackTestingSummary::get_max_consecutive_loss(&returns);

        // * Profit factor and Returns
        let [portfolio_value, gross_profit, gross_loss] =
            IBackTestingSummary::get_pv_gp_gl(&returns);
        summary.profit_factor = gross_profit / gross_loss;
        summary.returns = portfolio_value - Decimal::ONE;

        // TODO: Maximum Drawdown
        // todo!("Maximum Drawdown, From its peak to next lowest portfolio value");

        summary
    }
    pub fn get_total_trade(returns: &[Decimal]) -> u16 {
        returns
            .len()
            .to_u16()
            .expect("Failed to parse from usize to u16")
    }
    pub fn get_percent_profitable(&self) -> Decimal {
        Decimal::from_u16(self.winning_trade).expect("Failed to parse Decimal from u16")
            / Decimal::from_u16(self.total_trade).expect("Failed to parse Decimal from u16")
    }
    pub fn get_max_consective_win(returns: &[Decimal]) -> u16 {
        let mut consecutive_win: u16 = 0;
        let mut record: Vec<u16> = Vec::new();
        for index in 1..returns.len() {
            if returns[index] > Decimal::ZERO && returns[index - 1] > Decimal::ZERO {
                consecutive_win += 1;
            } else {
                record.push(consecutive_win);
                consecutive_win = 0;
            }
        }
        *record
            .iter()
            .max()
            .expect("Failed to parse max consecutive win")
            .max(&consecutive_win)
    }
    pub fn get_max_consecutive_loss(returns: &[Decimal]) -> u16 {
        let mut consecutive_loss: u16 = 0;
        let mut record: Vec<u16> = Vec::new();
        for index in 1..returns.len() {
            if returns[index] < Decimal::ZERO && returns[index - 1] < Decimal::ZERO {
                consecutive_loss += 1;
            } else {
                record.push(consecutive_loss);
                consecutive_loss = 0;
            }
        }
        *record
            .iter()
            .max()
            .expect("Failed to parse max consecutive loss")
            .max(&consecutive_loss)
    }
    pub fn get_pv_gp_gl(returns: &[Decimal]) -> [Decimal; 3] {
        // 1 = Initial fund
        let mut portfolio_value: Decimal = Decimal::ONE;
        let mut gross_profit: Decimal = Decimal::ZERO;
        let mut gross_loss: Decimal = Decimal::ZERO;
        returns
            .iter()
            .map(|value: &Decimal| {
                let compounded: Decimal = portfolio_value * (Decimal::ONE + value);

                match compounded.cmp(&portfolio_value) {
                    Ordering::Greater => gross_profit += compounded - portfolio_value,
                    Ordering::Equal => gross_profit += compounded - portfolio_value,
                    Ordering::Less => gross_loss += portfolio_value - compounded,
                }
                portfolio_value = compounded;
            })
            .for_each(drop);

        [portfolio_value, gross_profit, gross_loss]
    }
}
