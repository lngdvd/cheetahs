import * as web3 from "@solana/web3.js";
import * as spl from '@solana/spl-token';
import * as anchor from "@coral-xyz/anchor";
import { Peregrinus } from '../../target/types/peregrinus'
import base58 from "bs58";
import fs from "fs";


const program = anchor.workspace.Peregrinus as anchor.Program<Peregrinus>
const provider = anchor.AnchorProvider.env()
const connection = provider.connection;


export async function execTx(tx: web3.Transaction | web3.VersionedTransaction, signers: web3.Signer[], checked = false) {
	// const txs = new web3.Transaction()
	// txArr.forEach(tx => {
	// 	txs.add(tx)
	// });
	const txHash = await provider.sendAndConfirm(tx, signers)
	printTx(txHash, checked)
}


export function printTx(tx, checked = false) {
	if (checked) {
		console.log(
			"\nTx: ",
			`https://explorer.solana.com/tx/${tx}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899#ix-1`
		);
	}
}

export function findPdaWithId(seeds: Array<Buffer | Uint8Array>, programId: web3.PublicKey) {
	// return [publicKey, bump]
	return web3.PublicKey.findProgramAddressSync(seeds, programId)[0];
}


export function findPda(seeds: Array<Buffer | Uint8Array>) {
	// return [publicKey, bump]
	return web3.PublicKey.findProgramAddressSync(seeds, program.programId)[0];
}


export async function createNewToken(payer: web3.Keypair, ui_amount: number) {
	const mintAuthority = web3.Keypair.generate();
	const freezeAuthority = web3.Keypair.generate();

	const mint = await spl.createMint(
		connection,
		payer,
		mintAuthority.publicKey,
		freezeAuthority.publicKey,
		9 // We are using 9 to match the CLI decimal default exactly
	);
	// console.log(mint.toBase58());


	const mintInfo = await spl.getMint(
		connection,
		mint
	)
	// console.log(mintInfo.supply);


	const tokenAccount = await spl.getOrCreateAssociatedTokenAccount(
		connection,
		payer,
		mint,
		payer.publicKey
	)
	// console.log(tokenAccount.address.toBase58());


	const tokenAccountInfo = await spl.getAccount(
		connection,
		tokenAccount.address
	)
	// console.log(tokenAccountInfo.amount);
	// 0

	await spl.mintTo(
		connection,
		payer,
		mint,
		tokenAccount.address,
		mintAuthority,
		web3.LAMPORTS_PER_SOL * ui_amount // because decimals for the mint are set to 9
	)

	const mintInfo2 = await spl.getMint(
		connection,
		mint
	)

	// console.log(mintInfo2.supply);
	// 100

	const tokenAccountInfo2 = await spl.getAccount(
		connection,
		tokenAccount.address
	)
	// console.log(tokenAccountInfo2.amount);
	// 100

	return mint
}



/********************************/
// keypair
/********************************/
export async function keypairFromFile(filePath: string) {
	return web3.Keypair.fromSecretKey(Buffer.from(JSON.parse(fs.readFileSync(filePath, "utf-8"))));
}

export async function secretKeyToArray(privateKey: string) {
	return base58.decode(privateKey)
}

export async function secretToKeypair(privateKey: string) {
	return web3.Keypair.fromSecretKey(await secretKeyToArray(privateKey));
}
