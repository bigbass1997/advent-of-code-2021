use std::time::Instant;

fn main() {
    let input = std::fs::read_to_string("day6/input.txt").unwrap();
    let mut school: Vec<u8> = input.split(',').map(|s| s.parse().unwrap()).collect();
    let mut totals: Vec<u128> = Vec::new();
    
    let mut days = 0usize;
    loop {
        let mut to_add: Vec<u8> = Vec::new();
        for fish in &mut school {
            if *fish == 0 {
                *fish = 6;
                to_add.push(8);
            } else {
                *fish -= 1;
            }
        }
        school.append(&mut to_add);
        totals.push(school.len() as u128);
        
        days += 1;
        if days == 10 { break }
    }
    
    let mut pointer = days - 1;
    for _ in days..256 {
        let new_total = totals[pointer] + (totals[pointer - 6] - totals[pointer - 7]) + (totals[pointer - 8] - totals[pointer - 9]);
        totals.push(new_total);
        
        pointer += 1;
    }
    
    println!("Total after  80 days: {}", totals[79]);
    println!("Total after 256 days: {}", totals.last().unwrap());
}
