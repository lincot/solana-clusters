#!/usr/bin/env ts-node

import { promises as fsPromises } from "fs";

async function replaceInFile(
  filename: string,
  searchValue: string | RegExp,
  replacement: string
) {
  try {
    const contents = await fsPromises.readFile(filename, "utf-8");
    const replaced = contents.replace(searchValue, replacement);

    await fsPromises.writeFile(filename, replaced);
  } catch (err) {
    console.log(err);
  }
}

const cluster = process.argv[2];

if (!["devnet", "localnet", "mainnet", "testnet"].includes(cluster)) {
  console.log("must pass one of `devnet`, `localnet`, `mainnet`, `testnet`");
  process.exit(1);
}

console.log(`changing cluster to ${cluster}`);

replaceInFile(
  "programs/clusters/Cargo.toml",
  /default = \[.*\]/g,
  `default = ["${cluster}"]`
);
replaceInFile("Anchor.toml", /cluster = ".*"/g, `cluster = "${cluster}"`);
