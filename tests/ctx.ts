import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Clusters } from "../target/types/clusters";

export class Context {
  program: Program<Clusters>;

  cluster: string;

  constructor() {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    this.program = anchor.workspace.Clusters;

    const url = process.env.ANCHOR_PROVIDER_URL;
    this.cluster = "localnet";
    for (let cluster of ["devnet", "testnet", "mainnet"]) {
      if (url.includes(cluster)) {
        this.cluster = cluster;
        break;
      }
    }
  }
}
