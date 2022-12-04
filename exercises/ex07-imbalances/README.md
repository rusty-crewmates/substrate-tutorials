# Imbalances

# What are we talking about?

Remember how new bitcoins are only created as rewards to miners? That's one way to go, but there are a lot of other ways we can imagine to increase (and reduce) the total supply of a currency. Substrate allows you to do just that. You can code ways to create or destroy amounts in response to some on-chain events.
There is only one constraint: total_issuance of the currency should be strictly equal to the sum of the balances of all accounts. At least at the beginning and the end of each block.
The Substrate enforces this by making such operation return `Imbalances`:
- when you increase the `total_issusance`, you generate a `NegativeImbalance` that, if dropped, will reduce the `total_issusance`
- when you increase a user balance, you generate a `PositiveImbalance` that, if dropped, will increase the `total_issusance`

You can also merge two opposite imbalances together and effectively nullify them.

## What will you do?

We cooked you three increasingly complex extrinsics.
The first one, `mint`, will make you increase a user balance and the `total_issusance` at the same time, effectively spawning new tokens directly into the user account.
The second one, `slash`, introduce some imbalances arithmetics. You are going to remove tokens from a user balance, burn part of it, and give the rest to a special `Treasury` account.
In the third one, `sack`, you will take as many tokens as you can from a list of accounts, and give it all to another (lucky) account, except for a fixed amount that will go to the treasury for each account sacked.


## Some links

* The doc: https://docs.rs/frame-support/latest/frame_support/traits/trait.Imbalance.html and https://docs.rs/frame-support/latest/frame_support/traits/trait.Currency.html
* More explanations: https://blog.polymath.network/substrate-deep-dive-imbalances-8dfa89cc1d1

## What to focus on?
First, make sure to understand why we have to use such a mechanism, then have fun playing with the methods the `Imbalance` and `Currency` traits provide. There is no dark magic here, you will end up being imbalance-fluent in a blink of an eye.
