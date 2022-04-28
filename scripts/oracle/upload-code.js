import * as path from "path";
import fs from "fs";

import { client, wallets } from "../library.js";

import { LCDClient, Wallet, MsgStoreCode, MnemonicKey, MsgInstantiateContract, isTxError } from "@terra-money/terra.js";

const wallet = wallets.wallet_testnetyk;

const storeCode = new MsgStoreCode(wallet.key.accAddress, fs.readFileSync("./artifacts/oracle.wasm").toString("base64"));

// update price feature

// ANC on bombay-12
const storeCodeTx = await wallet.createAndSignTx({
  msgs: [storeCode],
});
const storeCodeTxResult = await client.tx.broadcast(storeCodeTx);

console.log(storeCodeTxResult);

if (isTxError(storeCodeTxResult)) {
  throw new Error(
    `store code failed. code: ${storeCodeTxResult.code}, codespace: ${storeCodeTxResult.codespace}, raw_log: ${storeCodeTxResult.raw_log}`
  );
}
 
const {
  store_code: { code_id },
} = storeCodeTxResult.logs[0].eventsByType;

console.log(code_id);
let data = JSON.stringify(code_id[0]);
fs.writeFileSync('code_id.json', data);