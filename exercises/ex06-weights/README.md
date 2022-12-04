# Weights

With Substrate, like with Ethereum, you have to pay to execute code. But unlike Ethereum, the cost of a call is not directly linked to the amount of instruction executed. In Substrate, it is up to the developer to fix the weight for each extrinsic. It can be an arbitrary value (like 0 or 42000000), or it can depend on the size of the extrinsic payload, or the value of some variable passed to the extrinsic. The cost of the call is called the *weight* of the call.
This price is paid *before* the transaction, meaning we need a way to evaluate the weight of the extrinsic before the execution.

There are multiple ways to do that, we'll cover them all in this tutorial.

## Part 1 - Arbitrary weights

This is by far, the simplest way to give a weight to an extrinsic. The developer arbitrarily gives a weight to an extrinsic.

So simply do it! You can give any value, depending on what you want the weight of this extrinsic to be, as the developer, but for this time, try to give the extrinsic `verify_address()` a weight of 10000 + 1 read.

You can find how to do this by reading [this documentation](https://docs.substrate.io/build/tx-weights-fees/) (look into the *Default weight annotations* chapter)

## Part 2 - Benchmarks

Benchmarks become useful when you want your extrinsics' weight to depend on the duration of execution, the number of storage access, and the amount of data processed...
Benchmarks give us a way to simulate the execution of an extrinsic, and output precise weights for them, based on computational resources.

In this part, we will learn how to make benchmarks, execute them, and link them to an extrinsic.
Your job will be to write them for `duplicate_and_store()`. We placed some good tips in the code.

To simulate the execution, Substrate needs a node.

So, the first step will be to integrate your pallet with the substrate-node-template, at the root of the repository.

### 1 - Writing benchmarks 

The first part, is, obviously, to write benchmarks. For this, edit the `ex06-weights/src/pallets/funglible_token/src/benchmarking.rs` file. Take the example of the first benchmark function we did for you :)

### 2 - executing the benchmarks

The first step of executing our benchmark is to move the pallet into the node template, and *integrate the pallet into the runtime*.

hint: to fill the `WeightInfo` parameter, look how they did for the pallet_balance :)

Then, once the integration is done, run this command to compile the runtime in benchmark mode:

```sh
cargo build --release --features runtime-benchmarks
```

This will compile our runtime, in release form (we want our benchmark to be as close to production as possible)
Then, we can start the actual benchmarks

```sh
./target/release/node-template benchmark pallet \
    --chain dev \
    --execution=wasm \
    --wasm-execution=compiled \
    --pallet "weights" \
    --extrinsic "*" \
    --steps 50 \
    --repeat 20 \
    --output pallets/weights/src/weights.rs
```

And you're all done :) You're newly generated weights are in the `pallets/weights/src/weights.rs` file. Move it into the exercice!

/!\ Your weight should match those in the `ex06-weights/src/solution_weights.rs` file. Benchmark depends on the machine running them, so plain values will be different, but the *structure* of the weight should correspond (the number of reads, writes, the variables...).

### 3 - Integrating the new weights into the benchmark

Now that you have your `weight.rs` file, you can now integrate weights into your pallet.

This is done in 2 steps:

* Add a Weight parameter to the config. You need to add the parameter type (usually `WeightInfo`) to your pallet's config and add the parameter value to the runtime. If you need help, take a look at how it's done with the *supersig* pallet (links below)

* Once it's done, you can now add weights to your extrinsic. To do this, you have to put this on top of each of your extrinsic:

```rust
#[pallet::weight(T::WeightInfo::/* your extrinsic name */(/* weights parameters */))]
```

We placed some helpful comments in the code 😉.

You will succeed once every test passes :).
Launch the tests by running:

```sh
$ cargo test
```

## Part 3 - Conditional weights

Instead of a unique value, you can pass some code into the weight macro. In this code, you have access to the variables of the extrinsic related to this weight.
Meaning, you can put a condition instead of a value, making it possible to arbitrarily decide if you want to use a weight or another, depending on your parameters.
frame_support define the weight macro as: `#[pallet::weight($ExpressionResultingInWeight)]`

We'll do the weights for the last two functions, in two different ways, so you'll see how far we can go:

* A - for ``store_maybe_hashed``, put a weight of 100 000 if the ``hash`` parameter is true, and 10 000 if it is false.

* B - for ``benchmarked_store_maybe_hashed``, we'll do things a little more complicated. Write *two* benchmark functions for this extrinsic, one making the benchmark with the ``hash`` parameter true, and the other with false. This will generate two weights for your function. Then, in your extrinsic's weight, put a condition, choosing the corresponding benchmark with the corresponding value of ``hash``

# Some links

* Transactions, weight, and fees: https://docs.substrate.io/build/tx-weights-fees/
* Benchmarks: https://docs.substrate.io/test/benchmark/
* An example of a benchmark file (supersig): https://github.com/kabocha-network/pallet_supersig/blob/master/src/benchmarking.rs
* Node template with supersig pallet: https://github.com/decentration/substrate-supersig-template

