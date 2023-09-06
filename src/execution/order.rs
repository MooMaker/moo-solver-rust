use {
    primitive_types::U256,
    num::{BigRational, FromPrimitive, One, Zero},
    crate::{
        execution::{
            xrate::XRate,
            errors::OrderError
        },
        models::order::{OrderModel, ExecutedOrderModel},
        utils::conversion::u256_to_big_rational
    }
};

pub struct Order<'a> {
    id: usize,
    model: &'a OrderModel,
    // Max limit of the order as an exchange rate
    max_limit: XRate,
    exec_buy_amount: Option<U256>,
    exec_sell_amount: Option<U256>
}


#[derive(PartialEq)]
pub enum OrderMatchType {
    LhsFilled,
    RhsFilled,
    BothFilled
}

impl<'a> Order<'a> {
    pub fn from_model(id: usize, model: &'a OrderModel) -> Self {
        Self {
            id,
            model,
            max_limit: XRate::new(
                model.sell_token,
                model.buy_token,
                u256_to_big_rational(&model.sell_amount),
                u256_to_big_rational(&model.buy_amount),
            ),
            exec_buy_amount: None,
            exec_sell_amount: None
        }
    }

    pub fn get_model(&self) -> &OrderModel {
        self.model
    }

    pub fn match_type(&self, other: &Order) -> Option<OrderMatchType> {
        if !self.overlaps(other) {
            return None;
        }

        if self.model.buy_amount < other.model.sell_amount && self.model.sell_amount < other.model.buy_amount {
            return Some(OrderMatchType::LhsFilled);
        }

        if self.model.buy_amount > other.model.sell_amount && self.model.sell_amount > other.model.buy_amount {
            return Some(OrderMatchType::RhsFilled);
        }

        return Some(OrderMatchType::BothFilled)
    }

    /// Defines if one order can be matched against the other
    pub fn overlaps(&self, other: &Order) -> bool {
        if self.model.sell_token != other.model.buy_token ||
            self.model.buy_token != other.model.sell_token {
            return false;
        }

        return self.model.buy_amount * other.model.buy_amount
            <= self.model.sell_amount * other.model.sell_amount;
    }

    pub fn max_buy_amount(&self) -> Option<U256> {
        return if !self.model.is_sell_order {
            Some(self.model.buy_amount)
        } else {
            None
        }
    }

    pub fn max_sell_amount(&self) -> Option<U256> {
        return if self.model.is_sell_order {
            Some(self.model.sell_amount)
        } else {
            None
        }
    }

    pub fn is_executed(&self) -> bool {
        self.exec_buy_amount.is_some() && self.exec_sell_amount.is_some()
    }


    /// Executes the order at given amounts.
    ///
    /// # Arguments
    ///
    /// * `buy_amount_value` - Buy amount.
    /// * `sell_amount_value` - Sell amount.
    /// * `buy_token_price` - Buy-token price.
    /// * `sell_token_price` - Sell-token price.
    /// * `amount_tol` - Accepted violation of the limit buy/sell amount constraints.
    /// * `xrate_tol` - Accepted violation of the limit exchange rate constraint per unit of buy token (default: 1e-6).
    pub fn execute(
        &mut self,
        buy_amount: U256,
        sell_amount: U256,
        buy_token_price: Option<f64>,
        sell_token_price: Option<f64>,
        amount_tol: Option<f64>,
        xrate_tol: Option<f64>,
    ) -> Result<(), OrderError> {
        let mut buy_amount_value = u256_to_big_rational(&buy_amount);
        let mut sell_amount_value = u256_to_big_rational(&sell_amount);

        let buy_token_price = buy_token_price.unwrap_or(0.0);
        let sell_token_price = sell_token_price.unwrap_or(0.0);

        let amount_tol = BigRational::from_f64(amount_tol.unwrap_or(1e-8))
            .expect("Amount tolerance must be a valid float number");
        let xrate_tol = BigRational::from_f64(xrate_tol.unwrap_or(1e-6))
            .expect("Exchange rate tolerance must be a valid float number");

        assert!(buy_amount_value >= -amount_tol.clone(), "Buy amount value is less than amount tolerance");
        assert!(sell_amount_value >= -amount_tol.clone(), "Sell amount value is less than amount tolerance");
        assert!(buy_token_price >= 0.0, "Buy token price is less than 0");
        assert!(sell_token_price >= 0.0, "Sell token price is less than 0");

        let xmax = self.max_buy_amount();
        let ymax = self.max_sell_amount();

        // (a) Check buyAmount: if too much above maxBuyAmount --> error!
        if let Some(xmax) = xmax {
            let xmax = u256_to_big_rational(&xmax);
            let one = BigRational::one();

            if buy_amount_value > &xmax * (&one + &amount_tol) &&
                buy_amount_value > &xmax + &amount_tol {
                return Err(
                    OrderError::InvalidExecutionRequest {
                        order_id: self.id,
                        details: format!(
                            "buy amount (exec) : {:?} buy amount (max) : {:?}",
                            buy_amount_value, xmax
                        )
                    }
                )
            }

            buy_amount_value = buy_amount_value.min(xmax);
        }

        // (b) Check sellAmount: if too much above maxSellAmount --> error!
        if let Some(ymax) = ymax {
            let ymax = u256_to_big_rational(&ymax);
            let one = BigRational::one();

            if sell_amount_value > &ymax * (&one + &amount_tol) &&
                sell_amount_value > &ymax + &amount_tol {
                return Err(
                    OrderError::InvalidExecutionRequest {
                        order_id: self.id,
                        details: format!(
                            "sell amount (exec) : {:?} sell amount (max) : {:?}",
                            sell_amount_value, ymax
                        )
                    }
                )
            }
        }

        // (c) if any amount is very small, set to zero.
        if buy_amount_value < amount_tol || sell_amount_value < amount_tol {
            buy_amount_value = BigRational::zero();
            sell_amount_value = BigRational::zero();
        }

        // Check limit price.
        if buy_amount_value > BigRational::zero() {
            assert!(sell_amount_value > BigRational::zero(), "Sell amount expected to be greater than 0");
            let xrate = XRate::new(
                self.model.buy_token,
                self.model.sell_token,
                u256_to_big_rational(&buy_amount),
                 u256_to_big_rational(&sell_amount),
            );
            if !self.is_executable(&xrate, &xrate_tol) {
                return Err(
                    OrderError::InvalidExecutionRequest {
                        order_id: self.id,
                        details: format!(
                            "buy amount (exec): {:?} sell amount (exec): {:?} xrate (exec): {:?} limit (max): {:?}",
                            buy_amount_value, sell_amount_value, xrate, self.max_limit
                        )
                    }
                )
            }
        }

        self.exec_buy_amount = Some(buy_amount);
        self.exec_sell_amount = Some(sell_amount);

        Ok(())
    }

    /// Determine if the order limit price satisfies a given market rate.
    ///
    /// # Arguments
    ///
    /// * buy_amount - Buy amount.
    /// * sell_amount - Sell amount.
    /// * xrate_tol - Accepted violation of the limit exchange rate constraint per unit of buy token (default: 1e-6).
    ///
    /// # Returns
    /// true, if order can be executed; false otherwise.
    pub fn is_executable(&self, x_rate: &XRate, xrate_tol: &BigRational) -> bool {
        assert!(xrate_tol >= &BigRational::zero(), "Exchange rate tolerance must be greater than or equal to 0");

        let converted_buy = x_rate.convert_unit(self.model.buy_token);
        let converted_sell = self.max_limit.convert_unit(self.model.buy_token);

        converted_buy <= converted_sell * (BigRational::one() + xrate_tol.clone())
    }
}

impl From<&Order<'_>> for ExecutedOrderModel {
    fn from(value: &Order) -> Self {
        Self {
            exec_buy_amount: value.exec_buy_amount.unwrap_or(U256::zero()),
            exec_sell_amount: value.exec_sell_amount.unwrap_or(U256::zero()),
        }
    }
}
