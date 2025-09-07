import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EnergyTrading } from "../target/types/energy_trading";
import { expect } from "chai";

describe("Simple Energy Trading Tests", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.EnergyTrading as Program<EnergyTrading>;

  it("Should initialize program successfully", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("✅ Program initialized:", tx);
    expect(tx).to.be.a("string");
  });

  it("Should have settleAuction method available", async () => {
    const methods = program.methods;
    expect(methods.settleAuction).to.exist;
    console.log("✅ settleAuction method is available");
  });

  it("Should have correct program ID", async () => {
    const programId = program.programId.toString();
    expect(programId).to.equal("4wEDVLBid4pKiXzkq8hT6zEWU9F62nDibPS38d2QSrJb");
    console.log("✅ Program ID is correct:", programId);
  });
});
