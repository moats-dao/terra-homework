import { client, wallets } from "../library.js";

import { LCDClient, MnemonicKey, MsgExecuteContract } from "@terra-money/terra.js";

// transfer feature

const contract_owner = wallets.wallet_testnetyk;

// ANC on bombay-12
const price_contract = "terra15wwpcv7ze99jk5vwru7ntjf9a77ydljuw727c5";

// Transfer 1 ANC.
const update_price = new MsgExecuteContract(
    contract_owner.key.accAddress, price_contract, {
        update_price: {
            price: 99,
        },
    });

const tx = await wallet.createAndSignTx({ msgs: [update_price] });
const result = await client.tx.broadcast(tx);

console.log(result);
