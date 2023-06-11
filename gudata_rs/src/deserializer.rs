use std::{path::Path, fs};

fn read_file(p: &Path) -> String
{
    let raw_string = fs::read_to_string(p)
        .expect("Couldn't access given file.");
    
    let mut trimmed_string = String::from("");
    for c in raw_string.chars()
    {
        if c != '\n' && c != '\r' && c != '\t' // checking if the character is a newline or tab and
                                               // if so removing it
        {
            trimmed_string.push(c); 
        }
    }

    trimmed_string
}

fn split_vars(raw_data: String) -> Vec<String>
{
    let mut vars: Vec<String> = vec![];
    let mut temp: String = String::new();

    for ch in raw_data.chars()
    {
        if ch != ';'
        {
            temp.push(ch);
        }
        else
        {
            vars.push(temp);
            temp = String::new();
        }
    }

    vars
}
