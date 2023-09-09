pub mod load;
pub mod maninpulations;

use load::load_image;
use maninpulations::{center_image_x, parse_colors};
use rand::seq::SliceRandom;
use std::env;
use termion::terminal_size;

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_directory = &args[1];
    let image_names = &args[2..];
    let image_name = image_names.choose(&mut rand::thread_rng()).unwrap();
    let image = load_image(&String::from(image_name.as_str()), &image_directory).unwrap();
    let image = parse_colors(&image).unwrap();
    let columns = terminal_size().unwrap_or((80, 20)).0;
    let image = center_image_x(&image, columns.into());
    print!("{}", &image);
}
