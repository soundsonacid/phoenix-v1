//! Phoenix is a limit order book exchange on the Solana blockchain.
//!
//! It exposes a set of instructions to create, cancel, and fill orders.
//! Each event that modifies the state of the book is recorded in an event log which can
//! be queried from a transaction signature after each transaction is confirmed. This
//! allows clients to build their own order book and trade history.
//!
//! The program is able to atomically match orders and settle trades on chain. This
//! is because each market has a fixed set of users that are allowed to place limit
//! orders on the book. Users who swap against the book will have their funds settle
//! instantaneously, while the funds of users who place orders on the book will be
//! immediately available for withdraw post fill.
//!

#[macro_use]
mod log;
pub mod program;
pub mod quantities;
// Note this mod is private and only exists for the purposes of IDL generation
mod shank_structs;
pub mod state;

use solana_program::pubkey::Pubkey;
// You need to import Pubkey prior to using the declare_id macro
use ellipsis_macros::declare_id;
declare_id!("PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY");