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

    await program.methods.allocateTokenAccount(alice.publicKey, 0, new BN(0))
      .accounts({
        slab: tokenSlab.publicKey,
      })
      .rpc();
  });
});

