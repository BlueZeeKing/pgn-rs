use std::fs::read_to_string;

use chess::Board;
use indicatif::ProgressBar;
use pgn_rs::{ PGNReader, Visitor, san::SAN };

const NUM_GAMES: u64 = 121332;

struct TestVisitor {
    chess: Board,
    progress: ProgressBar,
}

impl Visitor for TestVisitor {
    fn start_game(&mut self) {
        self.chess = Board::default();
    }

    fn end_game(&mut self) {
        self.progress.inc(1);
    }

    fn header(&mut self, _header: pgn_rs::Header) {
        // dbg!(header);
    }

    fn san(&mut self, san: SAN) {
        // dbg!(&san);
        let chess_move = san.to_move(&self.chess);
        // dbg!(chess_move.get_dest().to_string());
        self.chess = self.chess.make_move_new(chess_move);
    }
}

impl TestVisitor {
    fn new() -> Self {
        Self { chess: Board::default(), progress: ProgressBar::new(NUM_GAMES) }
    }
}

fn main() {
    let mut visitor = TestVisitor::new();
    let data = read_to_string("lichess_test_collection.pgn").unwrap();
    let reader = PGNReader::new(&data);
    reader.read(&mut visitor);
}