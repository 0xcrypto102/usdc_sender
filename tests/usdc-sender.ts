import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { UsdcSender } from "../target/types/usdc_sender";
import {
  PublicKey,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
  Keypair,
} from "@solana/web3.js";

import {
  ExtensionType,
  TOKEN_2022_PROGRAM_ID,
  getMintLen,
  createInitializeMintInstruction,
  createInitializeTransferHookInstruction,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  createMintToInstruction,
  getAssociatedTokenAddressSync,
  createApproveInstruction,
  createSyncNativeInstruction,
  NATIVE_MINT,
  TOKEN_PROGRAM_ID,
  getAccount,
  getOrCreateAssociatedTokenAccount,
  createTransferCheckedWithTransferHookInstruction,
  getMint,
  getTransferHook,
  getExtraAccountMetaAddress,
  getExtraAccountMetas,
  getAssociatedTokenAddress,
} from "@solana/spl-token";
import bs58 from 'bs58';
import { getGlobalState } from "../lib/scripts";
const admin = Keypair.fromSecretKey(
  bs58.decode("5ErjQUhaofK14cgTzAvNZCjhdAHfot9Kbfr5hYPGnbtk1CBEf6uwPb4ew9hLR46z21Uh2KRCXywBmbvC2ao9CYNs")
); 
const user = Keypair.fromSecretKey(
  bs58.decode("2LU9Gir9pDVEsUWrRHLUUdPaVM642EmMGubgyZg2LNYk1uyD4LNRR5HshCENmfTUD3nPMeN7FCJKxEdu48YSEpta")
);

describe("usdc-sender", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  const program = anchor.workspace.UsdcSender as Program<UsdcSender>;
  const connection = new anchor.web3.Connection(anchor.web3.clusterApiUrl("devnet"), "confirmed");

  // it("airdrop", async () => {
  //   try {
  //     console.log(`-- Airdropping 2 SOL --`)
  //     const fromAirDropSignature = await connection.requestAirdrop(
  //       userKp.publicKey,
  //       2 * anchor.web3.LAMPORTS_PER_SOL
  //     );
  //     await connection.confirmTransaction(fromAirDropSignature, "confirmed");
  //   } catch (err) {
  //     console.log(err);
  //   } finally {
  //     const pubKey = userKp.publicKey.toBase58()
  //     const balance = await connection.getBalance(userKp.publicKey) / 10 ** 9;
  //     console.table([{ Key: "User", Value: pubKey }, { Key: "Balance", Value: balance }])
  //   }
  // })

  it('Is initialized!', async () => {
    const [globalPool,_1] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("global-authority")],
      program.programId
    );

    const tx = await program.rpc.initialize({
      accounts: {
        globalPool,
        admin: admin.publicKey,
        systemProgram: SystemProgram.programId
      },
      signers: [admin]
    });
    console.log("transaction simulation result ==>",tx)
  });
  it('Deposit', async () => {
    const amount = 100000000;

    const [globalPool,_1] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("global-authority")],
      program.programId
    );

    const [userPool,_2] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user-authority"), user.publicKey.toBuffer()],
      program.programId
    );

    const mint = new PublicKey("8NtheYSKWDkCgWoc8HScQFkcCTF1FiFEbbriosZLNmtE");

    const fromAta = await getAssociatedTokenAddress(
      mint,
      user.publicKey
    );

    const [toAta, _3] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("token-vault")],
      program.programId
    );

    const tx = await program.rpc.deposit(
      new anchor.BN(amount), {
        accounts: {
          globalPool,
          userPool,
          mint,
          fromAta,
          toAta,
          user: user.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId
        },
        signers: [user]
      }
    );


    // const tx = await program.rpc.deposit(
    //   new anchor.BN(amount), 
    //   {
    //     accounts: {
    //       globalPool,
    //       userPool,
    //       mint,
    //       fromAta,
    //       toAta,
    //       user: user.publicKey,
    //       tokenProgram: TOKEN_PROGRAM_ID,
    //       systemProgram: SystemProgram.programId,
    //     },
    //     signers: [user]
    //   }
    // );
    console.log("depost tx=>", tx);
  });
  it('Withdraw', async () => {
    const amount = 100000000;

    const [globalPool,_1] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("global-authority")],
      program.programId
    );

    const [userPool,_2] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user-authority"), user.publicKey.toBuffer()],
      program.programId
    );

    const mint = new PublicKey("8NtheYSKWDkCgWoc8HScQFkcCTF1FiFEbbriosZLNmtE");

    const toAta = await getAssociatedTokenAddress(
      mint,
      user.publicKey
    );

    const [fromAta, _3] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("token-vault")],
      program.programId
    );

    const tx = await program.rpc.withdraw(
      new anchor.BN(amount), {
        accounts: {
          globalPool,
          userPool,
          mint,
          fromAta,
          toAta,
          user: user.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId
        },
        signers: [user]
      }
    );
    console.log("tx =>", tx);
  })
});
