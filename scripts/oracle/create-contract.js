import { client, wallets } from "../library.js";

import { MsgInstantiateContract } from '@terra-money/terra.js';

const wallet = wallets.wallet_testnetyk;

const code_id = ['68112']

const instantiate = new MsgInstantiateContract(
  wallet.key.accAddress,
  code_id[0], // code ID
  {
    //count: 0,
  }, // InitMsg
  { uluna: 1000000, uusd: 1000000 }, // init coins
  false // migratable
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