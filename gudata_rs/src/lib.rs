#[cfg(test)]
mod tests
{
    use std::path::Path;

    use crate::deserializer;

    #[test]
    fn test_1()
    {
        let file_data = deserializer::read_file(&Path::new("tests/test_1.gudata"));
        let var_arr = deserializer::split_vars(file_data);

        let mut arr:Vec<char> = vec![];
        for var in var_arr
        {
            let v: char = deserializer::read_vars(var);
            arr.push(v);
        }

        let a0 = arr[0];
        let a1 = arr[1];

        assert_eq!(a0, 'a');
        assert_eq!(a1, '\'');
    }
}

mod deserializer;
mod serializer;
