import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { AnchorProvider } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import depinProgramIDL from "../target/idl/depin_program.json";
import { DPIT_PROGRAM_ID } from "./constant";
import * as fs from "fs";

export const dpitProgramID = new PublicKey(DPIT_PROGRAM_ID);

export const dpitProgramInterface = JSON.parse(JSON.stringify(depinProgramIDL));

const solanaNetwork = web3.clusterApiUrl("devnet");
const opts: any = {
  preflightCommitment: "processed",
};

export const getProvider = (): {
  provider: AnchorProvider;
  connection: web3.Connection;
} => {
  try {
    //Creating a provider, the provider is authenication connection to solana
    const connection = new web3.Connection(
      solanaNetwork,
      opts.preflightCommitment
    );

    /// With config file
    const rawPayerKeypair = JSON.parse(
      fs.readFileSync("/Users/tarunjaiswal/.config/solana/id.json", "utf-8")
    );
    const privateKeyWallet = anchor.web3.Keypair.fromSecretKey(
      Buffer.from(rawPayerKeypair)
    );

    const provider: any = new AnchorProvider(
      connection,
      new NodeWallet(privateKeyWallet),
      opts
    );
    return { provider, connection };
  } catch (error) {
    console.log("provider:solana", error);
    throw error;
  }
};
