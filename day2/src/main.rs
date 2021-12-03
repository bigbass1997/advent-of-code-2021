
#[derive(Default)]
pub struct Pos {
    aim: i64,
    depth: i64,
    horiz: i64
}

fn main() {
    let input = std::fs::read_to_string("day2/input.txt").unwrap();
    let pairs: Vec<(&str, i64)> = input.lines().map(|line| {
        let mut split = line.split_whitespace();
        
        (split.next().unwrap(), split.next().unwrap().parse::<i64>().unwrap())
    }).collect();
    
    let mut pos = Pos::default();
    for pair in &pairs {
        match pair {
            ("forward", h) => {
                pos.horiz += h; 
                pos.depth += pos.aim * h; 
            },
            ("down", a) => pos.aim += a,
            ("up", a) => pos.aim -= a,
            _ => panic!()
        }
    }
    println!("Part1: {}, Part2: {}", pos.horiz * pos.aim, pos.horiz * pos.depth);
}

