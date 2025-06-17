import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Podcast } from "../target/types/podcast";
import { describe, expect, it, beforeAll, jest } from "@jest/globals";

import {
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { PublicKey, Keypair, SystemProgram, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import fs from "fs";

jest.setTimeout(20000);

describe("Podcast Program", () => {
  const MPL_TOKEN_METADATA_PROGRAM_ID = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Podcast as Program<Podcast>;

  // const authority = Keypair.generate();
  const authority = Keypair.fromSecretKey(
    Uint8Array.from(
      JSON.parse(
        fs.readFileSync("/Users/anbai/.config/solana/id.json", "utf-8")
      )
    )
  );

  beforeAll(async () => {
    const balance = await provider.connection.getBalance(authority.publicKey);
    if (balance < 1 * anchor.web3.LAMPORTS_PER_SOL) {
      console.log("💰 Airdropping to authority...");
      const sig = await provider.connection.requestAirdrop(
        authority.publicKey,
        2 * anchor.web3.LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(sig);
    }
  });


  async function createTestChannel(authority: Keypair) {
    const title = `Test Channel ${Math.floor(Math.random() * 100000)}`;
    const description = "Testing Channel Description";
    const imageUrl = "https://test.com/channel.png";

    const [channelPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("podcast_channel"),
        authority.publicKey.toBuffer(),
        Buffer.from(title),
      ],
      program.programId
    );

    await program.methods
      .createChannel(
        Buffer.from(title),
        Buffer.from(description),
        Buffer.from(imageUrl)
      )
      .accounts({
        authority: authority.publicKey,
        pdaPodcastChannel: channelPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([authority])
      .rpc();

    return channelPda;
  }

  async function createTestEpisode(
    authority: Keypair,
    channelPda: PublicKey,
    epTitle: string
  ) {
    const epDescription = "Test episode description";
    const epAudioUrl = "https://test.com/audio.mp3";
    const duration = new anchor.BN(1800); // 30 分钟

    // PDA 计算
    const [episodePda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("podcast_ep"),
        authority.publicKey.toBuffer(),
        Buffer.from(epTitle),
      ],
      program.programId
    );

    // 这里用 Keypair 生成 Mint，因为 mint 是一个独立账户，不是 PDA
    const mintKeypair = Keypair.generate();

    // airdrop 给 mintKeypair 以防止余额不足（用于创建账户）
    const sig = await provider.connection.requestAirdrop(
      mintKeypair.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(sig);

    // 计算 Metadata PDA 和 Edition PDA（必须用 mint 公钥计算）
    const [metadataPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintKeypair.publicKey.toBuffer(),
      ],
      MPL_TOKEN_METADATA_PROGRAM_ID
    );

    const [editionPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        MPL_TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintKeypair.publicKey.toBuffer(),
        Buffer.from("edition"),
      ],
      MPL_TOKEN_METADATA_PROGRAM_ID
    );

    await program.methods
      .createEp(
        Buffer.from(epTitle),
        Buffer.from(epDescription),
        Buffer.from(epAudioUrl),
        duration
      )
      .accounts({
        creator: authority.publicKey,
        pdaPodcastChannel: channelPda,
        pdaPodcastEp: episodePda,
        nftMint: mintKeypair.publicKey,
        nftMetadata: metadataPda,
        nftEdition: editionPda,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
        tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
      })
      .signers([authority, mintKeypair])
      .rpc();

    return { episodePda, mintKeypair, metadataPda, editionPda };
  }

  it("create channel && episode && mint NFT", async () => {
    try {
      const channelPda = await createTestChannel(authority);
      const epTitle = `Episode #${Math.floor(Math.random() * 10000)}`;
      const { episodePda, mintKeypair, metadataPda } =
        await createTestEpisode(authority, channelPda, epTitle);

      const episodeAccount = await program.account.podcastEp.fetch(episodePda);

      // 校验单集标题
      expect(Buffer.from(episodeAccount.title).toString()).toEqual(epTitle);

      // 校验 nftMint 与 metadata
      expect(episodeAccount.nftMint!.toBase58()).toEqual(
        mintKeypair.publicKey.toBase58()
      );
      expect(episodeAccount.nftMetadata!.toBase58()).toEqual(
        metadataPda.toBase58()
      );

      // 由于我们并未创建 edition，这里应当为 null
      expect(episodeAccount.nftEdition).toBeNull();

      console.log("✅ Episode Created");
      console.log("   > Mint:", episodeAccount.nftMint!.toBase58());
      console.log("   > Metadata:", episodeAccount.nftMetadata!.toBase58());
    } catch (error) {
      console.error("❌ Test failed:", error);
      throw error;
    }
  });
});
