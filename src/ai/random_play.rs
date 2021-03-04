use crate::game_engine::board::Board;
use rand::seq::IteratorRandom;
use rand::thread_rng;

pub struct RandomPlay {
}

impl RandomPlay {
    pub fn new() -> Self {
        Self {}
    }

    pub fn make_move(&self, board: Board) -> Option<Board> {
        let mut rng = thread_rng();

        let m = board.possible_moves().choose(&mut rng)?;

        Some(board.transition(m))
    }
}