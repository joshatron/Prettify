use prettify;

fn main() {
    let result = prettify::prettify_default(&String::from(r#"{"hello":"world"}"#));
    println!("{}", result);
}
