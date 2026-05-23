use soroban_sdk::contracterror;

#[contracterror]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContractError {
    InsufficientFunds = 1,
    StudentNotFound = 2,
    NotAuthorized = 3,
}