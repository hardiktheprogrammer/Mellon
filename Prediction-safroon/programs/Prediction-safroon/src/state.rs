use anchor_lang::prelude::*;

#[account]
pub struct Master {
    // Master Account hold the programe and the master account itself
    pub last_bet_id: u64,
}
