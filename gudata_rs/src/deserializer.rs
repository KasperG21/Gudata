use std::{path::Path, fs};

pub fn read_file(p: &Path) -> String
{
    let file = fs::read_to_string(p).expect("Couldn't access given file.");

    file
}
