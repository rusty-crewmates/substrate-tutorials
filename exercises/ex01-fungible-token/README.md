# Fungible Token

In this tutorial you will build simple fungible asset pallet.

## What to do ?

Edit the code in `pallets/assets/src/lib.rs` until all tests passes.
We placed some helpfull comments in the code ;)

Lauch the tests by running:

```sh
$ cargo test
```

## What to focus on ?

While writing those extrinsic you will use the core features of a Substrate pallet: storages, events and errors.
Try to remember how each one of those elements is declared and configured, and then how it is used in the extrinsics.

The pallet you are writing allow users to create their own coins, like the Ethereum's ERC20. That's cool. If you had to implement it entierly by yourself, what would you change ? 
