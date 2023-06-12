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
