import { PublicKey } from "@solana/web3.js";

export const DPIT_PROGRAM_ID: string =
  "3W7pnY6U3Aa7ERYf7KTwMmfNmyFRNTNivR4Ya6nKScXh";

export const UserAddress: PublicKey = new PublicKey(
  "Ex7y8SZSpd1BMDa5mMRe16CvevsH564EzmECLfxiNbV3"
);

export const GLOBAL = Buffer.from("global");
export const ESCROW = Buffer.from("escrow");
export const TOKEN_A = Buffer.from("TokenA");
export const TOKEN_B = Buffer.from("TokenB");
export const TOKEN_C = Buffer.from("TokenC");
export const LOCK = Buffer.from("lock");
export const tokenAMintAccount = new PublicKey(
  "9yj32Bk5Jv2jSG2fMw9P8TEvMNUcwbf5KZR5RWZJMTEX"
);
export const tokenBMintAccount = new PublicKey(
  "GRTh3GajaetVDCZeTEPn7SC45nqUcYCvh4gNjvnYhQoB"
);
export const tokenCMintAccount = new PublicKey(
  "8aoS8p9xHD3cLKgAjEngCSCcjk9wf2dXtarYjThQCTdP"
);
export const dpitMintAccount = new PublicKey(
  "CwmJZE2ByVbxEeB6UDcn2dQDpv2rkNUAhRy5PtoCrd5F"
);
