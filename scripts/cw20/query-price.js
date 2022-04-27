import { client, wallets } from '../library.js';

import {
  MsgExecuteContract,
  MnemonicKey,
  Coins,
  LCDClient,
} from "@terra-money/terra.js";

const price_contract = "terraOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO";

const response = await client.wasm.contractQuery(price_contract, { balance: { address: walletAddress } });

console.log(response);

