import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Group6PaymentProtocol } from "../target/types/group_6_payment_protocol";
import { SystemProgram, Transaction, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { assert, expect } from "chai";

describe("group_6_payment_protocol", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Group6PaymentProtocol as Program<Group6PaymentProtocol>;
  let client = anchor.web3.Keypair.generate();
  let freelancer = anchor.web3.Keypair.generate();
  let observer = anchor.web3.Keypair.generate();

  let projectInfoAccount: anchor.web3.PublicKey;
  let projectInfoAccountBump: number;

  let tokenEscrow: anchor.web3.PublicKey;
  let multisig: anchor.web3.PublicKey;

  let milestones = 5;
  let totalFundsForProject = new anchor.BN(LAMPORTS_PER_SOL * 20_000);

  before(async () => {
    [projectInfoAccount, projectInfoAccountBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("project_info_account"),
        client.publicKey.toBuffer(),
        freelancer.publicKey.toBuffer(),
      ],
      program.programId
    );


    [multisig,] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("multisig"),
        projectInfoAccount.toBuffer(),
      ],
      program.programId
    );

    [tokenEscrow,] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("token_escrow"),
        projectInfoAccount.toBuffer(),
      ],
      program.programId
    );
    const tx = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: provider.wallet.publicKey,
        toPubkey: client.publicKey,
        lamports: 20_100 * LAMPORTS_PER_SOL,
      }),
    );
    await provider.sendAndConfirm(tx,)

  })

  it("initializes a project", async () => {
    await program.methods
      .initializeProject(totalFundsForProject, milestones)
      .accounts({
        client: client.publicKey,
        freelancer: freelancer.publicKey,
        observer: observer.publicKey,
        multisig,
        projectInfoAccount,
        tokenEscrow
      })
      .signers([client])
      .rpc()

    const projectState = await program.account.projectInfo.fetch(projectInfoAccount);
    const newTokenEscrowBalance = await provider.connection.getBalance(tokenEscrow);


    expect(projectState.client).to.deep.equal(client.publicKey);
    expect(projectState.freelancer).to.deep.equal(freelancer.publicKey);
    expect(newTokenEscrowBalance).to.be.greaterThanOrEqual(totalFundsForProject.toNumber())
  })

  it("client cancels the project", async () => {
    const clientBalanceBefore = await provider.connection.getBalance(client.publicKey);
    const projectState = await program.account.projectInfo.fetch(projectInfoAccount);

    await program.methods
      .cancelProject()
      .accounts({
        client: client.publicKey,
        freelancer: freelancer.publicKey,
        projectInfoAccount,
        tokenEscrow
      })
      .signers([client])
      .rpc()
      .catch(e => console.log(e))


    const clientBalanceBAfter = await provider.connection.getBalance(client.publicKey);
    expect(clientBalanceBAfter).to.be.gte(clientBalanceBefore + projectState.totalProjectFunds.toNumber());
  })
});
