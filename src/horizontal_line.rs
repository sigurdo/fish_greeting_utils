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

    enum DistributeRemainderStrategy {
        PackLeft,
        PackRight,
        DistributeLeft,
        DistributeRight,
    }
    fn line_from_colored_sections_extend_last(sections: &Vec<(u16, (u8, u8, u8))>, distribute_remainder: DistributeRemainderStrategy, columns: u16, character: char, zsh_prompt_colors: bool) -> String {
        let mut num_undistributable_characters = columns % sections.len() as u16;
        let sum_section_widths = sections.iter().map(|section| { section.0 }).collect::<Vec<_>>().iter().sum::<u16>();
        match distribute_remainder {
            DistributeRemainderStrategy::PackLeft | DistributeRemainderStrategy::DistributeLeft => {
                let mut it = sections.iter();
                let mut line = String::new();
                while let Some((relative_width, color)) = it.next() {
                    let absolute_width = (relative_width * columns) / sum_section_widths
                        + if num_undistributable_characters > 0 {
                            let result = match distribute_remainder {
                                DistributeRemainderStrategy::PackLeft => num_undistributable_characters,
                                _ => 1,
                            };
                            num_undistributable_characters -= result;
                            result
                        } else { 0 };
                    let text = (0..absolute_width).map(|_i| character).collect();
                    line = line + colorize_string(text, *color, zsh_prompt_colors).as_str()
                }
                line
            },
            DistributeRemainderStrategy::PackRight | DistributeRemainderStrategy::DistributeRight => {
                let mut it = sections.iter().rev();
                let mut line = String::new();
                while let Some((relative_width, color)) = it.next() {
                    let absolute_width = (relative_width * columns) / sum_section_widths
                        + if num_undistributable_characters > 0 {
                            let result = match distribute_remainder {
                                DistributeRemainderStrategy::PackRight => num_undistributable_characters,
                                _ => 1,
                            };
                            num_undistributable_characters -= result;
                            result
                        } else { 0 };
                    let text = (0..absolute_width).map(|_i| character).collect();
                    line = colorize_string(text, *color, zsh_prompt_colors) + line.as_str()
                }
                line
            }
        }
    }

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
            let sections = vec![
                (1, (255, 0, 0)),
                (1, (255, 255, 0)),
                (1, (0, 128, 0)),
                (1, (0, 75, 255)),
            ];
            line_from_colored_sections_extend_last(&sections, DistributeRemainderStrategy::DistributeLeft, columns, character, zsh_prompt_colors)
        }
        "norge" => {
            let red = (187, 4, 11);
            let white = (255, 255, 255);
            let blue = (0, 47, 167);
            let sections = vec![(6, red), (1, white), (2, blue), (1, white), (12, red)];
            line_from_colored_sections_extend_last(&sections, DistributeRemainderStrategy::PackRight, columns, character, zsh_prompt_colors)
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
