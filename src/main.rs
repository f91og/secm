mod constants;
mod cmds;
mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("{}", constants::ERROR_MSG);
        return;
    }

    let secret_file = &utils::get_secret_file_path();

    let verb = args[1].trim();
    match verb {
        "make" => cmds::cmd_make(&args[2..], secret_file),
        "use" => cmds::cmd_use(args[2].trim(), secret_file),
        "add" => {
            let name = args[2].trim();
            let value = args[3].trim();
            utils::store_secret(name, value, secret_file);
        }
        _ => {print!("{} {}", constants::ERROR_MSG, verb);},
    }
}