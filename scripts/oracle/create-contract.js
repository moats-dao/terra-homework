import { client, wallets } from "../library.js";

import { MsgInstantiateContract, isTxError } from '@terra-money/terra.js';

const wallet = wallets.wallet_testnetyk;

import fs from "fs";
let code_id_rawdata = fs.readFileSync('code_id.json');
let code_id = JSON.parse(code_id_rawdata);
console.log(code_id);

const instantiate = new MsgInstantiateContract(
  wallet.key.accAddress,
  wallet.key.accAddress,
  parseInt(code_id), // code ID
  { price: "123" }, // InitMsg
  //{ uluna: 1000000, uusd: 1000000 }, // init coins
  {} // init coins
);

const instantiateTx = await wallet.createAndSignTx({
  msgs: [instantiate],
});
const instantiateTxResult = await client.tx.broadcast(instantiateTx);

console.log(instantiateTxResult);

if (isTxError(instantiateTxResult)) {
  throw new Error(
    `instantiate failed. code: ${instantiateTxResult.code}, codespace: ${instantiateTxResult.codespace}, raw_log: ${instantiateTxResult.raw_log}`
  );
}

const {
  instantiate_contract: { contract_address },
} = instantiateTxResult.logs[0].eventsByType;

console.log(contract_address);
let data = JSON.stringify(contract_address);
fs.writeFileSync('contract_address.json', data);