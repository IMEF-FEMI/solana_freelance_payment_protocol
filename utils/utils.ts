import * as anchor from "@project-serum/anchor";

export const getMultisigTransactionPda = async (
    program: anchor.Program,
    multisig: anchor.web3.PublicKey,
    seqno: number
): Promise<{ key: anchor.web3.PublicKey, bump: number }> => {

    const seqnoBn = new anchor.BN(seqno);
    const seqnoBuffer = seqnoBn.toBuffer('le', 4);

    let [key, bump] = await anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("multisig_transaction"), multisig.toBuffer(), seqnoBuffer], program.programId,
    );

    return {
        key,
        bump
    }
}
