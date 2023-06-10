use std::{path::Path, fs};

fn read_file(p: Path)
{
    fs::read_to_string(p)
}
