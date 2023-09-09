pub mod maninpulations;
use colored::{ColoredString, Colorize};
use maninpulations::center_image_x;
use std::io::{stdin, Read};
use std::{str, u8};
use termion::terminal_size;

struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

pub fn colorize_path(path: &String) -> String {
    // let mut rainbow_colors = vec![
    //     Rgb { r: 255, g: 0, b: 0 },
    //     Rgb {
    //         r: 255,
    //         g: 127,
    //         b: 0,
    //     },
    //     Rgb {
    //         r: 255,
    //         g: 255,
    //         b: 0,
    //     },
    //     Rgb { r: 0, g: 255, b: 0 },
    //     Rgb {
    //         r: 0,
    //         g: 127,
    //         b: 255,
    //     },
    //     Rgb {
    //         r: 165,
    //         g: 0,
    //         b: 255,
    //     },
    //     Rgb {
    //         r: 255,
    //         g: 0,
    //         b: 255,
    //     },
    //     Rgb {
    //         r: 165,
    //         g: 0,
    //         b: 255,
    //     },
    //     Rgb {
    //         r: 0,
    //         g: 127,
    //         b: 255,
    //     },
    //     Rgb { r: 0, g: 255, b: 0 },
    //     Rgb {
    //         r: 255,
    //         g: 255,
    //         b: 0,
    //     },
    //     Rgb {
    //         r: 255,
    //         g: 127,
    //         b: 0,
    //     },
    // ];
    let mut rainbow_colors = vec![
        "#ff0000", "#ff8800", "#ffff00", "#00ff00", "#0088ff", "#aa00ff", "#ff00ff", "#aa00ff",
        "#0088ff", "#00ff00", "#ffff00", "#ff8800",
    ];
    let mut rainbow_colors_iter = rainbow_colors.iter();
    let mut next_color = rainbow_colors_iter.next().unwrap();
    let mut path_colorized = String::new();
    let mut path_component = String::new();
    for character in path.chars() {
        if character != '\n' {
            path_component.push(character);
        }
        if character == '/' {
            path_colorized.push_str(&format!("%F{{{}}}{}", next_color, path_component));
            next_color = rainbow_colors_iter.next().unwrap_or_else(|| {
                // rainbow_colors.reverse();
                rainbow_colors_iter = rainbow_colors.iter();
                rainbow_colors_iter.next().unwrap()
            });
            path_component = String::new();
        }
    }

    path_colorized.push_str(&format!("%F{{{}}}{}", next_color, path_component));

    path_colorized

    // let path_components = path.split("/");

    // path_components
    //     .map(|component| format!("{}", format!("{}/", component).truecolor(255, 123, 255)))
    //     .collect::<Vec<String>>()
    //     .join("")
}

pub fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();

    print!("{}", colorize_path(&input));
}
