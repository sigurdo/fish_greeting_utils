// Parse ASCII art from other sources, for custom usage, primarily neofetch distro logos.
pub mod maninpulations;

use maninpulations::center_image_x;
use regex::Captures;
use regex::Regex;
use std::ops::Index;
use std::{
    io::{stdin, Read},
    str::FromStr,
};
use termion::terminal_size;

pub fn main() {
    let mut neofetch_source_code = String::new();
    stdin()
        .lock()
        .read_to_string(&mut neofetch_source_code)
        .unwrap();
    let distro_regex = Regex::new(
        r#"\n(.*)\)\n\s*?set_colors(.*?)\n\s*?read -rd '' ascii_data <<'EOF'\n((.|\n)*?)EOF"#,
    )
    .unwrap();
    let color_position_regex = Regex::new(r"\$\{c(\d)\}").unwrap();

    fn get_neofetch_color_escape(color_number: &str) -> String {
        let reset = "[0m";
        let bold = "[1m";
        match color_number {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" => format!("{reset}[3{color_number}m{bold}"),
            "7" | "fg" => format!("[37m{reset}"),
            _ => format!("{reset}[38;5;{color_number}m{bold}"),
        }
    }

    for distro_match in distro_regex.captures_iter(&neofetch_source_code) {
        let os_name = String::from_str(&distro_match[1]).unwrap();
        let colors = Vec::from_iter(distro_match[2].split_whitespace());
        let color_escapes: Vec<String> = colors
            .iter()
            .map(|color| get_neofetch_color_escape(color))
            .collect();
        // dbg!(&colors);
        // dbg!(&color_escapes);
        let mut ascii_art = String::from_str(&distro_match[3]).unwrap();
        let default_color_escape = get_neofetch_color_escape(&"fg");
        ascii_art = color_position_regex
            .replace_all(ascii_art.as_str(), |caps: &Captures| {
                if caps.len() < 2 {}
                let color_index = caps[1].parse::<usize>().unwrap() - 1;
                if color_index < color_escapes.len() {
                    return &color_escapes[color_index];
                } else {
                    return &default_color_escape;
                }
            })
            .into_owned();
        let centered = center_image_x(&ascii_art, terminal_size().unwrap_or((80, 20)).0.into());
        print!("[0m[37m{})\n", &os_name);
        print!("{}\n", &centered);
    }
}
