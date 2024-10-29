#![no_main]

mod gameparse;
use gameparse::parse_chess_input;

sp1_zkvm::entrypoint!(main);

use chess_engine::*;

const LOOKAHEAD: i32 = 5;

fn main() {  // Add return type for error handling
    let json_input = r#"{
        "w":["Ke1","Qf3","Bc4","Pd2","Pe4"],
        "b":["Ke8","Pf7","Pg7","Ph7","Pe7"],
        "moves":["e4e5","f3b7","c4e6","b7d7","d7f7"]
    }"#;
    
    let (board, moves) = match parse_chess_input(json_input) {
        Ok((board, moves)) => (board, moves),
        Err(e) => panic!("Failed to parse input") //return //Err(format!("Failed to parse chess input: {}", e))
    };
    let mut b = board;
    let mut history = vec![];

    loop {
        let s = "a2a4";  // Fixed: removed mut as it's not needed
        
        // Fixed: proper move parsing with consistent return types
        let m = if s.is_empty() {
            let (best_m, _, _) = b.get_best_next_move(LOOKAHEAD);
            best_m
        } else {
            match Move::try_from(s.to_string()) {
                Ok(m) => m,
                Err(e) => {
                    continue;
                }
            }
        };

        match b.play_move(m) {
            GameResult::Continuing(next_board) => {
                b = next_board;
                history.push(m);
            }

            GameResult::Victory(winner) => {
                if winner == Color::White {
                    return; //Ok(());
                } else {
                    panic!("Black wins.");
                }
            }

            GameResult::IllegalMove(x) => {
                continue;
            }

            GameResult::Stalemate => {
                return; //Ok(());
            }
        }
        break;
    }

    //Ok(())  // Add final return
}