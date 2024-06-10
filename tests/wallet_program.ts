import 'rpc-websockets/dist/lib/client';
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { WalletProgram } from "../target/types/wallet_program";
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddress, getAssociatedTokenAddressSync } from "@solana/spl-token";
import { SystemProgram, Keypair, PublicKey, Connection } from "@solana/web3.js";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { raydiumAmmProgram } from "./src";
import { AnchorProvider, BN } from "@coral-xyz/anchor";
import { CustomWallet } from "./utils/wallet";

describe("wallet_program", () => {
  // Configure the client to use the local cluster.

  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.WalletProgram as Program<WalletProgram>;
  const owner = Keypair.fromSecretKey(
    Uint8Array.from([113, 63, 93, 213, 68, 178, 22, 189, 136, 49, 33, 174, 196, 213, 238, 242, 164, 106, 9, 180, 15, 3, 238, 80, 159, 127, 118, 18, 231, 206, 240, 93, 21, 168, 99, 61, 85, 242, 222, 187, 12, 44, 91, 158, 122, 83, 103, 113, 125, 136, 28, 83, 108, 248, 78, 219, 197, 250, 38, 187, 70, 109, 130, 194])
  );

  it("proxy swap based in", async() => {
    const amountIn = 1000;
    const minimumAmountOut = 0;
    const ammProgram = new PublicKey("HWy1jotHpo6UqeQxx49dpYYdQB8wj9Qk9MdxwjLvDHB8");

    const accounts = {
      id: "HPL5LFbtpTh9weuXN5qYxoz5trz2uw9E9WeW6Xu3hSVd",
      baseMint: "8qcbzW9R6AQjbB9au4DQsUyAgBVxaTERctf5VK1fhZG9",
      quoteMint: "9riEQbfLX5hSDBkAbv4jFKYAHq83qSmfmptHnCGM3Ewr",
      lpMint: "HVeyxt2K3fBBgjZjVJzyhE6Vr3SCQhVqfnaNDQ6ccmhV",
      programId: "AMMjRTfWhP73x9fM6jdoXRfgFJXR97NFRkV8fYJUrnLE",
      authority: "C94dgABc4T1AisH4QDTrd7xkHK2VTuy8CUQXUsMGULCM",
      openOrders: "7mMgN9quWxcmdNimirmdrYfasKthm1mvknLdHc3xwqaJ",
      targetOrders: "BcHLBRrA4218YKZB3zA2HLCiesJNp33VaYcQZqhpWou9",
      baseVault: "BQviZQciaoaCHwwCS7VteFWqFj5mU7oo5ZAvJEd6nbK1",
      quoteVault: "5bUrLohrJQEm3cNjh2DfuNneJVdSfgv9vEFPqMXyQ7s5",
      withdrawQueue: "364yesUy8h2STJVBtjooC8zgx9QMZhfYfnfEibobcycJ",
      lpVault: "EumtSiiRvEyDsoXeH63G2vNgzYKZf1V5Yuhp44CgRiQP",
      marketProgramId: "EoTcMgcDRTJVZDMZWBoU6rhYHZfkNTVEAfz3uUJRcYGj",
      marketId: "5jjKoPK1C6weRxiCbvNEo34AhfE4ScqndNGSUhq136fo",
      marketAuthority: "HPQU5JAc1faWA21KUkTbTWc2pLKZtzUnJ9gL4Qp3rXcF",
      marketBaseVault: "J6C5vymW8DAoqZW83k5yiTtpDpkt1tePqY158xZWgUmT",
      marketQuoteVault: "5VFWwwdgjnF5SgH5VpwLi9UjRtY3HPN2Z88Mid24sPtG",
      marketBids: "FcMgHYt5sYWD52sWC6z67F9ujXgs3ct4jJR8tJe3Af1",
      marketAsks: "GBaskvipw6aSjQCe8Si4rL2e5KgdzECH333LYeMypbb1",
      marketEventQueue: "HBkfqZEPJ9kSoS39Kqmwydx8K9MUfTp7AwLMGkuZKJUb",
    };

    const coinMint = new PublicKey(accounts.baseMint);
    const pcMint = new PublicKey(accounts.quoteMint);

    const uerSourceTokenAccount = getAssociatedTokenAddressSync(
        coinMint,
        owner.publicKey
    );
   
    const uerDestinationTokenAccount = getAssociatedTokenAddressSync(
        pcMint,
        owner.publicKey
    );
   
    try {
      const tx = await program.rpc.proxySwapBaseIn(
        new anchor.BN(amountIn),
        new anchor.BN(minimumAmountOut), {
            accounts: {
                ammProgram,
                amm: new PublicKey(accounts.id),
                ammAuthority: new PublicKey(accounts.authority),
                ammOpenOrders: new PublicKey(accounts.openOrders),
                ammCoinVault: new PublicKey(accounts.baseVault),
                ammPcVault: new PublicKey(accounts.quoteVault),
                marketProgram: new PublicKey("EoTcMgcDRTJVZDMZWBoU6rhYHZfkNTVEAfz3uUJRcYGj"),
                market: new PublicKey(accounts.marketId),
                marketBids:  new PublicKey(accounts.marketBids),
                marketAsks: new PublicKey(accounts.marketAsks),
                marketEventQueue: new PublicKey(accounts.marketEventQueue),
                marketCoinVault: new PublicKey(accounts.marketBaseVault),
                marketPcVault: new PublicKey(accounts.marketQuoteVault),
                marketVaultSigner: new PublicKey(accounts.marketAuthority),
                userTokenSource: uerSourceTokenAccount,
                userTokenDestination: uerDestinationTokenAccount,
                userSourceOwner: owner.publicKey,
                tokenProgram: TOKEN_PROGRAM_ID,
            },
            signers: [owner]
        }
      );
    } catch (error) {
      console.log(error);
    }
  })
   
});
