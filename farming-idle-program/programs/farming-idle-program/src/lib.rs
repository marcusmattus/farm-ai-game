use anchor_lang::prelude::*;
use anchor_lang::solana_program::log::sol_log_compute_units;

use instructions::harvest_instruction::*;
use instructions::initialize_farm_instruction::*;
use instructions::initialize_leaderboard_instruction::*;
use instructions::submit_farm_instruction::*;
use instructions::upgrade_farm_instruction::*;

declare_id!("RkoKjJ7UVatbVegugEjq11Q5agPynBAZV2VhPrNp5kH");

// Globals
mod globals;

// Instructions
mod instructions;

// Accounts
mod pdas;

#[program]
pub mod farming_idle_program {
    use crate::instructions::upgrade_farm_instruction::run_upgrade_farm;

    use super::*;

    pub fn initialize(ctx: Context<InitializeFarm>, bump: u8) -> Result<()> {
        run_initialize_farm(ctx, bump)?;
        sol_log_compute_units();

        Ok(())
    }

    pub fn initialize_leaderboard(ctx: Context<InitializeLeaderboard>) -> Result<()> {
        run_initialize_leaderboard(ctx)?;
        sol_log_compute_units();

        Ok(())
    }

    pub fn harvest(ctx: Context<Harvest>) -> Result<()> {
        run_harvest(ctx)?;
        sol_log_compute_units();

        Ok(())
    }

    pub fn upgrade_farm(ctx: Context<UpgradeFarm>, upgrade_index: u8, amount: u8) -> Result<()> {
        run_upgrade_farm(ctx, upgrade_index, amount)?;
        sol_log_compute_units();

        Ok(())
    }

    pub fn submit_farm(ctx: Context<SubmitFarm>) -> Result<()> {
        run_submit_farm(ctx)?;
        sol_log_compute_units();

        Ok(())
    }
}
