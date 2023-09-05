use {
    num::{BigRational},
    primitive_types::{H160}
};

#[derive(Debug)]
pub struct XRate {
    token_a: H160,
    token_b: H160,
    token_a_balance: BigRational,
    token_b_balance: BigRational,
}

impl XRate {
    pub fn new(token_a: H160, token_b: H160, token_a_balance: BigRational, token_b_balance: BigRational) -> Self {
        Self {
            token_a,
            token_b,
            token_a_balance,
            token_b_balance,
        }
    }

    pub fn convert_unit(&self, token: H160) -> BigRational {
        if token == self.token_a {
            return &self.token_b_balance / &self.token_a_balance;
        }
        if token == self.token_b {
            return &self.token_a_balance / &self.token_b_balance;
        }
        panic!("Token {} is not in the exchange rate <{}/{}>", token, self.token_a, self.token_b);
    }
}
