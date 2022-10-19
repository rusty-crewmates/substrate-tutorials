use super::*;
// got the "file not included in module tree" warning ?
// look at the comment in Cargo.toml (in the feature section), and the one at the top of the lib.rs
// file
//
use frame_benchmarking::{account as benchmark_account, benchmarks};
use crate::Pallet as Weights;
use frame_system::RawOrigin;

benchmarks! {
	/////////////////////// Part 2 - benchmarks ///////////////////////

	//TODO: change this generic benchmark to benchmark the duplicate_and_store extrinsic
	benchmark_name {
		//this variable is a range, meaning the benchmark will be run with the different values of
		//s, to evaluate the weight of this specific parameter
		let s in 0 .. 1;
		todo("change this range to something that makes sense for your benchmark");

		let root = todo!("get the root origin, to sign our transactions");


		// Now that we have all the parameters we need for our extrinsic's benchmark, we can call
		// it:
	}: extrinsic_name(root, 0, s)
	verify {
		// Run some verifications here.
		// If something isn't right, the benchmark will throw an error and wont output values
		assert_eq!(1, 0);
	}

	/////////////////////// Part 3.A - conditional benchmarks ///////////////////////
	store_maybe_hashed_true {
		//TODO: prepare the datas for this benchmark (the account, the data, and the hash)
		let root = todo!("get the root origin, to sign our transactions");
		let data = todo!();
		let hash = todo!();
	}: store_maybe_hashed(root, data, hash)
	verify {
		//TODO: do some verification that your extrinsic did what it was supposed to do
	}

	store_maybe_hashed_false {
		//TODO: prepare the datas for this benchmark (the account, the data, and the hash)
		let root = todo!("get the root origin, to sign our transactions");
		let data = todo!();
		let hash = todo!();
	}: store_maybe_hashed(root, data, hash)
	verify {
		//TODO: do some verification that your extrinsic did what it was supposed to do
	}

	impl_benchmark_test_suite!(Weights, crate::mock::new_test_ext(), crate::mock::Test);
}

