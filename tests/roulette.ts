import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Roulette } from "../target/types/roulette";
import * as token from '@solana/spl-token';
import { LAMPORTS_PER_SOL, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";


describe("roulette", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Roulette as Program<Roulette>;

  it("Is initialized!", async () => {
    // Add your test here.
    const payer = provider.wallet.payer;
    const decimals = 6;

    const lamports = await token.getMinimumBalanceForRentExemptMint(provider.connection);
    const accountKeypair = anchor.web3.Keypair.generate();
    const programId = token.TOKEN_PROGRAM_ID;

    const player1Keypair = anchor.web3.Keypair.generate();
    const player2Keypair = anchor.web3.Keypair.generate();
    const bet1 = new anchor.BN("100");
    const bet2 = new anchor.BN("100");
    const tokenMint = anchor.web3.Keypair.generate();
    const mint = await token.createMint(provider.connection, payer, tokenMint.publicKey, null, decimals);
    const player1 = await token.getOrCreateAssociatedTokenAccount(provider.connection, payer, mint, player1Keypair.publicKey);
    const player2 = await token.getOrCreateAssociatedTokenAccount(provider.connection, payer, mint, player2Keypair.publicKey);
    await token.mintTo(provider.connection, payer, mint, player1.address, tokenMint, 1000);
    await token.mintTo(provider.connection, payer, mint, player2.address, tokenMint, 1000);
    console.log("tokenMintAddress:", tokenMint.publicKey, "tokenAddress", mint);
    const [gamepda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("example"), payer.publicKey.toBuffer()],
      program.programId
    )
    console.log("PDA", gamepda);
    const accounts = [
      {pubkey: player1.address, isSigner: false, isWritable: true},
      {pubkey: player2.address, isSigner: false, isWritable: true}
    ];
    const tx = await program.methods
    .initialize(new anchor.BN(123123), new anchor.BN(6), new anchor.BN(90),[player1Keypair.publicKey, player2Keypair.publicKey], [bet1, bet2], true, new anchor.BN(1))
    .rpc();
    // const gameAcc = await program.account.game.fetch(gamepda);
    // console.log(gameAcc);
    console.log("Your transaction signature", tx);
  });
});
