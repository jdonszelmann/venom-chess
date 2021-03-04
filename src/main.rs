
mod game_engine;
mod ai;

use game_engine::board::Board;

fn main() {
    let b = Board::DEFAULT_BOARD;

    println!("{}", b);
}
