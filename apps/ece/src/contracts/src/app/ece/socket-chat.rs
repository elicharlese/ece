use anchor_lang::prelude::*;
use std::collections::HashMap;

declare_id!("YourProgramIDHere");

#[account]
pub struct SocketChat {
    pub owner: Pubkey,
    pub participants: Vec<Pubkey>,
    pub messages: Vec<Message>,
    pub notes: Vec<Note>,
    pub files: Vec<File>,
    pub comments: Vec<Comment>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Message {
    pub sender: Pubkey,
    pub content: String,
    pub timestamp: u64,
    pub file_id: Option<u64>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Note {
    pub content: String,
    pub reference_url: Option<String>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct File {
    pub id: u64,
    pub name: String,
    pub content: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Comment {
    pub file_id: u64,
    pub line_number: u64,
    pub content: String,
    pub author: Pubkey,
}

#[program]
pub mod socket_chat {
    use super::*;

    pub fn new(ctx: Context<InitializeSocketChat>, owner: Pubkey) -> Result<()> {
        let socket_chat = &mut ctx.accounts.socket_chat;
        socket_chat.owner = owner;
        socket_chat.participants = Vec::new();
        socket_chat.messages = Vec::new();
        socket_chat.notes = Vec::new();
        socket_chat.files = Vec::new();
        socket_chat.comments = Vec::new();
        Ok(())
    }

    pub fn start_socket_chat(ctx: Context<StartSocketChat>) -> Result<()> {
        msg!("SocketChat application started");
        Ok(())
    }

    pub fn handle_chat(ctx: Context<HandleChat>, user: Pubkey, message: String, file_id: Option<u64>) -> Result<()> {
        let socket_chat = &mut ctx.accounts.socket_chat;
        socket_chat.participants.push(user.clone());
        let new_message = Message {
            sender: user.clone(),
            content: message.clone(),
            timestamp: Clock::get()?.unix_timestamp as u64,
            file_id,
        };
        socket_chat.messages.push(new_message);
        msg!("User {} sent message: {}", user, message);
        Ok(())
    }

    pub fn view_chat(ctx: Context<ViewChat>) -> Result<Vec<Message>> {
        let socket_chat = &ctx.accounts.socket_chat;
        Ok(socket_chat.messages.clone())
    }

    pub fn add_file(ctx: Context<AddFile>, name: String, content: String) -> Result<u64> {
        let socket_chat = &mut ctx.accounts.socket_chat;
        let file_id = socket_chat.files.len() as u64;
        socket_chat.files.push(File { id: file_id, name, content });
        msg!("File added with ID: {}", file_id);
        Ok(file_id)
    }

    pub fn add_comment(ctx: Context<AddComment>, user: Pubkey, file_id: u64, line_number: u64, content: String) -> Result<()> {
        let socket_chat = &mut ctx.accounts.socket_chat;
        let new_comment = Comment { file_id, line_number, content, author: user };
        socket_chat.comments.push(new_comment);
        msg!("Comment added");
        Ok(())
    }

    pub fn view_file_with_comments(ctx: Context<ViewFileWithComments>, file_id: u64) -> Result<Option<(File, Vec<Comment>)>> {
        let socket_chat = &ctx.accounts.socket_chat;
        let file = socket_chat.files.iter().find(|f| f.id == file_id).cloned();
        let comments: Vec<Comment> = socket_chat.comments.iter().filter(|c| c.file_id == file_id).cloned().collect();
        if let Some(f) = file {
            Ok(Some((f, comments)))
        } else {
            Ok(None)
        }
    }

    pub fn add_note(ctx: Context<AddNote>, content: String, reference_url: Option<String>) -> Result<()> {
        let socket_chat = &mut ctx.accounts.socket_chat;
        let new_note = Note { content, reference_url };
        socket_chat.notes.push(new_note);
        msg!("Note added");
        Ok(())
    }

    pub fn add_reference(ctx: Context<AddReference>, url: String, description: String) -> Result<()> {
        let socket_chat = &mut ctx.accounts.socket_chat;
        let new_note = Note { content: description, reference_url: Some(url) };
        socket_chat.notes.push(new_note);
        msg!("Reference added");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeSocketChat<'info> {
    #[account(init, payer = user, space = 8 + std::mem::size_of::<SocketChat>())]
    pub socket_chat: Account<'info, SocketChat>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StartSocketChat<'info> {
    pub socket_chat: Account<'info, SocketChat>,
}

#[derive(Accounts)]
pub struct HandleChat<'info> {
    pub socket_chat: Account<'info, SocketChat>,
}

#[derive(Accounts)]
pub struct ViewChat<'info> {
    pub socket_chat: Account<'info, SocketChat>,
}

#[derive(Accounts)]
pub struct AddFile<'info> {
    pub socket_chat: Account<'info, SocketChat>,
}

#[derive(Accounts)]
pub struct AddComment<'info> {
    pub socket_chat: Account<'info, SocketChat>,
}

#[derive(Accounts)]
pub struct ViewFileWithComments<'info> {
    pub socket_chat: Account<'info, SocketChat>,
}

#[derive(Accounts)]
pub struct AddNote<'info> {
    pub socket_chat: Account<'info, SocketChat>,
}

#[derive(Accounts)]
pub struct AddReference<'info> {
    pub socket_chat: Account<'info, SocketChat>,
}