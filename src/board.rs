use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::White => write!(f, "White"),
            Color::Black => write!(f, "Black"),
        }
    }
}

impl FromStr for Color {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Color::White),
            "b" => Ok(Color::Black),
            _ => Err(format!("Invalid color: {}", s)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl FromStr for PieceType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "p" | "P" => Ok(PieceType::Pawn),
            "n" | "N" => Ok(PieceType::Knight),
            "b" | "B" => Ok(PieceType::Bishop),
            "r" | "R" => Ok(PieceType::Rook),
            "q" | "Q" => Ok(PieceType::Queen),
            "k" | "K" => Ok(PieceType::King),
            _ => Err(format!("Invalid piece type: {}", s)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl FromStr for Piece {
    type Err = String;
    fn from_str(s: &str) -> Result<Piece, Self::Err> {
        if s.is_empty() {
            return Err("Empty piece string".to_string());
        }
        let first_char = s.chars().next().unwrap();
        let color = if first_char.is_uppercase() {
            Color::White
        } else {
            Color::Black
        };
        let piece_type = PieceType::from_str(&s[..1])?;
        Ok(Piece { piece_type, color })
    }
}

#[derive(Debug, PartialEq)]
pub struct Board {
    pub squares: [Option<Piece>; 64],
    pub active_color: Color,
    pub castling_rights: String,
    pub en_passant_square: Option<usize>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}

impl Board {
    pub fn parse_fen(fen: &str) -> Result<Self, String> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() < 4 {
            return Err(format!(
                "FEN must have at least 4 space-separated parts, got {}",
                parts.len()
            ));
        }

        let pieces_part = parts[0];
        let active_color = Color::from_str(parts[1])?;
        let castling_rights = parts[2].to_string();
        let en_passant_str = parts[3];
        let halfmove_clock = if parts.len() <= 4 {
            0
        } else {
            parts[4].parse::<u32>().map_err(|e| e.to_string())?
        };
        let fullmove_number = if parts.len() <= 5 {
            1
        } else {
            parts[5].parse::<u32>().map_err(|e| e.to_string())?
        };

        let mut squares = [None; 64];
        let mut square_idx = 0;

        for rank in pieces_part.split('/') {
            for c in rank.chars() {
                if let Some(empty) = c.to_digit(10) {
                    square_idx += empty as usize;
                } else {
                    if square_idx >= 64 {
                        return Err("FEN piece placement overflows board".to_string());
                    }
                    squares[square_idx] = Some(Piece::from_str(&c.to_string())?);
                    square_idx += 1;
                }
            }
        }

        if square_idx != 64 {
            return Err(format!(
                "FEN piece placement has {} squares, expected 64",
                square_idx
            ));
        }

        let en_passant_square = if en_passant_str == "-" {
            None
        } else {
            let mut chars = en_passant_str.chars();
            let file_char = chars.next().ok_or("Invalid en passant square")?;
            let rank_char = chars.next().ok_or("Invalid en passant square")?;
            if !('a'..='h').contains(&file_char) || !('1'..='8').contains(&rank_char) {
                return Err(format!("Invalid en passant square: {}", en_passant_str));
            }
            let file = (file_char as u8 - b'a') as usize;
            let rank = (rank_char as u8 - b'1') as usize + 1; // 1-8
            let idx = (8 - rank) * 8 + file;
            Some(idx)
        };

        Ok(Board {
            squares,
            active_color,
            castling_rights,
            en_passant_square,
            halfmove_clock,
            fullmove_number,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    #[test]
    fn parse_starting_position() {
        let board = Board::parse_fen(STARTING_FEN).unwrap();
        assert_eq!(board.active_color, Color::White);
        assert_eq!(board.castling_rights, "KQkq");
        assert_eq!(board.fullmove_number, 1);

        let white_pieces = board
            .squares
            .iter()
            .filter(|s| s.map(|p| p.color == Color::White).unwrap_or(false))
            .count();
        let black_pieces = board
            .squares
            .iter()
            .filter(|s| s.map(|p| p.color == Color::Black).unwrap_or(false))
            .count();
        assert_eq!(white_pieces, 16);
        assert_eq!(black_pieces, 16);

        // a8 = index 0 should be a black rook
        assert_eq!(
            board.squares[0],
            Some(Piece {
                piece_type: PieceType::Rook,
                color: Color::Black
            })
        );
        // e1 = index 60 should be a white king
        assert_eq!(
            board.squares[60],
            Some(Piece {
                piece_type: PieceType::King,
                color: Color::White
            })
        );
    }

    #[test]
    fn parse_en_passant() {
        // After 1.e4 c5 2.e5 d5, en passant square is d6
        let fen = "rnbqkbnr/pp2pppp/8/2ppP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3";
        let board = Board::parse_fen(fen).unwrap();
        // d6: file d=3, rank 6 → index (8-6)*8+3 = 19
        assert_eq!(board.en_passant_square, Some(19));
    }

    #[test]
    fn parse_invalid_fen_too_short() {
        assert!(Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w").is_err());
    }

    #[test]
    fn parse_invalid_fen_bad_piece() {
        assert!(
            Board::parse_fen("xnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").is_err()
        );
    }

    #[test]
    fn parse_black_to_move() {
        let fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
        let board = Board::parse_fen(fen).unwrap();
        assert_eq!(board.active_color, Color::Black);
        // e3: file e=4, rank 3 → index (8-3)*8+4 = 44
        assert_eq!(board.en_passant_square, Some(44));
    }
}
