use crate::constants::lengths::*;
use anchor_lang::prelude::*;
#[account]
pub struct PodcastChannel {
    pub id: Vec<u8>,                     // 频道唯一标识，用 Vec<u8> 优化存储
    pub title: Vec<u8>,                  // 频道标题
    pub description: Vec<u8>,            // 频道描述
    pub image_url: Vec<u8>,              // 频道封面图 URL
    pub creator: Pubkey,                 // 创作者钱包地址
    pub create_at: i64,                  // 创建时间戳（Unix 格式）
    pub updated_at: i64,                 // 更新时间戳（Unix 格式）
    pub is_nft: bool,                    // 是否关联 NFT
    pub nft_mint: Option<Pubkey>,        // NFT 铸币地址（可选）
    pub authority: Pubkey,               // 频道管理权限地址
    pub is_public: bool,                 // 是否公开
    pub rss_feed_url: String,        // RSS 订阅地址（固定长度数组存储）
    pub episodes: Vec<Pubkey>,           // 关联的播客单集（PodcastEp）账户地址列表
}

impl PodcastChannel {
    pub const LEN: usize = DISCRIMINATOR_LENGTH 
        + VEC_U8_LENGTH + STRING_MAX_LENGTH // id
        + VEC_U8_LENGTH + STRING_MAX_LENGTH // title
        + VEC_U8_LENGTH + STRING_MAX_LENGTH // description
        + VEC_U8_LENGTH + STRING_MAX_LENGTH // image_url
        + PUBLIC_KEY_LENGTH // creator
        + I64_LENGTH // create_at
        + I64_LENGTH // updated_at
        + BOOL_LENGTH // is_nft
        + (BOOL_LENGTH + PUBLIC_KEY_LENGTH) // nft_mint（Option 存储）
        + PUBLIC_KEY_LENGTH // authority
        + BOOL_LENGTH // is_public
        + VEC_U8_LENGTH + STRING_MAX_LENGTH // rss_feed_url
        + (VEC_U8_LENGTH + PUBLIC_KEY_LENGTH * STRING_MAX_LENGTH); // episodes（动态数组）
}
