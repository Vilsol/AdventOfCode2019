use std::collections::HashMap;
use std::fs;

pub fn main() {
    let input = fs::read_to_string("inputs/day3.txt")
        .expect("Something went wrong reading the file");

    let mut grid: HashMap<isize, HashMap<isize, isize>> = HashMap::new();

    let wires = input.split("\n");

    let mut closest = std::isize::MAX;
    let mut shortest = std::isize::MAX;

    let mut first = true;
    for wire in wires {
        let instructions = wire.split(",");
        let mut x: isize = 0;
        let mut y: isize = 0;
        let mut steps: isize = 0;

        for i in instructions {
            let direction: &str = &i[0..1];
            let count = (&i[1..] as &str).parse::<usize>().unwrap();

            for _ in 0..count {
                steps += 1;

                match direction {
                    "U" => y -= 1,
                    "D" => y += 1,
                    "L" => x -= 1,
                    "R" => x += 1,
                    _ => print!("{}", "Invalid direction")
                }

                let inner = grid.entry(x).or_insert(HashMap::new());

                if first {
                    if inner.get(&y) == None {
                        inner.insert(y, steps);
                    }
                } else {
                    if inner.get(&y) != None {
                        closest = std::cmp::min(closest, x.abs() + y.abs());
                        shortest = std::cmp::min(shortest, inner.get(&y).unwrap() + steps);
                    }
                }
            }
        }

        first = false;
    }

    println!("Closest collision: {}", closest);
    println!("Shortest collision: {}", shortest);
}
