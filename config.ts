// DO NOT EDIT directly. Use `scripts/changeCluster.ts CLUSTER`
const cluster: string = "localnet";

class Config {
  n: number;
}

// default configuration
let config: Config = { n: 0 };

// cluster-specific configuration
switch (cluster) {
  case "devnet":
    config.n = 1;
    break;
  case "localnet":
    config.n = 2;
    break;
  case "mainnet":
    config.n = 3;
    break;
  case "testnet":
    config.n = 4;
    break;
}

export default config;
