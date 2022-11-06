mod board;

use std::io;
use board::Board;

fn main() {
    let mut board = Board::new();

    let exit = false;
    while !exit {
        board.print();
        loop {
            board.player_turn();

            if board.check_win(&"X".to_string()) {
                println!("You win congratulations");
                break
            }

            if board.is_board_full() {
                println!("Board is full");
                break
            }


            board.ai_turn();

            if board.check_win(&"O".to_string()) {
                println!("Ai win");
                break
            }

            if board.is_board_full() {
                println!("Board is full");
                break
            }
        }

        if !ask_new_game() {
            println!("Ok, goodbye");
            break;
        }

        board.clear()
    }
}

fn ask_new_game() -> bool {
    println!("Want to restart? Inter: 1");

    let mut answer = String::new();
    match io::stdin().read_line(&mut answer) {
        Ok(_) => {},
        Err(e) => {
            println!("{}", e)
        }
    }

    return answer.replace("\n", "") == "1"
}
