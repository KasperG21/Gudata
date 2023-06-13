use std::{path::Path, fs, io::Write};

mod ser;

fn write_file(p: &Path, buf: &str)
{
    let mut file = fs::OpenOptions
        ::new()
        .write(true)
        .create(true)
        .open(p)
        .unwrap();

    file.write(buf.as_bytes())
        .expect("Couldn't acces file, maybe it doesn't exist or you don't have permissions to use the file.");
}
