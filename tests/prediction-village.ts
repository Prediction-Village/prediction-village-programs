import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PredictionVillage } from "../target/types/prediction_village";
import { expect } from "chai";

describe("prediction-village", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PredictionVillage as Program<PredictionVillage>;
  const [gamePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("game"), provider.wallet.publicKey.toBuffer()],
    program.programId
  );

  it("Initializes a new game", async () => {
    await program.methods
      .initGame()
      .accounts({
        game: gamePda,
        payer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const gameAccount = await program.account.game.fetch(gamePda);
    expect(gameAccount.status).to.deep.equal({ inProgress: {} });
  });

  it("Generates income for villages", async () => {
    await program.methods
      .generateIncome()
      .accounts({
        game: gamePda,
      })
      .rpc();

    const gameAccount = await program.account.game.fetch(gamePda);
    console.log(gameAccount);
    expect(gameAccount.lightForces.resources.goldIncome).to.be.greaterThan(0);
    expect(gameAccount.darkForces.resources.goldIncome).to.be.greaterThan(0);
  });

  it("Fails to restart an in-progress game", async () => {
    try {
      await program.methods
        .restartGame()
        .accounts({
          game: gamePda,
          authority: provider.wallet.publicKey,
        })
        .rpc();
      expect.fail("Should have failed");
    } catch (err) {
      expect(err.toString()).to.include("GameInProgress");
    }
  });
});
