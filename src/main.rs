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
    if args.len() != 2 || args[1] != "make" {
        println!("Usage: psm make");
    } else {
        let random_string = generate_random_string(12);
        println!("Generated random password: {}", random_string);
    }
}
