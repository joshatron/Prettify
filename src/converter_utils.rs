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
    } else if option_value.unwrap().to_lowercase().eq("true") {
        true
    } else if option_value.unwrap().to_lowercase().eq("false") {
        false
    } else {
        default
    }
}
