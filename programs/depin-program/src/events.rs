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
