use crate::error::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(title: Vec<u8>)]
pub struct PodcastChannelAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = PodcastChannel::LEN, // Adjust space as needed
        seeds = [b"podcast_channel", authority.key().as_ref(), &title],
        bump
    )]
    pub pda_podcast_channel: Account<'info, PodcastChannel>,

    pub system_program: Program<'info, System>,
}

pub fn create_channel_handler(
    ctx: Context<PodcastChannelAccount>,
    title: Vec<u8>,
    description: Vec<u8>,
    image_url: Vec<u8>,
) -> Result<()> {
    let lamports_balance = ctx.accounts.authority.lamports();
    if lamports_balance < 35_000_000 {
        return err!(ErrorCode::InsufficientBalance);
    }

    let podcast_channel = &mut ctx.accounts.pda_podcast_channel;

    podcast_channel.id = Vec::from("channel_".as_bytes())
        .into_iter()
        .chain(title.clone())
        .collect();
    podcast_channel.title = title;
    podcast_channel.description = description;
    podcast_channel.image_url = image_url;
    podcast_channel.creator = ctx.accounts.authority.key();
    podcast_channel.create_at = Clock::get()?.unix_timestamp;
    podcast_channel.updated_at = Clock::get()?.unix_timestamp;
    podcast_channel.is_nft = false;
    podcast_channel.nft_mint = None;
    podcast_channel.authority = ctx.accounts.authority.key();
    podcast_channel.is_public = true;
    podcast_channel.rss_feed_url = String::new(); // 使用空字符串初始化 RSS 订阅地址
    podcast_channel.episodes = Vec::new(); // 初始无关联单集
    Ok(())
}
