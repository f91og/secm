use crate::utils;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
// use serde_json::json;
// use std::fs::File;
// use std::io::Write;

pub fn cmd_make(args: &[String]) -> Result<(), String> {
    let mut length = 10;
    let mut advance = false;

    let name = args[0].trim();
    if name.starts_with("-") {
        return Err("invalid name".to_string());
    }

    let mut value = "".to_string();

    for i in 1..args.len() {
        let arg = args[i].trim().trim_start_matches("-");
        let arg_value: Vec<&str> = arg.split("=").collect();
        match arg_value[0] {
            "v" | "value" => {
                let value_arg = arg_value[1];
                if value_arg != "" {
                    value = value_arg.to_string();
                } else {
                    return Err("secret value is empty".to_string());
                }
            },
            "l" | "length" => {
                let length_arg = arg_value[1];
                if let Ok(length_arg) = length_arg.parse::<usize>() {
                // https://stackoverflow.com/questions/37936058/why-does-iterating-over-a-hashmapstr-str-yield-a-str
                // let random_string = generate_random_string(length, *cmd_args.get("advance").unwrap_or(&"false") == "true");
                    length = length_arg;
                } else {
                    return Err("length arg is not numeric".to_string());
                }
            },
            "a" | "advance" => advance = true,
            _ => return Err("invalid argument".to_string())
        }
    }

    if value == "" {
        value = utils::generate_random_string(length, advance);
    }

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    // ---- rust中所有权的问题 --------
    // 这里要使用value.to_owned())或value.clone()来查创建一个具有独立的所有权的新对象，对原始对象没有影响
    // set_contents()的参数是 data: String, 这意味着会发生所有权转移
    ctx.set_contents(value.to_owned()).expect("Failed to set clipboard content");
    println!("Generated secret {}: ********, copied to clipboard", name);
    // utils::store_secret(name, &value, secret_file);

    // ctx.set_contents(value).expect("Failed to set clipboard content");
    // println!("Generated secret {}: ********, copied to clipboard", name);
    // utils::store_secret(name, &value, secret_file);
    // ------------------------------

    Ok(()) // 只有写在最后的且没加分号的才会被当成返回值
}

// pub fn cmd_export() -> Result<(), String> {
//     let secret_file = "secrets.json";
//     let secrets = utils::get_secrets(); // secrets 是 Vec<(String, String)>
    
//     // 转换 secrets 为 JSON 格式
//     let mut secrets_json = Vec::new();
//     for (name, key) in secrets {
//         secrets_json.push(json!({
//             "name": name,
//             "key": key,
//         }));
//     }
    
//     // 打开文件并写入 JSON 数据
//     let mut file = File::create(secret_file).map_err(|e| format!("Unable to create secret file: {}", e))?;
//     let json_string = serde_json::to_string_pretty(&secrets_json)
//         .map_err(|e| format!("Failed to serialize secrets to JSON: {}", e))?;
    
//     file.write_all(json_string.as_bytes())
//         .map_err(|e| format!("Failed to write to secret file: {}", e))?;

//     println!("Exported all secrets to secrets.json");
//     Ok(())
// }