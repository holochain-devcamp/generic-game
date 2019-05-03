use std::convert::TryFrom;
use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::holochain_core_types::{
    cas::content::Address,
    dna::entry_types::Sharing,
    error::HolochainError,
    json::JsonString,
    validation::EntryValidationData,
    entry::Entry,
};

use crate::game_state::{GameState, PlayerState};
use crate::game_move::{Move, MoveType, Piece};

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct Game {
    pub player_1: Address,
    pub player_2: Address,
    pub created_at: u32,
}

/// Traverse the linked list rooted at a game to find all the moves
pub fn get_moves(game_address: &Address) -> ZomeApiResult<Vec<Move>> {
    match hdk::get_links(game_address, "")?.addresses().into_iter().next() {
        Some(first_move) => {
            let mut move_addresses = vec![first_move];
            let mut more = true;
            while more {
                more = match hdk::get_links(move_addresses.last().unwrap(), "")?.addresses().into_iter().next() {
                    Some(addr) => {
                        move_addresses.push(addr.clone());
                        true
                    },
                    None => {
                        false
                    },
                }
            }
            let moves: Vec<Move> = move_addresses.iter().map(|addr| {
                let move_entry = hdk::get_entry(addr).unwrap().unwrap();
                if let Entry::App(_, move_struct) = move_entry {
                    Move::try_from(move_struct).expect("Entry at address is type other than Move")
                } else {
                    panic!("Not an app entry!")
                }
            }).collect();
            Ok(moves)
        },
        None => {
            Ok(Vec::new())
        }
    }
}

pub fn get_state(game_address: &Address) -> ZomeApiResult<GameState> {
    let moves = get_moves(game_address)?;
    let new_state = moves.iter().fold(GameState::initial(), state_reducer);
    Ok(new_state)
}

/// takes a current game state and a move and progresses the state
/// assumes that moves are totally valid by this stage
fn state_reducer(current_state: GameState, next_move: &Move) -> GameState {
    match &next_move.move_type {
        MoveType::MovePiece{to, from} => {
            let mut board = board_sparse_to_dense(&current_state);
            // make the move by deleting the piece at the from position and adding one at the to position
            board[from.x][from.y] = 0;
            board[to.x][to.y] = 1;

            // check if any opponent pieces were taken in this move

            let (player_1_pieces, player_2_pieces) = board_dense_to_sparse(board);

            GameState{
                player_1: PlayerState {
                    pieces: player_1_pieces,
                    resigned: false,
                },
                player_2: PlayerState {
                    pieces: player_2_pieces,
                    resigned: false,
                },
                ..current_state
            }
        }
    }
}

fn board_sparse_to_dense(state: &GameState)-> [[u8; 8]; 8] {
    let mut board = [[0u8; 8]; 8];
    state.player_1.pieces.iter().for_each(|piece| {
        board[piece.x][piece.y] = 1;
    });
    state.player_2.pieces.iter().for_each(|piece| {
        board[piece.x][piece.y] = 2;
    });
    board
}

fn board_dense_to_sparse(board: [[u8; 8]; 8]) -> (Vec<Piece>, Vec<Piece>) {
    let mut player_1_pieces = Vec::new();
    let mut player_2_pieces = Vec::new();
    board.iter().enumerate().for_each(|(x, row)| {
        row.iter().enumerate().for_each(|(y, square)| {
            if *square == 1 {
                player_1_pieces.push(Piece{x , y});
            } else if *square == 2 {
                player_2_pieces.push(Piece{x , y});               
            }
        })
    });
    (player_1_pieces, player_2_pieces)
}

pub fn definition() -> ValidatingEntryType {
    entry!(
        name: "game",
        description: "Represents an occurence of a game between several agents",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: | validation_data: hdk::EntryValidationData<Game>| {
            match validation_data {
                EntryValidationData::Create{entry: _, validation_data: _} => {
                    Ok(())
                },
                _ => {
                    Err("Cannot modify or delete a game".into())
                }
            }
        }
    )
}
