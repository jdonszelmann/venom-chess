use crate::game_engine::board::Board;

pub mod random_play;
pub mod minimax;
pub mod alpha_beta;

pub trait Solver {
    fn make_move<B: Board>(&self, board: B) -> Option<B>;
}
