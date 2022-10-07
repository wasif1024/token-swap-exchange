use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize)]
pub enum Swap {
    Token1ForToken2 {
        /// amount of token1 wanna swap for token2
        amount: u64,
    },
    Token2ForToken1 {
        // amount of token2 wanna swap for token1
        amount: u64,
    },
}

impl Swap {
    // e.g. BTCUSD = 1 BTC = 50000 USD, thus rate = 1/50000 = 0.00002
    // <PAIR1>/<PAIR2> e.g. BTC/USD, ETH/USD, SOL/USD
    // calculate and return the amount of token to get from pool
    pub fn how_much_to_get(&self, rate: u64) -> u64 {
        match self {
            Swap::Token1ForToken2 { amount } => amount / rate,
            Swap::Token2ForToken1 { amount } => amount * rate,
        }
    }
}