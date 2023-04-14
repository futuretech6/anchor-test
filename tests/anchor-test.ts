import * as anchor from "@coral-xyz/anchor";

describe("anchor-test", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    // const program = anchor.workspace.AnchorTest as Program<AnchorTest>;

    const program = new anchor.Program(
        JSON.parse(
            require("fs").readFileSync("./target/idl/anchor_test.json", "utf8")
        ),
        new anchor.web3.PublicKey(
            "BinYbLe2RXbVWvntQybnye37Lc5k7XT2XCaFLwAWD85h"
        )
    );

    it("Is initialized!", async () => {
        // Add your test here.
        const tx = await program.methods.initialize().rpc();
        console.log("Your transaction signature", tx);
    });
});
