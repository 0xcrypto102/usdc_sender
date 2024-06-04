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
  //  2TYV72CtgYXCduE5hheeoux728zHcSyxPQAbhiCNf2Yy
  let owner = Keypair.fromSecretKey(
    Uint8Array.from([113, 63, 93, 213, 68, 178, 22, 189, 136, 49, 33, 174, 196, 213, 238, 242, 164, 106, 9, 180, 15, 3, 238, 80, 159, 127, 118, 18, 231, 206, 240, 93, 21, 168, 99, 61, 85, 242, 222, 187, 12, 44, 91, 158, 122, 83, 103, 113, 125, 136, 28, 83, 108, 248, 78, 219, 197, 250, 38, 187, 70, 109, 130, 194])
  );

  // F4aFMjFg7xqT2PSNWDhUuMPBKsjoRS8HGcunQhBr98SZ
  let user = Keypair.fromSecretKey(
    bs58.decode("2LU9Gir9pDVEsUWrRHLUUdPaVM642EmMGubgyZg2LNYk1uyD4LNRR5HshCENmfTUD3nPMeN7FCJKxEdu48YSEpta")
  );
  const usdc = new PublicKey("5hyJ6h3ABjF7zEBhc32LWT5ZUCkNx4AZkdRzKC1MUHRb");
  const usdt = new PublicKey("8NtheYSKWDkCgWoc8HScQFkcCTF1FiFEbbriosZLNmtE");
  const SOL_PRICE_FEED_ID = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
  const USDT_PRICE_FEED_ID = "0x2b89b9dc8fdf9f34709a5b106b472f0f39bb6ca9ce04b0fd7f2e971688e2e53b";
  const USDC_PRICE_FEED_ID = "0xeaa020c61cc479712813461ce153894a96a6c00b21ed0cfc2798d1f9a9e9c94a";


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
    
      const tx1 = await program.rpc.initializeUsdc(
        {
          accounts: {
            authority: owner.publicKey,
            config,
            vaultUsdcAccount,
            usdcMint: usdc,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId
          },
          signers: [owner]
        }
      );
      console.log("tx1->", tx1);
    } catch (error) {
      console.log(error);
    }
    
  });
  it("deposit usdc", async() => {
    const [userPool, _] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("USER-AUTHORITY"),
        user.publicKey.toBuffer()
      ],
      program.programId
    );

    const userWalletIndex = 1;

    const [userWallet, _1] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("USER-WALLET"),
        new anchor.BN(userWalletIndex).toBuffer("le", 4)
      ],
      program.programId
    );

    console.log("userWallet->", userWallet.toString());

    const fromAta = await getAssociatedTokenAddress(
      usdc,
      user.publicKey,
    );
    console.log("from->", fromAta.toString());

    const [to, _2] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("TOKEN-VAULT"),
        user.publicKey.toBuffer(),
        usdc.toBuffer()
      ],
      program.programId
    ); 

    console.log("toAta->", to.toString());

    const depositAmount = 10_000000;

    try {
      const tx = await program.rpc.depositUsdc(
        userWalletIndex,
        new anchor.BN(depositAmount), {
          accounts: {
            config,
            userPool,
            userWallet,
            mint: usdc,
            fromAta,
            toAta: to,
            user: user.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId
          },
          signers: [user]
        }
      );
    } catch (error) {
      console.log(error);
    }
  });

  it("forward usdc to admin", async () => {
    const userWalletIndex = 1;
    const [userSendAccount, _] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("TOKEN-VAULT"),
        user.publicKey.toBuffer(),
        usdc.toBuffer()
      ],
      program.programId
    );

    const [userWallet, _1] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("USER-WALLET"),
        new anchor.BN(userWalletIndex).toBuffer("le", 4)
      ],
      program.programId
    );

    const [userPool, _2] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("USER-AUTHORITY"),
        user.publicKey.toBuffer()
      ],
      program.programId
    );

    try {
      const tx = await program.rpc.forwardToAdmin(
        userWalletIndex, {
          accounts: {
            config,
            userSendAccount,
            vaultReceiveAccount: vaultUsdcAccount,
            mint: usdc,
            userWallet,
            userPool,
            user: user.publicKey,
            authority: user.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId
          },
          signers: [user]
        }
      );
      console.log("tx->", tx);
    } catch (error) {
      console.log(error)
    }

   
  });
  it("deposit usdt", async() => {
    const provider = anchor.AnchorProvider.local();
    console.log(provider.wallet.publicKey.toString());
    const user2 = provider.wallet.payer;
    console.log(user2.publicKey.toString());


    // const wallet = new NodeWallet(user);
    const wallet = provider.wallet;

    const [userPool, _] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("USER-AUTHORITY"),
        user2.publicKey.toBuffer()
      ],
      program.programId
    );

    const userWalletIndex = 2;

    const [userWallet, _1] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("USER-WALLET"),
        new anchor.BN(userWalletIndex).toBuffer("le", 4)
      ],
      program.programId
    );

    const fromAta = await getAssociatedTokenAddress(
      usdt,
      user2.publicKey,
    );
    console.log(fromAta.toString());

    const [toAta, _2] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("TOKEN-VAULT"),
        user2.publicKey.toBuffer(),
        usdt.toBuffer()
      ],
      program.programId
    );

    const HERMES_URL = "https://hermes.pyth.network/";
    const DEVNET_RPC_URL = "https://api.devnet.solana.com";


    const connection = program.provider.connection;

    const priceServiceConnection = new PriceServiceConnection(HERMES_URL, {
      priceFeedRequestConfig: { binary: true },
    });

   
    const pythSolanaReceiver = new PythSolanaReceiver({
      connection,
      wallet: wallet as Wallet,
    });

    const priceUpdateData = await priceServiceConnection.getLatestVaas([
      USDT_PRICE_FEED_ID,
      USDC_PRICE_FEED_ID
    ]);

    const transactionBuilder = pythSolanaReceiver.newTransactionBuilder({
      closeUpdateAccounts: false,
    });
    await transactionBuilder.addPostPriceUpdates(priceUpdateData);

    const depositAmount = 100000000;

    await transactionBuilder.addPriceConsumerInstructions(
      async (
        getPriceUpdateAccount: (priceFeedId: string) => PublicKey
      ): Promise<InstructionWithEphemeralSigners[]> => {
        return [
          {
            instruction: await program.methods
              .depositUsdt(userWalletIndex, new anchor.BN(depositAmount))
              .accounts({
                config,
                userPool,
                userWallet,
                mint: usdt,
                fromAta,
                toAta,
                usdcPriceUpdate: getPriceUpdateAccount(USDC_PRICE_FEED_ID),
                usdtPriceUpdate: getPriceUpdateAccount(USDT_PRICE_FEED_ID),
                user: user2.publicKey,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId
              })
              .instruction(),
              signers: [user2],
          },
        ];
      }
    );

    try {
      await pythSolanaReceiver.provider.sendAll(
        await transactionBuilder.buildVersionedTransactions({
          computeUnitPriceMicroLamports: 50000,
        }),
        user2,
        // { 
        //   skipPreflight: true,
        // }
      );
    } catch (error) {
      console.log(error);
    }
  });

  it("forward usdt to admin", async () => {
    const provider = anchor.AnchorProvider.local();
    const user2 = provider.wallet.payer;

    const userWalletIndex = 2;
    const [userSendAccount, _] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("TOKEN-VAULT"),
        user2.publicKey.toBuffer(),
        usdt.toBuffer()
      ],
      program.programId
    );

    const [userWallet, _1] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("USER-WALLET"),
        new anchor.BN(userWalletIndex).toBuffer("le", 4)
      ],
      program.programId
    );

    const [userPool, _2] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("USER-AUTHORITY"),
        user2.publicKey.toBuffer()
      ],
      program.programId
    );

    try {
      const tx = await program.rpc.forwardToAdmin(
        userWalletIndex, {
          accounts: {
            config,
            userSendAccount,
            vaultReceiveAccount: vaultUsdtAccount,
            mint: usdt,
            userWallet,
            userPool,
            user: user2.publicKey,
            authority: user2.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId
          },
          signers: [user2]
        }
      );
      console.log("tx->", tx);
    } catch (error) {
      console.log(error)
    }   
  });

  it("deposit sol", async() => {
    const provider = anchor.AnchorProvider.local();
    const user2 = provider.wallet.payer;
    const wallet = provider.wallet;

    const [userPool, _] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("USER-AUTHORITY"),
        user2.publicKey.toBuffer()
      ],
      program.programId
    );

    const userWalletIndex = 2;

    const [userWallet, _1] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("USER-WALLET"),
        new anchor.BN(userWalletIndex).toBuffer("le", 4)
      ],
      program.programId
    );

    const HERMES_URL = "https://hermes.pyth.network/";
    const DEVNET_RPC_URL = "https://api.devnet.solana.com";


    const connection = program.provider.connection;

    const priceServiceConnection = new PriceServiceConnection(HERMES_URL, {
      priceFeedRequestConfig: { binary: true },
    });

    const pythSolanaReceiver = new PythSolanaReceiver({
      connection,
      wallet: wallet as Wallet,
    });

    const priceSolUpdateData = await priceServiceConnection.getLatestVaas([
      SOL_PRICE_FEED_ID,
    ]);

    const priceUsdcUpdateData = await priceServiceConnection.getLatestVaas([
      USDC_PRICE_FEED_ID,
    ]);

    const transactionBuilder = pythSolanaReceiver.newTransactionBuilder({
      closeUpdateAccounts: true,
    });
    await transactionBuilder.addPostPriceUpdates([priceSolUpdateData[0]]);
    await transactionBuilder.addPostPriceUpdates([priceUsdcUpdateData[0]]);

    const depositAmount = 10000000;

    await transactionBuilder.addPriceConsumerInstructions(
      async (
        getPriceUpdateAccount: (priceFeedId: string) => PublicKey
      ): Promise<InstructionWithEphemeralSigners[]> => {
        return [
          {
            instruction: await program.methods
              .depositSol(userWalletIndex,new anchor.BN(depositAmount))
              .accounts({
                config,
                userPool,
                userWallet,
                usdcPriceUpdate: getPriceUpdateAccount(USDC_PRICE_FEED_ID),
                solPriceUpdate: getPriceUpdateAccount(SOL_PRICE_FEED_ID),
                user: user2.publicKey,
                tokenProgram: TOKEN_PROGRAM_ID,
                systemProgram: SystemProgram.programId
              })
              .instruction(),
              signers: [user2],
          },
        ];
      }
    );
    try {
      const tx = await pythSolanaReceiver.provider.sendAll(
        await transactionBuilder.buildVersionedTransactions({
          computeUnitPriceMicroLamports: 50000,
        }),
        user2
        // { skipPreflight: true }
      );
      console.log("tx->", tx);
    } catch (error) {
      console.log(error);
    }
  });

  it("forward sol to admin", async() => {
    const provider = anchor.AnchorProvider.local();
    const user2 = provider.wallet.payer;

    const userWalletIndex = 2;
 
    const [userWallet, _1] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("USER-WALLET"),
        new anchor.BN(userWalletIndex).toBuffer("le", 4)
      ],
      program.programId
    );

    const [masterWallet, _2] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("MASTER-WALLET"),
      ],
      program.programId
    );

    const [userPool, _3] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("USER-AUTHORITY"),
        user2.publicKey.toBuffer()
      ],
      program.programId
    );

    try {
      const tx = await program.rpc.forwardSolToAdmin(
        userWalletIndex, {
          accounts: {
            config,
            userWallet,
            masterWallet,
            userPool,
            authority: user2.publicKey,
            systemProgram: SystemProgram.programId
          },
          signers: [user2]
        }
      );
      console.log("tx->", tx);
    } catch (error) {
      console.log(error)
    }
  });
  it("withdraw usdc", async() => {
    const [userPool, _] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("USER-AUTHORITY"),
        user.publicKey.toBuffer()
      ],
      program.programId
    );

    const userReceiveAccount = await getAssociatedTokenAddress(
      usdc,
      user.publicKey
    );

    const userWalletIndex = 1;

    const withdrawAmount = 1000000;
    console.log("vaultUsdcAccount", vaultUsdcAccount.toString());

    try {
      const tx = await program.rpc.withdraw(
        new anchor.BN(withdrawAmount),
        {
          accounts: {
            config,
            userReceiveAccount,
            vaultSendAccount: vaultUsdcAccount,
            mint: usdc,
            userPool,
            authority: user.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId
          },
          signers: [user]
        }
      );
    } catch (error) {
      console.log(error);
    }
  })
  
  it("withdraw usdt by owner", async() => {
    const provider = anchor.AnchorProvider.local();
    const user2 = provider.wallet.payer;

    const userReceiveAccount = await getAssociatedTokenAddress(
      usdt,
      owner.publicKey
    );

    const withdrawAmount = 10000000;

    try {
      const tx = await program.rpc.withdrawUsdt(
        new anchor.BN(withdrawAmount),
        {
          accounts: {
            config,
            mint: usdt,
            fromAta: vaultUsdtAccount,
            toAta: userReceiveAccount,
            user: owner.publicKey,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
            systemProgram: SystemProgram.programId
          },
          signers: [owner]
        }
      );
    } catch (error) {
      console.log(error);
    }
  })

  it("withdraw sol by owner", async() => {
   
    const [masterWallet, _2] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("MASTER-WALLET"),
      ],
      program.programId
    );

    
    const withdrawAmount = 10000000;

    try {
      const tx = await program.rpc.withdrawSol(
        new anchor.BN(withdrawAmount),
        {
          accounts: {
            config,
            masterWallet,
            user: owner.publicKey,
            systemProgram: SystemProgram.programId
          },
          signers: [owner]
        }
      );
      console.log(tx);
    } catch (error) {
      console.log(error);
    }
  })

});
