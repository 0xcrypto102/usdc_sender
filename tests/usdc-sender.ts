import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { UsdcSender } from "../target/types/usdc_sender";

const userKp = anchor.web3.Keypair.generate()

describe("usdc-sender", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  const program = anchor.workspace.UsdcSender as Program<UsdcSender>;
  const connection = new anchor.web3.Connection(anchor.web3.clusterApiUrl("devnet"), "confirmed");

  it("airdrop", async () => {
    try {
      console.log(`-- Airdropping 2 SOL --`)
      const fromAirDropSignature = await connection.requestAirdrop(
        userKp.publicKey,
        2 * anchor.web3.LAMPORTS_PER_SOL
      );
      await connection.confirmTransaction(fromAirDropSignature, "confirmed");
    } catch (err) {
      console.log(err);
    } finally {
      const pubKey = userKp.publicKey.toBase58()
      const balance = await connection.getBalance(userKp.publicKey) / 10 ** 9;
      console.table([{ Key: "User", Value: pubKey }, { Key: "Balance", Value: balance }])
    }
  })

  it('Is initialized!', async () => {
    const [globalPool] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("global-authority")],
      program.programId
    )
    const [user] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user-authority"),],
      program.programId
    )

    const tx = await program.methods.initialize()
      .accounts({
        globalPool,
        user,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([])
      // .rpc();
      .transaction()

    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
    tx.feePayer = userKp.publicKey

    console.log("transaction simulation result ==>", await connection.simulateTransaction(tx,))
    // await connection.confirmTransaction(tx)
    // console.log("Your transaction signature", tx);
  });

  // it('Deposit', async () => {
  //   const [
  //     poolSigner,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       poolKeypair.publicKey.toBuffer(),
  //     ],
  //     program.programId
  //   );

  //   const amount = anchor.web3.LAMPORTS_PER_SOL / 10;
  //   const tx = await program.methods.deposit(new anchor.BN(amount), {
  //     accounts: {
  //       pool: poolKeypair.publicKey,
  //       authority: provider.wallet.publicKey,
  //       vault: poolSigner,
  //       depositor: provider.wallet.publicKey,
  //       poolSigner: poolSigner,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     },
  //   }).rpc();

  //   let contractLamports = (await provider.connection.getBalance(poolSigner));
  //   assert.equal(contractLamports, amount);
  // })

  // it('Withdraw', async () => {
  //   const [
  //     poolSigner,
  //     nonce,
  //   ] = await anchor.web3.PublicKey.findProgramAddress(
  //     [
  //       poolKeypair.publicKey.toBuffer(),
  //     ],
  //     program.programId
  //   );

  //   const amount = anchor.web3.LAMPORTS_PER_SOL / 10;
  //   const tx = await program.rpc.withdraw(new anchor.BN(amount), {
  //     accounts: {
  //       pool: poolKeypair.publicKey,
  //       authority: provider.wallet.publicKey,
  //       vault: poolSigner,
  //       receiver: provider.wallet.publicKey,
  //       poolSigner: poolSigner,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     },
  //   });

  //   let contractLamports = (await provider.connection.getBalance(poolSigner));
  //   assert.equal(contractLamports, 0);
  // })
});
