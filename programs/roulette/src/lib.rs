use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token};
use crate::borsh::maybestd::collections::HashMap;

declare_id!("9bBoqfPybm2uKfswK8NHysk4L8sa5UzMJeMmSbxq6J5t");


#[program]
pub mod roulette {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, revolver_size:u64) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.revolver_size = revolver_size;
        msg!("Counter account created. Current count: {}", game.revolver_size);
        Ok(())
    }


    // impl<T> Game {
    //     pub fn new_game(ctx: &Context<'_, '_, '_, '_, T>, tg_chat_id: u64, revolver_size: u64, min_bet: u64, hashed_bullet_chamber_index: Vec<u8>, players: Vec<Pubkey>, bets: Vec<u64>) -> Result<()> {
    //         let mut game = Game {
    //             revolver_size,
    //             min_bet,
    //             hashed_bullet_chamber_index,
    //             players,
    //             bets,
    //             in_progress: true,
    //             loser: 0,
    //         };
    //         ctx.accounts.game_data.data.borrow_mut().clone_from(&game);
    //         Ok(())
    //     }

    //     pub fn end_game(ctx: &Context<'_, '_, '_, '_, T>, tg_chat_id: u64, loser: u16) -> Result<()> {
    //         let game = &mut ctx.accounts.game_data.data.borrow_mut();
    //         game.loser = loser;
    //         game.in_progress = false;
    //         Ok(())
    //     }

    //     pub fn abort_game(ctx: &Context<'_, '_, '_, '_, T>) -> Result<()> {
    //         let game = &mut ctx.accounts.game_data.data.borrow_mut();
    //         game.in_progress = false;
    //         Ok(())
    //     }
    // }

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     let program = &mut ctx.program;
    //     program.owner = ctx.accounts.owner.key();
    //     program.revenue_wallet = ctx.accounts.revenue_wallet.key();
    //     program.betting_token_mint = ctx.accounts.betting_token_account.key();
    //     program.minimum_bet = ctx.accounts.minimum_bet;
    //     program.revenue_bps = ctx.accounts.revenue_bps;
    //     program.burn_bps = ctx.accounts.burn_bps;
    //     program.games = HashMap::new();
    //     Ok(())
    // }

    // pub fn new_game(ctx: Context<NewGame>) -> Result<()> {
    //     Game::new_game(&ctx, ctx.accounts.new_game.tg_chat_id, ctx.accounts.new_game.revolver_size, ctx.accounts.new_game.min_bet, ctx.accounts.new_game.hashed_bullet_chamber_index, ctx.accounts.new_game.players, ctx.accounts.new_game.bets)
    // }

    // pub fn end_game(ctx: Context<EndGame>) -> Result<()> {
    //     Game::end_game(&ctx, ctx.accounts.end_game.tg_chat_id, ctx.accounts.end_game.loser)
    // }

    // pub fn abort_game(ctx: Context<AbortGame>) -> Result<()> {
    //     Game::abort_game(&ctx)
    // }

    // pub fn abort_all_games(ctx: Context<AbortAllGames>) -> Result<()> {
    //     // Implement logic to abort all games
    // }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8 + 8 + 256 + 256 + 256 + 1 + 2)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct Game {
    pub revolver_size: u64,
    pub min_bet: u64,
    pub hashed_bullet_chamber_index: Vec<u8>,
    pub players: Vec<Pubkey>,
    pub bets: Vec<u64>,
    pub in_progress: bool,
    pub loser: u16,
}

// #[derive(Accounts)]
// pub struct NewGame<'info> {
//     #[account(mut)]
//     pub owner: Signer<'info>,
//     pub system_program: Program<'info, System>,
//     pub token_program: Program<'info, Token>,
//     pub betting_token_account: AccountInfo<'info>,
//     pub revenue_wallet: AccountInfo<'info>,
//     pub game_data: Program<'info, Game>,
// }

// #[derive(Accounts)]
// pub struct EndGame<'info> {
//     #[account(mut)]
//     pub owner: Signer<'info>,
//     pub game_data: Program<'info, Game>,
// }

// #[derive(Accounts)]
// pub struct AbortGame<'info> {
//     #[account(mut)]
//     pub owner: Signer<'info>,
//     pub game_data: Program<'info, Game>,
// }

// #[derive(Accounts)]
// pub struct AbortAllGames<'info> {
//     #[account(mut)]
//     pub owner: Signer<'info>,
// }

