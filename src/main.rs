use std::env;

mod lib;

fn main() {
    lib::run(get_number_from_args());
}

fn get_number_from_args() -> i8 {
    let args : Vec<String> = env::args().collect();

    if args.len() != 2 { return 0; }
    args[1].parse::<i8>().unwrap_or(0)
}
