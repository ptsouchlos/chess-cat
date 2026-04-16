use std::fmt;

use colored::Colorize;

use crate::board::{Board, Color, Piece, PieceType};

pub struct BoardDisplay<'a> {
    pub board: &'a Board,
    pub use_ascii: bool,
    pub use_nerd_font: bool,
}

// 3-char wide × 1-row tall cells: " piece " on 8×16px terminal fonts ≈ 24×16px
// per square — compact and readable.
// Board line width: 3 (rank) + 8×3 (squares) + 3 (rank) = 30 chars.

impl<'a> fmt::Display for BoardDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "    a  b  c  d  e  f  g  h    ")?;
        for rank in (1..=8usize).rev() {
            write!(f, "{}  ", rank)?;
            for file in 0..8usize {
                let idx = (8 - rank) * 8 + file;
                let is_light = (rank + file) % 2 == 0;
                let piece = self.board.squares[idx];

                let symbol = match piece {
                    Some(p) if self.use_ascii => piece_ascii(p),
                    Some(p) if self.use_nerd_font => piece_nerd_font(p),
                    Some(p) => piece_unicode(p),
                    None => ' ',
                };
                let cell = format!(" {} ", symbol);

                let colored = if is_light {
                    match piece {
                        Some(p) if p.color == Color::White => cell
                            .truecolor(255, 255, 255)
                            .bold()
                            .on_truecolor(240, 217, 181),
                        Some(_) => cell
                            .truecolor(20, 20, 20)
                            .bold()
                            .on_truecolor(240, 217, 181),
                        None => cell.on_truecolor(240, 217, 181),
                    }
                } else {
                    match piece {
                        Some(p) if p.color == Color::White => cell
                            .truecolor(255, 255, 255)
                            .bold()
                            .on_truecolor(181, 136, 99),
                        Some(_) => cell.truecolor(20, 20, 20).bold().on_truecolor(181, 136, 99),
                        None => cell.on_truecolor(181, 136, 99),
                    }
                };

                write!(f, "{}", colored)?;
            }
            writeln!(f, "  {}", rank)?;
        }
        writeln!(f, "    a  b  c  d  e  f  g  h    ")?;
        writeln!(f)?;
        write_info(self.board, f)
    }
}

// Column widths: 9 + 9 + 6 + 6 = 30 (matches board line width)
fn write_info(board: &Board, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let ep = board
        .en_passant_square
        .map(square_name)
        .unwrap_or_else(|| "-".to_string());
    let stm = board.active_color.to_string();
    let cast = &board.castling_rights;
    let mv = board.fullmove_number.to_string();

    write!(
        f,
        "{}",
        format!("{:^9}", "stm")
            .white()
            .bold()
            .on_truecolor(70, 130, 180)
    )?;
    write!(
        f,
        "{}",
        format!("{:^9}", "cast")
            .white()
            .bold()
            .on_truecolor(107, 142, 35)
    )?;
    write!(
        f,
        "{}",
        format!("{:^6}", "ep")
            .white()
            .bold()
            .on_truecolor(205, 133, 63)
    )?;
    writeln!(
        f,
        "{}",
        format!("{:^6}", "mv")
            .white()
            .bold()
            .on_truecolor(147, 112, 219)
    )?;

    write!(
        f,
        "{}",
        format!("{:^9}", stm)
            .truecolor(20, 20, 20)
            .on_truecolor(240, 217, 181)
    )?;
    write!(
        f,
        "{}",
        format!("{:^9}", cast)
            .truecolor(20, 20, 20)
            .on_truecolor(240, 217, 181)
    )?;
    write!(
        f,
        "{}",
        format!("{:^6}", ep)
            .truecolor(20, 20, 20)
            .on_truecolor(240, 217, 181)
    )?;
    write!(
        f,
        "{}",
        format!("{:^6}", mv)
            .truecolor(20, 20, 20)
            .on_truecolor(240, 217, 181)
    )
}

fn piece_unicode(piece: Piece) -> char {
    match (piece.color, piece.piece_type) {
        (Color::White, PieceType::King) => '♔',
        (Color::White, PieceType::Queen) => '♕',
        (Color::White, PieceType::Rook) => '♖',
        (Color::White, PieceType::Bishop) => '♗',
        (Color::White, PieceType::Knight) => '♘',
        (Color::White, PieceType::Pawn) => '♙',
        (Color::Black, PieceType::King) => '♚',
        (Color::Black, PieceType::Queen) => '♛',
        (Color::Black, PieceType::Rook) => '♜',
        (Color::Black, PieceType::Bishop) => '♝',
        (Color::Black, PieceType::Knight) => '♞',
        (Color::Black, PieceType::Pawn) => '♟',
    }
}

fn piece_ascii(piece: Piece) -> char {
    match (piece.color, piece.piece_type) {
        (Color::White, PieceType::King) => 'K',
        (Color::White, PieceType::Queen) => 'Q',
        (Color::White, PieceType::Rook) => 'R',
        (Color::White, PieceType::Bishop) => 'B',
        (Color::White, PieceType::Knight) => 'N',
        (Color::White, PieceType::Pawn) => 'P',
        (Color::Black, PieceType::King) => 'k',
        (Color::Black, PieceType::Queen) => 'q',
        (Color::Black, PieceType::Rook) => 'r',
        (Color::Black, PieceType::Bishop) => 'b',
        (Color::Black, PieceType::Knight) => 'n',
        (Color::Black, PieceType::Pawn) => 'p',
    }
}

fn piece_nerd_font(piece: Piece) -> char {
    // Material Design chess glyphs (md-chess_*). No white/black variants exist in
    // Nerd Fonts — team is conveyed by text color applied by the renderer.
    match piece.piece_type {
        PieceType::King => '\u{f0857}',   // md-chess_king
        PieceType::Queen => '\u{f085a}',  // md-chess_queen
        PieceType::Rook => '\u{f085b}',   // md-chess_rook
        PieceType::Bishop => '\u{f085c}', // md-chess_bishop
        PieceType::Knight => '\u{f0858}', // md-chess_knight
        PieceType::Pawn => '\u{f0859}',   // md-chess_pawn
    }
}

fn square_name(idx: usize) -> String {
    let file = idx % 8;
    let rank = 8 - idx / 8;
    format!("{}{}", (b'a' + file as u8) as char, rank)
}
