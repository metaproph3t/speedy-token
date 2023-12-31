import * as anchor from "@coral-xyz/anchor";
const { PublicKey, Signer, Keypair, SystemProgram } = anchor.web3;
const { BN, Program } = anchor;

import { startAnchor, Clock } from "solana-bankrun";
import { BankrunProvider } from "anchor-bankrun";
import { Program } from "@coral-xyz/anchor";

import { assert } from "chai";

const SPEEDY_TOKEN_PROGRAM_ID = new PublicKey(
  "Hj1cWGvmqaTruSZ2vETEwGBQNtWaJrYMWNhWozeSB4BN"
);

import { SpeedyToken } from "../target/types/speedy_token";
import * as SpeedyTokenIDL from "../target/idl/speedy_token.json";
export type SpeedyTokenProgram = Program<SpeedyToken>;

describe("speedy-token", async function () {
  let context, banksClient, provider, program, payer;

  before(async function () {
    context = await startAnchor("./", [], []);
    banksClient = context.banksClient;
    provider = new BankrunProvider(context);
    anchor.setProvider(provider);

    program = new anchor.Program<SpeedyTokenProgram>(
      SpeedyTokenIDL,
      SPEEDY_TOKEN_PROGRAM_ID,
      provider
    );

    payer = program.provider.wallet.payer;
  });

  it("tests pass", async function () {
    let mintSlab = Keypair.generate();
    let tokenSlab = Keypair.generate();
    let mintAuthority = Keypair.generate();
    let alice = Keypair.generate();
    let bob = Keypair.generate();

    await program.methods.initializeMintSlab()
      .preInstructions([
        await program.account.mintSlab.createInstruction(mintSlab, 10_000_000),
      ])
      .accounts({
        slab: mintSlab.publicKey,
        payer: payer.publicKey,
      })
      .signers([mintSlab])
      .rpc();

    await program.methods.allocateMint(mintAuthority.publicKey, 0)
      .accounts({
        slab: mintSlab.publicKey,
      })
      .rpc();

    await program.methods.initializeTokenAccountSlab()
      .preInstructions([
        await program.account.tokenAccountSlab.createInstruction(tokenSlab, 10_000_000),
      ])
      .accounts({
        slab: tokenSlab.publicKey,
        payer: payer.publicKey,
      })
      .signers([tokenSlab])
      .rpc();

    await program.methods.allocateTokenAccount(alice.publicKey, 0, 0)
      .accounts({
        slab: tokenSlab.publicKey,
      })
      .rpc();

    await program.methods.allocateTokenAccount(bob.publicKey, 0, 1)
      .accounts({
        slab: tokenSlab.publicKey,
      })
      .rpc();

    await program.methods.mintTo(0, new BN(100))
      .accounts({
        tokenSlab: tokenSlab.publicKey,
        mintSlab: mintSlab.publicKey,
      })
      .rpc();

    //console.log(program.instruction.mintTo.accounts({
    //    tokenSlab: tokenSlab.publicKey,
    //    mintSlab: mintSlab.publicKey,
    //  }));

    const buf = Buffer.alloc(16);
    buf.writeUint32LE(0, 0);
    buf.writeUint32LE(1, 4);
    buf.writeBigUint64LE(10n, 8);
    //console.log(buf);

    // we'll just send a bunch of these, bcuz manually serializing structs is
    // boring :)
    const ix = anchor.web3.Ed25519Program.createInstructionWithPrivateKey({message: buf, privateKey: alice.secretKey});
    // CUs don't reflect cache locality / branch mispredicts either so this shouldn't
    // affect things

    const tx = await program.methods.transfer()
      .accounts({
        slab: tokenSlab.publicKey,
        instructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
      })
     // .remainingAccounts({
     //   pubkey: alice.publicKey,
     //   isWritable: false,
     //   isSigner: true,
     // })
      .preInstructions([ix, ix, ix, ix, ix, ix, ix])
     // .signers([alice])
      .transaction();

    [tx.recentBlockhash] = (await banksClient.getLatestBlockhash())!;
    tx.sign(payer);
   // console.log(tx);

    let result = await banksClient.simulateTransaction(tx);
    console.log("Compute units consumed", result.meta.computeUnitsConsumed);

    await banksClient.processTransaction(tx);

    const storedTokenSlab = await program.account.tokenAccountSlab.fetch(tokenSlab.publicKey);
    const storedAlice = storedTokenSlab.tokenAccounts[0];
    const storedBob = storedTokenSlab.tokenAccounts[1];

    assert.equal(storedAlice.nonce, 8);
    assert.ok(storedAlice.balance.eqn(30));
    assert.ok(storedBob.balance.eqn(70));
  });
});

