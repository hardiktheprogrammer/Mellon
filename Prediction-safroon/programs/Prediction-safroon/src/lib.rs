mod constants;
mod state;
// use crate::state::*;
use crate::{constants::*, state::*};
use anchor_lang::{prelude::*, system_program};
use pyth_sdk_solana::{load_price_feed_from_account_info, PriceFeed};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
mod prediction_contract {

    use super::*;
    pub fn create_master() -> Result<()> {
        Ok(())

}
#[derive(Account)] // Account 

        #[account(
            init,
            seeds = [Master_SEED],
            bump = 1,
            payer = payer,
            space = 8 + 8
        )]
    
        pub master: Account<'info, Master>, // Account itself

        #[accout(mut)]
        pub payer: Signer<'info>,

        pub system_program: Program<'info, System> // system program
    }
}
