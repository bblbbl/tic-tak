use std::io;
use rand::Rng;

pub struct Board {
    cells: Vec<Vec<String>>
}

impl Board {
    pub fn new() -> Board {
        let mut cells: Vec<Vec<String>> = vec![];
        for _ in 0..3 {
            let mut row : Vec<String> = vec![];
            for _ in 0..3 {
                row.push(String::from("*"))
            }

            cells.push(row);
        }

        Board {
            cells
        }
    }

    pub fn print(&self) {
        for row in &self.cells {
            for cell in row {
                print!("{} ", cell)
            }
            println!()
        }
    }

    pub fn player_turn(&mut self) {
        loop {
            let mut x = String::new();
            let mut y = String::new();

            match io::stdin().read_line(&mut x) {
                Ok(_) => {},
                Err(e) => {
                    println!("{}", e)
                }
            }

            match io::stdin().read_line(&mut y) {
                Ok(_) => {},
                Err(e) => {
                    println!("{}", e)
                }
            }

            let ix = x.replace("\n", "").parse::<usize>().unwrap() - 1;
            let iy = y.replace("\n", "").parse::<usize>().unwrap() - 1;

            if self.is_cell_empty(ix, iy) {
                self.cells[ix][iy] = "X".to_string();
                self.print();
                break
            } else {
                println!("Cell not empty! Try again...")
            }
        }
    }

    pub fn ai_turn(&mut self) {
        let mut rng = rand::thread_rng();

        loop {
            let x = rng.gen_range(0..3);
            let y = rng.gen_range(0..3);

            if self.is_cell_empty(x, y) {
                self.cells[x][y] = "O".to_string();
                self.print();
                break;
            }
        }
    }

    pub fn check_win(&self, symbol: &String) -> bool {
        return self.check_verticals(symbol) || self.check_horizontals(symbol) || self.check_corners(symbol);
    }

    fn check_verticals(&self, symbol: &String) -> bool {
        for y in 0..3 {
            let mut is_win = true;
            for x in 0..3 {
                if self.cells[y][x] != symbol.to_string() {
                    is_win = false;
                    break;
                }
            }

            if is_win {
                return true;
            }
        }

        return false;
    }

    fn check_horizontals(&self, symbol: &String) -> bool {
        for x in 0..3 {
            let mut is_win = true;
            for y in 0..3 {
                if self.cells[y][x] != symbol.to_string() {
                    is_win = false;
                    break
                }
            }

            if is_win {
                return true
            }
        }

        return false
    }

    fn check_corners(&self, symbol: &String) -> bool {
        let sym_str = symbol.to_string();
        return (self.cells[0][0] == sym_str && self.cells[1][1] == sym_str && self.cells[2][2] == sym_str)
            || (self.cells[2][0] == sym_str && self.cells[1][1] == sym_str && self.cells[0][2] == sym_str)
    }

    pub fn is_board_full(&self) -> bool {
        for row in &self.cells {
            for cell in row {
                if cell != "*" {
                    return false
                }
            }
        }

        return true
    }

    pub fn clear(&mut self) {
        for y in 0..3 {
            for x in 0..3 {
                self.cells[y][x] = "*".to_string()
            }
        }
    }

    fn is_cell_empty(&self, x: usize, y: usize) -> bool{
        return self.cells[x][y] == "*".to_string();
    }
}