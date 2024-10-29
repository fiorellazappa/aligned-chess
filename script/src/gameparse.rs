use chess_engine::*;
use serde::Deserialize;
use std::str::FromStr;

// Constants for colors
const WHITE: Color = Color::White;
const BLACK: Color = Color::Black;

#[derive(Deserialize)]
pub struct ChessInput {
    w: Vec<String>, // White pieces
    b: Vec<String>, // Black pieces
    moves: Vec<String>,
}

#[derive(Debug)]
struct PiecePosition {
    piece_type: char,
    pos: Position,
}

impl FromStr for PiecePosition {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 3 {
            return Err("Invalid position string".to_string());
        }

        let piece_type = s.chars().next().unwrap();
        let pos_str = &s[1..]; // Get everything after the piece type
        let pos = Position::pgn(pos_str)?;

        Ok(PiecePosition { piece_type, pos })
    }
}

impl ChessInput {
    pub fn to_board(&self) -> Result<Board, String> {
        let mut builder = BoardBuilder::from(Board::empty());

        // Add white pieces
        for piece_str in &self.w {
            let pos = PiecePosition::from_str(piece_str)
                .map_err(|e| format!("Error parsing white piece {}: {}", piece_str, e))?;
            match pos.piece_type {
                'K' => builder = builder.piece(Piece::King(WHITE, pos.pos)),
                'Q' => builder = builder.piece(Piece::Queen(WHITE, pos.pos)),
                'B' => builder = builder.piece(Piece::Bishop(WHITE, pos.pos)),
                'P' => builder = builder.piece(Piece::Pawn(WHITE, pos.pos)),
                _ => return Err(format!("Unsupported piece type: {}", pos.piece_type)),
            }
        }

        // Add black pieces
        for piece_str in &self.b {
            let pos = PiecePosition::from_str(piece_str)
                .map_err(|e| format!("Error parsing black piece {}: {}", piece_str, e))?;
            match pos.piece_type {
                'K' => builder = builder.piece(Piece::King(BLACK, pos.pos)),
                'Q' => builder = builder.piece(Piece::Queen(BLACK, pos.pos)),
                'B' => builder = builder.piece(Piece::Bishop(BLACK, pos.pos)),
                'P' => builder = builder.piece(Piece::Pawn(BLACK, pos.pos)),
                _ => return Err(format!("Unsupported piece type: {}", pos.piece_type)),
            }
        }

        Ok(builder.build())
    }

    pub fn get_moves(&self) -> &Vec<String> {
        &self.moves
    }
}

pub fn parse_move(move_str: &str) -> Result<(Position, Position), String> {
    if move_str.len() != 4 {
        return Err("Move string must be exactly 4 characters".to_string());
    }

    let from_str = &move_str[0..2];
    let to_str = &move_str[2..4];

    let from = Position::pgn(from_str)?;
    let to = Position::pgn(to_str)?;

    Ok((from, to))
}

pub fn process_next_move(moves: &mut Vec<String>) -> Result<(Position, Position), String> {
    moves
        .get(0)
        .ok_or_else(|| "No moves remaining".to_string())
        .and_then(|move_str| parse_move(move_str))
        .map(|result| {
            moves.remove(0); // Only remove if parsing succeeded
            result
        })
}

// Example usage function
pub fn parse_chess_input(json_str: &str) -> Result<(Board, Vec<String>), String> {
    let input: ChessInput =
        serde_json::from_str(json_str).map_err(|e| format!("JSON parsing error: {}", e))?;

    let board = input.to_board()?;
    Ok((board, input.moves))
}
