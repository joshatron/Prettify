use std::collections::HashMap;

pub fn extract_u8(name: &str, converter_options: &HashMap<String, String>, default: u8) -> u8 {
    let option_value = converter_options.get(name);

    if option_value == None {
        default
    } else {
        match option_value.unwrap().parse::<u8>() {
            Ok(i) => i,
            Err(_) => default,
        }
    }
}

pub fn extract_bool(
    name: &str,
    converter_options: &HashMap<String, String>,
    default: bool,
) -> bool {
    let option_value = converter_options.get(name);

    if option_value == None {
        default
    } else if option_value.unwrap().eq_ignore_ascii_case("true") {
        true
    } else if option_value.unwrap().eq_ignore_ascii_case("false") {
        false
    } else {
        default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_u8() {
        let mut options = HashMap::new();
        options.insert(String::from("valid_true1"), String::from("true"));
        options.insert(String::from("valid_true2"), String::from("TRUE"));
        options.insert(String::from("valid_true3"), String::from("tRUe"));
        options.insert(String::from("valid_false1"), String::from("false"));
        options.insert(String::from("valid_false2"), String::from("FALSE"));
        options.insert(String::from("valid_false3"), String::from("FaLSe"));
        options.insert(String::from("invalid1"), String::from("tru"));
        options.insert(String::from("invalid2"), String::from("t"));
        options.insert(String::from("invalid3"), String::from("f"));
        options.insert(String::from("invalid4"), String::from("57"));
        options.insert(String::from("empty"), String::from(""));

        assert_eq!(extract_bool("valid_true1", &options, false), true);
        assert_eq!(extract_bool("valid_true2", &options, false), true);
        assert_eq!(extract_bool("valid_true3", &options, false), true);
        assert_eq!(extract_bool("valid_false1", &options, true), false);
        assert_eq!(extract_bool("valid_false2", &options, true), false);
        assert_eq!(extract_bool("valid_false3", &options, true), false);
        assert_eq!(extract_bool("invalid1", &options, false), false);
        assert_eq!(extract_bool("invalid2", &options, false), false);
        assert_eq!(extract_bool("invalid3", &options, true), true);
        assert_eq!(extract_bool("invalid4", &options, false), false);
        assert_eq!(extract_bool("empty", &options, true), true);
        assert_eq!(extract_bool("not_present", &options, true), true);
    }

    #[test]
    fn test_extract_bool() {
        let mut options = HashMap::new();
        options.insert(String::from("valid1"), String::from("8"));
        options.insert(String::from("valid2"), String::from("5"));
        options.insert(String::from("valid_biggest"), String::from("255"));
        options.insert(String::from("valid_smallest"), String::from("0"));
        options.insert(String::from("not_number1"), String::from("a"));
        options.insert(String::from("not_number2"), String::from("5d"));
        options.insert(String::from("empty"), String::from(""));
        options.insert(String::from("too_big"), String::from("256"));
        options.insert(String::from("too_small"), String::from("-1"));

        assert_eq!(extract_u8("valid1", &options, 1), 8);
        assert_eq!(extract_u8("valid2", &options, 1), 5);
        assert_eq!(extract_u8("valid_biggest", &options, 1), 255);
        assert_eq!(extract_u8("valid_smallest", &options, 1), 0);
        assert_eq!(extract_u8("not_number1", &options, 1), 1);
        assert_eq!(extract_u8("not_number2", &options, 3), 3);
        assert_eq!(extract_u8("empty", &options, 37), 37);
        assert_eq!(extract_u8("too_big", &options, 26), 26);
        assert_eq!(extract_u8("too_small", &options, 0), 0);
    }
}
