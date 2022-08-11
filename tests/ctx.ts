import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
import { Keypair } from "@solana/web3.js";
import { Clusters } from "../target/types/clusters";

export class Context {
  provider: anchor.AnchorProvider;

  program: Program<Clusters>;
  
  payer: Keypair;

  constructor() {
    this.provider = anchor.AnchorProvider.env();
    anchor.setProvider(this.provider);
    this.program = anchor.workspace.Clusters;
    this.payer = (this.provider.wallet as NodeWallet).payer;
  }
  
  async setup() {
    
  await this.provider.connection.confirmTransaction(
    await this.provider.connection.requestAirdrop(
      this.payer.publicKey,
      200_000_000
    )
  );  }
}
