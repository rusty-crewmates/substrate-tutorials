# Hooks

Substrate offers a way to automatically execute code on some events (when a block is being created or finalized, when there is a runtime upgrade...) allowing to add more specific logic to the chain.

## What to do?

The aim of this exercice is to allow a certain transaction to happen only on specifics blocks.
For this, you will have a storage that contains a bool, and a extrinsic that can only work if the bool is true.

You will have to make sure that the bool is only true if the block number is a a multiple of 2, and set to false at the end of each blocks, to ensure it will be false for the next block.

We placed some helpful comments in the code 😉.

You will succeed once every tests passes :).
Launch the tests by running:

```sh
$ cargo test
```

## some links

* Transaction lifecycle: https://docs.substrate.io/fundamentals/transaction-lifecycle/

## What to focus on


Storage and extrinsics are already completed, you only need to focus on the hooks logic.
