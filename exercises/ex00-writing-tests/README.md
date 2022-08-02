# Writing tests

A Substrate-node logic execution happens in the [runtime](https://docs.substrate.io/main-docs/fundamentals/runtime-intro/). This logic is divided by a set of modules called [pallets](https://docs.substrate.io/main-docs/fundamentals/runtime-intro/#composing-a-runtime-with-pallets). Substrate provides [FRAME](https://docs.substrate.io/main-docs/fundamentals/runtime-intro/#frame), a framework to help you create new pallets compatible with Substrate. While oversimplifying a pallet can be thought of as a [storage](https://docs.substrate.io/main-docs/fundamentals/state-transitions-and-storage/) and some dispatchable functions (or [extrinsics](https://docs.substrate.io/main-docs/fundamentals/transaction-types/)) that allow interacting with it.

In future exercises, you will be writing pallets yourself, but in this one, the pallet is already coded, and all you have to do is test it by writing tests. It will be an excellent first contact with Substrate syntax and familiarize you with its testing suite.

## What is in this exercise?
We created a really simple pallet, `flipper`, that you will have to test.
This pallet allows the users to store a boolean in the storage, calling the `set_value` extrinsic, and then flip its value by calling the `flip_value` extrinsic.

## To do
In ```ex00-writing-tests/flipper/src/``` you will find a ```.lib``` file that defines the pallet configuration, storage, and dispatchable functions. 
In ```ex00-writing-tests/flipper/src/tests/``` you will find the ```mock.rs``` file, which simulates the runtime execution. Your test will run against this specific implementation of your pallet. You don't need to update it. All you have to do is write the missing code in the ```flipper.rs``` file. The goal is to check the correct behavior of the ```set_value()``` and ```flip_value()``` functions.
1. Fill the ```set_value_ok()``` test to ensure ```set_value()``` is storing the value passed as an argument.
2. Fill the ```set_value_err_already_set()``` test to ensure that this extrinsic cannot be called twice successfully.
3. Fill the ```flip_value_ok()``` test to ensure calls to ```flip_value()``` lead to the boolean value being inverted.
4. Read ```flip_function()``` code and imagine a scenario that will lead to an error when calling it

## Some links
* Awesome Rusty: https://github.com/rusty-crewmates/awesome-rusty
* Pallet skeleton: https://docs.substrate.io/tutorials/work-with-pallets/custom-pallet/
* Information about the tests with Substrate: https://docs.substrate.io/main-docs/test/
* Some macros you could need: https://docs.rs/frame-support/latest/frame_support/index.html#macros
* About Substrate's Origin: https://docs.substrate.io/main-docs/build/origins/
* Events and Errors: https://docs.substrate.io/main-docs/build/events-errors/

## Ensure everything is ok
`cargo check`  
`cargo test`
