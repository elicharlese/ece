use anchor_lang::prelude::*;
use anchor_lang::solana_program::{pubkey::Pubkey, program::invoke};
use std::collections::HashMap;

declare_id!("YourProgramIDHere");

#[account]
pub struct NanoNodeSettings {
    pub version: String,
    pub strategy: String,
    pub strategy_file: String,
    pub network: String,
}

#[account]
pub struct NanoNode {
    pub settings: HashMap<Pubkey, NanoNodeSettings>,
    pub terminal: HashMap<Pubkey, String>,
    pub output_log: HashMap<Pubkey, String>,
    pub statistics: HashMap<Pubkey, String>,
}

#[program]
pub mod nano_node {
    use super::*;

    pub fn initialize(ctx: Context<InitializeNanoNode>) -> Result<()> {
        let nano_node = &mut ctx.accounts.nano_node;
        nano_node.settings = HashMap::new();
        nano_node.terminal = HashMap::new();
        nano_node.output_log = HashMap::new();
        nano_node.statistics = HashMap::new();
        Ok(())
    }

    pub fn set_settings(ctx: Context<SetSettings>, account_id: Pubkey, version: String, strategy: String, strategy_file: String, network: String) -> Result<()> {
        let nano_node = &mut ctx.accounts.nano_node;
        let settings = NanoNodeSettings {
            version,
            strategy,
            strategy_file,
            network,
        };
        nano_node.settings.insert(account_id, settings);
        Ok(())
    }

    pub fn get_settings(ctx: Context<GetSettings>, account_id: Pubkey) -> Result<Option<NanoNodeSettings>> {
        let nano_node = &ctx.accounts.nano_node;
        Ok(nano_node.settings.get(&account_id).cloned())
    }

    pub fn set_terminal(ctx: Context<SetTerminal>, account_id: Pubkey, terminal: String) -> Result<()> {
        let nano_node = &mut ctx.accounts.nano_node;
        nano_node.terminal.insert(account_id, terminal);
        Ok(())
    }

    pub fn get_terminal(ctx: Context<GetTerminal>, account_id: Pubkey) -> Result<Option<String>> {
        let nano_node = &ctx.accounts.nano_node;
        Ok(nano_node.terminal.get(&account_id).cloned())
    }

    pub fn set_output_log(ctx: Context<SetOutputLog>, account_id: Pubkey, output_log: String) -> Result<()> {
        let nano_node = &mut ctx.accounts.nano_node;
        nano_node.output_log.insert(account_id, output_log);
        Ok(())
    }

    pub fn get_output_log(ctx: Context<GetOutputLog>, account_id: Pubkey) -> Result<Option<String>> {
        let nano_node = &ctx.accounts.nano_node;
        Ok(nano_node.output_log.get(&account_id).cloned())
    }

    pub fn set_statistics(ctx: Context<SetStatistics>, account_id: Pubkey, statistics: String) -> Result<()> {
        let nano_node = &mut ctx.accounts.nano_node;
        nano_node.statistics.insert(account_id, statistics);
        Ok(())
    }

    pub fn get_statistics(ctx: Context<GetStatistics>, account_id: Pubkey) -> Result<Option<String>> {
        let nano_node = &ctx.accounts.nano_node;
        Ok(nano_node.statistics.get(&account_id).cloned())
    }
}

#[derive(Accounts)]
pub struct InitializeNanoNode<'info> {
    #[account(init, payer = user, space = 8 + std::mem::size_of::<NanoNode>())]
    pub nano_node: Account<'info, NanoNode>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetSettings<'info> {
    #[account(mut)]
    pub nano_node: Account<'info, NanoNode>,
}

#[derive(Accounts)]
pub struct GetSettings<'info> {
    pub nano_node: Account<'info, NanoNode>,
}

#[derive(Accounts)]
pub struct SetTerminal<'info> {
    #[account(mut)]
    pub nano_node: Account<'info, NanoNode>,
}

#[derive(Accounts)]
pub struct GetTerminal<'info> {
    pub nano_node: Account<'info, NanoNode>,
}

#[derive(Accounts)]
pub struct SetOutputLog<'info> {
    #[account(mut)]
    pub nano_node: Account<'info, NanoNode>,
}

#[derive(Accounts)]
pub struct GetOutputLog<'info> {
    pub nano_node: Account<'info, NanoNode>,
}

#[derive(Accounts)]
pub struct SetStatistics<'info> {
    #[account(mut)]
    pub nano_node: Account<'info, NanoNode>,
}

#[derive(Accounts)]
pub struct GetStatistics<'info> {
    pub nano_node: Account<'info, NanoNode>,
}