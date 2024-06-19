import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import * as web3 from "@solana/web3.js";
import * as spl from "@solana/spl-token";
import { Peregrinus } from "../target/types/peregrinus";

import {
	printTx
} from "../app/shared/utils";


describe("about token 2022 with token extension", () => {
	const provider = anchor.AnchorProvider.env()
	anchor.setProvider(provider);
	const program = anchor.workspace.Peregrinus as Program<Peregrinus>;
	const wallet = provider.wallet as anchor.Wallet;

	it("Mint cNFT", async () => {

	});
});
