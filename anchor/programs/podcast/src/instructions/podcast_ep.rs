use crate::error::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3},
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::types::{Creator as MetadataCreator, DataV2};

#[derive(Accounts)]
#[instruction(title: Vec<u8>)]
pub struct PodcastEpAccount<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = PodcastEp::LEN,
        seeds = [b"podcast_ep", creator.key().as_ref(), &title],
        bump
    )]
    pub pda_podcast_ep: Account<'info, PodcastEp>,

    #[account(mut, has_one = creator @ ErrorCode::InvalidChannelAuthority)]
    pub pda_podcast_channel: Account<'info, PodcastChannel>,

    #[account(
        init,
        payer = creator,
        mint::decimals = 0,
        mint::authority = creator,
        mint::freeze_authority = creator
    )]
    pub nft_mint: Account<'info, Mint>,

    /// CHECK: This will be validated against PDA
    #[account(mut)]
    pub nft_metadata: UncheckedAccount<'info>,

    /// Associated token account for creator
    #[account(
        init_if_needed,
        payer = creator,
        associated_token::mint = nft_mint,
        associated_token::authority = creator
    )]
    pub creator_nft_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: token metadata program
    pub token_metadata_program: UncheckedAccount<'info>,
}

pub fn create_ep_handler(
    ctx: Context<PodcastEpAccount>,
    etitle: Vec<u8>,
    description: Vec<u8>,
    audio_url: Vec<u8>,
    duration: u64,
) -> Result<()> {
    let creator = &ctx.accounts.creator;
    let podcast_ep = &mut ctx.accounts.pda_podcast_ep;
    let podcast_channel = &mut ctx.accounts.pda_podcast_channel;

    // Validate metadata PDA
    let mint_key = ctx.accounts.nft_mint.key();
    let metadata_seeds = &[
        b"metadata",
        mpl_token_metadata::ID.as_ref(),
        mint_key.as_ref(),
    ];
    let (expected_metadata, _) =
        Pubkey::find_program_address(metadata_seeds, &mpl_token_metadata::ID);
    require_keys_eq!(
        expected_metadata,
        ctx.accounts.nft_metadata.key(),
        ErrorCode::InvalidMetadataPda
    );

    // Initialize podcast episode state
    podcast_ep.id = b"ep_".iter().chain(etitle.iter()).cloned().collect();
    podcast_ep.title = etitle.clone();
    podcast_ep.description = description;
    podcast_ep.audio_url = audio_url;
    podcast_ep.creator = creator.key();
    podcast_ep.duration = duration;
    podcast_ep.published_at = Clock::get()?.unix_timestamp;
    podcast_ep.updated_at = Clock::get()?.unix_timestamp;
    podcast_ep.podcast_channel_id = podcast_channel.id.clone();
    podcast_ep.collection = None;

    // NFT state
    podcast_ep.is_nft = true;
    podcast_ep.nft_mint = Some(mint_key);
    podcast_ep.nft_metadata = Some(ctx.accounts.nft_metadata.key());
    podcast_ep.nft_edition = None;
    podcast_ep.nft_holders = vec![NftHolder {
        address: creator.key(),
        amount: 1,
    }];

    // Update channel
    podcast_channel.episodes.push(podcast_ep.key());
    podcast_channel.updated_at = Clock::get()?.unix_timestamp;

    // Metadata data
    let raw_name = String::from_utf8(etitle.clone()).map_err(|_| ErrorCode::InvalidUtf8)?;
    let name = format!("{:<32}", raw_name.chars().take(32).collect::<String>());
    let symbol = "ZOKU".to_string();
    let uri = "https://example.com/your-nft.json".to_string();

    let metadata_data = DataV2 {
        name,
        symbol,
        uri,
        seller_fee_basis_points: 0,
        creators: Some(vec![MetadataCreator {
            address: creator.key(),
            verified: false,
            share: 100,
        }]),
        collection: None,
        uses: None,
    };

    // Create metadata
    create_metadata_accounts_v3(
        CpiContext::new(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.nft_metadata.to_account_info(),
                mint: ctx.accounts.nft_mint.to_account_info(),
                mint_authority: creator.to_account_info(),
                payer: creator.to_account_info(),
                update_authority: creator.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
        ),
        metadata_data,
        true,
        true,
        None,
    )?;

    // Mint to creator's ATA
    let cpi_accounts = MintTo {
        mint: ctx.accounts.nft_mint.to_account_info(),
        to: ctx.accounts.creator_nft_account.to_account_info(),
        authority: creator.to_account_info(),
    };
    mint_to(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        1,
    )?;

    Ok(())
}
