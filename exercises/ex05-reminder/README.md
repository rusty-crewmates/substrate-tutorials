# Hooks

Substrate offers a way to automatically execute code on some events (when a block is being created or finalized, when there is a runtime upgrade...) allowing to add more specific logic to the chain.

## What to do?

The aim of this exercice is to schedule an event to be sent at a specific blocknumber, and notify (by another event) how many scheduled events have been processed every blocks.
For this, you will have a storage map that contains a list of event as value, and a blocknumber as a key.
You will also have a storage that count how many event have been processed, and an extrinsic to schedule events.

the aim is to use the `on_initialize` hook to first reset the counter, execute events, and increase the new counter,
and the `on_finalize` hook emit the events

on_initialize also return the weight used in the hook. We placed a first occurence so let you see how it works.
(tip: on_initialize can be done in one read and two writes !)

We placed some helpful comments in the code 😉.

You will succeed once every tests passes :).
Launch the tests by running:

```sh
$ cargo test
```

## some links

* Transaction lifecycle: https://docs.substrate.io/fundamentals/transaction-lifecycle/
* Hooks technical documentation: https://paritytech.github.io/substrate/master/frame_support/traits/trait.Hooks.html#method.on_idle

## What to focus on

Storage and extrinsics are already completed, you only need to focus on the hooks logic.
