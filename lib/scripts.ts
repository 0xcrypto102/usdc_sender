import * as anchor from '@coral-xyz/anchor';
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import {
  PublicKey,
  SystemProgram,
  Transaction,
} from '@solana/web3.js';
import {
  SENDER_PROGRAM_ID,
  GLOBAL_AUTHORITY_SEED,
  GlobalPool,
  USER_POOL_SEED,
  UserPool,
  USDC_MINT,
} from './types';
import { getAssociatedTokenAccount } from './utils';

export const getGlobalState = async (
  program: anchor.Program,
): Promise<GlobalPool | null> => {
  const [globalAuthority, _] = await PublicKey.findProgramAddress(
    [Buffer.from(GLOBAL_AUTHORITY_SEED)],
    SENDER_PROGRAM_ID
  );
  try {
    let globalState = await program.account.globalPool.fetch(globalAuthority);
    return globalState as unknown as GlobalPool;
  } catch {
    return null;
  }
}

export const getUserPoolState = async (
  userAddress: PublicKey,
  program: anchor.Program,
): Promise<UserPool | null> => {
  if (!userAddress) return null;

  const [userPool] = await PublicKey.findProgramAddress(
    [Buffer.from(USER_POOL_SEED), userAddress.toBuffer()],
    SENDER_PROGRAM_ID,
  );
  console.log('User Pool PDA: ', userPool.toBase58());
  try {
    let poolState = await program.account.userPool.fetch(userPool);
    return poolState as unknown as UserPool;
  } catch {
    return null;
  }
}

export const createInitializeTx = async (
  admin: PublicKey,
  program: anchor.Program,
) => {
  const [globalPool] = await PublicKey.findProgramAddress(
    [Buffer.from(GLOBAL_AUTHORITY_SEED)],
    SENDER_PROGRAM_ID,
  );

  let tx = new Transaction();
  console.log('==>initializing program', globalPool.toBase58());

  tx.add(program.instruction.initialize(
    {
      accounts: {
        admin,
        globalPool,
        systemProgram: SystemProgram.programId,
      },
      instructions: [],
      signers: [],
    }));

  return tx;
}

export const createDepositTx = async (
  userAddress: PublicKey,
  amount: number,
  program: anchor.Program,
) => {
  let tx = new Transaction();

  const [globalPool] = await PublicKey.findProgramAddress(
    [Buffer.from(GLOBAL_AUTHORITY_SEED)],
    SENDER_PROGRAM_ID,
  );

  const [userPool] = await PublicKey.findProgramAddress(
    [Buffer.from(USER_POOL_SEED), userAddress.toBuffer()],
    SENDER_PROGRAM_ID,
  );

  const userTokenAccount = await getAssociatedTokenAccount(userAddress, USDC_MINT);
  const vaultTokenAccount = await getAssociatedTokenAccount(globalPool, USDC_MINT);

  console.log('==> Depositing', userAddress.toBase58(), 'Amount', amount);
  tx.add(program.instruction.deposit(new anchor.BN(amount), {
    accounts: {
      user: userAddress,
      globalPool,
      userPool,
      mint: USDC_MINT,
      fromAta: userTokenAccount,
      toAta: vaultTokenAccount,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    },
    instructions: [],
    signers: [],
  }));

  return tx;
}

export const createWithdrawTx = async (
  userAddress: PublicKey,
  amount: number,
  program: anchor.Program,
) => {
  let tx = new Transaction();

  const [globalPool] = await PublicKey.findProgramAddress(
    [Buffer.from(GLOBAL_AUTHORITY_SEED)],
    SENDER_PROGRAM_ID,
  );

  const [userPool] = await PublicKey.findProgramAddress(
    [Buffer.from(USER_POOL_SEED), userAddress.toBuffer()],
    SENDER_PROGRAM_ID,
  );

  const userTokenAccount = await getAssociatedTokenAccount(userAddress, USDC_MINT);
  const vaultTokenAccount = await getAssociatedTokenAccount(globalPool, USDC_MINT);

  console.log('==> Withdrawing', userAddress.toBase58(), 'Amount', amount);
  tx.add(program.instruction.withdraw(new anchor.BN(amount), {
    accounts: {
      user: userAddress,
      globalPool,
      userPool,
      mint: USDC_MINT,
      fromAta: vaultTokenAccount,
      toAta: userTokenAccount,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    },
    instructions: [],
    signers: [],
  }));

  return tx;
}

export const createWithdrawAdminTx = async (
  userAddress: PublicKey,
  admin: PublicKey,
  amount: number,
  program: anchor.Program
) => {
  let tx = new Transaction();

  const [globalPool] = await PublicKey.findProgramAddress(
    [Buffer.from(GLOBAL_AUTHORITY_SEED)],
    SENDER_PROGRAM_ID,
  );

  const [userPool] = await PublicKey.findProgramAddress(
    [Buffer.from(USER_POOL_SEED), userAddress.toBuffer()],
    SENDER_PROGRAM_ID,
  );

  const userTokenAccount = await getAssociatedTokenAccount(userAddress, USDC_MINT);
  const vaultTokenAccount = await getAssociatedTokenAccount(globalPool, USDC_MINT);

  console.log('==> Withdrawing as admin', userAddress.toBase58());
  tx.add(program.instruction.withdrawAdmin(new anchor.BN(amount), {
    accounts: {
      admin,
      user: userAddress,
      globalPool,
      userPool,
      mint: USDC_MINT,
      fromAta: vaultTokenAccount,
      toAta: userTokenAccount,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    },
    instructions: [],
    signers: [],
  }));

  return tx;
}
