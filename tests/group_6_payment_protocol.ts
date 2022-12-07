import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Group6PaymentProtocol } from "../target/types/group_6_payment_protocol";

describe("group_6_payment_protocol", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Group6PaymentProtocol as Program<Group6PaymentProtocol>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
