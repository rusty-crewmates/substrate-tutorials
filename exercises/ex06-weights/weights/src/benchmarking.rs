use super::*;
// got the "file not included in module tree" warning ?
// look at the comment in Cargo.toml (in the feature section), and the one at the top of the lib.rs
// file

use crate::Pallet as Weights;
use frame_benchmarking::benchmarks;
use frame_system::RawOrigin;

benchmarks! {
	/////////////////////// Part 2 - benchmarks ///////////////////////

	//TODO: change this generic benchmark to benchmark the duplicate_and_store extrinsic
	benchmark_name {
		//this variable is a range, meaning the benchmark will be run with the different values of
		//s, to evaluate the weight of this specific parameter
		let s in 0 .. 1;
		todo("change this range to something that makes sense for your benchmark");

		let alice: T::AccountId = todo!("Look at the bottom of this file...
										Try to use the helper function to get an account");

		//we got the address of alice, but the account still doesnt exist, as it does not possess
		//any fund ! try to add some
		//tip: we integrated pallet-balances in this pallet, so... maybe you can use it ?
		//hint: https://paritytech.github.io/substrate/master/frame_support/traits/tokens/currency/trait.Currency.html


	}: extrinsic_name(RawOrigin::Signed(alice), s)
	verify {
		// Run some verifications here.
		// If something isn't right, the benchmark will throw an error
		assert_eq!(1, 0);
	}

	/////////////////////// Part 3.A - conditional benchmarks ///////////////////////
	store_maybe_hashed_true {
		//TODO: prepare the datas for this benchmark (the account, the data, and the hash)
		let alice: todo!();
		let data = todo!();
		let hash = todo!();
	}: store_maybe_hashed(RawOrigin::Signed(alice), data, hash)
	verify {
		//TODO: do some verification that your extrinsic did what it was supposed to do
	}

	store_maybe_hashed_false {
		//TODO: prepare the datas for this benchmark (the account, the data, and the hash)
		let alice: todo!();
		let data = todo!();
		let hash = todo!();
	}: store_maybe_hashed(RawOrigin::Signed(alice), data, hash)
	verify {
		//TODO: do some verification that your extrinsic did what it was supposed to do
	}

	impl_benchmark_test_suite!(Weights, crate::mock::new_test_ext(), crate::mock::Test);
}

//this utility function generate an accountId, based on a string. this is usefull to sign calls !
pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
	let account: T::AccountId = benchmark_account(name, 0, 0);
	account
}
