/// 程序中使用的各种数据结构和类型的长度常量
pub mod lengths {
    /// 账户 discriminator 的字节长度
    pub const DISCRIMINATOR_LENGTH: usize = 8;

    /// Solana 公钥的字节长度
    pub const PUBLIC_KEY_LENGTH: usize = 32;

    /// Rust Vec<u8> 前缀存储长度信息的字节数
    pub const VEC_U8_LENGTH: usize = 4;

    /// 字符串允许的最大长度（以字节为单位）
    pub const STRING_MAX_LENGTH: usize = 256;

    /// 布尔值的字节长度
    pub const BOOL_LENGTH: usize = 1;

    /// i64 整数的字节长度
    pub const I64_LENGTH: usize = 8;

    /// u64 整数的字节长度
    pub const U64_LENGTH: usize = 8;

    pub const MAX_NFT_HOLDERS: usize = 100;

    pub const PODCAST_CHANNEL_SEED: &str = "podcast_channel";
    pub const PODCAST_EP_SEED: &str = "podcast_ep";
    pub const NFT_MINT_SEED: &str = "nft_mint";
    pub const METADATA: &str = "metadata";
}
