use super::*;

/// The struct containing instructions for creating tokens
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CreateParams {
    /// Token Name
    pub name: String,

    /// Symbol
    pub symbol: String,

    /// URI
    pub uri: String,
}
