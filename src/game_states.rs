use crate::board::Board;
use crate::errors::TttError;
use crate::player_move_states::{PlayerMoveState, PlayerMoveRequest};

const MAX_PLAYERS: u8 = 2;

pub trait GameState {
    fn run(&self, board: &mut Board) -> Result<(), TttError>;
    fn next(&self, board: &Board) -> Result<Box<dyn GameState>, TttError>;
}

pub struct GameEnterInfo {
    player_id: u8,
}

pub struct GamePlayerMove {
    player_id: u8,
}

struct GamePlayerWin {
    player_id: u8,
}
struct GamePlayerDraw {}

impl GameEnterInfo {
    pub fn new(player_id: u8) -> Self {
        Self { player_id }
    }

    fn request_name(&self) -> String {
        println!("Player {}: Please enter your name:", self.player_id);
        let mut name = String::new();
        std::io::stdin()
            .read_line(&mut name)
            .expect("Unable to read name provided");
        String::from(name.trim())
    }
}

impl GameState for GameEnterInfo {
    fn run(&self, board: &mut Board) -> Result<(), TttError> {
        let name = self.request_name();
        board.names.insert(self.player_id, name);
        Ok(())
    }

    fn next(&self, _board: &Board) -> Result<Box<dyn GameState>, TttError> {
        if self.player_id < MAX_PLAYERS {
            Ok(Box::new(GameEnterInfo::new(self.player_id + 1)))
        } else {
            Ok(Box::new(GamePlayerMove::new(1)))
        }
    }
}

impl GamePlayerMove {
    pub fn new(player_id: u8) -> Self {
        Self {
            player_id,
        }
    }
}

impl GameState for GamePlayerMove {
    fn run(&self, board: &mut Board) -> Result<(), TttError> {
        let mut player_state: Box<dyn PlayerMoveState> = Box::new(PlayerMoveRequest::new(self.player_id));
        loop {
            match player_state.run(board) {
                Err(TttError::StatesOver) => return Ok(()),
                Err(e) => return Err(e),
                Ok(state) => {player_state = state;},
            }
        }
    }

    fn next(&self, board: &Board) -> Result<Box<dyn GameState>, TttError> {
        let next_id = (self.player_id % MAX_PLAYERS) + 1;
        match (board.check_win(self.player_id)?, board.is_full()?) {
            (true, _) => Ok(Box::new(GamePlayerWin::new(self.player_id))),
            (false, true) => Ok(Box::new(GamePlayerDraw::new())),
            (false, false) => Ok(Box::new(GamePlayerMove::new(next_id))),
        }
    }
}

impl GamePlayerWin {
    pub fn new(player_id: u8) -> Self {
        Self { player_id }
    }
}

impl GameState for GamePlayerWin {
    fn run(&self, board: &mut Board) -> Result<(), TttError> {
        let name = board.get_name(self.player_id)?;
        println!("\n\nP{}: {} is the winner!", self.player_id, name);
        println!("The final board state is:\n");
        board.display()?;
        println!();
        Ok(())
    }

    fn next(&self, _board: &Board) -> Result<Box<dyn GameState>, TttError> {
        Err(TttError::StatesOver)
    }
}

impl GamePlayerDraw {
    pub fn new() -> Self {
        Self {}
    }
}

impl GameState for GamePlayerDraw {
    fn run(&self, board: &mut Board) -> Result<(), TttError> {
        println!("\n\nThe game ended in a draw!");
        println!("The final board state is:\n");
        board.display()?;
        println!();
        Ok(())
    }

    fn next(&self, _board: &Board) -> Result<Box<dyn GameState>, TttError> {
        Err(TttError::StatesOver)
    }
}
