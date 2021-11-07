use std::collections::HashMap;

pub fn extract_u16(name: &str, converter_options: &HashMap<String, String>, default: u16) -> u16 {
    let option_value = converter_options.get(name);

    if option_value == None {
        default
    } else {
        match option_value.unwrap().parse::<u16>() {
            Ok(i) => i,
            Err(_) => default,
        }
    }
}
