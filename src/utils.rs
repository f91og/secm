use crypto::{symmetriccipher::SymmetricCipherError, aes, buffer::{RefWriteBuffer, RefReadBuffer}};
use crypto::aes::KeySize::KeySize256;
use crypto::blockmodes::PkcsPadding;
use crypto::buffer::{WriteBuffer, ReadBuffer};
use rand::Rng;
use rand::seq::SliceRandom;
use std::str;
use std::{fs::File, io::{Write, BufReader, BufRead, BufWriter}, collections::BTreeMap};
use security_framework::os::macos::keychain::SecKeychain;

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

// pub fn store_secret(name: &str, value: &str, secret_file: &str) {
//     let row = format!("{}: {}\n", name, value);

//     // 打开文件并以追加模式写入
//     let mut file = fs::OpenOptions::new()
//         .write(true)
//         .append(true)
//         .open(secret_file)
//         .expect("Unable to open file");

//     // 写入内容到文件
//     file.write_all(row.as_bytes())
//         .expect("Unable to write to file");

//     println!("Secret stored in {}", secret_file);
// }

// pub fn get_secret(name: &str, secret_file: &str) -> Option<String> {
//     if let Ok(file_content) = fs::read_to_string(secret_file) {
//         for line in file_content.lines() {
//             if line.starts_with(&format!("{}:", name)) {
//                 // 找到匹配的行，解析出秘密值并返回
//                 let secret = line.splitn(2, ':').nth(1);
//                 if let Some(secret) = secret {
//                     return Some(secret.trim().to_string());
//                 }
//             }
//         }
//     }
//     None // 如果未找到匹配的secret，返回 None
// }

pub fn get_secret_file_path() -> String {
    if let Some(home_dir) = dirs::home_dir() {
        let home_path = home_dir.to_str().expect("Invalid home directory").to_string();
        format!("{}/.psm_secret", home_path)
    } else {
        panic!("Unable to determine home directory");
    }
}

// change this function to return secret name list rather than a map
pub fn get_secrets(secret_file: &str) -> BTreeMap<String, String> {
    let mut secrets = BTreeMap::new();

    let file = File::open(secret_file).expect("Unable to open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let mut parts = line.split(" ");
        // _ = parts.next().expect("Unable to get secret"); // todo: make secret possible to be empty
        let name = parts.next().expect("Unable to get secret name");
        let value = parts.next().expect("Unable to get secret value");
        secrets.insert(name.to_string(), value.trim().to_string());
    }

    secrets
}

// a method that sync a Vec<string> to secret file
pub fn sync_secrets_to_file(secrets: &BTreeMap<String, String>) {
    let file = File::create(get_secret_file_path()).expect("Unable to open file");
    let mut writer = BufWriter::new(file);

    // iterate through secrets map and write each to file
    for (name, value) in secrets.iter() {
        let secret = format!("{} {}\n", name, value);
        let key = get_secm_key();
        let encrypt_secret = aes256_cbc_encrypt(secret.as_bytes(), key.as_bytes(), &[0; 16]);
        writer.write_all(secret.as_bytes()).expect("Unable to write secret");
    }
}

pub fn generate_secm_key() {
    let password = generate_random_string(10, true);
    let keychain = SecKeychain::default().expect("Unable to get default keychain");

    keychain.set_generic_password("secm", "secm", password.as_bytes()).expect("Unable to set password");
}

pub fn get_secm_key() -> String {
    let keychain = SecKeychain::default().expect("Unable to get default keychain");
    let (password, _) = keychain.find_generic_password("secm", "secm").unwrap();
    return String::from_utf8(password.as_ref().to_vec()).unwrap()
}

/// Encrypt a buffer with the given key and iv using AES256/CBC/Pkcs encryption.
fn aes256_cbc_encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, SymmetricCipherError> {
    let mut encryptor = aes::cbc_encryptor(KeySize256, key, iv, PkcsPadding);

    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);
    let mut read_buffer = RefReadBuffer::new(data);
    let mut final_result = Vec::new();

    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferUnderflow => break,
            _ => continue,
        }
    }

    Ok(final_result)
}

/// Decrypt a buffer with the given key and iv using AES256/CBC/Pkcs encryption.
fn aes256_cbc_decrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(KeySize256, key, iv, PkcsPadding);

    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);
    let mut read_buffer = RefReadBuffer::new(data);
    let mut final_result = Vec::new();

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferUnderflow => break,
            _ => continue,
        }
    }

    Ok(final_result)
}

#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_generate_and_get_secm_key() {
        // Test generate_secm_key
        generate_secm_key();

        // Test get_secm_key
        let key = get_secm_key();
        assert!(!key.is_empty());  // Assuming the key should not be empty
    }

    #[test]
    fn test_aes256_cbc() {
        use rand::{RngCore, rngs::OsRng};
        let mut rng = OsRng::default();

        let mut key = [0; 32];
        let mut iv = [0; 16];
        rng.fill_bytes(&mut key);
        rng.fill_bytes(&mut iv);

        let data = "Hello, world!";
        let encrypted_data = aes256_cbc_encrypt(data.as_bytes(), &key, &iv).unwrap();
        let decrypted_data = aes256_cbc_decrypt(encrypted_data.as_slice(), &key, &iv).unwrap();

        let result = String::from_utf8(decrypted_data).unwrap();

        assert_eq!(data, result);
        println!("{}", result);
    }
}
