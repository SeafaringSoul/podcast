// src/errors.rs
use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient SOL balance to create channel")]
    InsufficientBalance,

    #[msg("该单集不是 NFT")]
    EpisodeNotNFT,
    #[msg("Invalid Metaplex Metadata program ID")]
    InvalidMetadataProgramId,

    #[msg("Invalid episode error")]
    InvalidEpisodeCreator,
    #[msg("Metadata PDA 与 Mint 不匹配")]
    InvalidMetadataPda,

    #[msg("Title 不是有效的 UTF-8 字符串")]
    InvalidUtf8,

    #[msg("频道权限不匹配")]
    InvalidChannelAuthority,
}
