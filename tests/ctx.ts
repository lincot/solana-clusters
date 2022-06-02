import * as anchor from "@project-serum/anchor";
import * as process from "process";
import { Program } from "@project-serum/anchor";
import { Connection } from "@solana/web3.js";
import { Clusters } from "../target/types/clusters";

export class Context {
  connection: Connection;

  program: Program<Clusters>;

  cluster: string;

  constructor() {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    this.connection = provider.connection;
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
