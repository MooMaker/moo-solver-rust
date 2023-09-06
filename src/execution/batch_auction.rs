use {
    std::collections::BTreeMap,
    primitive_types::{H160, U256},
    crate::{
        models::{
            auction::{BatchAuctionModel, SettledBatchAuctionModel},
            token::TokenInfoModel
        },
        execution::{
            errors::BatchAuctionError,
            order::{Order, OrderMatchType}
        },
        utils::mutability::get_mut_pair
    }
};
use crate::models::order::ExecutedOrderModel;

#[derive(Default)]
pub struct BatchAuction<'a> {
    name: String,
    orders: BTreeMap<usize, Order<'a>>,
    prices: BTreeMap<H160, U256>,
    ref_token: H160
}

impl<'a> BatchAuction<'a> {
    pub fn from_model(name: String, value: &'a BatchAuctionModel) -> Self {
        let orders = value.orders
            .iter()
            .map(|(id, order)| {
                (*id, Order::from_model(*id, order))
            })
            .collect::<BTreeMap<usize, Order>>();

        let ref_token = find_ref_token(&value.tokens);

        Self {
            orders,
            name,
            ref_token,
            ..Default::default()
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn solve(&mut self) -> Result<(), BatchAuctionError> {
        let keys = self.orders.keys().cloned().collect::<Vec<usize>>();
        for i in 0..keys.len() {
            for j in i+1..keys.len() {
                let (order_i, order_j) = get_mut_pair(
                &mut self.orders, &keys[i], &keys[j]
                );

                // We found a solution. Send it back to driver
                if let Some(match_type) = order_i.match_type(order_j) {
                    if match_type == OrderMatchType::BothFilled {
                        order_i.execute(
                            order_j.get_model().sell_amount,
                            order_j.get_model().buy_amount,
                            None, None,None,None
                        )?;

                        order_j.execute(
                            order_i.get_model().sell_amount,
                            order_i.get_model().buy_amount,
                            None, None,None,None
                        )?;

                        // This is sell price for order i
                        self.prices.insert(
                            order_i.get_model().sell_token,
                            order_j.get_model().sell_amount
                        );

                        // This is buy price for order i
                        self.prices.insert(
                            order_i.get_model().buy_token,
                            order_i.get_model().sell_amount
                        );

                        return Ok(())
                    }
                }
            }
        }
        Ok(())
    }
}

fn find_ref_token(tokens: &BTreeMap<H160, TokenInfoModel>) -> H160 {
    let mut ref_token = None;
    let mut max_normalize_priority = 0;
    for (address, token) in tokens {
        let normalize_priority = token.normalize_priority.unwrap_or(0);
        if normalize_priority > max_normalize_priority {
            ref_token = Some(*address);
            max_normalize_priority = normalize_priority;
        }
    }

    ref_token.expect("No reference token found")
}

impl From<&BatchAuction<'_>> for SettledBatchAuctionModel {
    fn from(value: &BatchAuction) -> Self {
        let executed_orders = value.orders
            .iter()
            .filter(|(_, order)| order.is_executed())
            .map(|(id, order)| {
                (*id, order.into())
            })
            .collect::<BTreeMap<usize, ExecutedOrderModel>>();

        SettledBatchAuctionModel {
            orders: executed_orders,
            ref_token: value.ref_token,
            prices: value.prices.clone()
        }
    }
}
