mod constants;
mod handler;
mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("{}", constants::ERROR_MSG);
        return;
    }

    let verb = args[1].trim();
    match verb {
        "make" => handler::handle_make(&args[2..]),
        "use" => println!("use command will be implemented soon"),
        _ => {
        }
    }
}