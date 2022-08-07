pub mod maninpulations;
use std::{io::{stdin, Read}};
use maninpulations::center_image;
use termion::terminal_size;

pub fn main() {
    let columns: usize = terminal_size().unwrap_or((80, 20)).0.into();
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    print!("{}", center_image(&input, columns));
}
