#!/usr/bin/env ts-node

import * as fs from "fs";

function commentInFile(filename: string, start: number, end: number) {
  const contents = fs.readFileSync(filename, "utf-8");
  let lines = contents.split("\n");

  for (let i = start - 1; i < end; i++) {
    lines[i] = "// " + lines[i];
  }

  const res = lines.join("\n");

  fs.writeFileSync(filename, res);
}

const filename = process.argv[2];
const from = Number(process.argv[3]);
const to = Number(process.argv[4]);

commentInFile(filename, from, to);
