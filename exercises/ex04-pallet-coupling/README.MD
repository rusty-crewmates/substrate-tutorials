# Pallet coupling

In this exercise, you are going to learn how to call functions between pallets. And to do so we are going to build a marketplace pallet, that will allow users to buy and sell NFTs.

## Loosely vs thightly coupled pallets

As you maybe already understood, pallets are generic over a certain runtime. They are supposed to be generic enough to be used by other blockchains, with entirely different runtimes. It's only during the compilation that the runtime will inject its specific types into the pallets to make them work all together.

It happens in runtime/src/lib.rs (in the substrate-node-template submodule, at the root of this repository). Here is an example:

```rust
parameter_types! {
	pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}
impl pallet_timestamp::Config for Runtime {
	type MinimumPeriod = MinimumPeriod;
	/// A timestamp: milliseconds since the Unix epoch.
	type Moment = u64;
	type OnTimestampSet = Aura;
	type WeightInfo = ();
}
```

`MinimumPeriod`, `Moment`, `OnTimestampSet`, and `WeightInfo` are types defined in the `pallet_timestamp` config:

```rust
	/// The pallet configuration trait
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Type used for expressing timestamp.
		type Moment: Parameter
			+ Default
			+ AtLeast32Bit
			+ Scale<Self::BlockNumber, Output = Self::Moment>
			+ Copy
			+ MaxEncodedLen
			+ scale_info::StaticTypeInfo;
		/// Something which can be notified when the timestamp is set. Set this to `()` if not
		/// needed.
		type OnTimestampSet: OnTimestampSet<Self::Moment>;
		/// The minimum period between blocks. Beware that this is different from the *expected*
		/// period that the block production apparatus provides. Your chosen consensus system will
		/// generally work with this to determine a sensible block time. e.g. For Aura, it will be
		/// double this period on default settings.
		#[pallet::constant]
		type MinimumPeriod: Get<Self::Moment>;
		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}
```

In this runtime, moment is set as a `u64` but it could have been any type that checks all the requirements described in the config.
And `OnTimestampSet` is set to `Aura` which is another pallet. The only requirement is that the type must implement the trait `OnTimestampSet<Self::Moment>`, and remember, through the magic of macro, pallets are types and can implement traits.

So this is the first way of making pallets interact with each other, by implementing a trait on one and injecting it in the config of the other at runtime. This way the second one will be able to call every function defined by the trait.

The second way to access other pallets is also through the config. Check this line:
```rust
	pub trait Config: frame_system::Config {
        ...
    }
```

Here we specify that this pallet requires frame_system::Config. By doing so every function defined by the pallet frame_system will be available for this pallet.

We can talk about loosely coupled and tightly coupled pallets.
The first is more flexible, allows reusability, and limits the access to the pallet through a trait acting as an interface to only expose some handpicked functions. It's also very convenient for mocking and testing.
The second gives full access and makes the second pallet unusable without the first one.
As a rule of thumb try to only tightly couple your pallets to Substrate core pallets, and only when the other solution doesn't really make sense. Try to prefer as much as possible loosely coupled pallets.

More on the subject: https://docs.substrate.io/main-docs/build/pallet-coupling/

# What to do?

We implemented the same functionality, a marketplace for our NFTs, the two ways:
- `pallet_loose_marketplace` must use the trait `Sellable` defined in `marketplace-nfts/src/types.rs` to interact with the `pallet_markeplace_nft`
- `pallet_tight_marketplace` should add `pallet_marketplace_nft` to its config to directly make calls to its storage

You have to do this, and write the calls to the `pallet_marketplace_nft` for each marketplace.
There is a few todo!() in the code preventing compilation. Start there and make the tests pass.
