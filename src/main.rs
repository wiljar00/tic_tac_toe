use std::io;

// Represents the player's mark on the board
#[derive(PartialEq, Clone, Copy, Debug)]
enum Mark {
    X,
    O,
    Empty,
}

// Represents the game board
struct Board {
    cells: [Mark; 9],
}

impl Board {
    fn new() -> Board {
        Board {
            cells: [Mark::Empty; 9],
        }
    }

    fn print(&self) {
        println!("\n-------------");
        for row in 0..3 {
            print!("| ");
            for col in 0..3 {
                let mark = self.cells[row * 3 + col];
                let symbol = match mark {
                    Mark::X => "X",
                    Mark::O => "O",
                    Mark::Empty => " ",
                };
                print!("{} | ", symbol);
            }
            println!("\n-------------");
        }
    }

    fn make_move(&mut self, index: usize, player: Mark) -> bool {
        if index >= 9 || self.cells[index] != Mark::Empty {
            return false;
        }
        self.cells[index] = player;
        true
    }

    fn check_winner(&self, player: Mark) -> bool {
        let patterns = [
            // Rows
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
            // Columns
            [1, 4, 7],
            [2, 5, 8],
            [3, 6, 9],
            // Diagonals
            [1, 5, 9],
            [3, 5, 7],
        ];
        for pattern in patterns.iter() {
            if pattern.iter().all(|&i| self.cells[i] == player) {
                return true;
            }
        }
        false
    }
}

fn title() {
    println!("  ---     ---     ---     ");
    println!("  Want to play a game?    ");
    println!("  ---     ---     ---     ");
    println!("\n  How about TicTacToe?");
}

fn demo_board() {
    println!("-------------");
    println!("| 1 | 2 | 3 |");
    println!("-------------");
    println!("| 4 | 5 | 6 |");
    println!("-------------");
    println!("| 7 | 8 | 9 |");
    println!("-------------");
}

fn instructions() {
    println!("\n-- HOW TO PLAY -- ");
    println!("\nAlright, this is a game for 2 people! ...because you'd never game alone.....right? me neither!");
    println!("Below is a board showing all the moves you can make when asked.");
    println!("Each player chooses a number to a corresponding box until someone wins.");
    println!("\nGame on!\n");
}

fn main() {
    title();
    instructions();
    demo_board();

    println!("\n --------------------------- \n");

    let mut board = Board::new();
    let mut current_player = Mark::X;

    loop {
        println!("\n");
        println!("Current board:");
        board.print();

        println!("Player {:?}, enter your move (1-9):", current_player);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if !board.make_move(input, current_player) {
            println!("Invalid move, try again.");
            continue;
        }

        if board.check_winner(current_player) {
            println!("Player {:?} wins!", current_player);
            break;
        }

        if !board.cells.iter().any(|&cell| cell == Mark::Empty) {
            println!("It's a draw!");
            break;
        }

        current_player = match current_player {
            Mark::X => Mark::O,
            Mark::O => Mark::X,
            Mark::Empty => Mark::Empty,
        };
    }
}
