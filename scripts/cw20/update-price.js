import { client, wallets } from "../library.js";

import { LCDClient, MnemonicKey, MsgExecuteContract } from "@terra-money/terra.js";

// update price feature

const wallet = wallets.wallet_testnetyk;

// ANC on bombay-12
const contract_address = "terra1a0ym2ml0p95w33hw2tqwql3e06k59gg8eqks89";

// Transfer 1 ANC.
const cw20Send = new MsgExecuteContract(
    wallet.key.accAddress, contract_address, { // 
    update_price: {
    price: 99,
  },
});

const tx = await wallet.createAndSignTx({ msgs: [cw20Send] });
const result = await client.tx.broadcast(tx);

console.log(result);
