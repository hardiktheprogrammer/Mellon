use anchor_lang::prelude::*;

#[account]
pub struct Master {
    pub last_bet_id: u64,
}
