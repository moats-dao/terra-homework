import { client, wallets } from '../library.js';

import {
  MsgExecuteContract,
  MnemonicKey,
  Coins,
  LCDClient,
} from "@terra-money/terra.js";

const wallet = wallets.wallet_testnetyk;

const price_contract = "terra15wwpcv7ze99jk5vwru7ntjf9a77ydljuw727c5";

const response = await client.wasm.contractQuery(price_contract, { query_price: {} });

console.log(response);

