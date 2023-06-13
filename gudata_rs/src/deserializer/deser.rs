pub trait FromGudata<T>
{
    fn deserialize(&self) -> T;
}

impl FromGudata<String> for String
{
    fn deserialize(&self) -> String
    {
        let mut should_read = false;
        let mut count = 0;

        let mut trimmed = String::new();
        for ch in self.chars()
        {
            if !should_read && ch == '\"'
            {
                should_read = true;
            }
            else if should_read && ch == '\"' && count == self.len()-1
            {
                should_read = false;
            }
            else if should_read
            {
                trimmed.push(ch);
            }
            count += 1;
        }

        trimmed
    }
}

impl FromGudata<char> for String
{
    fn deserialize(&self) -> char
    {
        let mut should_read = false;
        
        for ch in self.chars()
        {
            if ch == '\'' && !should_read
            {
                should_read = true;
            }
            else if should_read
            {
                return ch;
            }
        }

        panic!("No value found");
    }
}

impl FromGudata<i128> for String
{
    fn deserialize(&self) -> i128
    {
        self.parse().unwrap()     
    }
}

impl FromGudata<u128> for String
{
    fn deserialize(&self) -> u128
    {
        self.parse().unwrap()    
    }
}

impl FromGudata<f64> for String
{
    fn deserialize(&self) -> f64
    {
        self.parse().unwrap()    
    }
}

impl FromGudata<bool> for String
{
    fn deserialize(&self) -> bool
    {
        self.parse().unwrap()    
    }
}
