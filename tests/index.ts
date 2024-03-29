import { expect } from "chai";
import { Context } from "./ctx";
import config from "../config";
import { Keypair } from "@solana/web3.js";

const ctx = new Context();

before(async () => {
  await ctx.setup();
  
});

describe("clusters", () => {
  it("Initialize", async () => {
    await ctx.program.methods.initialize().rpc();
  });

  it("Ping", async () => {
    let listener: number;
    const event = await new Promise((resolve, _) => {
      listener = ctx.program.addEventListener("Pong", (event, _) => {
        resolve(event);
      });
      ctx.program.methods.ping().rpc();
    });
    await ctx.program.removeEventListener(listener);
    // @ts-ignore
    expect(event.n).to.eql(config.n);
  });
  
  it("DifferentState", async () => {
    const myAcc = new Keypair();
    
    await ctx.program.methods.differentState().accounts({
      myAcc: myAcc.publicKey,
      payer: ctx.payer.publicKey,
    }).signers([myAcc, ctx.payer]).rpc();
  });
});
