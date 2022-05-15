use std::{env, char};

fn main() {
    let args: Vec<String> = env::args().collect();
    let hex = args[1].as_str().trim_start_matches("U+").trim_start_matches("0x");
    let code = u32::from_str_radix(hex, 16).unwrap();
    println!("{}", char::from_u32(code).unwrap().to_string());
}
