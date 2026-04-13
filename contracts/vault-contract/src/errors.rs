use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VaultError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    NegativeAmount = 3,
    InsufficientBalance = 4,
    Unauthorized = 5,
    InvalidToken = 6,
    RewardCalculationError = 7,
}
