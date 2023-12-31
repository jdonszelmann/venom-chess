use crate::game_engine::chess_move::{Location, Move, Extra};
use crate::game_engine::board::Board;
use crate::game_engine::color::Color::{Black, White};
use crate::game_engine::king_check::king_check;
use crate::game_engine::chess_move::Extra::{Captures, Quiet, QueenCastle, KingCastle, DoublePawn, EnPassantCaptures};

pub fn pawn_moves_black(location: Location, board: &impl Board) -> Vec<Move> {
    let mut moves = Vec::new();
    let our_color = board.piece_at(location).color();


    if location.y == 7 {
        return moves;
    }

    let in_front = (location.x, location.y + 1).into();
    if board.piece_at(in_front).is_empty() {
        if location.y == 6 {
            moves.push((location, in_front, Extra::KnightPromotion).into());
            moves.push((location, in_front, Extra::BishopPromotion).into());
            moves.push((location, in_front, Extra::RookPromotion).into());
            moves.push((location, in_front, Extra::QueenPromotion).into());
        } else {
            moves.push((location, in_front, Quiet).into());

            if location.y == 1 {
                let double_in_front = (location.x, location.y + 2).into();
                if board.piece_at(double_in_front).is_empty() {
                    moves.push((location, double_in_front, DoublePawn).into());
                }
            }
        }
    }

    if location.x > 0 {
        let other = (location.x - 1, location.y + 1).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                if location.y == 6 {
                    moves.push((location, other, Extra::KnightPromotionCapture).into());
                    moves.push((location, other, Extra::BishopPromotionCapture).into());
                    moves.push((location, other, Extra::RookPromotionCapture).into());
                    moves.push((location, other, Extra::QueenPromotionCapture).into());
                } else {
                    moves.push((location, other, Captures).into());
                }
            }
        }
    }

    if location.x < 7 {
        let other = (location.x + 1, location.y + 1).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                if location.y == 6 {
                    moves.push((location, other, Extra::KnightPromotionCapture).into());
                    moves.push((location, other, Extra::BishopPromotionCapture).into());
                    moves.push((location, other, Extra::RookPromotionCapture).into());
                    moves.push((location, other, Extra::QueenPromotionCapture).into());
                } else {
                    moves.push((location, other, Captures).into());
                }
            }
        }
    }

    if board.get_en_passant() != 8 {
        if location.y == 4 {
            if location.x + 1 == board.get_en_passant() {
                moves.push((location, (location.x + 1, location.y + 1).into(), EnPassantCaptures).into());
            }
            if location.x - 1 == board.get_en_passant() {
                moves.push((location, (location.x - 1, location.y + 1).into(), EnPassantCaptures).into());
            }
        }
    }

    moves
}

pub fn pawn_moves_white(location: Location, board: &impl Board) -> Vec<Move> {
    let mut moves = Vec::new();
    let our_color = board.piece_at(location).color();


    if location.y == 0 {
        return moves;
    }

    let in_front = (location.x, location.y - 1).into();
    if board.piece_at(in_front).is_empty() {
        if location.y == 1 {
            moves.push((location, in_front, Extra::KnightPromotion).into());
            moves.push((location, in_front, Extra::BishopPromotion).into());
            moves.push((location, in_front, Extra::RookPromotion).into());
            moves.push((location, in_front, Extra::QueenPromotion).into());
        } else {
            moves.push((location, in_front, Quiet).into());

            if location.y == 6 {
                let double_in_front = (location.x, location.y - 2).into();
                if board.piece_at(double_in_front).is_empty() {
                    moves.push((location, double_in_front, DoublePawn).into());
                }
            }
        }
    }

    if location.x > 0 {
        let other = (location.x - 1, location.y - 1).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                if location.y == 1 {
                    moves.push((location, other, Extra::KnightPromotionCapture).into());
                    moves.push((location, other, Extra::BishopPromotionCapture).into());
                    moves.push((location, other, Extra::RookPromotionCapture).into());
                    moves.push((location, other, Extra::QueenPromotionCapture).into());
                } else {
                    moves.push((location, other, Captures).into());
                }
            }
        }
    }

    if location.x < 7 {
        let other = (location.x + 1, location.y - 1).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                if location.y == 1 {
                    moves.push((location, other, Extra::KnightPromotionCapture).into());
                    moves.push((location, other, Extra::BishopPromotionCapture).into());
                    moves.push((location, other, Extra::RookPromotionCapture).into());
                    moves.push((location, other, Extra::QueenPromotionCapture).into());
                } else {
                    moves.push((location, other, Captures).into());
                }
            }
        }
    }

    if board.get_en_passant() != 8 {
        if location.y == 3 {
            if location.x + 1 == board.get_en_passant() {
                moves.push((location, (location.x + 1, location.y - 1).into(), EnPassantCaptures).into());
            }
            if location.x - 1 == board.get_en_passant() {
                moves.push((location, (location.x - 1, location.y - 1).into(), EnPassantCaptures).into());
            }
        }
    }

    moves
}


pub fn bishop_moves(location: Location, board: &impl Board) -> Vec<Move> {
    let mut moves = Vec::new();
    let our_color = board.piece_at(location).color();

    for off in 1..(location.x.min(location.y) + 1) {
        let other = (location.x - off, location.y - off).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other, Captures).into());
            }
            break;
        }
        moves.push((location, other, Quiet).into());
    }

    for off in 1..((7 - location.x).min(location.y) + 1) {
        let other = (location.x + off, location.y - off).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other, Captures).into());
            }
            break;
        }
        moves.push((location, other, Quiet).into());
    }

    for off in 1..((7 - location.x).min(7 - location.y) + 1) {
        let other = (location.x + off, location.y + off).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other, Captures).into());
            }
            break;
        }
        moves.push((location, other, Quiet).into());
    }

    for off in 1..(location.x.min(7 - location.y) + 1) {
        let other = (location.x - off, location.y + off).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other, Captures).into());
            }
            break;
        }
        moves.push((location, other, Quiet).into());
    }

    moves
}

pub fn knight_moves(location: Location, board: &impl Board) -> Vec<Move> {
    let mut moves = Vec::new();
    let our_color = board.piece_at(location).color();

    for (x, y) in &[(location.x + 2, location.y + 1), (location.x + 2, location.y - 1),
        (location.x - 2, location.y + 1), (location.x - 2, location.y - 1),
        (location.x + 1, location.y + 2), (location.x - 1, location.y + 2),
        (location.x + 1, location.y - 2), (location.x - 1, location.y - 2)] {
        if *x < 0 || *x >= 8 {
            continue;
        }

        if *y < 0 || *y >= 8 {
            continue;
        }

        let l = (*x, *y).into();

        let piece = board.piece_at(l);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, l, Captures).into());
            }
        } else {
            moves.push((location, l, Quiet).into());
        }
    }

    moves
}

pub fn rook_moves(location: Location, board: &impl Board) -> Vec<Move> {
    let mut moves = Vec::new();

    let our_color = board.piece_at(location).color();

    for x in (location.x + 1)..8 {
        let other = (x, location.y).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other, Captures).into());
            }

            break;
        }

        moves.push((location, other, Quiet).into());
    }

    for x in (0..location.x).rev() {
        let other = (x, location.y).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other, Captures).into());
            }

            break;
        }

        moves.push((location, other, Quiet).into());
    }

    for y in (location.y + 1)..8 {
        let other = (location.x, y).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other, Captures).into());
            }

            break;
        }

        moves.push((location, other, Quiet).into());
    }

    for y in (0..location.y).rev() {
        let other = (location.x, y).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other, Captures).into());
            }

            break;
        }

        moves.push((location, other, Quiet).into());
    }

    moves
}

pub fn king_moves(location: Location, board: &impl Board) -> Vec<Move> {
    let mut moves = Vec::new();
    let our_color = board.piece_at(location).color();

    for (x, y) in &[(location.x + 1, location.y), (location.x + 1, location.y + 1),
        (location.x, location.y + 1), (location.x - 1, location.y + 1),
        (location.x - 1, location.y), (location.x - 1, location.y - 1),
        (location.x, location.y - 1), (location.x + 1, location.y - 1)] {
        if *x < 0 || *x >= 8 {
            continue;
        }

        if *y < 0 || *y >= 8 {
            continue;
        }

        let l = (*x, *y).into();

        let piece = board.piece_at(l);
        if !piece.is_empty() && piece.color() == our_color {
            continue;
        }

        let castling_rights = board.get_castling_rights();
        if !king_check(board, our_color) {
            if our_color == Black {
                if castling_rights[0] {
                    if board.piece_at((1, 0)).is_empty()
                        && board.piece_at((2, 0)).is_empty()
                        && board.piece_at((3, 0)).is_empty() {
                        let temp_board = board.clone();
                        temp_board.transition(((4, 0), (3, 0)).into());
                        if !king_check(&temp_board, our_color) {
                            moves.push((location, (2, 0).into(), QueenCastle).into());
                        }
                    }
                }
                if castling_rights[1] {
                    if board.piece_at((5, 0)).is_empty()
                        && board.piece_at((6, 0)).is_empty() {
                        let temp_board = board.clone();
                        temp_board.transition(((4, 0), (5, 0)).into());
                        if !king_check(&temp_board, our_color) {
                            moves.push((location, (6, 0).into(), KingCastle).into());
                        }
                    }
                }
            }

            if our_color == White {
                if castling_rights[2] {
                    if board.piece_at((1, 7)).is_empty()
                        && board.piece_at((2, 7)).is_empty()
                        && board.piece_at((3, 7)).is_empty() {
                        let temp_board = board.clone();
                        temp_board.transition(((4, 7), (3, 7)).into());
                        if !king_check(&temp_board, our_color) {
                            moves.push((location, (2, 7).into(), QueenCastle).into());
                        }
                    }
                }
                if castling_rights[3] {
                    if board.piece_at((5, 7)).is_empty()
                        && board.piece_at((6, 7)).is_empty() {
                        let temp_board = board.clone();
                        temp_board.transition(((4, 7), (5, 7)).into());
                        if !king_check(&temp_board, our_color) {
                            moves.push((location, (6, 7).into(), KingCastle).into());
                        }
                    }
                }
            }
        }

        if piece.is_empty() {
            moves.push((location, l, Quiet).into());
        } else {
            moves.push((location, l, Captures).into());
        }
    }

    moves
}

pub fn queen_moves(location: Location, board: &impl Board) -> Vec<Move> {
    let mut moves = bishop_moves(location, board);
    moves.append(&mut rook_moves(location, board));
    moves
}