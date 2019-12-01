use std::env;

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return
    }

    run_day(args[1].as_ref())
}

fn run_day(day: &str) {
    match day {
        "1" => day1::main(),
        _ => print!("{}", "Invalid day")
    }
}