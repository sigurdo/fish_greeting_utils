use colored::Colorize;
use itertools::Itertools;
use regex::Regex;
use std::{io, vec::Vec};
use substring::Substring;

pub fn decolorize_image(image: &String) -> String {
    let no_color_escape_regex = Regex::new(r"(?m)(?:\A|m)((?:.|\n)*?)(?:\z|)").unwrap();
    let mut cleaned_image = String::new();
    for clean_part in no_color_escape_regex.captures_iter(&image) {
        cleaned_image.push_str(&clean_part[1]);
    }
    cleaned_image
}

pub fn center_image(image: &String, width: usize) -> String {
    #[derive(Debug)]
    struct ImageRow {
        image_row: String,
        width: usize,
    }
    let color_escape_regex = Regex::new(r"(.*?m)").unwrap();
    let image_rows = image.split("\n").map(|image_row| {
        let mut width = image_row.chars().count();
        for color_escape in color_escape_regex.captures_iter(&image_row) {
            width -= &color_escape[0].chars().count();
        }
        ImageRow {
            image_row: image_row.to_owned(),
            width: width,
        }
    });

    let image_width = image_rows.to_owned().max_by_key(|image_row| image_row.width).unwrap().width;
    let mut centered_image = String::new();
    for mut image_row in image_rows {
        let spaces_before = (width - image_width) / 2;
        for _ in 0..spaces_before {
            image_row.image_row.insert(0, ' ');
        }
        let spaces_after = spaces_before + (image_width - image_row.width);
        for _ in 0..spaces_after {
            image_row.image_row.push(' ');
        }
        centered_image.push_str(image_row.image_row.as_str());
        centered_image.push('\n');
    };

    centered_image
}

pub fn parse_colors(image: &String) -> io::Result<String> {
    let image_rows = image.split("\n");
    #[derive(Debug, PartialEq, Eq)]
    enum Section {
        None,
        Colors,
        Image,
    }
    let mut current_section = Section::None;
    let mut image_section = String::new();
    #[derive(Debug, PartialEq, Eq)]
    struct Color {
        identifier: char,
        color: String,
    }
    let mut colors: Vec<Color> = Vec::new();
    for image_row in image_rows {
        if current_section == Section::Image {
            image_section.push_str(&image_row);
            image_section.push_str("\n");
            continue;
        }
        // If we were not in the image section we can trim the line and continue if it was nothing on it
        let image_row = image_row.trim();
        if image_row == "" {
            continue;
        };
        if image_row == "colors:" {
            current_section = Section::Colors;
            continue;
        }
        if image_row == "image:" {
            current_section = Section::Image;
            continue;
        }
        if current_section == Section::Colors {
            let (identifier, color) = image_row.split(" ").next_tuple().unwrap();
            assert!(identifier.chars().count() == 1);
            let identifier = identifier.chars().nth(0).unwrap();
            colors.push(Color {
                identifier: identifier,
                color: String::from(color),
            });
        }
    }

    let mut image_section_colored = String::new();
    loop {
        let color_character = image_section
            .chars()
            .rev()
            .find_position(|&character| colors.iter().any(|color| color.identifier == character));

        if color_character.is_none() {
            // Insert the rest with normal color
            image_section_colored.insert_str(0, format!("{}", image_section.normal()).as_str());
            break;
        }
        // Can unwrap here, since the loop would have broken if color_character was None
        let (index, identifier) = color_character.unwrap();
        let index = image_section.chars().count() - 1 - index;
        // Can unwrap here, since we did already find this identifier in colors
        let color = colors
            .iter()
            .find(|color| color.identifier == identifier)
            .unwrap();

        let string_to_color = image_section
            .substring(index + 1, image_section.chars().count())
            .to_owned();
        image_section = String::from(image_section.substring(0, index));
        let string_colored = match color.color.as_str() {
            "brcyan" => string_to_color.bright_cyan(),
            _ => string_to_color.normal(),
        };
        image_section_colored.insert_str(0, format!("{}", string_colored).as_str());
    }

    Ok(image_section_colored)
}
