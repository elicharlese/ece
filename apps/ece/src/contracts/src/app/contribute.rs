use anchor_lang::prelude::*;
use std::collections::HashMap;

declare_id!("YourProgramIDHere");

#[program]
pub mod contribute {
    use super::*;

    pub fn initialize_campaign(ctx: Context<InitializeCampaign>, id: u64, name: String, description: String, target_amount: u128, start_date: u64, end_date: u64) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        campaign.id = id;
        campaign.name = name;
        campaign.description = description;
        campaign.target_amount = target_amount;
        campaign.current_amount = 0;
        campaign.start_date = start_date;
        campaign.end_date = end_date;
        campaign.owner = ctx.accounts.owner.key().to_string();
        Ok(())
    }

    pub fn fund_campaign(ctx: Context<FundCampaign>, amount: u128) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        if campaign.end_date > now() {
            campaign.current_amount += amount;
            msg!("Funded campaign {} with amount {}", campaign.id, amount);
        } else {
            return Err(ErrorCode::CampaignEnded.into());
        }
        Ok(())
    }

    fn now() -> u64 {
        Clock::get()?.unix_timestamp as u64
    }
}

#[account]
pub struct Campaign {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub target_amount: u128,
    pub current_amount: u128,
    pub start_date: u64,
    pub end_date: u64,
    pub owner: String,
}

#[derive(Accounts)]
pub struct InitializeCampaign<'info> {
    #[account(init, payer = owner, space = 8 + std::mem::size_of::<Campaign>())]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FundCampaign<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
}

#[error]
pub enum ErrorCode {
    #[msg("The campaign has already ended.")]
    CampaignEnded,
}