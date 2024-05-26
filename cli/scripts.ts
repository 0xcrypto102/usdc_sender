import { Program, web3 } from '@coral-xyz/anchor';
import * as anchor from '@coral-xyz/anchor';
import {
  Keypair,
  PublicKey,
} from '@solana/web3.js';
import fs from 'fs';
import path from 'path';
import NodeWallet from '@coral-xyz/anchor/dist/cjs/nodewallet';

import { GlobalPool, GLOBAL_AUTHORITY_SEED, SENDER_PROGRAM_ID, UserPool } from '../lib/types';
import { IDL as SenderIDL } from "../target/types/usdc_sender";
import {
  createDepositTx,
  createInitializeTx,
  createWithdrawAdminTx,
  createWithdrawTx,
  getGlobalState,
  getUserPoolState
} from '../lib/scripts';

let solConnection = null;
let payer = null;
let program: Program = null;

// Address of the deployed program.
let programId = new anchor.web3.PublicKey(SENDER_PROGRAM_ID);

export const setClusterConfig = async (cluster: web3.Cluster) => {
  solConnection = new web3.Connection(web3.clusterApiUrl(cluster));
  const walletKeypair = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync(path.resolve(process.env.ANCHOR_WALLET), 'utf-8'))), { skipValidation: true });
  const wallet = new NodeWallet(walletKeypair);
  // anchor.setProvider(anchor.AnchorProvider.local(web3.clusterApiUrl(cluster)));
  // Configure the client to use the local cluster.
  anchor.setProvider(new anchor.AnchorProvider(solConnection, wallet, { skipPreflight: true, commitment: 'confirmed' }));
  payer = wallet;

  // Generate the program client from IDL.
  program = new anchor.Program(SenderIDL as anchor.Idl, programId);
  console.log('ProgramId: ', program.programId.toBase58());

  const [globalAuthority] = await PublicKey.findProgramAddress(
    [Buffer.from(GLOBAL_AUTHORITY_SEED)],
    program.programId
  );
  console.log('GlobalAuthority: ', globalAuthority.toBase58());
}

export const initProject = async (
) => {
  const tx = await createInitializeTx(payer.publicKey, program);
  const { blockhash } = await solConnection.getRecentBlockhash('confirmed');
  tx.feePayer = payer.publicKey;
  tx.recentBlockhash = blockhash;
  payer.signTransaction(tx);
  let txId = await solConnection.sendTransaction(tx, [(payer as NodeWallet).payer]);
  await solConnection.confirmTransaction(txId, "confirmed");
  console.log("txHash =", txId);
}

export const depositToken = async (
  sol: number,
) => {
  let userAddress = payer.publicKey;
  console.log(userAddress.toBase58(), sol);

  const tx = await createDepositTx(userAddress, sol, program);
  const { blockhash } = await solConnection.getRecentBlockhash('confirmed');
  tx.feePayer = payer.publicKey;
  tx.recentBlockhash = blockhash;
  payer.signTransaction(tx);
  let txId = await solConnection.sendTransaction(tx, [(payer as NodeWallet).payer]);
  await solConnection.confirmTransaction(txId, "confirmed");
  console.log("Your transaction signature", txId);
}

export const withdrawToken = async (
  sol: number,
) => {
  let userAddress = payer.publicKey;
  console.log(userAddress.toBase58(), sol);

  const tx = await createWithdrawTx(userAddress, sol, program);
  const { blockhash } = await solConnection.getRecentBlockhash('confirmed');
  tx.feePayer = payer.publicKey;
  tx.recentBlockhash = blockhash;
  payer.signTransaction(tx);
  let txId = await solConnection.sendTransaction(tx, [(payer as NodeWallet).payer]);
  await solConnection.confirmTransaction(txId, "confirmed");
  console.log("Your transaction signature", txId);
}

export const withdrawTokenAdmin = async (
  sol: number,
  userAddress: PublicKey,
) => {
  let admin = payer.publicKey;
  console.log(userAddress.toBase58(), sol);

  const tx = await createWithdrawAdminTx(userAddress, admin, sol, program);
  const { blockhash } = await solConnection.getRecentBlockhash('confirmed');
  tx.feePayer = payer.publicKey;
  tx.recentBlockhash = blockhash;
  payer.signTransaction(tx);
  let txId = await solConnection.sendTransaction(tx, [(payer as NodeWallet).payer]);
  await solConnection.confirmTransaction(txId, "confirmed");
  console.log("Your transaction signature", txId);
}

export const getUserPoolInfo = async (
  userAddress: PublicKey,
) => {
  const userPool: UserPool = await getUserPoolState(userAddress, program);
  return {
    creditAmount: userPool.creditAmount.toNumber(),
  };
}

export const getGlobalInfo = async () => {
  const globalPool: GlobalPool = await getGlobalState(program);
  const result = {
    admin: globalPool.admin.toBase58(),
  };

  return result;
}