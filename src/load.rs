use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

pub fn load_image(image_name: &String, image_directory: &String) -> io::Result<String> {
    let mut file = File::open(Path::new(&format!("{}/{}.txt", image_directory, image_name)))?;
    let mut image = String::new();
    file.read_to_string(&mut image)?;
    Ok(image)
}
