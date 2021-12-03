use std::collections::HashMap;

type Count = (u32, u32);

fn calc_counts(width: usize, list: &Vec<u16>) -> HashMap<usize, Count> {
    let mut counts: HashMap<usize, Count> = HashMap::new();
    for line in list {
        for i in (0..width).rev() {
            if !counts.contains_key(&i) { counts.insert(i, Default::default()); }
            
            if line & (1 << i) == 0 {
                counts.get_mut(&i).unwrap().0 += 1;
            } else {
                counts.get_mut(&i).unwrap().1 += 1;
            }
        }
    }
    
    counts
}
        
fn most_common(counts: &HashMap<usize, Count>, bit: &usize) -> u16 {
    if counts[bit].0 > counts[bit].1 {
        0
    } else if counts[bit].1 > counts[bit].0 {
        1
    } else {
        1
    }
}

fn least_common(counts: &HashMap<usize, Count>, bit: &usize) -> u16 {
    if counts[bit].0 < counts[bit].1 {
        0
    } else if counts[bit].1 < counts[bit].0 {
        1
    } else {
        0
    }
}

fn solve_part2(mut list: Vec<u16>, width: usize, common_value: fn(&HashMap<usize, Count>, &usize) -> u16) -> u16 {
    for bit in (0..width).rev() {
        let counts = calc_counts(width, &list);
        let mask = common_value(&counts, &bit) << bit;
        
        let mut to_remove: Vec<usize> = vec![];
        for (i, line) in list.iter().enumerate() {
            if (line & (1 << bit)) != (mask & (1 << bit)) {
                to_remove.push(i);
            }
        }
        
        to_remove.sort();
        to_remove.reverse();
        for i in to_remove {
            list.remove(i);
        }
        
        if list.len() == 1 {
            break;
        }
    }
    
    list.first().unwrap().to_owned()
}

fn main() {
    let input = std::fs::read_to_string("day3/input.txt").unwrap();
    
    let lines: Vec<u16> = input.lines().map(|line| u16::from_str_radix(line, 2).unwrap()).collect();
    let width = input.lines().next().unwrap().len();
    
    // Part 1 //
    let counts = calc_counts(width, &lines);
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for i in (0..width).rev() {
        gamma.push_str(&(!(counts[&i].0 > counts[&i].1) as u8).to_string());
        epsilon.push_str(&((counts[&i].0 > counts[&i].1) as u8).to_string());
    }
    println!("Part 1: {}", u32::from_str_radix(&gamma, 2).unwrap() * u32::from_str_radix(&epsilon, 2).unwrap());
    
    // Part 2 //
    let oxygen = solve_part2(lines.clone(), width, most_common) as u32;
    let co2 = solve_part2(lines.clone(), width, least_common) as u32;
    println!("{}", oxygen * co2);
}
