use crate::game_engine::board::Board;

pub mod random_play;
pub mod minimax;
pub mod alpha_beta;
pub mod alpha_beta_transp;

pub trait Solver {
    fn make_move<B: Board>(&mut self, board: B) -> Option<B>;
}
