import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DepinProgram } from "../target/types/depin_program";
import { BN } from "bn.js";
import { assert } from "chai";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import {
  getOrCreateAssociatedTokenAccount,
  getAssociatedTokenAddress,
  createMint,
  mintToChecked,
  getAccount,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

// Create test keypairs
const admin = anchor.web3.Keypair.generate();
const payer = anchor.web3.Keypair.generate();
const user1 = anchor.web3.Keypair.generate();
const user2 = anchor.web3.Keypair.generate();
const vault = anchor.web3.Keypair.generate();
const mintAuthority = anchor.web3.Keypair.generate();

// Constant seeds
const GLOBAL = Buffer.from("global");
const ESCROW = Buffer.from("escrow");
const TOKEN_A = Buffer.from("TokenA");
const TOKEN_B = Buffer.from("TokenB");
const TOKEN_C = Buffer.from("TokenC");

// Constant values
const DECIMALS = 9;

describe("depin-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.DepinProgram as Program<DepinProgram>;

  // Declare PDAs
  let pdaGlobal,
    pdaEscrowA,
    pdaEscrowB,
    pdaEscrowC = null;

  // Declare nft mints
  var tokenAMintAccount,
    tokenBMintAccount,
    tokenCMintAccount,
    dpitMintAccount = null;

  const confirmTransaction = async (tx) => {
    const latestBlockHash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: tx,
    });
  };

  it("Initialize test accounts", async () => {
    // Airdrop sol to the test users
    let adminSol = await provider.connection.requestAirdrop(
      admin.publicKey,
      anchor.web3.LAMPORTS_PER_SOL
    );
    await confirmTransaction(adminSol);

    let payerSol = await provider.connection.requestAirdrop(
      payer.publicKey,
      anchor.web3.LAMPORTS_PER_SOL
    );
    await confirmTransaction(payerSol);

    let user1Sol = await provider.connection.requestAirdrop(
      user1.publicKey,
      anchor.web3.LAMPORTS_PER_SOL
    );
    await confirmTransaction(user1Sol);

    let user2Sol = await provider.connection.requestAirdrop(
      user2.publicKey,
      anchor.web3.LAMPORTS_PER_SOL
    );
    await confirmTransaction(user2Sol);

    let mintAuthoritySol = await provider.connection.requestAirdrop(
      mintAuthority.publicKey,
      anchor.web3.LAMPORTS_PER_SOL
    );
    await confirmTransaction(mintAuthoritySol);

    // Create mint token with decimals
    tokenAMintAccount = await createMint(
      provider.connection,
      payer,
      mintAuthority.publicKey,
      null,
      DECIMALS
    );

    tokenBMintAccount = await createMint(
      provider.connection,
      payer,
      mintAuthority.publicKey,
      null,
      DECIMALS
    );

    tokenCMintAccount = await createMint(
      provider.connection,
      payer,
      mintAuthority.publicKey,
      null,
      DECIMALS
    );

    dpitMintAccount = await createMint(
      provider.connection,
      payer,
      mintAuthority.publicKey,
      null,
      DECIMALS
    );

    // Create Associated Token Address
    let user1TokenAATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      tokenAMintAccount,
      user1.publicKey
    );

    let user1TokenBATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      tokenBMintAccount,
      user1.publicKey
    );

    let user1TokenCATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      tokenCMintAccount,
      user1.publicKey
    );

    // Mint tokens to user
    await mintToChecked(
      provider.connection,
      payer,
      tokenAMintAccount,
      user1TokenAATA.address,
      mintAuthority,
      100000 * LAMPORTS_PER_SOL,
      DECIMALS
    );

    await mintToChecked(
      provider.connection,
      payer,
      tokenBMintAccount,
      user1TokenBATA.address,
      mintAuthority,
      100000 * LAMPORTS_PER_SOL,
      DECIMALS
    );

    await mintToChecked(
      provider.connection,
      payer,
      tokenCMintAccount,
      user1TokenCATA.address,
      mintAuthority,
      100000 * LAMPORTS_PER_SOL,
      DECIMALS
    );
  });

  it("Initialize global account", async () => {
    [pdaGlobal] = anchor.web3.PublicKey.findProgramAddressSync(
      [GLOBAL],
      program.programId
    );

    [pdaEscrowA] = anchor.web3.PublicKey.findProgramAddressSync(
      [ESCROW, TOKEN_A],
      program.programId
    );

    [pdaEscrowB] = anchor.web3.PublicKey.findProgramAddressSync(
      [ESCROW, TOKEN_B],
      program.programId
    );

    [pdaEscrowC] = anchor.web3.PublicKey.findProgramAddressSync(
      [ESCROW, TOKEN_C],
      program.programId
    );

    // Test initialize instruction
    let init = await program.methods
      .initialize()
      .accounts({
        globalState: pdaGlobal,
        escrowAccountA: pdaEscrowA,
        escrowAccountB: pdaEscrowB,
        escrowAccountC: pdaEscrowC,
        tokenA: tokenAMintAccount,
        tokenB: tokenBMintAccount,
        tokenC: tokenCMintAccount,
        mintAccount: dpitMintAccount,
        payer: admin.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([admin])
      .rpc();

    await confirmTransaction(init);

    let globalState = await program.account.globalState.fetch(pdaGlobal);
    assert.equal(globalState.tokenA.toString(), tokenAMintAccount.toString());
    assert.equal(globalState.tokenB.toString(), tokenBMintAccount.toString());
    assert.equal(globalState.tokenC.toString(), tokenCMintAccount.toString());
    assert.equal(
      globalState.mintAccount.toString(),
      dpitMintAccount.toString()
    );
  });

  it("Test Mint Tokens", async () => {
    let tokenA = new BN(LAMPORTS_PER_SOL);
    let tokenB = new BN(LAMPORTS_PER_SOL);
    let tokenC = new BN(LAMPORTS_PER_SOL);

    let user1TokenAATA = await getAssociatedTokenAddress(
      tokenAMintAccount,
      user1.publicKey
    );

    let user1TokenABalanceBefore = Number(
      (await getAccount(provider.connection, user1TokenAATA)).amount
    );

    let user1TokenBATA = await getAssociatedTokenAddress(
      tokenBMintAccount,
      user1.publicKey
    );

    let user1TokenBBalanceBefore = Number(
      (await getAccount(provider.connection, user1TokenBATA)).amount
    );

    let user1TokenCATA = await getAssociatedTokenAddress(
      tokenCMintAccount,
      user1.publicKey
    );

    let user1TokenCBalanceBefore = Number(
      (await getAccount(provider.connection, user1TokenCATA)).amount
    );

    let user1DpitATA = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      payer,
      dpitMintAccount,
      user1.publicKey
    );

    let user1DpitBalanceBefore = Number(
      (await getAccount(provider.connection, user1DpitATA.address)).amount
    );

    // Test mint instruction
    let mint = await program.methods
      .mint(tokenA, tokenB, tokenC)
      .accounts({
        globalState: pdaGlobal,
        mintAccount: dpitMintAccount,
        escrowAccountA: pdaEscrowA,
        escrowAccountB: pdaEscrowB,
        escrowAccountC: pdaEscrowC,
        tokenAAta: user1TokenAATA,
        tokenBAta: user1TokenBATA,
        tokenCAta: user1TokenCATA,
        toAccount: user1DpitATA.address,
        authority: user1.publicKey,
        mintAuthority: mintAuthority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([user1, mintAuthority])
      .rpc();

    await confirmTransaction(mint);

    // Check token balances
    let user1TokenABalanceAfter = Number(
      (await getAccount(provider.connection, user1TokenAATA)).amount
    );

    let user1TokenBBalanceAfter = Number(
      (await getAccount(provider.connection, user1TokenBATA)).amount
    );

    let user1TokenCBalanceAfter = Number(
      (await getAccount(provider.connection, user1TokenCATA)).amount
    );

    let user1DpitBalanceAfter = Number(
      (await getAccount(provider.connection, user1DpitATA.address)).amount
    );

    assert.equal(
      user1TokenABalanceAfter,
      user1TokenABalanceBefore - Number(tokenA)
    );
    assert.equal(
      user1TokenBBalanceAfter,
      user1TokenBBalanceBefore - Number(tokenB)
    );
    assert.equal(
      user1TokenCBalanceAfter,
      user1TokenCBalanceBefore - Number(tokenC)
    );
    assert.equal(
      user1DpitBalanceAfter,
      user1DpitBalanceBefore + Number(LAMPORTS_PER_SOL)
    );

    // Check Escrow Account balances
    let escrowAccountA = await provider.connection.getTokenAccountBalance(
      pdaEscrowA
    );
    assert.equal(Number(escrowAccountA.value.amount), Number(tokenA));

    let escrowAccountB = await provider.connection.getTokenAccountBalance(
      pdaEscrowB
    );
    assert.equal(Number(escrowAccountB.value.amount), Number(tokenB));

    let escrowAccountC = await provider.connection.getTokenAccountBalance(
      pdaEscrowC
    );
    assert.equal(Number(escrowAccountC.value.amount), Number(tokenC));
  });

  it("Test Burn Tokens", async () => {
    let amount = new BN(LAMPORTS_PER_SOL);
    let tokenA = new BN((40 * Number(amount)) / 100);
    let tokenB = new BN((30 * Number(amount)) / 100);
    let tokenC = new BN((30 * Number(amount)) / 100);

    let user1TokenAATA = await getAssociatedTokenAddress(
      tokenAMintAccount,
      user1.publicKey
    );

    let user1TokenABalanceBefore = Number(
      (await getAccount(provider.connection, user1TokenAATA)).amount
    );

    let user1TokenBATA = await getAssociatedTokenAddress(
      tokenBMintAccount,
      user1.publicKey
    );

    let user1TokenBBalanceBefore = Number(
      (await getAccount(provider.connection, user1TokenBATA)).amount
    );

    let user1TokenCATA = await getAssociatedTokenAddress(
      tokenCMintAccount,
      user1.publicKey
    );

    let user1TokenCBalanceBefore = Number(
      (await getAccount(provider.connection, user1TokenCATA)).amount
    );

    let user1DpitATA = await getAssociatedTokenAddress(
      dpitMintAccount,
      user1.publicKey
    );

    let user1DpitBalanceBefore = Number(
      (await getAccount(provider.connection, user1DpitATA.address)).amount
    );

    let escrowAccountABefore = Number(
      (await provider.connection.getTokenAccountBalance(pdaEscrowA)).value
        .amount
    );

    let escrowAccountBBefore = Number(
      (await provider.connection.getTokenAccountBalance(pdaEscrowB)).value
        .amount
    );

    let escrowAccountCBefore = Number(
      (await provider.connection.getTokenAccountBalance(pdaEscrowC)).value
        .amount
    );

    // Test burn instruction
    let burn = await program.methods
      .burn(amount)
      .accounts({
        globalState: pdaGlobal,
        mintAccount: dpitMintAccount,
        escrowAccountA: pdaEscrowA,
        escrowAccountB: pdaEscrowB,
        escrowAccountC: pdaEscrowC,
        tokenAAta: user1TokenAATA,
        tokenBAta: user1TokenBATA,
        tokenCAta: user1TokenCATA,
        fromAccount: user1DpitATA.address,
        authority: user1.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([user1])
      .rpc();

    await confirmTransaction(burn);

    // Check token balances
    let user1TokenABalanceAfter = Number(
      (await getAccount(provider.connection, user1TokenAATA)).amount
    );

    let user1TokenBBalanceAfter = Number(
      (await getAccount(provider.connection, user1TokenBATA)).amount
    );

    let user1TokenCBalanceAfter = Number(
      (await getAccount(provider.connection, user1TokenCATA)).amount
    );

    let user1DpitBalanceAfter = Number(
      (await getAccount(provider.connection, user1DpitATA.address)).amount
    );

    assert.equal(
      user1TokenABalanceAfter,
      user1TokenABalanceBefore + Number(tokenA)
    );
    assert.equal(
      user1TokenBBalanceAfter,
      user1TokenBBalanceBefore + Number(tokenB)
    );
    assert.equal(
      user1TokenCBalanceAfter,
      user1TokenCBalanceBefore + Number(tokenC)
    );
    assert.equal(
      user1DpitBalanceAfter,
      user1DpitBalanceBefore - Number(LAMPORTS_PER_SOL)
    );

    // Check Escrow Account balances
    let escrowAccountAAfter = Number(
      (await provider.connection.getTokenAccountBalance(pdaEscrowA)).value
        .amount
    );

    let escrowAccountBAfter = Number(
      (await provider.connection.getTokenAccountBalance(pdaEscrowB)).value
        .amount
    );

    let escrowAccountCAfter = Number(
      (await provider.connection.getTokenAccountBalance(pdaEscrowC)).value
        .amount
    );

    assert.equal(escrowAccountAAfter, escrowAccountABefore - Number(tokenA));
    assert.equal(escrowAccountBAfter, escrowAccountBBefore - Number(tokenB));
    assert.equal(escrowAccountCAfter, escrowAccountCBefore - Number(tokenC));
  });
});
