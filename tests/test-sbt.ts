import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Peregrinus } from "../target/types/peregrinus";
import * as web3 from "@solana/web3.js";
import * as spl from "@solana/spl-token";

import {
  printTx
} from "../app/shared/utils";


describe("about token 2022 with token extension", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  const program = anchor.workspace.Peregrinus as Program<Peregrinus>;
  const wallet = provider.wallet as anchor.Wallet;

  async function testCreateNonTransferableToken() {
    const mintKeypair = web3.Keypair.generate();
    // Address for Mint Account
    const mint = mintKeypair.publicKey;
    // Decimals for Mint Account
    const decimals = 2;
    // Authority that can mint new tokens
    const mintAuthority = wallet.publicKey;
    // Size of Mint Account with extension
    const mintLen = spl.getMintLen([spl.ExtensionType.NonTransferable]);
    // Minimum lamports required for Mint Account
    const lamports = await provider.connection.getMinimumBalanceForRentExemption(mintLen);

    console.log(`mintLen: ${mintLen}`);
    console.log(`lamports: ${lamports}`);

    // Instruction to invoke System Program to create new account
    const createAccountInstruction = web3.SystemProgram.createAccount({
      fromPubkey: wallet.payer.publicKey, // Account that will transfer lamports to created account
      newAccountPubkey: mint, // Address of the account to create
      space: mintLen, // Amount of bytes to allocate to the created account
      lamports, // Amount of lamports transferred to created account
      programId: spl.TOKEN_2022_PROGRAM_ID, // Program assigned as owner of created account
    });
    const transaction = new web3.Transaction().add(
      createAccountInstruction,
    );

    const tx = await web3.sendAndConfirmTransaction(
      provider.connection,
      transaction,
      [wallet.payer, mintKeypair] // Signers
    );

    return tx
  }

  it("Mint SBT", async () => {
    const mintKeypair = web3.Keypair.generate();
    const mintInstruction = await program.methods.mintSbt()
      .accountsPartial({
        mint: mintKeypair.publicKey,
      }).instruction();

    const transaction = new web3.Transaction().add(
      mintInstruction,
    );

    const tx = await web3.sendAndConfirmTransaction(
      provider.connection,
      transaction,
      [wallet.payer, mintKeypair] // Signers
    );
    printTx(tx)
  });
});
