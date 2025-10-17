import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import {
  createMint,
  getAccount,
  getAssociatedTokenAddress,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";

describe("escrow-contract", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  const payer = provider.wallet.payer;
  anchor.setProvider(provider);
  provider.connection.requestAirdrop(provider.wallet.publicKey, 2000000000)
  const program = anchor.workspace.escrow as Program<Escrow>;


  //pree stuff
  const escrow = anchor.web3.Keypair.generate();
  const receiver = anchor.web3.Keypair.generate();


  const [vaultauthority] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), escrow.publicKey.toBuffer()],
    program.programId
  );

  let mint;

  it("Intialize escrow ", async () => {
    mint = await createMint(connection, payer, payer.publicKey, null, 6);

    const amount = new anchor.BN(10000);
    const initializerTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      payer,
      mint,
      provider.wallet.publicKey
    );

    await mintTo(
      connection,
      payer,
      mint,
      initializerTokenAccount.address,
      payer,
      1_000_000
    );


    const tx = await program.methods.initializeEscrow(amount, receiver.publicKey)
      .accounts({
        initializer: provider.wallet.publicKey,
        initializerTokenAcc: initializerTokenAccount.address,
        escrowAccount: escrow.publicKey,
        vaultAuthority: vaultauthority,
        mint: mint
      }).signers([escrow])
      .rpc()
    console.log("Initialization escrow : ", tx);
  });

  it("Claim Escrow", async () => {
    const vault = await getAssociatedTokenAddress(mint, vaultauthority, true)
    const receiverTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      payer,
      mint,
      receiver.publicKey
    );

    let vaultamount = await getAccount(connection, vault)
    console.log("Vault amount before ", vaultamount.amount);

    console.log("Receiver token address  :", receiverTokenAccount.amount);

    const tx = await program.methods.claimEscrow().accounts({
      escrowAccount: escrow.publicKey,
      vault: vault,
      receiverTokenAccount: receiverTokenAccount.address,
    }).rpc()


    console.log("Claim Escrow", tx)


    const updatedReceiverAccount = await getAccount(connection, receiverTokenAccount.address);
    console.log("Receiver token balance after:", Number(updatedReceiverAccount.amount));

    vaultamount = await getAccount(connection, vault)
    console.log("Vault amount AFTER ", vaultamount.amount);
  });
});
