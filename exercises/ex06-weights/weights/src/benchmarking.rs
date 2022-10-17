#![cfg(feature = "runtime-benchmarks")]

use crate::Pallet;
use super::*;
use frame_benchmarking::{benchmarks, account as benchmark_account};
use frame_system::RawOrigin;
use sp_std::vec;
// use pallet_balances::Pallet as Balances;

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
	let account: T::AccountId = benchmark_account(name, 0, 0);
	account
}

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
}

