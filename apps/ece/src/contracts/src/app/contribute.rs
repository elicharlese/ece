use anchor_lang::prelude::*;
use std::collections::HashMap;

<<<<<<< HEAD
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
=======
// Structure to hold information about your startups
struct MyStartup {
    id: u64,
    name: String,
    description: String,
    target_amount: u128,
    current_amount: u128,
    start_date: u64,
    end_date: u64,
    owner: String,
}

impl MyStartup {
    fn new(id: u64, name: String, description: String, target_amount: u128, current_amount: u128, start_date: u64, end_date: u64, owner: String) -> Self {
        MyStartup {
            id,
            name,
            description,
            target_amount,
            current_amount,
            start_date,
            end_date,
            owner,
        }
    }

    fn fund(&mut self, amount: u128) {
        if self.end_date > now() {
            self.current_amount += amount;
        }
    }
}

// Structure to hold information about startups you sponsor
struct SponsoredStartup {
    id: u64,
    name: String,
    description: String,
    target_amount: u128,
    current_amount: u128,
    start_date: u64,
    end_date: u64,
    sponsor: String,
}

impl SponsoredStartup {
    fn new(id: u64, name: String, description: String, target_amount: u128, current_amount: u128, start_date: u64, end_date: u64, sponsor: String) -> Self {
        SponsoredStartup {
            id,
            name,
            description,
            target_amount,
            current_amount,
            start_date,
            end_date,
            sponsor,
>>>>>>> a99842ae8ecbc6004289317e6f3875bd16d6b7ef
        }
        Ok(())
    }

<<<<<<< HEAD
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
=======
    fn fund(&mut self, amount: u128) {
        if self.end_date > now() {
            self.current_amount += amount;
        }
    }
}

// Existing FreelanceRequest structure can be used for freelance requests
// No changes needed for FreelanceRequest

// Hypothetical function to get the current time
fn now() -> u64 {
    Utc::now().timestamp() as u64
}

// New section for Freelance Quote Requests
struct FreelanceRequest {
    user_id: u64,
    project_name: String,
    project_description: String,
    budget: u128,
    deadline: u64,
}

impl FreelanceRequest {
    // Initializes a new freelance request
    fn new(user_id: u64, project_name: String, project_description: String, budget: u128, deadline: u64) -> Self {
        FreelanceRequest {
            user_id,
            project_name,
            project_description,
            budget,
            deadline,
        }
    }

    // Function to generate a quote
    fn generate_quote(&self) -> String {
        // Placeholder logic for generating a quote
        format!(
            "Quote for project '{}': Estimated cost is {} with a deadline of {}.",
            self.project_name, self.budget, self.deadline
        )
    }
>>>>>>> a99842ae8ecbc6004289317e6f3875bd16d6b7ef
}