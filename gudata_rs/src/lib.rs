#[cfg(test)]
mod tests
{
    use std::path::Path;

    use crate::deserializer;

    #[test]
    fn print_file()
    {
        let read = deserializer::read_file(Path::new("tests/test_1.gudata"));
        let should_be = "Mike: Person\r\n{\r\n\t\"Key\": String = \"Value\",\r\n\t\"Key\": bool = true,\r\n}\r\n";

        assert_eq!(read, should_be);
    }
}

mod deserializer;
mod serializer;

