import pprint as pp
import base64
import json, urllib
from urllib.request import Request

from terra_sdk.client.lcd.api.tx import CreateTxOptions
#from terra_sdk.client.localterra import LocalTerra
from terra_sdk.client.lcd import LCDClient
from terra_sdk.core.wasm import MsgStoreCode, MsgInstantiateContract, MsgExecuteContract
from terra_sdk.core.fee import Fee
from terra_sdk.key.mnemonic import MnemonicKey

url_request = Request("https://fcd.terra.dev/v1/txs/gas_prices", headers=    {"User-Agent": "Mozilla/5.0"})
live_gas_prices = json.loads(urllib.request.urlopen(url_request).read().decode())

pp.pprint(live_gas_prices)