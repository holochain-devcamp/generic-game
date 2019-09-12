/**
 * All of this code is specific to the game Checkers
 * By changing the moves, state, reducer and validation rules you can implement you own game.
 */

pub mod state;
pub mod validation;
pub mod moves;

pub use self::{
    state::{
        GameState,
    },
    moves::{
        MoveType,
    },
};
