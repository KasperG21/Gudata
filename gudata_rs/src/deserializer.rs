use std::{path::Path, fs};

mod deser;
pub use deser::FromGudata;

pub fn read_file(p: &Path) -> String
{
    let raw_string = fs::read_to_string(p)
        .expect("Couldn't access given file.");
    
    let mut trimmed_string = String::from("");
    for c in raw_string.chars()
    {
        if c != '\n' && c != '\r' && c != '\t'              // checking if the character is a newline, space or tab and
                                                            // if so removing it
        {
            trimmed_string.push(c); 
        }
    }

    trimmed_string
}

pub fn split_vars(raw_data: String) -> Vec<String>
{
    let mut vars: Vec<String> = vec![];
    let mut temp: String = String::new();
    let mut should_note = false;

    for ch in raw_data.chars()
    {
        if ch == '='
        {
            should_note = true;
        }
        else if ch == ';'
        {
            let t = temp.trim().to_string();
            vars.push(t);
            temp = String::new();
            should_note = false;
        }
        else if should_note
        {
            temp.push(ch);
        }
    }

    vars
}
