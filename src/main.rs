pub mod load;
pub mod maninpulations;

use load::load_image;
use maninpulations::{center_image, parse_colors};
use termion::terminal_size;
use std::env;
use rand::seq::SliceRandom;

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_names = &args[1..];
    let image_name = image_names.choose(&mut rand::thread_rng()).unwrap();
    let image = load_image(&String::from(image_name.as_str())).unwrap();
    let image = parse_colors(&image).unwrap();
    let columns = terminal_size().unwrap_or((80, 20)).0;
    let image = center_image(&image, columns.into());
    print!("{}", &image);
}
