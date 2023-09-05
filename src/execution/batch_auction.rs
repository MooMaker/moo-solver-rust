use {
    std::collections::BTreeMap,
    crate::{
        models::auction::BatchAuctionModel,
        execution::{
            errors::BatchAuctionError,
            order::{Order, OrderMatchType}
        },
        utils::mutability::get_mut_pair
    }
};

#[derive(Default)]
pub struct BatchAuction<'a> {
    name: String,
    orders: BTreeMap<usize, Order<'a>>
}

impl<'a> BatchAuction<'a> {
    pub fn from_model(name: String, value: &'a BatchAuctionModel) -> Self {
        let orders = value.orders
            .iter()
            .map(|(id, order)| {
                (*id, Order::from_model(*id, order))
            })
            .collect::<BTreeMap<usize, Order>>();

        Self {
            orders,
            name
        }
    }


    /// Set auction instance name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
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
                    }
                }
            }
        }
        Ok(())
    }

}
