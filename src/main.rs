use utils::get_secret_file_path;

mod constants;
mod handler;
mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("{}", constants::ERROR_MSG);
        return;
    }

    let secret_file = &get_secret_file_path();

    let verb = args[1].trim();
    match verb {
        "make" => handler::handle_make(&args[2..], secret_file),
        "use" => {
            let name = args[2].trim();
            let res = utils::get_secret(name, secret_file).unwrap_or("Secret not found".to_string());
            println!("{}", res);
        },
        "add" => {
            let name = args[2].trim();
            let value = args[3].trim();
            utils::store_secret(name, value, secret_file);
        }
        _ => {print!("{} {}", constants::ERROR_MSG, verb);},
    }
}