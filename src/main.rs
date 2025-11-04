mod object;
mod renderer;
mod random_colour;

use clap::Parser;
use object::Liqtui;
use terminal_size::{Width, terminal_size};
use getch_rs::{Getch, Key};
use std::io::Write;

/// TUI Liquid Sort Game
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of bottles and different colours
    #[arg(short, long, default_value_t = 6)]
    number: usize,

    /// How many individual units will bottles hold
    #[arg(short, long, default_value_t = 4)]
    depth: usize,

    /// Empty bottles to add
    #[arg(short, long, default_value_t = 2)]
    empty: usize,
}

const MSG: &str = "Press the keys shown over the bottles to select one, Ctrl+C to exit";

fn render_full(game: &Liqtui, columns: usize) -> usize {
    let output = renderer::render_game(game, columns);
    print!("{}", output);
    print!("\n\x1b[2m{}\x1b[0m", MSG);
    std::io::stdout().flush().expect("Failed to flush");
    output.matches('\n').count()
}

fn main() {
    let args = Args::parse();

    if args.depth == 0 || (args.number == 0 && args.empty == 0) {
        println!("Nothing can possibly go wrong, huh? I'm not stopping you though");
    } else if args.empty == 0 {
        println!("Congratulations, you are stuck");
    }
    let mut game = Liqtui::new(args.number, args.depth, args.empty);

    let (Width(width), _) = terminal_size().expect("failed to get terminal size");
    let columns = ((width as f64 + 3.0) / 6.0).floor() as usize;
    print!("\x1b[?25l"); // hide cursor
    let mut lines_printed = render_full(&game, columns);

    let getch = Getch::new();
    loop {
        match getch.getch() {
            Ok(Key::Char(character)) => {
                // playing the game
                if let Some(index) = renderer::KEYS.find(character) {
                    game.click_on_index(index);
                    print!("\r\x1b[{}A", lines_printed + (MSG.len() as f32 / width as f32).ceil() as usize);
                    lines_printed = render_full(&game, columns);
                    if game.check_win() {
                        break;
                    }
                }
            },
            Ok(Key::Ctrl('d')) | Ok(Key::Ctrl('c')) => break,
            Err(err) => { eprintln!("Error: {}", err); break; },
            Ok(_) => {},
        }
    }
    println!("\x1b[?25h"); // show cursor
}
