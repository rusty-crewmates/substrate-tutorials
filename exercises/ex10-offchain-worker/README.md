# Offchain workers 

Substrate offers the possibility for your node to run `offchain workers`. Those are a specific kind of hooks that can easely read the runtime storage, while being able to communicate with the rest of the internet (through `http` calls).  
They are automaticaly triggered by the node when it add a block to it's chain.

In this exercise we are going to build a pallet that can act as an onchain Bitcoin price oracle.  
The offchain worker will query the Coinbase API in order to get the current price of BTC in USD. Then submit an unsigned transaction to the pool in order to set the price in the pallet storage.

## What to do?
- fill the `set_btc_price` call
- impl the `offchain_worker` hook
- impl the `validate_unsigned` method
- write the `fetch_btc_price` and `fetch_btc_price_and_send_unsigned_transaction` functions

With all this together our oracle pallet should work, and the tests should pass.

## Some links
Here is some documentation about the offchain workers and they different types
- https://docs.substrate.io/reference/how-to-guides/offchain-workers/
The substrate crate contains an example of the same usage of an offchain worker. It is really close to what you will have to do here.
- https://github.com/paritytech/substrate/blob/polkadot-v0.9.28/frame/examples/offchain-worker


## What next?
If you want to push things further, experiment with other types of transactions: signed and unsigned but containing a signature.
With those three variations you should be ready for any configuration you may face.
