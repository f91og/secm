use rand::Rng;
use rand::seq::SliceRandom;

pub fn generate_random_string(length: usize, advance: bool) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    const DIGIT: &[u8] = b"0123456789";
    const SYMBOL: &[u8] = b"!@#$%^&*-_=+;:,./?";

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