use anchor_lang::prelude::*;
use crate::{constants::lengths::*, state::NftHolder};

#[account]
pub struct PodcastEp {
    pub id: Vec<u8>,                 // 单集唯一标识
    pub title: Vec<u8>,              // 单集标题
    pub description: Vec<u8>,        // 单集描述
    pub audio_url: Vec<u8>,          // 音频文件 URL
    pub creator: Pubkey,             // 创作者钱包地址
    pub duration: u64,               // 音频时长（秒）
    pub published_at: i64,           // 发布时间戳（Unix 格式）
    pub updated_at: i64,             // 更新时间戳（Unix 格式）
    pub podcast_channel_id: Vec<u8>, // 所属频道 ID
    pub collection: Option<Pubkey>,  // 所属 NFT 集合（可选）

    pub is_nft: bool,                // 是否作为 NFT
    pub nft_mint: Option<Pubkey>,         // NFT 的 Mint 地址
    pub nft_metadata: Option<Pubkey>,     // NFT 元数据地址
    pub nft_edition: Option<Pubkey>,      // NFT 版本地址
    pub nft_holders: Vec<NftHolder>,  // NFT 持有者列表 (地址, 持有数量)

}

impl PodcastEp {
    pub const LEN: usize = DISCRIMINATOR_LENGTH 
        + VEC_U8_LENGTH + STRING_MAX_LENGTH // id
        + VEC_U8_LENGTH + STRING_MAX_LENGTH // title
        + VEC_U8_LENGTH + STRING_MAX_LENGTH // description
        + VEC_U8_LENGTH + STRING_MAX_LENGTH // audio_url
        + PUBLIC_KEY_LENGTH // creator
        + U64_LENGTH // duration
        + I64_LENGTH // published_at
        + I64_LENGTH // updated_at
        + VEC_U8_LENGTH + STRING_MAX_LENGTH // podcast_channel_id
        + BOOL_LENGTH // is_nft
        + (BOOL_LENGTH + PUBLIC_KEY_LENGTH) // nft_mint
        + (BOOL_LENGTH + PUBLIC_KEY_LENGTH) // nft_metadata
        + (BOOL_LENGTH + PUBLIC_KEY_LENGTH) // nft_edition
        + (BOOL_LENGTH + PUBLIC_KEY_LENGTH) // collection
        + VEC_U8_LENGTH + MAX_NFT_HOLDERS * (PUBLIC_KEY_LENGTH + U64_LENGTH); // nft_holders
}
