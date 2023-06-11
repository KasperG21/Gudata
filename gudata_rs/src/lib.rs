#[cfg(test)]
mod tests
{
    use std::path::Path;

    use crate::deserializer;

    #[test]
    fn print_file()
    {
        let read = deserializer::read_file(Path::new("tests/test_1.gudata"));
        let should_be = "";

        assert_eq!(read, should_be);
    }
}

mod deserializer;
mod serializer;
