use anchor_lang::prelude::*;
use std::collections::HashMap;

declare_id!("YourProgramIDHere");

#[account]
pub struct ThePublic {
    pub data_storage: HashMap<Pubkey, String>,
    pub posts: HashMap<u64, Post>,
    pub topics: HashMap<u64, Topic>,
    pub users: HashMap<Pubkey, User>,
    pub network_connections: HashMap<Pubkey, String>,
    pub oracle_account: Pubkey,
    pub moderators: Vec<Pubkey>,
    pub post_count: u64,
    pub topic_count: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Post {
    pub account_id: Pubkey,
    pub content: String,
    pub timestamp: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Topic {
    pub account_id: Pubkey,
    pub title: String,
    pub timestamp: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct User {
    pub account_id: Pubkey,
    pub username: String,
    pub connected_device: Option<String>,
    pub privacy_settings: PrivacySettings,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PrivacySettings {
    pub show_posts: bool,
    pub show_topics: bool,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            show_posts: true,
            show_topics: true,
        }
    }
}

impl Default for ThePublic {
    fn default() -> Self {
        Self {
            data_storage: HashMap::new(),
            posts: HashMap::new(),
            topics: HashMap::new(),
            users: HashMap::new(),
            network_connections: HashMap::new(),
            oracle_account: Pubkey::default(),
            moderators: vec![Pubkey::default()],
            post_count: 0,
            topic_count: 0,
        }
    }
}

#[program]
pub mod the_public {
    use super::*;

    #[init]
    pub fn new(ctx: Context<InitializeThePublic>, oracle_account: Pubkey) -> Result<()> {
        let the_public = &mut ctx.accounts.the_public;
        the_public.data_storage = HashMap::new();
        the_public.posts = HashMap::new();
        the_public.topics = HashMap::new();
        the_public.users = HashMap::new();
        the_public.network_connections = HashMap::new();
        the_public.oracle_account = oracle_account;
        the_public.moderators = vec![ctx.accounts.owner.key()];
        the_public.post_count = 0;
        the_public.topic_count = 0;
        Ok(())
    }

    pub fn add_post(ctx: Context<ModifyPost>, account_id: Pubkey, content: String) -> Result<()> {
        let the_public = &mut ctx.accounts.the_public;
        let post = Post {
            account_id,
            content,
            timestamp: Clock::get()?.unix_timestamp as u64,
        };
        the_public.posts.insert(the_public.post_count, post);
        the_public.post_count += 1;
        Ok(())
    }

    pub fn add_topic(ctx: Context<ModifyTopic>, account_id: Pubkey, title: String) -> Result<()> {
        let the_public = &mut ctx.accounts.the_public;
        let topic = Topic {
            account_id,
            title,
            timestamp: Clock::get()?.unix_timestamp as u64,
        };
        the_public.topics.insert(the_public.topic_count, topic);
        the_public.topic_count += 1;
        Ok(())
    }

    pub fn moderate_post(ctx: Context<ModeratePost>, post_id: u64) -> Result<()> {
        let caller = ctx.accounts.owner.key();
        let the_public = &mut ctx.accounts.the_public;
        require!(the_public.moderators.contains(&caller), ErrorCode::Unauthorized);
        the_public.posts.remove(&post_id);
        Ok(())
    }

    pub fn moderate_topic(ctx: Context<ModerateTopic>, topic_id: u64) -> Result<()> {
        let caller = ctx.accounts.owner.key();
        let the_public = &mut ctx.accounts.the_public;
        require!(the_public.moderators.contains(&caller), ErrorCode::Unauthorized);
        the_public.topics.remove(&topic_id);
        Ok(())
    }

    pub fn register_user(ctx: Context<ModifyUser>, account_id: Pubkey, username: String) -> Result<()> {
        let the_public = &mut ctx.accounts.the_public;
        let user = User {
            account_id,
            username,
            connected_device: None,
            privacy_settings: PrivacySettings::default(),
        };
        the_public.users.insert(account_id, user);
        Ok(())
    }

    pub fn update_privacy_settings(ctx: Context<ModifyUser>, account_id: Pubkey, show_posts: bool, show_topics: bool) -> Result<()> {
        let the_public = &mut ctx.accounts.the_public;
        let user = the_public.users.get_mut(&account_id).ok_or(ErrorCode::UserNotFound)?;
        user.privacy_settings = PrivacySettings {
            show_posts,
            show_topics,
        };
        Ok(())
    }

    pub fn connect_to_network(ctx: Context<ModifyUser>, account_id: Pubkey, network: String) -> Result<()> {
        let the_public = &mut ctx.accounts.the_public;
        the_public.network_connections.insert(account_id, network);
        Ok(())
    }

    pub fn disconnect_from_network(ctx: Context<ModifyUser>, account_id: Pubkey) -> Result<()> {
        let the_public = &mut ctx.accounts.the_public;
        the_public.network_connections.remove(&account_id);
        Ok(())
    }

    pub fn request_data_from_oracle(ctx: Context<RequestData>, query: String) -> Result<()> {
        // Implement oracle request logic here
        Ok(())
    }

    pub fn oracle_callback(ctx: Context<OracleCallback>, account_id: Pubkey, data: String) -> Result<()> {
        let the_public = &mut ctx.accounts.the_public;
        the_public.data_storage.insert(account_id, data);
        Ok(())
    }

    pub fn get_data(ctx: Context<GetData>, account_id: Pubkey) -> Result<Option<String>> {
        let the_public = &ctx.accounts.the_public;
        Ok(the_public.data_storage.get(&account_id).cloned())
    }

    pub fn get_posts(ctx: Context<GetPosts>, from: u64, to: u64) -> Result<Vec<Post>> {
        let the_public = &ctx.accounts.the_public;
        let posts: Vec<Post> = (from..to)
            .filter_map(|id| the_public.posts.get(&id).cloned())
            .collect();
        Ok(posts)
    }

    pub fn get_topics(ctx: Context<GetTopics>, from: u64, to: u64) -> Result<Vec<Topic>> {
        let the_public = &ctx.accounts.the_public;
        let topics: Vec<Topic> = (from..to)
            .filter_map(|id| the_public.topics.get(&id).cloned())
            .collect();
        Ok(topics)
    }
}

#[derive(Accounts)]
pub struct InitializeThePublic<'info> {
    #[account(init, payer = owner, space = 8 + std::mem::size_of::<ThePublic>())]
    pub the_public: Account<'info, ThePublic>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyPost<'info> {
    pub the_public: Account<'info, ThePublic>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ModifyTopic<'info> {
    pub the_public: Account<'info, ThePublic>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ModeratePost<'info> {
    pub the_public: Account<'info, ThePublic>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ModerateTopic<'info> {
    pub the_public: Account<'info, ThePublic>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ModifyUser<'info> {
    pub the_public: Account<'info, ThePublic>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct RequestData<'info> {
    pub the_public: Account<'info, ThePublic>,
}

#[derive(Accounts)]
pub struct OracleCallback<'info> {
    pub the_public: Account<'info, ThePublic>,
}

#[derive(Accounts)]
pub struct GetData<'info> {
    pub the_public: Account<'info, ThePublic>,
}

#[derive(Accounts)]
pub struct GetPosts<'info> {
    pub the_public: Account<'info, ThePublic>,
}

#[derive(Accounts)]
pub struct GetTopics<'info> {
    pub the_public: Account<'info, ThePublic>,
}

#[error]
pub enum ErrorCode {
    #[msg("User not found.")]
    UserNotFound,
    #[msg("Unauthorized action.")]
    Unauthorized,
}