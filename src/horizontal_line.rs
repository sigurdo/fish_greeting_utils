use std::vec;

use clap::{arg, Parser};
use colored::Colorize;
use hsv::hsv_to_rgb;
use rand::{seq::SliceRandom, Rng};
// use rand::prelude::*;
use rand::SeedableRng;
// use rand_chacha::ChaChaRng8Core;
use rand_chacha::ChaCha8Rng;
use termion::terminal_size;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct HorizontalLineArgs {
    #[arg(short, long, default_value = "rainbow")]
    theme: String,

    #[arg(short, long)]
    width: Option<u16>,

    #[arg(short, long)]
    zsh_prompt_colors: bool,

    #[arg(short, long, default_value = "-")]
    character: char,

    #[arg(short, long)]
    seed: Option<u64>,
}

pub fn colorize_string(string: String, rgb: (u8, u8, u8), zsh_prompt: bool) -> String {
    let (r, g, b) = rgb;
    if zsh_prompt {
        format!("%F{{#{:02x}{:02x}{:02x}}}{}%f", r, g, b, string)
    } else {
        format!("{}", string.truecolor(r, g, b))
    }
}

pub fn horizontal_line(args: &HorizontalLineArgs) -> Result<String, String> {
    let theme = &args.theme;
    let width = args.width;
    let zsh_prompt_colors = args.zsh_prompt_colors;
    let character = args.character;
    let mut rng = if args.seed.is_some() {
        ChaCha8Rng::seed_from_u64(args.seed.unwrap())
    } else {
        ChaCha8Rng::from_entropy()
    };

    let (columns, _rows) = if width.is_some() {
        (width.unwrap(), 0)
    } else {
        terminal_size().unwrap()
    };

    // let mut line = String::new();

    let line = match theme.as_str() {
        "rainbow" => {
            let hue_start = rng.gen_range(0.0..360.0);
            let hue_frequency = rng.gen_range(1.0..2.0);
            let line = (0..columns)
                .map(|i| {
                    let rgb = hsv_to_rgb(
                        (hue_start + (f64::from(i) / f64::from(columns)) * hue_frequency * 360.0)
                            % 360.0,
                        1.0,
                        1.0,
                    );
                    colorize_string(String::from(character), rgb, zsh_prompt_colors)
                })
                .collect();
            line
        }
        "white" => {
            let line = (0..columns).map(|_i| character).collect();
            line
        }
        "taktlaus" => {
            let colors = {
                let mut colors = vec![(255, 0, 0), (255, 255, 0), (0, 128, 0), (0, 75, 255)];
                colors.shuffle(&mut rng);
                colors
            };
            let mut num_undistributable_characters = columns % colors.len() as u16;
            let line = colors
                .iter()
                .map(|rgb| {
                    let text_length = columns / colors.len() as u16
                        + if num_undistributable_characters > 0 {
                            num_undistributable_characters -= 1;
                            1
                        } else {
                            0
                        };
                    let text = (0..text_length).map(|_i| character).collect();
                    colorize_string(text, *rgb, zsh_prompt_colors)
                })
                .collect();
            line
        }
        _ => {
            return Err(format!("theme {} not recognized", theme));
        }
    };

    Ok(line)
}

pub fn main() {
    // let mut input = String::new();
    // stdin().lock().read_to_string(&mut input).unwrap();
    let args = HorizontalLineArgs::parse();

    match horizontal_line(&args) {
        Ok(line) => println!("{}", line),
        Err(message) => {
            println!("Could not generate horizontal line:");
            println!("{}", message);
        }
    };
}
