use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};


pub const ANCHOR_DISCRIMATOR_SIZE: usize = 8;
#[cfg(test)]
mod tests;

declare_id!("B1TUGS5xdcHujsxBSq1MNDmiVsyzCKa2Y91nQDSouFrS");


#[error_code]
pub enum GameError {
    #[msg("Invalid player")]
    InvalidPlayer,

    #[msg("Position already taken")]
    PositionAlreadyTaken,

    #[msg("Game already finished")]
    GameAlreadyFinished,

    #[msg("Not player's turn")]
    NotPlayersTurn,

      #[msg("Invalid position")]
    InvalidPosition,
}
#[program]
pub mod block_tac {
    use super::*;

    pub fn init_game(ctx:Context<InitGame>,
         _game_id: u64,
        ready_player_o:Pubkey,
    ) -> Result<()> {

            let game_account = &mut ctx.accounts.game_account;
            game_account.ready_player_x = ctx.accounts.signer.key();
            game_account.ready_player_o = ready_player_o;
            game_account.board = [0; 9];
            game_account.game_state = GameState::XMove;

        Ok(())
    }

    pub fn next_move(ctx:Context<NextMove>, position:u8)->Result<()> {
                let game_account = &mut ctx.accounts.game_account;
                let player = ctx.accounts.player.key();

// validate player
                if player != game_account.ready_player_x && player != game_account.ready_player_o {
             return Err(GameError::InvalidPlayer.into());
                }
        // validate position:
                if position >= 9 {
                    return Err(GameError::InvalidPosition.into());
                }
    
    // position already taken
                if game_account.board[position as usize] != 0 {
                    return Err(GameError::PositionAlreadyTaken.into());
                }

                // validate the right player's turn
                match game_account.game_state {
                    GameState::XMove => {
                        if player != game_account.ready_player_x {
                            return Err(GameError::NotPlayersTurn.into());
                        }
                    }
                    GameState::OMove => {
                        if player != game_account.ready_player_o {
                            return Err(GameError::NotPlayersTurn.into());
                        }
                    }
                    _ => {}
                }

        
        match game_account.game_state {
    GameState::XMove => {
        game_account.board[position as usize] = 1;
    }
    GameState::OMove => {
        game_account.board[position as usize] = 2;
    }
    _ => {
        
    }
}

        // call check winner 
        
        // update game state: 
             match game_account.game_state {
    GameState::XMove => {
        game_account.game_state = GameState::OMove;
    }
    GameState::OMove => {
        game_account.game_state = GameState::XMove;
    }
    _ => {
        
    }
}

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


#[derive(Accounts)]
pub struct NextMove<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    
    #[account(mut)]
    pub game_account: Account<'info, GameAccount>,
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
    XMove,
    OMove,
    Draw,
}


