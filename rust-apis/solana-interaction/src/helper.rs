use super::*;

pub fn get_deserialize_struct<T: AnchorSerialize + AnchorDeserialize + Sized>(
    mut data: &[u8],
) -> T {
    AnchorDeserialize::deserialize(&mut data).unwrap()
}

pub fn get_pda(seeds: &[&[u8]], program_id: &Pubkey) -> Pubkey {
    let (pubkey, _) = Pubkey::find_program_address(seeds, program_id);
    pubkey
}
