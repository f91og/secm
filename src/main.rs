use rand::Rng;
use rand::seq::SliceRandom;

fn generate_random_string(length: usize, advance: bool) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    const DIGIT: &[u8] = b"0123456789";
    const SYMBOL: &[u8] = b"!@#$%^&*()-_=+[]{};:,<.>/?";

    let mut rng = rand::thread_rng();
    if advance {
        let password_str: String = (0..length-2)
            .map(|_| {
                let idx =rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        let digit = DIGIT[rng.gen_range(0..DIGIT.len())] as char;
        let symbol = SYMBOL[rng.gen_range(0..DIGIT.len())] as char;
        let pre_password = format!("{}{}{}", password_str, digit, symbol);
        let mut char_vec: Vec<char> = pre_password.chars().collect();
        char_vec.shuffle(&mut rng);
        char_vec.into_iter().collect()
    } else {
        let password: String = (0..length)
            .map(|_| {
                let idx =rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        password
    }
}

fn handle_make(args: &[String]) {
    let mut length = 10;
    let mut advance = false;

    for i in 0..args.len() {
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
                println!("Invalid argument. Usage: psm make -l=10 -a=true");
                return;
            }
        }
    }

    let random_string = generate_random_string(length, advance);
    println!("Generated password: {}", random_string);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: psm make|use)");
        return;
    }

    let verb = args[1].trim();
    match verb {
        "make" => handle_make(&args[2..]),
        "use" => println!("use command will be implemented soon"),
        _ => {
        }
    }
}
