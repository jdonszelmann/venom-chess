use crate::game_engine::board::Board;
use crate::game_engine::piece::Color;

impl Board {
    pub fn king_check(&self, color: Color) -> bool{

        let king_loc = self.king_location(color).unwrap();

        for off in 1..(king_loc.x.min(king_loc.y)) {
            let other = (king_loc.x - off, king_loc.y - off).into();
            let piece = self.piece_at(other);
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
            let other = (king_loc.x + off, king_loc.y - off).into();
            let piece = self.piece_at(other);
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
            let other = (king_loc.x + off, king_loc.y + off).into();
            let piece = self.piece_at(other);
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
            let other = (king_loc.x - off, king_loc.y + off).into();
            let piece = self.piece_at(other);
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
            let other = (x, king_loc.y).into();
            let piece = self.piece_at(other);
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
            let other = (x, king_loc.y).into();
            let piece = self.piece_at(other);
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
            let other = (king_loc.x, y).into();
            let piece = self.piece_at(other);
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
            let other = (king_loc.x, y).into();
            let piece = self.piece_at(other);
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

            let l = (*x, *y).into();

            let piece = self.piece_at(l);
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

            let l = (*x, *y).into();

            let piece = self.piece_at(l);
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
                    let other = (king_loc.x-1, king_loc.y + 1).into();
                    let piece = self.piece_at(other);
                    if !piece.is_empty() {
                        if piece.color() != color {
                            if piece == WhitePawn {
                                return true
                            }
                        }
                    }
                }
                if king_loc.x<7{
                    let other = (king_loc.x+1, king_loc.y + 1).into();
                    let piece = self.piece_at(other);
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
                    let other = (king_loc.x-1, king_loc.y - 1).into();
                    let piece = self.piece_at(other);
                    if !piece.is_empty() {
                        if piece.color() != color {
                            if piece == BlackPawn {
                                return true
                            }
                        }
                    }
                }
                if king_loc.x<7{
                    let other = (king_loc.x+1, king_loc.y - 1).into();
                    let piece = self.piece_at(other);
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
}