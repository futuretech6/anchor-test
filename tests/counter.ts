import * as anchor from "@coral-xyz/anchor";

describe("anchor-test", () => {
    const provider = anchor.AnchorProvider.env();

    anchor.setProvider(provider);

    const program = new anchor.Program(
        JSON.parse(
            require("fs").readFileSync("./target/idl/counter.json", "utf8")
        ),
        new anchor.web3.PublicKey(
            "Bz3bY4X3oPQsBoUr4B4LWHW2Jf3JYh2UwsAHhmdRN1bq"
        )
    );

    const seed = anchor.utils.bytes.utf8.encode("storage");

    let storagePubkey: anchor.web3.PublicKey;

    [storagePubkey] = anchor.web3.PublicKey.findProgramAddressSync(
        [seed],
        program.programId
    );

    it("Initializing", async () => {
        const tx = await program.methods
            .initialize(new anchor.BN(0))
            .accounts({
                storage: storagePubkey,
                authority: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .rpc();

        console.log("Initialized");
        console.log("Your transaction signature", tx);
    });

    it("Executes a method on the program", async () => {
        const tx = await program.methods
            .increse()
            .accounts({
                storage: storagePubkey,
                authority: provider.wallet.publicKey,
            })
            .rpc();

        console.log("Increase + 1");
        console.log("Your transaction signature", tx);
    });
});
