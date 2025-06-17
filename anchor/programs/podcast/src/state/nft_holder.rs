use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct NftHolder {
    pub address: Pubkey,
    pub amount: u64,
}
