# Writing tests

A Substrate-node logic execution happens in the [runtime](https://docs.substrate.io/v3/concepts/runtime/), this logic is divided by a set of modules called [pallets](https://docs.substrate.io/v3/getting-started/glossary/#pallet). Substrate provide [FRAME](https://docs.substrate.io/v3/runtime/frame/) a framework to help you create new pallets compatible with Substrate. While oversimplifying a pallet can be thought as a [storage](https://docs.substrate.io/v3/runtime/storage/) and some dispatchable functions (or [extrinsics](https://docs.substrate.io/v3/concepts/extrinsics/)) that allow to interact with it.

To have a better understanding about the pallets architecture and how to use the dispatchable functions, you will have in this exercise to make some tests to check their behaviours.

## To do
In ```pallets/flipper/src/``` you will find a .lib that define the pallet configuration, storage and dispatchable functions. The mock file simulate the execution of the runtime, you don't need to update it, neither the benchmark file. All you have to do is to make 4 tests to check the behaviour of the ```set_value()``` and ```flip_value()``` functions.
1. Fill the ```set_value_ok()``` test to ensure the good behaviour of ```set_value()``` function.
2. Fill the ```set_value_err_already_set()``` test to verify if the function returns the expected error.
3. Fill the ```flip_value_ok()``` test to ensure the good behaviour of ```flip_value()``` function.
4. Make another test to check the behaviour in the case where an error occured in the ```flip_function()```.

> Don't forget to comment your code
## Some links
* Awesome Rusty : https://github.com/rusty-crewmates/awesome-rusty
* Pallet skeleton : https://docs.substrate.io/v3/runtime/frame/#skeleton-of-a-pallet
* Informations about the tests with Substrate : https://docs.substrate.io/v3/runtime/testing/
* Some macros you could need : https://docs.rs/frame-support/2.0.0-rc4/frame_support/#macros
* About Substrate's Origin : https://docs.substrate.io/v3/runtime/origins/
* Events and Errors : https://docs.substrate.io/v3/runtime/events-and-errors/

## Ensure everything is ok
`cargo check`  
`cargo test`
