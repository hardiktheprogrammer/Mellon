use anchor_lang::prelude::*;

#[account]
pub struct Master {
    // Master Account hold the programe and the master account itself
    pub last_bet_id: u64,
}

#[account]
pub struct Bet {
    pub id: u64,     // new ID
    pub amount: u64, // Beting amount wroth
    pub prediction_a: BetingPrediction,
    pub prediction_b: Option<BetingPrediction>, // an Option
    pub state: BetState,
    pub pyth_price_key: Pubkey,
}
