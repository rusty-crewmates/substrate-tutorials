use super::*;
// got the "file not included in module tree" warning ?
// look at the comment in Cargo.toml (in the feature section), and the one at the top of the lib.rs
// file
//
use crate::Pallet as Weights;
use frame_benchmarking::{account as benchmark_account, benchmarks};
use frame_system::RawOrigin;

benchmarks! {
	/////////////////////// Part 2 - benchmarks ///////////////////////

	duplicate_and_store_benchmark {
		let caller = RawOrigin::Signed(get_account::<T>("caller"));
		let s in 0 .. 10000;
	}: duplicate_and_store(caller, 0, s)
	verify {
		assert!(VecDup::<T>::get().unwrap().len() == s as usize);
	}

	/////////////////////// Part 3.A - conditional benchmarks ///////////////////////
	store_maybe_hashed_true {
		let caller = RawOrigin::Signed(get_account::<T>("caller"));
		let data = vec![1; 100_000];
		let hash = true;
	}: benchmarked_store_maybe_hashed(caller, data, hash)
	verify {
	}

	store_maybe_hashed_false {
		let caller = RawOrigin::Signed(get_account::<T>("caller"));
		let data = vec![1; 100_000];
		let hash = false;
	}: benchmarked_store_maybe_hashed(caller, data, hash)
	verify {
	}

	impl_benchmark_test_suite!(Weights, crate::mock::new_test_ext(), crate::mock::Test);
}
