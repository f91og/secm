use crypto::{symmetriccipher::SymmetricCipherError, aes, buffer::{RefWriteBuffer, RefReadBuffer}};
use crypto::aes::KeySize::KeySize256;
use crypto::blockmodes::PkcsPadding;
use crypto::buffer::{WriteBuffer, ReadBuffer, BufferResult};
use rand::Rng;
use rand::seq::SliceRandom;
use std::{str, path::Path};
use std::{fs::File, io::Write, collections::BTreeMap};
use std::io::Read;
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

pub fn get_secret_file_path() -> String {
    if let Some(home_dir) = dirs::home_dir() {
        let home_path = home_dir.to_str().expect("Invalid home directory").to_string();
        format!("{}/.secrets", home_path)
    } else {
        panic!("Unable to determine home directory");
    }
}

// change this function to return secret name list rather than a map
pub fn get_secrets(secret_file: &str) -> BTreeMap<String, String> {
    let mut secrets = BTreeMap::new();

    // if file does not exist, create it
    if !Path::new(secret_file).exists() {
        File::create(secret_file).expect("Unable to create file");
    }
    let mut file = File::open(secret_file).expect("Unable to open file");
    // if file is empty, return empty map
    if file.metadata().expect("Unable to get file metadata").len() == 0 {
        return secrets;
    }
    let mut buff = Vec::<u8>::new();
    file.read_to_end(&mut buff).expect("Unable to read data from file");

    let key = get_secm_key();
    let key_bytes = key.as_bytes();
    let mut key_32: [u8; 32] = [0; 32];
    for (i, byte) in key_bytes.iter().enumerate() {
        key_32[i] = *byte;
    }

    let iv = [0; 16];
    let decrypted_data = aes256_cbc_decrypt(buff.as_slice(), &key_32, &iv).unwrap();

    let result = String::from_utf8(decrypted_data).unwrap();

    // split result by \n
    result.split("\n").for_each(|line| {
        let mut parts = line.split(" ");
        // _ = parts.next().expect("Unable to get secret"); // todo: make secret possible to be empty
        let name = parts.next().expect("Unable to get secret name");
        let value = parts.next().expect("Unable to get secret value");
        secrets.insert(name.to_string(), value.trim().to_string());
    });

    secrets
}

// a method that sync a Vec<string> to secret file
pub fn sync_secrets_to_file(secrets: &BTreeMap<String, String>, file_path: &str) {
    let mut file = File::create(file_path).expect("Unable to open file");

    // iterate through secrets map and write each to file
    let mut contents: Vec<String> = vec![];
    for (name, value) in secrets.iter() {
        let secret = format!("{} {}", name, value);
        contents.push(secret);
    }

    let key = get_secm_key();
    let key_bytes = key.as_bytes();
    let mut key_32: [u8; 32] = [0; 32];
    for (i, byte) in key_bytes.iter().enumerate() {
        key_32[i] = *byte;
    }
    let iv = [0; 16];

    let encrypted = aes256_cbc_encrypt(contents.join("\n").as_bytes(), &key_32, &iv).unwrap();
    file.write_all(&encrypted).expect("Unable to write secret");
}

pub fn generate_secm_key() {
    let keychain = SecKeychain::default().expect("Unable to get default keychain");
    let res = keychain.find_generic_password("secm", "secm");
    if res.is_err() {
        let key = generate_random_string(32, false);
        keychain.add_generic_password("secm", "secm", key.as_bytes()).expect("Unable to set secm key");
    }
}

pub fn get_secm_key() -> String {
    let keychain = SecKeychain::default().expect("Unable to get default keychain");
    let (password, _) = keychain.find_generic_password("secm", "secm").unwrap();
    return String::from_utf8(password.as_ref().to_vec()).unwrap()
}

/// Encrypt a buffer with the given key and iv using AES256/CBC/Pkcs encryption.
fn aes256_cbc_encrypt(data: &[u8], key: &[u8; 32], iv: &[u8; 16]) -> Result<Vec<u8>, SymmetricCipherError> {
    let mut encryptor = aes::cbc_encryptor(KeySize256, key, iv, PkcsPadding);

    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);
    let mut read_buffer = RefReadBuffer::new(data);
    let mut final_result = Vec::new();

    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            _ => continue,
        }
    }

    Ok(final_result)
}

/// Decrypt a buffer with the given key and iv using AES256/CBC/Pkcs encryption.
fn aes256_cbc_decrypt(data: &[u8], key: &[u8; 32], iv: &[u8; 16]) -> Result<Vec<u8>, SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(KeySize256, key, iv, PkcsPadding);

    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);
    let mut read_buffer = RefReadBuffer::new(data);
    let mut final_result = Vec::new();

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            _ => continue,
        }
    }

    Ok(final_result)
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_secm_key() {
        generate_secm_key();

        let key = get_secm_key();
        assert!(!key.is_empty());
    }

    #[test]
    fn test_aes256_cbc() {
        use rand::{RngCore, rngs::OsRng};
        let mut rng = OsRng::default();

        let mut key = [0; 32];
        let mut iv = [0; 16];
        rng.fill_bytes(&mut key);
        rng.fill_bytes(&mut iv);

        let data = "Hello World";
        let encrypted_data = aes256_cbc_encrypt(data.as_bytes(), &key, &iv).unwrap();
        let mut file = File::create("secret").expect("Unable to open file");
        // let mut writer = BufWriter::new(file);   bufferWriter需要指编码格式，否则写入结果是乱码而且结果不是二进制文件

        file.write_all(&encrypted_data).expect("Unable to write secret");

        let mut buff = Vec::<u8>::new();
        let mut file = File::open("secret").expect("Unable to open file");
        file.read_to_end(&mut buff).expect("Unable to read data from file");

        let decrypted_data = aes256_cbc_decrypt(buff.as_slice(), &key, &iv).unwrap();

        let result = String::from_utf8(decrypted_data).unwrap();

        assert_eq!(data, result);
        println!("{}", result);
    }

    #[test]
    fn test_wr_u8() {
        let mut file = File::create("test").expect("Unable to open file");
        // Write a slice of bytes to the file
        file.write_all(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]).expect("Unable to write secret");
        let mut file = File::open("test").expect("Unable to open file");
        // read the same file back into a Vec of bytes
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer).expect("Unable to read data from file");
        println!("{:?}", buffer);
    }

    #[test]
    fn test_decrypt_encrypt_with_file() {
        generate_secm_key();
        
        let mut secrets = BTreeMap::new();
        secrets.insert("key1".to_string(), "value1".to_string());
        secrets.insert("key2".to_string(), "value2".to_string());
        sync_secrets_to_file(&secrets, "test");
        let res = get_secrets("test");
        assert_eq!(res["key1"], "value1");
        println!("{:?}", res);
    }
}
