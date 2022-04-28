import { client, wallets } from '../library.js';

import fs from "fs";

import {
  MsgExecuteContract,
  MnemonicKey,
  Coins,
  LCDClient,
} from "@terra-money/terra.js";

const wallet = wallets.wallet_testnetyk;

//const contract_address = "terra15wwpcv7ze99jk5vwru7ntjf9a77ydljuw727c5";
let contract_address_rawdata = fs.readFileSync('contract_address.json');
let contract_address = JSON.parse(contract_address_rawdata);
console.log(contract_address);

const response = await client.wasm.contractQuery(contract_address, { query_price: {} });

console.log(response);

