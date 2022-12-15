import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Group6PaymentProtocol } from "../target/types/group_6_payment_protocol";
import { SystemProgram, Transaction, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { expect } from "chai";
import { getMultisigTransactionPda } from "../utils/utils";

describe("Client goes through with a project", () => {
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

    let milestones = 4;
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


    it("at least 2 of three members agrees to start initialized project", async () => {

        let multisigData = await program.account.multisig.fetch(multisig)
        expect(multisigData.seqno).to.equal(0);

        let projectData = await program.account.projectInfo.fetch(projectInfoAccount)
        expect(projectData.status).to.equal(0);
        //Any of the parties can initialize the start project transaction
        //of any multisig transaction at that

        //create transaction
        const accounts = [
            {
                pubkey: projectInfoAccount,
                isWritable: true,
                isSigner: false
            },
            {
                pubkey: multisig,
                isWritable: false,
                isSigner: true
            }
        ];
        const data = program.coder.instruction.encode("start_project", {})

        const multisigTxPda = await getMultisigTransactionPda(program as anchor.Program, multisig, multisigData.seqno);

        await program.methods.
            createTransaction(program.programId, accounts, data)
            .accounts({
                multisig,
                projectInfoAccount,
                proposer: client.publicKey,
                transaction: multisigTxPda.key
            })
            .signers([client])
            .rpc()

        // approve -- only needs two signers to approve and execute
        await program.methods
            .approve()
            .accounts({
                multisig,
                transaction: multisigTxPda.key,
                owner: freelancer.publicKey,
                multisigSigner: multisig,
                projectInfoAccount
            })
            .remainingAccounts(accounts.map(
                account => account.pubkey.equals(multisig) ?
                    { ...account, isSigner: false } : account
            )
                .concat({
                    pubkey: program.programId,
                    isWritable: false,
                    isSigner: false,
                }))

            .signers([freelancer])
            .rpc()
            .catch(e => console.log(e));

        const txData = await program.account.transaction.fetch(multisigTxPda.key)
        expect(txData.didExecute).to.equal(true);

        projectData = await program.account.projectInfo.fetch(projectInfoAccount)
        expect(projectData.status).to.equal(1);
    })

    it("client can't cancels the project at this point", async () => {

        const result = await program.methods
            .cancelProject()
            .accounts({
                client: client.publicKey,
                freelancer: freelancer.publicKey,
                projectInfoAccount,
                tokenEscrow
            })
            .signers([client])
            .rpc()
            .catch(e => e)

        expect(result.error.errorCode.number).to.equal(6013);
        expect(result.error.errorCode.code).to.equal('InvalidStatus');

    })

    it("both parties agrees that current milestone has been completed", async () => {
        let projectData = await program.account.projectInfo.fetch(projectInfoAccount)
        expect(projectData.milestonesReached).to.equal(0);

        let multisigData = await program.account.multisig.fetch(multisig)

        const accounts = [
            {
                pubkey: projectInfoAccount,
                isWritable: true,
                isSigner: false
            },
            {
                pubkey: multisig,
                isWritable: false,
                isSigner: true
            }
        ];
        const data = program.coder.instruction.encode("mark_current_milestone_completed", {})

        const multisigTxPda = await getMultisigTransactionPda(program as anchor.Program, multisig, multisigData.seqno);

        await program.methods.
            createTransaction(program.programId, accounts, data)
            .accounts({
                multisig,
                projectInfoAccount,
                proposer: client.publicKey,
                transaction: multisigTxPda.key
            })
            .signers([client])
            .rpc()

        // approve -- only needs two signers to approve and execute
        await program.methods
            .approve()
            .accounts({
                multisig,
                transaction: multisigTxPda.key,
                owner: freelancer.publicKey,
                multisigSigner: multisig,
                projectInfoAccount
            })
            .remainingAccounts(accounts.map(
                account => account.pubkey.equals(multisig) ?
                    { ...account, isSigner: false } : account
            )
                .concat({
                    pubkey: program.programId,
                    isWritable: false,
                    isSigner: false,
                }))

            .signers([freelancer])
            .rpc()
            .catch(e => console.log(e));

        projectData = await program.account.projectInfo.fetch(projectInfoAccount)
        expect(projectData.milestonesReached).to.equal(1);
    })

    it("freelancer can now withdraw funds for the achieved milestone", async () => {
        // expect(multisigData.seqno).to.equal(0);
        const freelancerBalance = await provider.connection.getBalance(freelancer.publicKey);
        expect(freelancerBalance).to.equal(0);

        await program.methods
            .withdrawMilestoneFunds()
            .accounts({
                freelancer: freelancer.publicKey,
                projectInfoAccount,
                tokenEscrow
            })
            .signers([freelancer])
            .rpc()
        // project_info.milestone_funds_withdrawn
        // amount of funds
        const newFreelancerBalance = await provider.connection.getBalance(freelancer.publicKey);
        expect(newFreelancerBalance).to.equal(LAMPORTS_PER_SOL * 5_000);
    })

    it("freelancer has raised a dispute and observer has check the task and voted in favour of the freelancer", async () => {
        console.log("both observer and freelancer vote to mark current milestone as completed");
        let projectData = await program.account.projectInfo.fetch(projectInfoAccount)
        expect(projectData.milestonesReached).to.equal(1);

        let multisigData = await program.account.multisig.fetch(multisig)

        const accounts = [
            {
                pubkey: projectInfoAccount,
                isWritable: true,
                isSigner: false
            },
            {
                pubkey: multisig,
                isWritable: false,
                isSigner: true
            }
        ];
        const data = program.coder.instruction.encode("mark_current_milestone_completed", {})

        const multisigTxPda = await getMultisigTransactionPda(program as anchor.Program, multisig, multisigData.seqno);

        await program.methods.
            createTransaction(program.programId, accounts, data)
            .accounts({
                multisig,
                projectInfoAccount,
                proposer: observer.publicKey,
                transaction: multisigTxPda.key
            })
            .signers([observer])
            .rpc()

        // approve -- only needs two signers to approve and execute
        await program.methods
            .approve()
            .accounts({
                multisig,
                transaction: multisigTxPda.key,
                owner: freelancer.publicKey,
                multisigSigner: multisig,
                projectInfoAccount
            })
            .remainingAccounts(accounts.map(
                account => account.pubkey.equals(multisig) ?
                    { ...account, isSigner: false } : account
            )
                .concat({
                    pubkey: program.programId,
                    isWritable: false,
                    isSigner: false,
                }))

            .signers([freelancer])
            .rpc()
            .catch(e => console.log(e));

        projectData = await program.account.projectInfo.fetch(projectInfoAccount)
        expect(projectData.milestonesReached).to.equal(2);

    })

    it("freelancer can now withdraw funds for the new milestone", async () => {
        // expect(multisigData.seqno).to.equal(0);
        const freelancerBalance = await provider.connection.getBalance(freelancer.publicKey);
        expect(freelancerBalance).to.equal(LAMPORTS_PER_SOL * 5_000);

        await program.methods
            .withdrawMilestoneFunds()
            .accounts({
                freelancer: freelancer.publicKey,
                projectInfoAccount,
                tokenEscrow
            })
            .signers([freelancer])
            .rpc()
        // project_info.milestone_funds_withdrawn
        // amount of funds
        const newFreelancerBalance = await provider.connection.getBalance(freelancer.publicKey);
        expect(newFreelancerBalance).to.equal(LAMPORTS_PER_SOL * 10_000);
    })

    it("marks the remaining milestone as completed", async () => {
        let projectData = await program.account.projectInfo.fetch(projectInfoAccount)
        expect(projectData.milestonesReached).to.equal(2);

        let multisigData = await program.account.multisig.fetch(multisig)
        for (let i = 0; i < 2; i++) {
            const accounts = [
                {
                    pubkey: projectInfoAccount,
                    isWritable: true,
                    isSigner: false
                },
                {
                    pubkey: multisig,
                    isWritable: false,
                    isSigner: true
                }
            ];
            const data = program.coder.instruction.encode("mark_current_milestone_completed", {})

            const multisigTxPda = await getMultisigTransactionPda(program as anchor.Program, multisig, multisigData.seqno);

            await program.methods.
                createTransaction(program.programId, accounts, data)
                .accounts({
                    multisig,
                    projectInfoAccount,
                    proposer: client.publicKey,
                    transaction: multisigTxPda.key
                })
                .signers([client])
                .rpc()

            // approve -- only needs two signers to approve and execute
            await program.methods
                .approve()
                .accounts({
                    multisig,
                    transaction: multisigTxPda.key,
                    owner: freelancer.publicKey,
                    multisigSigner: multisig,
                    projectInfoAccount
                })
                .remainingAccounts(accounts.map(
                    account => account.pubkey.equals(multisig) ?
                        { ...account, isSigner: false } : account
                )
                    .concat({
                        pubkey: program.programId,
                        isWritable: false,
                        isSigner: false,
                    }))

                .signers([freelancer])
                .rpc()
                .catch(e => console.log(e));
        }

        projectData = await program.account.projectInfo.fetch(projectInfoAccount)
        expect(projectData.milestonesReached).to.equal(4);
    })

    it("freelancer withdraws the remaining funds", async () => {
        // expect(multisigData.seqno).to.equal(0);
        const freelancerBalance = await provider.connection.getBalance(freelancer.publicKey);
        expect(freelancerBalance).to.equal(LAMPORTS_PER_SOL * 10_000);

        await program.methods
            .withdrawMilestoneFunds()
            .accounts({
                freelancer: freelancer.publicKey,
                projectInfoAccount,
                tokenEscrow
            })
            .signers([freelancer])
            .rpc()
        // project_info.milestone_funds_withdrawn
        // amount of funds
        const newFreelancerBalance = await provider.connection.getBalance(freelancer.publicKey);
        expect(newFreelancerBalance).to.equal(LAMPORTS_PER_SOL * 20_000);
   
    })

    it('completes the project', async () => {
        const projectData = await program.account.projectInfo.fetch(projectInfoAccount)
        expect(projectData.status).to.equal(2);
    })
});
