use crate::game_engine::chess_move::Move;
use crate::game_engine::board::Board;
use crate::game_engine::color::Color;

#[derive(Debug)]
pub struct MoveRes<B>{
    pub board: B,
    pub mv: Move,
}

pub fn order_moves(moves: Vec<Move>, board: & impl Board) -> Vec<MoveRes<impl Board>> {
    let mut orderd = Vec::new();
    for m in moves {
        orderd.push(MoveRes {board:board.transition(m), mv: m});
    }

    if board.current_player() == Color::White {
        orderd.sort_by_cached_key(|a| -a.board.get_material_score());
    } else {
        orderd.sort_by_cached_key(|a| a.board.get_material_score());
    }

    orderd
}