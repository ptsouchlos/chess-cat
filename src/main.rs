mod board;
mod display;

use clap::Parser;

use board::Board;
use display::BoardDisplay;

const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Parser)]
#[command(
    name = "chess-cat",
    about = "Visualize chess positions from FEN notation"
)]
struct Cli {
    /// FEN string to visualize (defaults to starting position)
    fen: Option<String>,

    /// Use ASCII piece letters instead of Unicode symbols
    #[arg(long)]
    ascii: bool,

    /// Use Nerd Font chess glyphs instead of Unicode symbols
    #[arg(long)]
    nerd_font: bool,

    /// Disable colored output
    #[arg(long)]
    no_color: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.no_color {
        colored::control::set_override(false);
    }

    let fen = cli.fen.as_deref().unwrap_or(STARTING_FEN);

    match Board::parse_fen(fen) {
        Ok(board) => println!(
            "{}",
            BoardDisplay {
                board: &board,
                use_ascii: cli.ascii,
                use_nerd_font: cli.nerd_font,
            }
        ),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
