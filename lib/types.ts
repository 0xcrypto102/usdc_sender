import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from '@solana/web3.js';

export const GLOBAL_AUTHORITY_SEED = "global-authority";
export const USER_POOL_SEED = "user-authority";

export const SENDER_PROGRAM_ID = new PublicKey("7pB3zj3voieg4msvT7oTxc6xRDDPDjiCy3J4Sr4ogPaC");

export const USDC_MINT = new PublicKey("9nUq4Fka3feiCT96c9B4njbsVTSH8oALS65LL3fT937J");
export const USDC_DECIMAL = 1_000_000;


export interface GlobalPool {
  // 8 + 32
  admin: PublicKey,      // 32
}

export interface UserPool {
  // 8 + 8
  creditAmount: anchor.BN,        // 8
}