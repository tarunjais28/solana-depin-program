import * as anchor from "@coral-xyz/anchor";
import { getProvider, dpitProgramInterface } from "./solanaService";
import { DepinProgram } from "../target/types/depin_program";
import { Program } from "@coral-xyz/anchor";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  getAccount,
  getAssociatedTokenAddress,
} from "@solana/spl-token";
import {
  UserAddress,
  dpitMintAccount,
  ESCROW,
  GLOBAL,
  LOCK,
  TOKEN_A,
  TOKEN_B,
  TOKEN_C,
  tokenAMintAccount,
  tokenBMintAccount,
  tokenCMintAccount,
} from "./constant";
import * as fs from "fs";
import { PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { BN } from "bn.js";

const { provider }: any = getProvider();
if (!provider) throw new Error("Provider not available");
let program: any = new anchor.Program(
  dpitProgramInterface,
  provider
) as Program<DepinProgram>;

const [pdaGlobal] = anchor.web3.PublicKey.findProgramAddressSync(
  [GLOBAL],
  program.programId
);

const [pdaEscrow] = anchor.web3.PublicKey.findProgramAddressSync(
  [ESCROW],
  program.programId
);

const [pdaEscrowA] = anchor.web3.PublicKey.findProgramAddressSync(
  [ESCROW, TOKEN_A],
  program.programId
);

const [pdaEscrowB] = anchor.web3.PublicKey.findProgramAddressSync(
  [ESCROW, TOKEN_B],
  program.programId
);

const [pdaEscrowC] = anchor.web3.PublicKey.findProgramAddressSync(
  [ESCROW, TOKEN_C],
  program.programId
);

const init = async () => {
  await program.methods
    .initialize()
    .accounts({
      globalState: pdaGlobal,
      payer: UserAddress,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .rpc();
};

const initEscrows1 = async () => {
  await program.methods
    .initializeEscrows1()
    .accounts({
      globalState: pdaGlobal,
      escrowAccount: pdaEscrow,
      escrowAccountA: pdaEscrowA,
      mintAccount: dpitMintAccount,
      tokenA: tokenAMintAccount,
      payer: UserAddress,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .rpc();
};

const initEscrows2 = async () => {
  await program.methods
    .initializeEscrows2()
    .accounts({
      globalState: pdaGlobal,
      escrowAccountB: pdaEscrowB,
      escrowAccountC: pdaEscrowC,
      tokenB: tokenBMintAccount,
      tokenC: tokenCMintAccount,
      payer: UserAddress,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .rpc();
};

const getTokenSupplies = async () => {
  var supply = (await provider.connection.getTokenSupply(tokenAMintAccount))
    .value.uiAmount;
  console.log("tokenA supply: ", supply);
  supply = (await provider.connection.getTokenSupply(tokenBMintAccount)).value
    .uiAmount;
  console.log("tokenB supply: ", supply);
  supply = (await provider.connection.getTokenSupply(tokenCMintAccount)).value
    .uiAmount;
  console.log("tokenC supply: ", supply);
  supply = (await provider.connection.getTokenSupply(dpitMintAccount)).value
    .uiAmount;
  console.log("dpit supply: ", supply);
};

const getTokenBalances = async (userAddress: PublicKey) => {
  var userATA = await getAssociatedTokenAddress(tokenAMintAccount, userAddress);
  var userBalance = (await getAccount(provider.connection, userATA)).amount;
  console.log("tokenA balance ", userBalance);

  userATA = await getAssociatedTokenAddress(tokenBMintAccount, userAddress);
  var userBalance = (await getAccount(provider.connection, userATA)).amount;
  console.log("tokenB balance ", userBalance);

  userATA = await getAssociatedTokenAddress(tokenCMintAccount, userAddress);
  var userBalance = (await getAccount(provider.connection, userATA)).amount;
  console.log("tokenC balance ", userBalance);

  userATA = await getAssociatedTokenAddress(dpitMintAccount, userAddress);
  var userBalance = (await getAccount(provider.connection, userATA)).amount;
  console.log("dpit token balance ", userBalance);
};

const getProgramData = async () => {
  let globalAccount = await program.account.globalState.fetch(pdaGlobal);
  console.log("Global data: ", globalAccount);

  let stakedAmount = await provider.connection.getTokenAccountBalance(
    pdaEscrow
  );
  console.log("\n\nTotal staked amount: ", stakedAmount.value.uiAmount);

  let tokenA = await provider.connection.getTokenAccountBalance(pdaEscrowA);
  console.log("Escrow tokenA balance: ", tokenA.value.uiAmount);

  let tokenB = await provider.connection.getTokenAccountBalance(pdaEscrowB);
  console.log("Escrow tokenB balance: ", tokenB.value.uiAmount);

  let tokenC = await provider.connection.getTokenAccountBalance(pdaEscrowC);
  console.log("Escrow tokenC balance: ", tokenC.value.uiAmount);
};

const getStakedData = async (user: PublicKey) => {
  let [pdaStakeAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [LOCK, user.toBytes()],
    program.programId
  );

  let stakedAccount = await program.account.stakeState.fetch(pdaStakeAccount);
  console.log("Staked amount:", Number(stakedAccount.stakedAmount));
  console.log("Staked at:", Number(stakedAccount.stakedAt));
  console.log("Staked amount:", Number(stakedAccount.rewards));
  console.log("Staked amount:", Number(stakedAccount.penality));
};

const mint = async (user: PublicKey) => {
  let tokenA = new BN(10 * LAMPORTS_PER_SOL);
  let tokenB = new BN(10 * LAMPORTS_PER_SOL);
  let tokenC = new BN(10 * LAMPORTS_PER_SOL);

  let userTokenAATA = await getAssociatedTokenAddress(tokenAMintAccount, user);

  let userTokenBATA = await getAssociatedTokenAddress(tokenBMintAccount, user);

  let userTokenCATA = await getAssociatedTokenAddress(tokenCMintAccount, user);

  let userDpitATA = await getAssociatedTokenAddress(dpitMintAccount, user);

  await program.methods
    .mint(tokenA, tokenB, tokenC)
    .accounts({
      globalState: pdaGlobal,
      mintAccount: dpitMintAccount,
      escrowAccountA: pdaEscrowA,
      escrowAccountB: pdaEscrowB,
      escrowAccountC: pdaEscrowC,
      tokenAAta: userTokenAATA,
      tokenBAta: userTokenBATA,
      tokenCAta: userTokenCATA,
      toAccount: userDpitATA,
      authority: user,
      mintAuthority: UserAddress,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .rpc();
};

const burn = async (user: PublicKey) => {
  let amount = new BN(2 * LAMPORTS_PER_SOL);

  let userTokenAATA = await getAssociatedTokenAddress(tokenAMintAccount, user);

  let userTokenBATA = await getAssociatedTokenAddress(tokenBMintAccount, user);

  let userTokenCATA = await getAssociatedTokenAddress(tokenCMintAccount, user);

  let userDpitATA = await getAssociatedTokenAddress(dpitMintAccount, user);

  await program.methods
    .burn(amount)
    .accounts({
      globalState: pdaGlobal,
      mintAccount: dpitMintAccount,
      escrowAccountA: pdaEscrowA,
      escrowAccountB: pdaEscrowB,
      escrowAccountC: pdaEscrowC,
      tokenAAta: userTokenAATA,
      tokenBAta: userTokenBATA,
      tokenCAta: userTokenCATA,
      fromAccount: userDpitATA,
      authority: user,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .rpc();
};

const stake = async (user: PublicKey) => {
  let [pdaStakeAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [LOCK, user.toBytes()],
    program.programId
  );

  let amount = new BN(2 * LAMPORTS_PER_SOL);

  let userATA = await getAssociatedTokenAddress(dpitMintAccount, user);

  await program.methods
    .stake(amount)
    .accounts({
      globalState: pdaGlobal,
      stakeState: pdaStakeAccount,
      escrowAccount: pdaEscrow,
      userVault: userATA,
      vaultAuthority: user,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .rpc();
};

const unstake = async (user: PublicKey) => {
  let [pdaStakeAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [LOCK, user.toBytes()],
    program.programId
  );

  let userATA = await getAssociatedTokenAddress(dpitMintAccount, user);

  await program.methods
    .unstake()
    .accounts({
      globalState: pdaGlobal,
      stakeState: pdaStakeAccount,
      escrowAccount: pdaEscrow,
      userVault: userATA,
      vaultAuthority: user,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .rpc();
};

export {
  init,
  initEscrows1,
  initEscrows2,
  getTokenSupplies,
  getTokenBalances,
  getProgramData,
  mint,
  burn,
  stake,
  getStakedData,
  unstake,
};
