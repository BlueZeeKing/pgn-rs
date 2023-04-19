use std::fs::read_to_string;

use chess::Board;
use pgn_rs::{ PGNReader, Visitor, san::SAN };

struct TestVisitor {
    chess: Board,
}

impl Visitor for TestVisitor {
    fn start_game(&mut self) {
        self.chess = Board::default();
    }

    fn header(&mut self, _header: pgn_rs::Header) {
        // dbg!(header);
    }

    fn san(&mut self, san: SAN) {
        self.chess = self.chess.make_move_new(san.to_move(&self.chess));
    }
}

impl TestVisitor {
    fn new() -> Self {
        Self { chess: Board::default() }
    }
}

fn main() {
    let mut visitor = TestVisitor::new();
    let data = read_to_string("test.pgn").unwrap();
    let reader = PGNReader::new(&data);
    reader.read(&mut visitor);
}