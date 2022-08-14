use std::{error::Error, fmt};
use crate::coordinate::Coordinate;

#[derive(Debug, Clone)]
pub enum TttError {
    InvalidEntry(String),
    UsedSpace(Coordinate<i8>),
    DuplicateEntry(Coordinate<i8>),
    NoPlayer(u8),
    StatesOver,
}

impl fmt::Display for TttError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            TttError::InvalidEntry(entry) => format!("Entry \"{}\" is not a valid input.", entry),
            TttError::UsedSpace(entry) => format!("Coord \"{}\" has already been used.", entry),
            TttError::DuplicateEntry(coord) => format!(
                "Fatal Logic Error.  Coord {} was duplicated.  Debug required",
                coord
            ),
            TttError::NoPlayer(player_id) => format!("No player with id {}", player_id),
            TttError::StatesOver => String::from("Game over\n"),
        };
        write!(f, "{}", msg)
    }
}

impl Error for TttError {}