/*
// first/naive solution
fn part_one() {
    let input = std::fs::read_to_string("day1/input.txt").unwrap();
    let input: Vec<u64> = input.lines().map(|line| line.parse::<u64>().unwrap()).collect();
    
    let mut previous = *input.first().unwrap();
    let mut increases = 0;
    for i in 1..input.len() {
        let next = input[i];
        if next > previous { increases += 1 }
        previous = next;
    }
    println!("Increases: {}", increases);
}

// first/naive solution:
fn part_two() {
    let input: Vec<u64> = std::fs::read_to_string("day1/input.txt").unwrap()
        .lines().map(|line| line.parse::<u64>().unwrap()).collect();
    
    let mut previous: u64 = input[0..3].iter().sum();
    let mut increases = 0;
    for i in 1..(input.len() - 2) {
        let next = input[i..(i + 3)].iter().sum();
        if next > previous { increases += 1 }
        previous = next;
    }
    println!("Window: 3, Increases: {}", increases);
}*/

// rewrote to use different strategy
fn main() {
    let input: Vec<u64> = std::fs::read_to_string("day1/input.txt").unwrap()
        .lines().map(|line| line.parse::<u64>().unwrap()).collect();
    
    let solve = |window: usize| {
        let mut increases = 0;
        for i in 0..(input.len() - window) {
            if input[i] < input[i + window] { increases += 1 }
        }
        //println!("Window: {}, Increases: {}", window, increases);
    };
    solve(1);
    solve(3);
}

