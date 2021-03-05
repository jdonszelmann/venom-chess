use crate::game_engine::chess_move::{Location, Move};
use crate::game_engine::board::Board;

pub fn pawn_moves_black(location: Location, board: &impl Board) -> Vec<Move> {
    let mut moves = Vec::new();
    let our_color = board.piece_at(location).color();


    if location.y == 7 {
        return moves
    }

    let in_front = (location.x, location.y + 1).into();
    if board.piece_at(in_front).is_empty(){
        moves.push((location, in_front).into());

        if location.y == 1 {
            let double_in_front = (location.x, location.y + 2).into();
            if board.piece_at(double_in_front).is_empty(){
                moves.push((location, double_in_front).into());
            }
        }
    }

    if location.x>0{
        let other = (location.x-1, location.y + 1).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other).into());
            }
        }
    }

    if location.x<7{
        let other = (location.x+1, location.y + 1).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other).into());
            }
        }
    }

    moves
}

pub fn pawn_moves_white(location: Location, board: &impl Board) -> Vec<Move> {
    let mut moves = Vec::new();
    let our_color = board.piece_at(location).color();


    if location.y == 0 {
        return moves
    }

    let in_front = (location.x, location.y - 1).into();
    if board.piece_at(in_front).is_empty(){
        moves.push((location, in_front).into());

        if location.y == 6 {
            let double_in_front = (location.x, location.y - 2).into();
            if board.piece_at(double_in_front).is_empty(){
                moves.push((location, double_in_front).into());
            }
        }
    }

    if location.x>0{
        let other = (location.x-1, location.y - 1).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other).into());
            }
        }
    }

    if location.x<7{
        let other = (location.x+1, location.y - 1).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other).into());
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
                moves.push((location, other).into());
            }
            break;
        }
        moves.push((location, other).into());
    }

    for off in 1..((7 - location.x).min(location.y) + 1) {
        let other = (location.x + off, location.y - off).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other).into());
            }
            break;
        }
        moves.push((location, other).into());
    }

    for off in 1..((7 - location.x).min(7 - location.y) + 1) {
        let other = (location.x + off, location.y + off).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other).into());
            }
            break;
        }
        moves.push((location, other).into());
    }

    for off in 1..(location.x.min(7 - location.y) + 1) {
        let other = (location.x - off, location.y + off).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other).into());
            }
            break;
        }
        moves.push((location, other).into());
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
        if !piece.is_empty() && piece.color() == our_color {
            continue;
        }

        moves.push((location, l).into());
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
                moves.push((location, other).into());
            }

            break;
        }

        moves.push((location, other).into());
    }

    for x in (0..location.x).rev() {
        let other = (x, location.y).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other).into());
            }

            break;
        }

        moves.push((location, other).into());
    }

    for y in (location.y + 1)..8 {
        let other = (location.x, y).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other).into());
            }

            break;
        }

        moves.push((location, other).into());
    }

    for y in (0..location.y).rev() {
        let other = (location.x, y).into();
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != our_color {
                moves.push((location, other).into());
            }

            break;
        }

        moves.push((location, other).into());
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

        moves.push((location, l).into());
    }

    moves
}

pub fn queen_moves(location: Location, board: &impl Board) -> Vec<Move> {
    let mut moves = bishop_moves(location,board);
    moves.append(&mut rook_moves(location,board));
    moves
}