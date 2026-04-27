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
    ZeroDepositAmount = 8,   // issue #2
    ContractPaused = 9,      // for #16
    CooldownNotMet = 10,  // for #14
}