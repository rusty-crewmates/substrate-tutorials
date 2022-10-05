# Benchmarks

In Substrate, each transaction have a cost, depending on the ressources used by an extrinsic (the duration of execution, the number of storage access...). This is called the *weight of the extrinsic*. This cost is paid *before* the transaction, meaning we need a way to evaluate the weight of the extrinsic before the execution.

This is where the benchmarks comes in. Remember how you need to put a weight value on top of every extrinsics ? Benchmarks gives us a way to simulate the execution of an extrinsic, and output precise weights for each extrinsics.

In this tutorial, we will learn how to make benchmarks, execute them, and link them to an extrinsic. We will write benchmarks for the pallet we built on `exercice 01 - fungible token` :)

To simulate the execution, Substrate need a node. So, a mockup runtime will be provided, with the pallet already integrated.

## What to do?

<!-- This tutorial is in 3 parts. You will work in `ex06-benchmarks/src/pallets/funglible_token` folder for the two firs. -->

# 1 - Writing benchmarks 

The first part, is, obviously, to write benchmarks. For this, edit the `ex06-benchmarks/src/pallets/funglible_token/src/benchmarking.rs` file. We already did one of the five benchmark function to inspire you, and placed some helpful comments in the code.

# 2 - executing the benchmarks

The first step of executing our benchmark is to move into the runtime folder `ex06-benchmarks/src`
Then, run this command:

```sh
cargo build --release --features runtime-benchmarks
```

This will compile our runtime, in release form (we want our benchmark to be as close as production as possible)
Then, we can start the actual benchmarks

```sh
./target/release/node-template benchmark pallet \
    --chain dev \
    --execution=wasm \
    --wasm-execution=compiled \
    --pallet "fungible-token" \
    --extrinsic "*" \
    --steps 50 \
    --repeat 20 \
    --output pallets/fungible-token/src/weights.rs
```

And you're all done :) You're newly generated weights are in the `pallets/fungible-token/src/weights.rs` file.

/!\ Your weight should match those in the `pallets/fungible-token/src/solution_weights.rs` file. Benchmark depends on the machine running them, so plain values will be different, but the *structure* of the weight should correspond (the number of reads, writes, the variables...).

# 3 - Integrating the new weights to the benchmark

Now that you have your `weight.rs` file, you can now integrate weights to your pallet.

This is done in 2 steps:

* Add a Weight parameter to config. You need to add the parameter type (usually `WeightInfo`) to your pallet's config, and add the parameter value to the runtime. If you need help, take a look at how it's done with the *supersig* pallet (links below)

* Once it's done, you can now add weights to your extrinsics. To do this, you have to put this on top of each of your extrinsics:

```rust
#[pallet::weight(T::WeightInfo::/* your extrinsic name */(/* weights parameters */))]
```

We placed some helpful comments in the code 😉.

You will succeed once every tests passes :).
Launch the tests by running:

```sh
$ cargo test
```

## some links

* Benchmarks: https://docs.substrate.io/test/benchmark/
* An exemple of a benchmark file (supersig): https://github.com/kabocha-network/pallet_supersig/blob/master/src/benchmarking.rs
* Node template with supersig pallet: https://github.com/decentration/substrate-supersig-template
