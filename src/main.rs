mod board;
mod display;
mod theme;

use clap::Parser;

use board::Board;
use display::BoardDisplay;
use theme::Theme;

const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(clap::ValueEnum, Clone, Default)]
enum ThemeChoice {
    #[default]
    Classic,
    Green,
    Ocean,
    HighContrast,
    Mono,
}

impl ThemeChoice {
    fn theme(&self) -> Theme {
        match self {
            ThemeChoice::Classic => Theme::CLASSIC,
            ThemeChoice::Green => Theme::GREEN,
            ThemeChoice::Ocean => Theme::OCEAN,
            ThemeChoice::HighContrast => Theme::HIGH_CONTRAST,
            ThemeChoice::Mono => Theme::MONO,
        }
    }
}

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

    /// Use plain Unicode chess symbols instead of Nerd Font glyphs
    #[arg(long)]
    unicode: bool,

    /// Disable colored output
    #[arg(long)]
    no_color: bool,

    /// Board color theme
    #[arg(long, value_enum, default_value_t = ThemeChoice::Classic)]
    theme: ThemeChoice,

    /// Show the board from Black's perspective
    #[arg(long)]
    flip: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.no_color {
        colored::control::set_override(false);
    }

    let fen = cli.fen.as_deref().unwrap_or(STARTING_FEN);

    // There isn't really a reliable way to detect automatically if a nerd font is installed/used
    // in the running terminal.
    let use_nerd_font = !cli.ascii && !cli.unicode;

    match Board::parse_fen(fen) {
        Ok(board) => println!(
            "{}",
            BoardDisplay {
                board: &board,
                use_ascii: cli.ascii,
                use_nerd_font,
                theme: cli.theme.theme(),
                flip: cli.flip,
            }
        ),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
