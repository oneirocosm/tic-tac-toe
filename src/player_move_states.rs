use crate::board::Board;
use crate::coordinate::Coordinate;
use crate::errors::TttError;

pub trait PlayerMoveState {
    fn run(&self, board: &mut Board) -> Result<Box<dyn PlayerMoveState>, TttError>;
}

pub struct PlayerMoveRequest { player_id: u8 }

struct PlayerMoveReRequest{
    player_id: u8,
    input_err: TttError,
}

struct PlayerMoveParse {
    player_id: u8,
    text: String,
}

struct PlayerMoveCheck {
    player_id: u8,
    coord: Coordinate<i8>,
}

impl PlayerMoveRequest {
    pub fn new(player_id: u8) -> Self {
        Self { player_id }
    }
}

impl PlayerMoveReRequest {
    pub fn new(player_id: u8, input_err: TttError) -> Self {
        Self {
            player_id,
            input_err,
        }
    }
}

impl PlayerMoveParse {
    pub fn new(player_id: u8, text: String) -> Self {
        Self {
            player_id,
            text,
        }
    }
}


impl PlayerMoveCheck {
    pub fn new(player_id: u8, coord: Coordinate<i8>) -> Self {
        Self {
            player_id,
            coord,
        }
    }
}

impl PlayerMoveState for PlayerMoveRequest {
    fn run(&self, board: &mut Board) -> Result<Box<dyn PlayerMoveState>, TttError> {
        let input = &mut String::new();
        let name = board.get_name(self.player_id)?;
        println!("\nThe current board state is:\n");
        board.display()?;
        println!("\nP{}: {} is up next.", self.player_id, name);
        println!("Please enter your next move (e.g. a1, b2):");
        std::io::stdin()
            .read_line(input)
            .expect("Unable to read input");
        Ok(Box::new(PlayerMoveParse::new(self.player_id, input.clone())))
    }
}

impl PlayerMoveState for PlayerMoveReRequest {
    fn run(&self, board: &mut Board) -> Result<Box<dyn PlayerMoveState>, TttError> {
        let input = &mut String::new();  
        let name = board.get_name(self.player_id)?;
        println!("P{}: {} {}, please enter a valid space: ", self.player_id, self.input_err, name);
        std::io::stdin()
            .read_line(input)
            .expect("Unable to read input");
        Ok(Box::new(PlayerMoveParse::new(self.player_id, input.clone())))
    }
}


impl PlayerMoveState for PlayerMoveParse {
    fn run(&self, _board: &mut Board) -> Result<Box<dyn PlayerMoveState>, TttError> {
        let trimmed = self.text.trim();
        let (col, row) = trimmed.split_at(1);
        let x = match col.to_lowercase().as_str() {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            _ => return Ok(Box::new(PlayerMoveReRequest::new(self.player_id, TttError::InvalidEntry(trimmed.to_string())))),
        };
        let y = match row {
            "1" => 0,
            "2" => 1,
            "3" => 2,
            _ => return Ok(Box::new(PlayerMoveReRequest::new(self.player_id, TttError::InvalidEntry(trimmed.to_string())))),
        };
        Ok(Box::new(PlayerMoveCheck::new(self.player_id, Coordinate::new(y, x))))
    }
}

impl PlayerMoveState for PlayerMoveCheck {
    fn run(&self, board: &mut Board) -> Result<Box<dyn PlayerMoveState>, TttError> {
        match board.update(self.player_id, self.coord) {
            Err(TttError::UsedSpace(coord)) => Ok(Box::new(PlayerMoveReRequest::new(self.player_id, TttError::UsedSpace(coord)))),
            Err(err) => Err(err),
            Ok(()) => Err(TttError::StatesOver),
        }
    }
}