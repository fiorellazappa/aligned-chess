#![no_main]

//mod gameparse;
//use gameparse::parse_chess_input;
//use tiny_keccak::{Hasher, Sha3};


sp1_zkvm::entrypoint!(main);
// naive solution

//use chess_engine::*;

fn main() {  // Add return type for error handling
    /* 
    let json_input = r#"{
        "w":["Ke1","Qf3","Bc4","Pd2","Pe4"],
        "b":["Ke8","Pf7","Pg7","Ph7","Pe7"],
        "moves":["e4e5","f3b7","c4e6","b7d7","d7f7"]
    }"#;
    */
    let json_input = sp1_zkvm::io::read::<String>();  

    let expected_input = String::from("{\"b\":[\"Kf8\",\"Pf7\",\"Pg7\",\"Ph7\",\"Pe7\"],\"moves\":[\"f3f7\"],\"w\":[\"Ke1\",\"Qf3\",\"Bc4\",\"Pd2\",\"Pe4\"]}");

    // TODO, this is a naive approach, we need to do a engine to validate moves, and pick best move for PC
    // for now this will work on moves that force player to something, example checkmate

    // Compare strings properly
    if json_input != expected_input {
        panic!("Invalid input");
    }
}