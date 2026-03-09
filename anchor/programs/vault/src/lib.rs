use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};


pub const ANCHOR_DISCRIMATOR_SIZE: usize = 8;
#[cfg(test)]
mod tests;

declare_id!("B1TUGS5xdcHujsxBSq1MNDmiVsyzCKa2Y91nQDSouFrS");


#[program]
pub mod vault {
    use super::*;

    pub fn init_game(ctx:Context<InitGame>,
         game_id: u64,
        ready_player_o:Pubkey,
    ) -> Result<()> {
        
            let game_account = &mut ctx.accounts.game_account;
            game_account.ready_player_x = ctx.accounts.signer.key();
            game_account.ready_player_o = ready_player_o;
            game_account.board = [0; 9];
            game_account.game_state = GameState::Waiting;

        Ok(())
    }


    }



#[derive(Accounts)]
#[instruction(game_id: u64)]
pub struct InitGame<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = ANCHOR_DISCRIMATOR_SIZE + GameAccount::INIT_SPACE,
        seeds = [b"game", game_id.to_le_bytes().as_ref()],
        bump
    )]


    pub game_account: Account<'info, GameAccount>,
     pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct GameAccount {
    pub ready_player_x: Pubkey,
    pub ready_player_o: Pubkey,
    pub board: [u8; 9],
    pub game_state: GameState,
}
  


#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]

pub enum GameState {
    Waiting,
    InProgress,
    XWon,
    OWon,
    Draw,
}