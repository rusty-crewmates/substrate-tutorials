# Mock

In order to run your tests you want to mock a runtime around your pallet. You don't want to mock your whole blockchain, it would be too much work, and defect the purpose of mocking.
What you want to do is to provide a really simple runtime, with only what is needed by your pallet. In other words, only the things your pallet depends on, which are solely defined in it's config.

## What to do?

In `lib.rs` uncomment the line that import the `tests` module. Try to run the tests, it will fail, because we removed the whole mock runtime.  
You job is to fill the `mock.rs` file with a mock runtime that will make it possible to compile and pass the tests.  
You are not supposed to modify the content of the tests at any point. 
