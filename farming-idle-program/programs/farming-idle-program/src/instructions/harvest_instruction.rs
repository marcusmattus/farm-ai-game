use crate::globals::constants::BASE_POINTS;
use anchor_lang::prelude::*;

use crate::pdas::farm_account::Farm;

#[derive(Accounts)]
pub struct Harvest<'info> {
    #[account(
        mut,
        has_one = player,
    )]
    pub farm: Account<'info, Farm>,

    #[account(mut)]
    pub player: Signer<'info>,
}

pub fn run_harvest(ctx: Context<Harvest>) -> Result<()> {
    let farm = &mut ctx.accounts.farm;
    let now = Clock::get()?.unix_timestamp;
    let mut points_to_add: u64 = 1;

    // TODO: Verify that address of farm === PDA derived from seeds ['farm', player, farm.owner]
    // This verifies that the farm account is the same farm account created by the inital owner-player initialization

    // automated clickers
    {
        let seconds_passed = now - farm.last_harvested;

        for i in 0..=BASE_POINTS.len() - 1 {
            points_to_add = points_to_add.saturating_add(
                farm.farm_upgrades[i] as u64 * BASE_POINTS[i] * seconds_passed as u64,
            )
        }
    }

    {
        farm.last_harvested = now;
        farm.harvest_points = farm.harvest_points.saturating_add(points_to_add);
    }

    Ok(())
}
