use std::str::FromStr;

use chess::{ Piece, Square, File, Rank, Board, ChessMove, MoveGen };

enum Error {
    PieceNotFound,
}

fn get_piece(letter: char) -> Result<Piece, Error> {
    match letter {
        'P' => Ok(Piece::Pawn),
        'N' => Ok(Piece::Knight),
        'B' => Ok(Piece::Bishop),
        'R' => Ok(Piece::Rook),
        'Q' => Ok(Piece::Queen),
        'K' => Ok(Piece::King),
        _ => Err(Error::PieceNotFound),
    }
}

#[derive(Debug)]
pub enum SAN {
    Normal {
        piece: Piece,
        dest: Square,
        capture: bool,
        promotion: Option<Piece>,
        check: bool,
        check_mate: bool,
        starting_rank: Option<Rank>,
        starting_file: Option<File>,
    },
    Castle {
        is_king_side: bool,
    },
}

impl SAN {
    pub fn new(san: &str) -> Self {
        if san == "O-O" {
            return Self::Castle { is_king_side: true };
        } else if san == "O-O-O" {
            return Self::Castle { is_king_side: false };
        }

        let (piece, remaining) = match get_piece(san.chars().next().unwrap()) {
            Ok(piece) => (piece, &san[1..]),
            Err(_) => (Piece::Pawn, san),
        };

        let (check, check_mate, remaining) = match remaining.chars().last().unwrap() {
            '+' => (true, false, &remaining[..remaining.len() - 1]),
            '#' => (false, true, &remaining[..remaining.len() - 1]),
            _ => (false, false, remaining),
        };

        let (promotion, remaining) = match get_piece(remaining.chars().last().unwrap()) {
            Ok(piece) => (Some(piece), &remaining[..remaining.len() - 2]),
            Err(_) => (None, remaining),
        };

        let (dest, remaining) = (
            Square::from_str(&remaining[remaining.len() - 2..]).unwrap(),
            &remaining[..remaining.len() - 2],
        );

        let (capture, remaining) = if
            !remaining.is_empty() &&
            &remaining[remaining.len() - 1..] == "x"
        {
            (true, &remaining[..remaining.len() - 1])
        } else {
            (false, remaining)
        };

        let (starting_file, remaining) = if !remaining.is_empty() {
            match File::from_str(&remaining[0..1]) {
                Ok(file) => (Some(file), &remaining[1..]),
                Err(_) => (None, remaining),
            }
        } else {
            (None, remaining)
        };

        let starting_rank = if !remaining.is_empty() {
            match Rank::from_str(&remaining[0..1]) {
                Ok(file) => Some(file),
                Err(_) => None,
            }
        } else {
            None
        };

        Self::Normal {
            piece,
            dest,
            capture,
            promotion,
            check,
            check_mate,
            starting_rank,
            starting_file,
        }
    }

    pub fn to_move(&self, position: &Board) -> ChessMove {
        match self {
            Self::Castle { is_king_side } =>
                ChessMove::new(
                    Square::make_square(position.side_to_move().to_my_backrank(), File::E),
                    Square::make_square(position.side_to_move().to_my_backrank(), if *is_king_side {
                        File::G
                    } else {
                        File::C
                    }),
                    None
                ),
            Self::Normal {
                piece,
                dest,
                capture: _,
                promotion: _,
                check: _,
                check_mate: _,
                starting_rank,
                starting_file,
            } => {
                MoveGen::new_legal(position)
                    .find(
                        |chess_move|
                            chess_move.get_dest() == *dest &&
                            position.piece_on(chess_move.get_source()).unwrap() == *piece &&
                            (starting_rank.is_none() ||
                                matches!(starting_rank, Some(rank) if rank == &chess_move.get_source().get_rank())) &&
                            (starting_file.is_none() ||
                                matches!(starting_file, Some(file) if file == &chess_move.get_source().get_file()))
                    )
                    .unwrap()
            }
        }
    }
}