use anchor_lang::prelude::*;
// use anchor_spl::{associated_token::AssociatedToken, token::{transfer, Mint, Token, TokenAccount, Transfer},};
use anchor_spl::{token::{Mint, Token, TokenAccount, Transfer, transfer}, associated_token::{AssociatedToken}};

declare_id!("AiXuPBF1sDDthJjzdph6k43Rva2WdPW7kMjAmWxYhPNS");

#[program]
pub mod roulette {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, telegram_id:u64, revolver_size:u64, min_bet:u64, /*hashed_bullet_chamber_index: Vec<u8>,*/playerList:Vec<Pubkey>, bets: Vec<u64>, in_progress: bool, loser: u16) -> Result<()> {
        let game = &mut ctx.accounts.game;
        let players = ctx.remaining_accounts;
        game.telegram_id = telegram_id;
        game.revolver_size = revolver_size;
        game.min_bet = min_bet;
        // game.hashed_bullet_chamber_index = hashed_bullet_chamber_index;
        game.bets = bets;
        game.in_progress = in_progress;
        game.loser = loser;
        game.playerList = playerList;
        msg!("Game data saved");
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    sender: Signer<'info>,
    #[account(
        init,
        seeds = [b"example", sender.key().as_ref()],
        bump, 
        payer = sender, 
        space = 8 + 8 + 8 + 256 + 256 + 256 + 1 + 2 + 256
    )]
    pub game: Account<'info, Game>,
    system_program: Program<'info, System>,
}

#[account]
pub struct Game {
    pub telegram_id: u64,
    pub revolver_size: u64,
    pub min_bet: u64,
    // pub hashed_bullet_chamber_index: Vec<u8>,
    pub playerList: Vec<Pubkey>,
    pub bets: Vec<u64>,
    pub in_progress: bool,
    pub loser: u16,
}