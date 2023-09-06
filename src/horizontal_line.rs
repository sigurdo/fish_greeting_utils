use clap::Parser;
use colored::Colorize;
use hsv::hsv_to_rgb;
use rand::prelude::*;
use termion::terminal_size;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::from("rainbow"))]
    theme: String,

    #[arg(short, long)]
    width: Option<u16>,
}

pub fn horizontal_line(theme: String, width: Option<u16>) -> String {
    let (columns, _rows) = if width.is_some() {
        (width.unwrap(), 0)
    } else {
        terminal_size().unwrap()
    };

    let mut line = String::new();

    let hue_start = rand::random::<f64>() * 360.0;
    let hue_frequency = 1.0 + rand::random::<f64>() * 1.0;

    for i in 0..columns {
        let (r, g, b) = match theme.as_str() {
            "rainbow" => hsv_to_rgb(
                (hue_start + (f64::from(i) / f64::from(columns)) * hue_frequency * 360.0) % 360.0,
                1.0,
                1.0,
            ),
            _ => hsv_to_rgb(0.0, 0.0, 1.0),
        };
        // println!("rgb: {}, {}, {}, i: {}", r, g, b, i);
        line += format!("{}", "-".truecolor(r, g, b)).as_str();
    }

    line
}

pub fn main() {
    // let mut input = String::new();
    // stdin().lock().read_to_string(&mut input).unwrap();
    let args = Args::parse();

    println!("{}", horizontal_line(args.theme, args.width));
}
