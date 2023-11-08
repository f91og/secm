use crate::utils;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

pub fn cmd_make(args: &[String], secret_file: &str) -> Result<(), String> {
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
    utils::store_secret(name, &value, secret_file);

    // ctx.set_contents(value).expect("Failed to set clipboard content");
    // println!("Generated secret {}: ********, copied to clipboard", name);
    // utils::store_secret(name, &value, secret_file);
    // ------------------------------

    Ok(()) // 只有写在最后的且没加分号的才会被当成返回值
}

// pub fn cmd_use(name: &str, secret_file: &str) {
//     let secret = utils::get_secret(name, secret_file).unwrap_or("Secret not found".to_string());
//     let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
//     ctx.set_contents(secret.to_owned()).expect("Failed to set clipboard content");
//     println!("Use secret {}: ********, copied to clipboard", name);
// }