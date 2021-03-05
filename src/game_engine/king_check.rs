use crate::game_engine::board::Board;
use crate::game_engine::piece::Piece;
use crate::game_engine::piece::Piece::*;
use crate::game_engine::color::Color::*;
use crate::game_engine::color::Color;
use crate::game_engine::chess_move::Location;

pub fn king_location(board: &impl Board, color: Color) -> Option<Location> {
    if color == Color::White {
        for y in 0..8 {
            for x in 0..8 {
                let other = board.piece_at((x, y));
                if other == Piece::WhiteKing{
                    return Some((x,y).into());
                }
            }
        }
    } else {
        for y in 0..8 {
            for x in 0..8 {
                let other = board.piece_at((x, y));
                if other == Piece::BlackKing{
                    return Some((x,y).into());
                }
            }
        }
    }
    None
}


pub fn king_check(board: &impl Board, color: Color) -> bool{

    let king_loc = king_location(board, color).unwrap();

    for off in 1..(king_loc.x.min(king_loc.y)) {
        let other = (king_loc.x - off, king_loc.y - off);
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != color {
                if piece.is_bishop() || piece.is_queen(){
                    return true
                }
            }
            break;
        }
    }

    for off in 1..((7 - king_loc.x).min(king_loc.y)) {
        let other = (king_loc.x + off, king_loc.y - off);
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != color {
                if piece.is_bishop() || piece.is_queen(){
                    return true
                }
            }
            break;
        }
    }

    for off in 1..((7 - king_loc.x).min(7 - king_loc.y)) {
        let other = (king_loc.x + off, king_loc.y + off);
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != color {
                if piece.is_bishop() || piece.is_queen(){
                    return true
                }
            }
            break;
        }
    }

    for off in 1..(king_loc.x.min(7 - king_loc.y)) {
        let other = (king_loc.x - off, king_loc.y + off);
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != color {
                if piece.is_bishop() || piece.is_queen(){
                    return true
                }
            }
            break;
        }
    }

    // ==================

    for x in (king_loc.x + 1)..8 {
        let other = (x, king_loc.y);
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != color {
                if piece.is_rook() || piece.is_queen(){
                    return true
                }
            }
            break;
        }
    }

    for x in (0..king_loc.x).rev() {
        let other = (x, king_loc.y);
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != color {
                if piece.is_rook() || piece.is_queen(){
                    return true
                }
            }
            break;
        }
    }

    for y in (king_loc.y + 1)..8 {
        let other = (king_loc.x, y);
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != color {
                if piece.is_rook() || piece.is_queen(){
                    return true
                }
            }
            break;
        }
    }

    for y in (0..king_loc.y).rev() {
        let other = (king_loc.x, y);
        let piece = board.piece_at(other);
        if !piece.is_empty() {
            if piece.color() != color {
                if piece.is_rook() || piece.is_queen(){
                    return true
                }
            }
            break;
        }
    }

    // ==================

    for (x, y) in &[(king_loc.x + 2, king_loc.y + 1), (king_loc.x + 2, king_loc.y - 1),
        (king_loc.x - 2, king_loc.y + 1), (king_loc.x - 2, king_loc.y - 1),
        (king_loc.x + 1, king_loc.y + 2), (king_loc.x - 1, king_loc.y + 2),
        (king_loc.x + 1, king_loc.y - 2), (king_loc.x - 1, king_loc.y - 2)] {
        if *x < 0 || *x >= 8 {
            continue;
        }

        if *y < 0 || *y >= 8 {
            continue;
        }

        let piece = board.piece_at((*x, *y));
        if !piece.is_empty() {
            if piece.color() != color {
                if piece.is_knight() {
                    return true
                }
            }
            break;
        }
    }

    for (x, y) in &[(king_loc.x + 1, king_loc.y), (king_loc.x + 1, king_loc.y + 1),
        (king_loc.x, king_loc.y + 1), (king_loc.x - 1, king_loc.y + 1),
        (king_loc.x - 1, king_loc.y), (king_loc.x - 1, king_loc.y - 1),
        (king_loc.x, king_loc.y - 1), (king_loc.x + 1, king_loc.y - 1)] {
        if *x < 0 || *x >= 8 {
            continue;
        }

        if *y < 0 || *y >= 8 {
            continue;
        }

        let piece = board.piece_at((*x, *y));
        if !piece.is_empty() {
            if piece.color() != color {
                if piece.is_king() {
                    return true
                }
            }
            break;
        }

    }

    if color == Black{
        if king_loc.y<7 {
            if king_loc.x > 0 {
                let other = (king_loc.x-1, king_loc.y + 1);
                let piece = board.piece_at(other);
                if !piece.is_empty() {
                    if piece.color() != color {
                        if piece == WhitePawn {
                            return true
                        }
                    }
                }
            }
            if king_loc.x<7{
                let other = (king_loc.x+1, king_loc.y + 1);
                let piece = board.piece_at(other);
                if !piece.is_empty() {
                    if piece.color() != color {
                        if piece == WhitePawn {
                            return true
                        }
                    }
                }
            }
        }
    } else {
        if king_loc.y>0 {
            if king_loc.x > 0 {
                let other = (king_loc.x-1, king_loc.y - 1);
                let piece = board.piece_at(other);
                if !piece.is_empty() {
                    if piece.color() != color {
                        if piece == BlackPawn {
                            return true
                        }
                    }
                }
            }
            if king_loc.x<7{
                let other = (king_loc.x+1, king_loc.y - 1);
                let piece = board.piece_at(other);
                if !piece.is_empty() {
                    if piece.color() != color {
                        if piece == BlackPawn {
                            return true
                        }
                    }
                }
            }
        }
    }

    false
}