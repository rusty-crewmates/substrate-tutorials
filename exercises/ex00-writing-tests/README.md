# Writing tests

A Substrate-node logic execution happens in the [runtime](https://docs.substrate.io/v3/concepts/runtime/). This logic is divided by a set of modules called [pallets](https://docs.substrate.io/v3/getting-started/glossary/#pallet). Substrate provides [FRAME](https://docs.substrate.io/v3/runtime/frame/), a framework to help you create new pallets compatible with Substrate. While oversimplifying a pallet can be thought of as a [storage](https://docs.substrate.io/v3/runtime/storage/) and some dispatchable functions (or [extrinsics](https://docs.substrate.io/v3/concepts/extrinsics/)) that allow interacting with it.

To have a better understanding of a pallet's architecture and how to use its dispatchable functions, you will have in this exercise to complete some tests to check their behaviors.

## To do
In ```ex00-writing-tests/flipper/src/``` you will find a ```.lib``` file that defines the pallet configuration, storage, and dispatchable functions. 
In ```ex00-writing-tests/flipper/src/tests/``` you will find the ```mod.rs``` file which by convention is used as the contents of the ````mod tests;```` module declaration in ```../.lib```, and the ```mock.rs``` file which simulates the execution of the runtime. You don't need to update them. All you have to do is to write the missing code in 4 tests, in the ```flipper.rs``` file. The goal is to check the correct behavior of the ```set_value()``` and ```flip_value()``` functions.
1. Fill the ```set_value_ok()``` test to ensure the good behaviour of the ```set_value()``` function.
2. Fill the ```set_value_err_already_set()``` test to verify if the function returns the expected error.
3. Fill the ```flip_value_ok()``` test to ensure the good behaviour of the ```flip_value()``` function.
4. Make another test to check the behavior in the case where an error occurred in the ```flip_function()```.

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
