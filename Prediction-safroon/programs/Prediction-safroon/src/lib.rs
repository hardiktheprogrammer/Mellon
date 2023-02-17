mod state;
// use crate::state::*;
use anchor_lang::{prelude::*, system_program};
use pyth_sdk_solana::{load_price_feed_from_account_info, PriceFeed};
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
mod prediction_contract {

    use super::*;

    pub fn create_master() -> Result<()> {}
}

#[derive(Accounts)]
pub struct CreateMaster<'info> {}
