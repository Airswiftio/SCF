#![no_std]

mod contract;
mod interface;
mod admin;
mod storage_types;
mod metadata;
mod event;
mod owner;
mod errors;
mod balance;
mod approval;
mod test;

pub use crate::contract::NonFungibleTokenClient;