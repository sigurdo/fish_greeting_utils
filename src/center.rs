pub mod maninpulations;
use clap::{arg, Parser};
use maninpulations::{center_image_x, center_image_y};
use std::io::{stdin, Read};
use termion::terminal_size;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CenterArgs {
    #[arg(short, long)]
    x_only: bool,

    #[arg(short, long)]
    y_only: bool,
}

pub fn main() {
    let args = CenterArgs::parse();
    let (columns, rows) = terminal_size().unwrap_or((80, 20));
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let mut result = input;
    if !args.y_only {
        result = center_image_x(&result, columns as usize);
    }
    if !args.x_only {
        result = center_image_y(&result, rows as usize);
    }
    print!("{}", result);
}
