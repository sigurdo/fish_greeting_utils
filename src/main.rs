pub mod load;
pub mod maninpulations;

use load::load_image;
use maninpulations::{center_image, parse_colors};

fn main() {
    let image = load_image(&String::from("arch")).unwrap();
    println!("Loaded image:\n{}", image);
    let image = parse_colors(&image).unwrap();
    println!("Color parsed image:\n{}", image);
}
