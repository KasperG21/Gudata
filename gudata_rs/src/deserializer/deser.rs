pub trait FromGudata<T>
{
    fn deserialize(&self) -> T;
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

impl FromGudata<(i128, i128)> for String
{
    fn deserialize(&self) -> (i128, i128)
    {
        let mut temp_1 = String::new();
        let mut temp_2 = String::new();
        let mut working_temp = 0;

        for ch in self.chars()
        {
            if ch != '(' && ch != ')'
            {
                if ch != ',' 
                {
                    match working_temp
                    {
                        0 => temp_1.push(ch),
                        1 => temp_2.push(ch),
                        _ => {},
                    }
                }
                else
                {
                    working_temp += 1; 
                }
            }
        }

        (temp_1.parse().unwrap(),
        temp_2.parse().unwrap())
    }
}
