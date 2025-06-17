#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("FZKxcgMWhMmXVoWM7iwN8GNzmE8WHZ1pt7SiJDCUsFtR");

#[program]
pub mod podcast {

    use super::*;

    pub fn create_channel(
        ctx: Context<PodcastChannelAccount>,
        title: Vec<u8>,
        description: Vec<u8>,
        image_url: Vec<u8>,
    ) -> Result<()> {
        instructions::create_channel_handler(ctx, title, description, image_url)
    }

    pub fn create_ep(
        ctx: Context<PodcastEpAccount>,
        etitle: Vec<u8>,
        description: Vec<u8>,
        audio_url: Vec<u8>,
        duration: u64,
    ) -> Result<()> {
        instructions::create_ep_handler(ctx, etitle, description, audio_url, duration)
    }
}
