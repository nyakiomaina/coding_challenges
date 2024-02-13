mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <json_string>", args[0]);
        std::process::exit(1);
    }

    let input =  &args[1];
    let tokens = lexer::tokenize(input);
    let is_valid = parser::parse(tokens);

    if is_valid {
        println!("Valid JSON");
        std::process::exit(0);
    } else {
        println!("Invalid JSON");
        std::process::exit(1);
    }
}