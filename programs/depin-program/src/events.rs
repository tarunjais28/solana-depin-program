use super::*;

#[event]
pub struct InitializeEvent {}
#[event]
pub struct CreateTokenEvent {
    /// Token Name
    pub name: String,
}

#[event]
pub struct MintEvent {
    pub amount: u64,
}

#[event]
pub struct BurnEvent {
    pub token_a: u64,
    pub token_b: u64,
    pub token_c: u64,
}
