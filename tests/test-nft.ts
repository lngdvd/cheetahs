// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { Peregrinus } from "../target/types/peregrinus";
// import {
// 	findMasterEditionPda,
// 	findMetadataPda,
// 	mplTokenMetadata,
// 	findCollectionAuthorityRecordPda,
// } from "@metaplex-foundation/mpl-token-metadata";

// describe("test normal nft", () => {
// 	// Configure the client to use the local cluster.
// 	anchor.setProvider(anchor.AnchorProvider.env());

// 	const program = anchor.workspace.Peregrinus as Program<Peregrinus>;
// const connection = program.provider.connection;
// const umi = createUmi(connection.rpcEndpoint)
//     .use(walletAdapterIdentity(anchor.AnchorProvider.env().wallet))
//     .use(mplTokenMetadata());

// 	it("Is initialized!", async () => {
// 		// Add your test here.
// 		const tx = await program.methods.initialize().rpc();
// 		console.log("Your transaction signature", tx);
// 	});
// });
