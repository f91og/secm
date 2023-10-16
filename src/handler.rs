use crate::constants::ERROR_MSG;
use crate::utils;

pub fn handle_make(args: &[String], secret_file: &str) {
    let mut length = 10;
    let mut advance = false;

    let name = args[0].trim();
    if name.starts_with("-") {
        println!("{}", ERROR_MSG);
        return;
    }

    for i in 1..args.len() {
        let arg = args[i].trim().trim_start_matches("-");
        let arg_value: Vec<&str> = arg.split("=").collect();
        match arg_value[0] {
            "l" | "length" => {
                let length_arg = arg_value[1];
                // let length = cmd_args.get("length").unwrap_or(&"10");
                if let Ok(length_arg) = length_arg.parse::<usize>() {
                // https://stackoverflow.com/questions/37936058/why-does-iterating-over-a-hashmapstr-str-yield-a-str
                // let random_string = generate_random_string(length, *cmd_args.get("advance").unwrap_or(&"false") == "true");
                    length = length_arg
                } else {
                    println!("length arg is not numeric");
                    return;
                }
            },
            "a" | "advance" => advance = true,
            _ => {
                println!("{}", ERROR_MSG);
                return;
            }
        }
    }

    let random_string = &utils::generate_random_string(length, advance);
    println!("Generated secret {}: {}", name, random_string);
    utils::store_secret(name, random_string, secret_file);
}