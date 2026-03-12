mod object;
mod renderer;
mod random_colour;

use clap::Parser;
use object::Liqtuid;
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

    /// Start indexing with `qwerty...` instead of `123456...`
    #[arg(short = 'k', long)]
    no_num_keys: bool,
}

const MSG: &str = "Press the keys shown over the bottles to select one, Ctrl+C to exit";

fn render_full(game: &Liqtuid, columns: usize) -> usize {
    let output = renderer::render_game(game, columns);
    print!("{}", output);
    print!("\n\x1b[2m{}\x1b[0m", MSG);
    std::io::stdout().flush().expect("Failed to flush");
    output.matches('\n').count()
}

fn reprint_all(game: &Liqtuid, columns: usize, lines_printed: usize, width: u16) -> usize {
    print!("\r\x1b[{}A", lines_printed + (MSG.len() as f32 / width as f32).ceil() as usize);
    render_full(&game, columns)
}

fn new_game(args: &Args) -> (Liqtuid, u16, usize) {
    let game = Liqtuid::new(args.number, args.depth, args.empty, args.no_num_keys);
    let (Width(width), _) = terminal_size().expect("failed to get terminal size");
    let columns = ((width as f64 + 3.0) / 6.0).floor() as usize;
    (game, width, columns)
}

fn main() {
    let args = Args::parse();

    if args.depth == 0 || (args.number == 0 && args.empty == 0) {
        println!("Nothing can possibly go wrong, huh? I'm not stopping you though");
    } else if args.empty == 0 {
        println!("Congratulations, you are stuck");
    } else if args.depth == 1 || args.number == 1 {
        println!("Congratulations on a fair victory");
    }
    let (mut game, mut width, mut columns) = new_game(&args);
    print!("\x1b[?25l"); // hide cursor
    let mut lines_printed = render_full(&game, columns);

    let getch = Getch::new();
    loop {
        match getch.getch() {
            Ok(Key::Char(character)) => {
                // playing the game
                if let Some(index) = game.keys.find(character) {
                    game.click_on_index(index);
                    lines_printed = reprint_all(&game, columns, lines_printed, width);
                    if game.check_win() {
                        break;
                    }
                }
            },
            Ok(Key::Ctrl('d')) | Ok(Key::Ctrl('c')) => break,
            Ok(Key::Ctrl('r')) => {
                // regenerate colours
                game.regenerate_colours();
                lines_printed = reprint_all(&game, columns, lines_printed, width);
            },
            Ok(Key::Ctrl('n')) => {
                // new game
                (game, width, columns) = new_game(&args);
                lines_printed = reprint_all(&game, columns, lines_printed, width);
            },
            Err(err) => { eprintln!("Error: {}", err); break; },
            Ok(_) => {},
        }
    }
    println!("\x1b[?25h"); // show cursor
    // kinda inefficient to run twice but whatever
    // so it does not run when ctrl+c
    if game.check_win() {
        println!("Done in {} moves", game.moves);
    }
}
