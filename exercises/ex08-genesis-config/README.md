# Genesis config

You may want to initialize some storages values in the genesis state of your blockchain. An usecase could be to pre-mint some assets to some accounts. To do so, each pallet comes with an optional GenesisConfig functionality.

## What to do?

We copied the `nft` pallet from previous exercises and add some minor changes.
1. The body of the `mint` and `transfer` externals has been factorized in the `inner_mint` and `inner_transfer` functions. This way you will be able to call them during the `GenesisBuild` phase.
2. We declared a `GenesisConfig` struct and a `GenesisAssetList` type

You will have to use those functions and types in order to fill the body of the `GenesisBuild::build` method. It should create the initial assets and dispatch them to their rightfull owners.

## some links

* substrate `GenesisConfig` tutorial: https://docs.substrate.io/reference/how-to-guides/basics/configure-genesis-state/
