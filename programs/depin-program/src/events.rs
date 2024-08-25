use super::*;

#[event]
pub struct CreateTokenEvent {
    /// Token Name
    pub name: String,
}
