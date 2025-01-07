use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    OfferEmpty = 1,
    OfferExist = 2,
    OfferChanged = 3,
    AdminExist = 4,
    NotAuthorized = 5,
    TokenNotSupported = 6,
    TCDisabled = 7,
    TCNotLoaned = 8,
    TCAlreadyLoaned = 9,
    RecipientNotFound = 10,
    InvalidAmount = 11,
}
