use std::fs;

pub fn main() {
    let input = fs::read_to_string("inputs/day1.txt")
        .expect("Something went wrong reading the file");

    let lines = input.split("\n");

    let mut primary: f32 = 0.0;
    let mut total_fuel = 0.0;
    for line in lines {
        let line_fuel = get_fuel_required(line.parse().unwrap());
        primary += line_fuel;
        total_fuel += line_fuel;

        let mut more_fuel = get_fuel_required(line_fuel);
        while more_fuel > 0.0 {
            total_fuel += more_fuel;
            more_fuel = get_fuel_required(more_fuel)
        }
    }

    println!("Primary fuel usage: {}", primary);
    println!("Total fuel usage: {}", total_fuel);
}

fn get_fuel_required(mass: f32) -> f32 {
    return (mass / 3.0).floor() - 2.0;
}