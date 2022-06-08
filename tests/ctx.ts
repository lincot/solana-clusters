import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Clusters } from "../target/types/clusters";

enum Cluster {
  Devnet,
  Localnet,
  Mainnet,
  Testnet,
}

export class Context {
  program: Program<Clusters>;

  cluster: Cluster;

  n: number;

  constructor() {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    this.program = anchor.workspace.Clusters;

    const url = process.env.ANCHOR_PROVIDER_URL;

    if (url.includes("devnet")) {
      this.cluster = Cluster.Devnet;
    } else if (url.includes("mainnet")) {
      this.cluster = Cluster.Mainnet;
    } else if (url.includes("testnet")) {
      this.cluster = Cluster.Testnet;
    } else {
      this.cluster = Cluster.Localnet;
    }

    // default configuration
    this.n = 0;

    // cluster-specific configuration
    switch (this.cluster) {
      case Cluster.Devnet:
        this.n = 1;
        break;
      case Cluster.Localnet:
        this.n = 2;
        break;
      case Cluster.Mainnet:
        this.n = 3;
        break;
      case Cluster.Testnet:
        this.n = 4;
        break;
    }
  }
}
