# Weights

In Substrate, each transaction have a cost, depending on the ressources used by an extrinsic (the duration of execution, the number of storage access, the amount of data processed...). This price usually correspond to what is called the *weight of the extrinsic*. This is paid *before* the transaction, meaning we need a way to evaluate the weight of the extrinsic before the execution.

There is multiple ways to do that, we'll cover them all in this tutorial.

## Arbitrary weights

This is by far, the simplest way to give a weight to an extrinsic. The developper arbitrary give a weight to an extrinsic.

So simply do it ! You can give any value, depending of what you want the weight of this extrinsic to be, as the developper, but for this time, try to give the extrinsic `extrinsic1()` a weight of 10000 + 1 read.

You can find how to do this by reading [this documentation](https://docs.substrate.io/build/tx-weights-fees/) (look into the *Default weight annotations* chapter)

## Benchmarks

Benchmarks become usefull when you want your extrinsics' weight to depends on the duration of execution, the number of storage access, the amount of data processed...
Benchmarks gives us a way to simulate the execution of an extrinsic, and output precise weights for them, based on computational ressources.

In this part, we will learn how to make benchmarks, execute them, and link them to an extrinsic.
We already wrote benchmarks for `extrinsic2()`, so your job will be to write them for `extrinsic3()` and `extrinsic4()` (this one have a dynamic weight, that depends of the length of the vector) :)

To simulate the execution, Substrate need a node. So, a mockup runtime will be provided, with the pallet already integrated.

### 1 - Writing benchmarks 

The first part, is, obviously, to write benchmarks. For this, edit the `ex06-benchmarks/src/pallets/funglible_token/src/benchmarking.rs` file. Take example on the first benchmark function we did for you :)

### 2 - executing the benchmarks

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

### 3 - Integrating the new weights to the benchmark

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

## Conditional weights

Remember how you can simply give a value in the parenthesis of the weight of an extrinsic ? These parenthesis are not for parameters, but actual scope.
Meaning, you can put a condition instead, using the parameters of the extrinsics, making it possible to arbitrary decide if you want to use a weight or another, depending on your parameters.

We'll do the weights for the last two function, in two different way, so you'll see how far we can go:

* for ``extrinsic5``, put a weight of 100 000 if the ``hash`` parameter is true, and 10 000 if it is false.

* for ``extrinsic6``, we'll do things a little more complicated. Write *two* benchmark function for this extrinsic, one making the benchmark with the ``hash`` parameter true, and the other with false. This will generate two weights for your function. Then, in your extrinsic's weight, put a condition, choosing the corresponding benchmark with the corresponding value of ``hash``

# some links

* Transactions, weight and fees: https://docs.substrate.io/build/tx-weights-fees/
* Benchmarks: https://docs.substrate.io/test/benchmark/
* An exemple of a benchmark file (supersig): https://github.com/kabocha-network/pallet_supersig/blob/master/src/benchmarking.rs
* Node template with supersig pallet: 
