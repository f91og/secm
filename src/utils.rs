use rand::Rng;
use rand::seq::SliceRandom;
use std::{fs, fs::File, io::{Write, BufReader, BufRead}};

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

pub fn store_secret(name: &str, value: &str, secret_file: &str) {
    let row = format!("{}: {}\n", name, value);

    // 打开文件并以追加模式写入
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(secret_file)
        .expect("Unable to open file");

    // 写入内容到文件
    file.write_all(row.as_bytes())
        .expect("Unable to write to file");

    println!("Secret stored in {}", secret_file);
}

pub fn get_secret(name: &str, secret_file: &str) -> Option<String> {
    if let Ok(file_content) = fs::read_to_string(secret_file) {
        for line in file_content.lines() {
            if line.starts_with(&format!("{}:", name)) {
                // 找到匹配的行，解析出秘密值并返回
                let secret = line.splitn(2, ':').nth(1);
                if let Some(secret) = secret {
                    return Some(secret.trim().to_string());
                }
            }
        }
    }
    None // 如果未找到匹配的secret，返回 None
}

pub fn get_secret_file_path() -> String {
    if let Some(home_dir) = dirs::home_dir() {
        let home_path = home_dir.to_str().expect("Invalid home directory").to_string();
        format!("{}/.psm_secret", home_path)
    } else {
        panic!("Unable to determine home directory");
    }
}

// change this function to return secret name list rather than a map
pub fn get_secret_list(secret_file: &str) -> Vec<String> {
    let mut names = Vec::new();
    let file = File::open(secret_file).expect("Unable to open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let mut parts = line.split(":");
        let name = parts.next().expect("Unable to get name");
        names.push(name.trim().to_string());
    }
    names
}