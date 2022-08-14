mod board;
mod coordinate;
mod errors;
mod game_states;
mod player_move_states;

use crate::board::Board;
use crate::errors::TttError;
use crate::game_states::{GameEnterInfo, GameState};

fn main() -> Result<(), TttError> {
    println!("Welcome to Tic-Tac-Toe!!!\n");
    let mut engine = GameEngine::new();
    engine.run()
}

struct GameEngine {
    state: Box<dyn GameState>,
    board: Board,
}

impl GameEngine {
    fn run(&mut self) -> Result<(), TttError> {
        loop {
            self.state.run(&mut self.board)?;
            match self.state.next(&self.board) {
                Err(TttError::StatesOver) => return Ok(()),
                Err(e) => {
                    println!("Fatal Error: {}", e);
                    return Err(e);
                },
                Ok(state) => {self.state = state;},
            }
        }
    }

    fn new() -> Self {
        Self {
            state: Box::new(GameEnterInfo::new(1)),
            board: Board::new(),
        }
    }
}
