use crate::game_engine::board::{BasicBoard, Board};
use rand::seq::IteratorRandom;
use rand::thread_rng;

pub struct RandomPlay {
}

impl RandomPlay {
    pub fn new() -> Self {
        Self {}
    }

    pub fn make_move<B: Board>(&self, board: B) -> Option<B> {
        let mut rng = thread_rng();

        let m = board.all_moves().into_iter().choose(&mut rng)?;

        Some(board.transition(m))
    }
}