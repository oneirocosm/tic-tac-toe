use std::collections::{BTreeSet, HashMap, HashSet};
use crate::coordinate::Coordinate;
use crate::errors::TttError;

const OUT_UP_LEFT: char = '\u{2554}';
const OUT_UP_RIGHT: char = '\u{2557}';
const OUT_DN_LEFT: char = '\u{255A}';
const OUT_DN_RIGHT: char = '\u{255D}';
const OUT_HORIZ_UNCONN: char = '\u{2550}';
const OUT_HORIZ_TOP_CONN: char = '\u{2564}';
const OUT_HORIZ_BTM_CONN: char = '\u{2567}';
const OUT_VERT_UNCONN: char = '\u{2551}';
const OUT_VERT_LEFT_CONN: char = '\u{255F}';
const OUT_VERT_RIGHT_CONN: char = '\u{2562}';
const IN_HORIZ_UNCONN: char = '\u{2500}';
const IN_VERT_UNCONN: char = '\u{2502}';
const IN_CONN: char = '\u{253C}';

#[derive(Default, Debug, Clone)]
pub struct Board {
    win_states: HashSet<BTreeSet<Coordinate<i8>>>,
    cur_state: HashMap<u8, BTreeSet<Coordinate<i8>>>,
    pub names: HashMap<u8, String>,
}

impl Board {
    pub fn new() -> Self {
        let top_row = BTreeSet::from([
            Coordinate::new(0, 0),
            Coordinate::new(0, 1),
            Coordinate::new(0, 2),
        ]);
        let mid_row = BTreeSet::from([
            Coordinate::new(1, 0),
            Coordinate::new(1, 1),
            Coordinate::new(1, 2),
        ]);
        let low_row = BTreeSet::from([
            Coordinate::new(2, 0),
            Coordinate::new(2, 1),
            Coordinate::new(2, 2),
        ]);
        let left_col = BTreeSet::from([
            Coordinate::new(0, 0),
            Coordinate::new(1, 0),
            Coordinate::new(2, 0),
        ]);
        let mid_col = BTreeSet::from([
            Coordinate::new(0, 1),
            Coordinate::new(1, 1),
            Coordinate::new(2, 1),
        ]);
        let right_col = BTreeSet::from([
            Coordinate::new(0, 2),
            Coordinate::new(1, 2),
            Coordinate::new(2, 2),
        ]);
        let neg_diag = BTreeSet::from([
            Coordinate::new(0, 0),
            Coordinate::new(1, 1),
            Coordinate::new(2, 2),
        ]);
        let pos_diag = BTreeSet::from([
            Coordinate::new(2, 0),
            Coordinate::new(1, 1),
            Coordinate::new(0, 2),
        ]);

        let win_states = HashSet::from([
            top_row, mid_row, low_row, left_col, mid_col, right_col, neg_diag, pos_diag,
        ]);

        let all_coords = BTreeSet::from([
            Coordinate::new(0, 0),
            Coordinate::new(0, 1),
            Coordinate::new(0, 2),
            Coordinate::new(1, 0),
            Coordinate::new(1, 1),
            Coordinate::new(1, 2),
            Coordinate::new(2, 0),
            Coordinate::new(2, 1),
            Coordinate::new(2, 2),
        ]);

        Self {
            win_states,
            cur_state: HashMap::from([(0, all_coords), (1, BTreeSet::new()), (2, BTreeSet::new())]),
            names: HashMap::new(),
        }
    }

    pub fn update(&mut self, player: u8, coord: Coordinate<i8>) -> Result<(), TttError> {
        let coord = self
            .cur_state
            .get_mut(&0)
            .ok_or(TttError::NoPlayer(0))?
            .take(&coord)
            .ok_or(TttError::UsedSpace(coord))?;

        if !self
            .cur_state
            .get_mut(&player)
            .ok_or(TttError::NoPlayer(player))?
            .insert(coord)
        {
            return Err(TttError::DuplicateEntry(coord));
        }
        Ok(())
    }

    fn get_coord(&self, y: i8, x: i8) -> Result<&str, TttError> {
        let coord: Coordinate<i8> = Coordinate::new(y, x);
        if self.cur_state.get(&1)
            .ok_or(TttError::NoPlayer(1))?
            .contains(&coord) {
            Ok("X")
        } else if self.cur_state.get(&2)
            .ok_or(TttError::NoPlayer(2))?
            .contains(&coord) {
            Ok("O")
        } else {
            Ok(" ")
        }
    }

    pub fn get_name(&self, player_id: u8) -> Result<String, TttError> {
        Ok(self.names
            .get(&player_id)
            .ok_or(TttError::NoPlayer(player_id))?
            .clone())
    }

    pub fn check_win(&self, player_id: u8) -> Result<bool, TttError> {
        let player_coords = self.cur_state.get(&player_id)
            .ok_or(TttError::NoPlayer(player_id))?;
        Ok(self.win_states
            .iter()
            .any(|win_state| win_state.is_subset(player_coords)))
    }

    pub fn is_full(&self) -> Result<bool, TttError> {
        Ok(self.cur_state.get(&0)
            .ok_or(TttError::NoPlayer(0))?
            .is_empty())
    }

    pub fn display(&self) -> Result<(), TttError> {
        println!("     a b c");
        println!(
            "    {}{}{}{}{}{}{}",
            OUT_UP_LEFT,
            OUT_HORIZ_UNCONN,
            OUT_HORIZ_TOP_CONN,
            OUT_HORIZ_UNCONN,
            OUT_HORIZ_TOP_CONN,
            OUT_HORIZ_UNCONN,
            OUT_UP_RIGHT
        );
        println!(
            "   1{}{}{}{}{}{}{}",
            OUT_VERT_UNCONN,
            self.get_coord(0, 0)?,
            IN_VERT_UNCONN,
            self.get_coord(0, 1)?,
            IN_VERT_UNCONN,
            self.get_coord(0, 2)?,
            OUT_VERT_UNCONN
        );
        println!(
            "    {}{}{}{}{}{}{}",
            OUT_VERT_LEFT_CONN,
            IN_HORIZ_UNCONN,
            IN_CONN,
            IN_HORIZ_UNCONN,
            IN_CONN,
            IN_HORIZ_UNCONN,
            OUT_VERT_RIGHT_CONN
        );
        println!(
            "   2{}{}{}{}{}{}{}",
            OUT_VERT_UNCONN,
            self.get_coord(1, 0)?,
            IN_VERT_UNCONN,
            self.get_coord(1, 1)?,
            IN_VERT_UNCONN,
            self.get_coord(1, 2)?,
            OUT_VERT_UNCONN
        );
        println!(
            "    {}{}{}{}{}{}{}",
            OUT_VERT_LEFT_CONN,
            IN_HORIZ_UNCONN,
            IN_CONN,
            IN_HORIZ_UNCONN,
            IN_CONN,
            IN_HORIZ_UNCONN,
            OUT_VERT_RIGHT_CONN
        );
        println!(
            "   3{}{}{}{}{}{}{}",
            OUT_VERT_UNCONN,
            self.get_coord(2, 0)?,
            IN_VERT_UNCONN,
            self.get_coord(2, 1)?,
            IN_VERT_UNCONN,
            self.get_coord(2, 2)?,
            OUT_VERT_UNCONN
        );
        println!(
            "    {}{}{}{}{}{}{}",
            OUT_DN_LEFT,
            OUT_HORIZ_UNCONN,
            OUT_HORIZ_BTM_CONN,
            OUT_HORIZ_UNCONN,
            OUT_HORIZ_BTM_CONN,
            OUT_HORIZ_UNCONN,
            OUT_DN_RIGHT
        );
        Ok(())
    }
}
