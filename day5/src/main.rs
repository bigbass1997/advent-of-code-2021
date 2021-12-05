use std::cmp::{max, min};
use cgmath::Vector2;

type Segment = (Vector2<u16>, Vector2<u16>);

fn mark_grid(grid: &mut Vec<Vec<u16>>, seg: &Segment, diagonals: bool) {
    if seg.0.x == seg.1.x {
        let smaller = min(seg.0.y, seg.1.y);
        let larger = max(seg.0.y, seg.1.y);
        for y in smaller..=larger {
            grid[seg.0.x as usize][y as usize] += 1;
        }
    } else if seg.0.y == seg.1.y {
        let smaller = min(seg.0.x, seg.1.x);
        let larger = max(seg.0.x, seg.1.x);
        for x in smaller..=larger {
            grid[x as usize][seg.0.y as usize] += 1;
        }
    } else if diagonals {
        if seg.0.x > seg.1.x && seg.0.y > seg.1.y {
            for i in 0..=(seg.0.x - seg.1.x) {
                grid[(seg.1.x + i) as usize][(seg.1.y + i) as usize] += 1;
            }
        } else if seg.0.x < seg.1.x && seg.0.y < seg.1.y {
            for i in 0..=(seg.1.x - seg.0.x) {
                grid[(seg.0.x + i) as usize][(seg.0.y + i) as usize] += 1;
            }
        } else if seg.0.x > seg.1.x && seg.0.y < seg.1.y {
            for i in 0..=(seg.0.x - seg.1.x) {
                grid[(seg.1.x + i) as usize][(seg.1.y - i) as usize] += 1;
            }
        } else if seg.0.x < seg.1.x && seg.0.y > seg.1.y {
            for i in 0..=(seg.1.x - seg.0.x) {
                grid[(seg.1.x - i) as usize][(seg.1.y + i) as usize] += 1; // double check if it doesn't work
            }
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("day5/input.txt").unwrap();
    let mut segments: Vec<Segment> = Vec::new();
    let mut upper_bound = Vector2::new(0usize, 0usize);
    for line in input.lines() {
        let parsed: Vec<Vec<u16>> = line.split(" -> ").map(|point| point.split(',').map(|s| s.parse().unwrap()).collect()).collect();
        segments.push((
            Vector2::new(parsed[0][0], parsed[0][1]),
            Vector2::new(parsed[1][0], parsed[1][1])
        ));
        upper_bound.x = max(upper_bound.x, max(parsed[0][0] as usize, parsed[1][0] as usize));
        upper_bound.y = max(upper_bound.y, max(parsed[0][1] as usize, parsed[1][1] as usize));
    }
    
    
    // part 1 //
    let mut grid = vec![vec![0u16; upper_bound.y + 1]; upper_bound.x + 1];
    for segment in &segments {
        mark_grid(&mut grid, &segment, false);
    }
    
    let mut count = 0usize;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] >= 2 {
                count += 1;
            }
        }
    }
    println!("Part 1: {}", count);
    
    
    // part 2 //
    let mut grid = vec![vec![0u16; upper_bound.y + 1]; upper_bound.x + 1];
    for segment in &segments {
        mark_grid(&mut grid, &segment, true);
    }
    
    let mut count = 0usize;
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] >= 2 {
                count += 1;
            }
        }
    }
    println!("Part 2: {}", count);
}
