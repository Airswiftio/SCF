use soroban_sdk::{contract, contracterror, contractimpl, log, symbol_short, Env, Symbol};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    OfferEmpyt = 1,
    OfferExist = 2,
    OfferChanged = 3,
    AdminExist = 4,
}