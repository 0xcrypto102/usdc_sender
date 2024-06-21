import 'rpc-websockets/dist/lib/client';
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { WalletProgram } from "../target/types/wallet_program";
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from "@solana/spl-token";
import { SystemProgram, Keypair, PublicKey } from "@solana/web3.js";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { PythSolanaReceiver, InstructionWithEphemeralSigners } from "@pythnetwork/pyth-solana-receiver";
import { PriceServiceConnection } from "@pythnetwork/price-service-client";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
import { useAnchorWallet, useConnection } from "@solana/wallet-adapter-react";
import { Wallet } from "@coral-xyz/anchor";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("wallet_program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.WalletProgram as Program<WalletProgram>;
  let config, vaultUsdtAccount, vaultUsdcAccount, masterWallet: PublicKey;
  let config_bump, vaultUsdtAccount_bump,vaultUsdcAccount_bump, masterWallet_bump: Number;
  //  FjTpMEaveUt8DyaPhWGEbwYp2PoBkPMxKVkXiyFWVnHw
  let owner = Keypair.fromSecretKey(
    bs58.decode("ktoBaW3Tuc4VDW9txJFhnHC2HzybadDifVXVFmUdFm2T9MCWh4wrvyPNnLcSKNf5rVDusJybhqfjz3qoiRaof8f")
  );

  // F4aFMjFg7xqT2PSNWDhUuMPBKsjoRS8HGcunQhBr98SZ
  let user = Keypair.fromSecretKey(
    bs58.decode("2LU9Gir9pDVEsUWrRHLUUdPaVM642EmMGubgyZg2LNYk1uyD4LNRR5HshCENmfTUD3nPMeN7FCJKxEdu48YSEpta")
  );
  const usdc = new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
  const usdt = new PublicKey("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB");
  // const usdc = new PublicKey("5hyJ6h3ABjF7zEBhc32LWT5ZUCkNx4AZkdRzKC1MUHRb");
  // const usdt = new PublicKey("8NtheYSKWDkCgWoc8HScQFkcCTF1FiFEbbriosZLNmtE");
  let authority =  Keypair.generate();
  
  it("GET PDA", async() => {
    [config, config_bump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("CONFIG")
      ],
      program.programId
    );

    [vaultUsdtAccount, vaultUsdtAccount_bump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("TOKEN-VAULT"),
        usdt.toBuffer()
      ],
      program.programId
    );

    [vaultUsdcAccount, vaultUsdcAccount_bump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("TOKEN-VAULT"),
        usdc.toBuffer()
      ],
      program.programId
    );

    [masterWallet, masterWallet_bump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("MASTER-WALLET")
      ],
      program.programId
    );
  });
  it("Is initialized!", async () => {
    try {
      const tx = await program.rpc.initialize(
        config_bump,
        {
          accounts: {
            authority: owner.publicKey,
            config,
            vaultUsdtAccount,
            masterWallet,
            usdtMint: usdt,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId
          },
          signers: [owner]
        }
      );
      console.log("tx->", tx);
    
      // const tx1 = await program.rpc.initializeUsdc(
      //   {
      //     accounts: {
      //       authority: owner.publicKey,
      //       config,
      //       vaultUsdcAccount,
      //       usdcMint: usdc,
      //       tokenProgram: TOKEN_PROGRAM_ID,
      //       systemProgram: SystemProgram.programId
      //     },
      //     signers: [owner]
      //   }
      // );
      // console.log("tx1->", tx1);
    } catch (error) {
      console.log(error);
    }
    
  });

  // it("initialize user wallet", async() => {
  //   const userWalletIndex = 1;

  //   const [userWallet, _1] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       Buffer.from("USER-WALLET"),
  //       new anchor.BN(userWalletIndex).toBuffer("le", 4)
  //     ],
  //     program.programId
  //   );

  //   const [userUsdtSendAccount,_3] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       Buffer.from("TOKEN-VAULT"),
  //       new anchor.BN(userWalletIndex).toBuffer("le", 4),
  //       usdt.toBuffer()
  //     ],
  //     program.programId
  //   ); 

  //   const [userUsdcSendAccount,_2] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       Buffer.from("TOKEN-VAULT"),
  //       new anchor.BN(userWalletIndex).toBuffer("le", 4),
  //       usdc.toBuffer()
  //     ],
  //     program.programId
  //   ); 

  //   const tx = await program.rpc.initializeUserWallet(
  //     userWalletIndex, {
  //       accounts: {
  //         authority: user.publicKey,
  //         userWallet: userWallet,
  //         userUsdtSendAccount: userUsdtSendAccount,
  //         usdtMint: usdt,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         systemProgram: SystemProgram.programId
  //       },
  //       signers: [user]
  //     }
  //   );
  // });

  // it("forward usdt to admin", async () => {
  //   const provider = anchor.AnchorProvider.local();
  //   const user2 = provider.wallet.payer;
  //   const wallet = provider.wallet;

  //   const userWalletIndex = 2;
  //   const [userSendAccount, _] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       Buffer.from("TOKEN-VAULT"),
  //       new anchor.BN(userWalletIndex).toBuffer("le", 4),
  //       usdt.toBuffer()
  //     ],
  //     program.programId
  //   );

  //   const [userWallet, _1] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       Buffer.from("USER-WALLET"),
  //       new anchor.BN(userWalletIndex).toBuffer("le", 4)
  //     ],
  //     program.programId
  //   );

  //   const [userPool, _2] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       Buffer.from("USER-AUTHORITY"),
  //       new anchor.BN(userWalletIndex).toBuffer("le", 4),
  //     ],
  //     program.programId
  //   );

   
  //   await program.rpc.forwardUsdtToAdmin(
  //     userWalletIndex, {
  //       accounts: {
  //         config,
  //         userSendAccount,
  //         vaultReceiveAccount: vaultUsdtAccount,
  //         mint: usdt,
  //         userWallet,
  //         userPool,
  //         authority: user2.publicKey,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //         systemProgram: SystemProgram.programId
  //       },
  //       signers: [user2]
  //     }
  //   )

  // });

  // it("forward sol to admin", async() => {
  //   const provider = anchor.AnchorProvider.local();
  //   const user2 = provider.wallet.payer;
  //   const wallet = provider.wallet;

  //   const userWalletIndex = 2;
 
  //   const [userWallet, _1] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       Buffer.from("USER-WALLET"),
  //       new anchor.BN(userWalletIndex).toBuffer("le", 4)
  //     ],
  //     program.programId
  //   );

  //   const [masterWallet, _2] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       Buffer.from("MASTER-WALLET"),
  //     ],
  //     program.programId
  //   );

  //   const [userPool, _3] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       Buffer.from("USER-AUTHORITY"),
  //       new anchor.BN(userWalletIndex).toBuffer("le", 4)
  //     ],
  //     program.programId
  //   );

  //   const forwardAmount = 10000000;

  //   await program.rpc.forwardSolToAdmin(
  //     userWalletIndex,
  //     new anchor.BN(forwardAmount), {
  //       accounts: {
  //         config,
  //         userWallet,
  //         masterWallet,
  //         userPool,
  //         authority: user2.publicKey,
  //         systemProgram: SystemProgram.programId
  //       },
  //       signers: [user2]
  //     }
  //   )
  // });
 
  // it("withdraw usdt by owner", async() => {
  //   const provider = anchor.AnchorProvider.local();
  //   const user2 = provider.wallet.payer;
  //   const userWalletIndex = 2;

  //   const [userPool, _3] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       Buffer.from("USER-AUTHORITY"),
  //       new anchor.BN(userWalletIndex).toBuffer("le", 4)
  //     ],
  //     program.programId
  //   );


  //   const userReceiveAccount = await getAssociatedTokenAddress(
  //     usdt,
  //     owner.publicKey
  //   );

  //   const withdrawAmount = 10000000;

  //   try {
  //     const tx = await program.rpc.withdrawUsdt(
  //       userWalletIndex,
  //       new anchor.BN(withdrawAmount),
  //       {
  //         accounts: {
  //           config,
  //           mint: usdt,
  //           fromAta: vaultUsdtAccount,
  //           toAta: userReceiveAccount,
  //           userPool,
  //           user: owner.publicKey,
  //           receiver: owner.publicKey,
  //           tokenProgram: TOKEN_PROGRAM_ID,
  //           associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
  //           systemProgram: SystemProgram.programId
  //         },
  //         signers: [owner]
  //       }
  //     );
  //   } catch (error) {
  //     console.log(error);
  //   }
  // });

  // it("withdraw sol by owner", async() => {
  //   const userWalletIndex = 2;

  //   const [userPool, _3] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       Buffer.from("USER-AUTHORITY"),
  //       new anchor.BN(userWalletIndex).toBuffer("le", 4)
  //     ],
  //     program.programId
  //   );

  //   const [masterWallet, _2] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       Buffer.from("MASTER-WALLET"),
  //     ],
  //     program.programId
  //   );
    
  //   const withdrawAmount = 1000000;

  //   try {
  //     const tx = await program.rpc.withdrawSol(
  //       new anchor.BN(withdrawAmount),
  //       {
  //         accounts: {
  //           config,
  //           masterWallet,
  //           userPool,
  //           user: owner.publicKey,
  //           receiver: owner.publicKey,
  //           systemProgram: SystemProgram.programId
  //         },
  //         signers: [owner]
  //       }
  //     );
  //     console.log(tx);
  //   } catch (error) {
  //     console.log(error);
  //   }
  // });

});
