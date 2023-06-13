#[cfg(test)]
mod tests
{
    use std::path::Path;

    use crate::{deserializer::{self, FromGudata}, serializer};

    #[test]
    fn test_1()
    {
        let vars = deserializer::read_file(&Path::new("tests/test_1.gudata"));
        let var_arr = deserializer::split_vars(vars);

        let arr: Vec<String> = var_arr.into_iter().map(| var: String | var.deserialize()).collect();
    }

    #[test]
    fn test_2()
    {
        let f_d = deserializer::read_file(&Path::new("tests/test_2.gudata"));
        let v_arr = deserializer::split_vars(f_d);

        let mut a = vec![];
        for v in v_arr
        {
            let t: Person = v.deserialize();
            a.push(t);
        }

        let a0 = &a[0];

        assert_eq!(a0.name, String::from("Mike"));
        assert_eq!(a0.age, 52);
    }

    struct Person
    {
        name: String,
        age: i32,
    }

    impl deserializer::FromGudata<Person> for String
    {
        fn deserialize(&self) -> Person
        {
            let mut should_note = false;
            let mut should_note_v2 = false;
            let mut name = String::new();
            let mut age_raw = String::new();
            let mut working_var = 1;

            for ch in self.chars()
            {
                if ch == ':' && !should_note
                {
                    should_note = true;
                }
                else if ch == ',' && should_note
                {
                    should_note = false;
                    working_var += 1;
                }
                else if should_note
                {
                    if ch == '\"' && should_note_v2 == false
                    {
                        should_note_v2 = true;
                    }
                    else if ch == '\"' && should_note_v2
                    {
                        should_note_v2 = true;
                    }
                    else if should_note_v2
                    {
                        match working_var
                        {
                            1 => name.push(ch),
                            2 => age_raw.push(ch),
                            _ => {},
                        }
                    }
                }
            }

            Person
            {
                name: name.trim().to_string(),
                age: age_raw.trim().parse().expect("Value provided wasn't a number..."),
            }
        }
    }
}

pub mod deserializer;
pub mod serializer;
