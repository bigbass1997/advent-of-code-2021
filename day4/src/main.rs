
struct Cell {
    val: u8,
    marked: bool,
}
impl Cell {
    pub fn new(val: u8) -> Self { Self {
        val,
        marked: false
    }}
}

struct BingoCard {
    cells: Vec<Cell>,
    winner: bool,
}
impl BingoCard {
    pub fn search_bingo(&self) -> bool {
        for row in self.cells.chunks(5) {
            let mut any_not_marked = false;
            for cell in row {
                if !cell.marked {
                    any_not_marked = true;
                    break;
                }
            }
            
            if !any_not_marked {
                return true;
            }
        }
        
        for col in 0..5 {
            let mut any_not_marked = false;
            for row in 0..5 {
                let i = col + (5 * row);
                if !self.cells[i].marked {
                    any_not_marked = true;
                    break;
                }
            }
            
            if !any_not_marked {
                return true;
            }
        }
        
        false
    }
    
    pub fn update_card(&mut self, number: u8) {
        for cell in &mut self.cells {
            if cell.val == number {
                cell.marked = true;
                return;
            }
        }
    }
    
    pub fn sum_unmarked(&self) -> u32 {
        let mut sum = 0u32;
        for cell in &self.cells {
            if !cell.marked {
                sum += cell.val as u32;
            }
        }
        
        sum
    }
}


fn main() {
    let input = std::fs::read_to_string("day4/input.txt").unwrap();
    let mut lines = input.lines();
    let numbers: Vec<u8> = lines.next().unwrap().split(',').map(|s| s.parse::<u8>().unwrap()).collect();
    
    let mut cards = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() { // create new board using next 5 lines
            let mut cells = Vec::new();
            for _ in 0..5 {
                let line_numbers: Vec<u8> = lines.next().unwrap().split_whitespace().map(|s| s.parse::<u8>().unwrap()).collect();
                for number in line_numbers {
                    cells.push(Cell::new(number));
                }
            }
            cards.push(BingoCard {
                cells,
                winner: false
            });
        }
    }
    
    // Let's call out some numbers, and see who wins!
    let cards_len = cards.len();
    let mut wins = 0usize;
    for number in numbers {
        for card in &mut cards {
            card.update_card(number);
            if !card.winner && card.search_bingo() {
                card.winner = true;
                if wins == 0 {
                    println!("First Bingo! {}", card.sum_unmarked() * (number as u32));
                }
                
                wins += 1;
                
                if wins == cards_len {
                    println!("Last Bingo! {}", card.sum_unmarked() * (number as u32));
                }
            }
        }
    }
}
