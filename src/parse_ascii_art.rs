// Parse ASCII art from other sources, for custom usage, primarily neofetch distro logos.
pub mod maninpulations;

use regex::Regex;
use regex::Captures;
use std::ops::Index;
use std::{io::{stdin, Read}, str::FromStr};
use maninpulations::center_image;

pub fn main() {
    // let args: Vec<String> = env::args().collect();
    // let image_directory = &args[1];
    // let image_names = &args[2..];
    // let image_name = image_names.choose(&mut rand::thread_rng()).unwrap();
    // let image = load_image(&String::from(image_name.as_str()), &image_directory).unwrap();
    // let image = parse_colors(&image).unwrap();
    // let columns = terminal_size().unwrap_or((80, 20)).0;
    // let image = center_image(&image, columns.into());

    let mut neofetch_source_code = String::new();
    stdin().lock().read_to_string(&mut neofetch_source_code).unwrap();
    let distro_regex = Regex::new(r#""(.*?)".*?\)\n\s*?set_colors(.*?)\n\s*?read -rd '' ascii_data <<'EOF'\n((.|\n)*?)EOF"#).unwrap();
    let color_position_regex = Regex::new(r"\$\{c(\d)\}").unwrap();

    fn get_neofetch_color_escape(color_number: &str) -> String {
        let reset = "[0m";
        let bold = "[1m";
        // let color_nr = "2";
        // let color = format!("{reset}[3{color_nr}m{bold}Ok, la oss nå se{reset}");
        match color_number {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" => format!("{reset}[3{color_number}m{bold}"),
            "7" | "fg" => format!("[37m{reset}"),
            _ => format!("{reset}[38;5;{color_number}m{bold}")
        }
    }

    for distro_match in distro_regex.captures_iter(&neofetch_source_code) {
        let os_name = String::from_str(&distro_match[1]).unwrap();
        dbg!(os_name);
        let colors = Vec::from_iter(distro_match[2].split_whitespace());
        let color_escapes: Vec<String> = colors.iter().map(|color| get_neofetch_color_escape(color)).collect();
        // dbg!(&colors);
        // dbg!(&color_escapes);
        let mut ascii_art = String::from_str(&distro_match[3]).unwrap();
        // fn replace_with_color_escape<R: Replacer>(
        //     re: Regex,
        //     src: &str,
        //     mut rep: R,
        // ) -> String {
        //     let color_index = src.parse::<u32>().unwrap();
        //     let color_escape = color_escapes[color_index];
        //     let dst = re.replace_all(text, rep)
        //     dst.into_owned()
        // }
        // ascii_art = color_position_regex.replace_all(&ascii_art, |caps: &Captures| {
        //     let color_index = caps[1].parse::<u32>().unwrap();
        //     let color_escape = color_escapes[color_index];
        //     color_escape
        // }).into_owned();
        let default_color_escape = get_neofetch_color_escape(&"fg");
        ascii_art = color_position_regex.replace_all(ascii_art.as_str(), |caps: &Captures| {
            if caps.len() < 2 {
            }
            // dbg!(&caps);
            // dbg!(&caps[1]);
            let color_index = caps[1].parse::<usize>().unwrap() - 1;
            if color_index < color_escapes.len() {
                return &color_escapes[color_index];
            }
            else {
                return &default_color_escape;
            }
        }).into_owned();
        // let color_index
        // dbg!(&color_escapes[0]);
        // let mut ascii_art_color_escaped = String::new();
        // for escape_match in color_position_regex.find_iter(&ascii_art) {
        //     let color_index = escape_match.as_str().parse::<u32>().unwrap();
        //     let color_escape = color_escapes[color_index];
        //     ascii_art_color_escaped.push_str(ascii_art.as_bytes().take(limit));
        // }
        // dbg!(&ascii_art);
        let centered = center_image(&ascii_art, 170);
        print!("{}\n", &centered);
    }
    // let reset = "[0m";
    // let bold = "[1m";
    // let color_nr = "2";
    // let color = format!("{reset}[3{color_nr}m{bold}Ok, la oss nå se{reset}");
    // dbg!(&color);
    // print!("{}\n", color);
    // print!("noko mer her\n");


}
