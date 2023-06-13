pub trait ToGudata
{
    fn serialize(&self, name: &str) -> String;
}

impl ToGudata for String
{
    fn serialize(&self, name: &str) -> String
    {
        String::from
        (
            format!("{} = \"{}\";", name, self)
        )
    }
}

impl ToGudata for &str
{
    fn serialize(&self, name: &str) -> String
    {
        String::from
        (
            format!("{} = \"{}\";", name, self)
        )
    }
}

impl ToGudata for char
{
    fn serialize(&self, name: &str) -> String
    {
        String::from
        (
            format!("{} = \'{}\';", name, self)
        )
    }
}

impl ToGudata for i128
{
    fn serialize(&self, name: &str) -> String
    {
        String::from
        (
            format!("{} = {};", name, self)
        )    
    }
}

impl ToGudata for u128
{
    fn serialize(&self, name: &str) -> String
    {
        String::from
        (
            format!("{} = {};", name, self)
        )    
    }
}

impl ToGudata for f64
{
    fn serialize(&self, name: &str) -> String
    {
        String::from
        (
            format!("{} = {};", name, self)
        )
    }
}

impl ToGudata for bool
{
    fn serialize(&self, name: &str) -> String
    {
        String::from
        (
            format!("{} = {};", name, self)
        )
    }
}
