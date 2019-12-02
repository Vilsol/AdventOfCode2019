use std::fs;

pub fn main() {
    let input = fs::read_to_string("inputs/day2.txt")
        .expect("Something went wrong reading the file");

    let mut ops: Vec<usize> = input.split(",").filter_map(|x| x.parse().ok()).collect();

    // Replacements
    ops[1] = 12;
    ops[2] = 2;

    println!("End Part 1 Value: {}", run_computer(&ops));

    let target: usize = 19690720;
    for noun in 0..99 {
        ops[1] = noun;
        for verb in 0..99 {
            ops[2] = verb;

            if run_computer(&ops) == target {
                println!("End Part 2 Value: 100 * {} + {} = {}", noun, verb, 100 * noun + verb);
                return;
            }
        }
    }
}

fn run_computer(ops: &[usize]) -> usize {
    let mut memory = vec![Default::default(); ops.len()];

    memory.clone_from_slice(ops);

    let mut position = 0;
    loop {
        let op = memory[position];
        if op == 1 {
            let first = memory[position + 1];
            let second = memory[position + 2];
            let target = memory[position + 3];
            memory[target] = memory[first] + memory[second];
        } else if op == 2 {
            let first = memory[position + 1];
            let second = memory[position + 2];
            let target = memory[position + 3];
            memory[target] = memory[first] * memory[second];
        } else if op == 99 {
            break
        }

        position += 4
    }

    return memory[0]
}