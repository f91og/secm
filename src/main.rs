use rand::Rng;

fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx =rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    password
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: psm make|use)");
        return;
    }

    let verb = args[1].trim();
    match verb {
        "make" => {
            let mut length: usize = 10;
            if args.len() > 2 {
                    length = match args[2].trim().strip_prefix("-l=") {
                    Some(val) => val.parse().unwrap_or(10),
                    None => {
                        println!("Invalid argument format. Usage: psm make -l=10");
                        return;
                    }
                };
            } 
            let random_string = generate_random_string(length);
            println!("Generated password: {}", random_string);
        },
        "use" => {
            println!("use command will be implemented soon");
        },
        _ => {
        }
    }
}
