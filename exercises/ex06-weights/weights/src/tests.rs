use crate as pallet_weights;
use crate::{mock::*, Error};
use frame_support::{
	assert_noop, assert_ok,
	traits::Get
};
use frame_support::weights::{GetDispatchInfo, PostDispatchInfo, RuntimeDbWeight};

// use sp_weights::RuntimeDbWeight;
#[test]
fn verify_address_test() {
	new_test_ext().execute_with(|| {
		let db_weights: RuntimeDbWeight = <Test as frame_system::Config>::DbWeight::get();
		let weight = pallet_weights::Call::<Test>::verify_address {  }.get_dispatch_info().weight;
		assert_eq!(weight, 10_000 + db_weights.reads(1));
	});
}

#[test]
fn duplicate_test() {
	new_test_ext().execute_with(|| {
		let db_weights: RuntimeDbWeight = <Test as frame_system::Config>::DbWeight::get();
		let weight1 = pallet_weights::Call::<Test>::duplicate_and_store { elem: 0, count: 1 }.get_dispatch_info().weight;
		let weight2 = pallet_weights::Call::<Test>::duplicate_and_store { elem: 0, count: 1000 }.get_dispatch_info().weight;

		assert!(weight1 < weight2);
		assert!(weight1 > db_weights.writes(1));

	});
}

#[test]
fn store_maybe_hashed_test() {
	new_test_ext().execute_with(|| {
		let weight1 = pallet_weights::Call::<Test>::store_maybe_hashed { data: vec!{1, 2, 3}, hash: true }.get_dispatch_info().weight;
		let weight2 = pallet_weights::Call::<Test>::store_maybe_hashed { data: vec!{1, 2, 3}, hash: false }.get_dispatch_info().weight;

		assert_eq!(weight1, 100_000);
		assert_eq!(weight2, 10_000);

	});
}

#[test]
fn benchmarked_store_maybe_hashed_test() {
	new_test_ext().execute_with(|| {

		let long_vec = vec![1; 100000];
		let weight1 = pallet_weights::Call::<Test>::store_maybe_hashed { data: long_vec.clone(), hash: true }.get_dispatch_info().weight;
		let weight2 = pallet_weights::Call::<Test>::store_maybe_hashed { data: long_vec, hash: false }.get_dispatch_info().weight;

		assert!(weight1 > 100_000);
		assert!(weight2 > 10_000);
		assert!(weight1 > weight2);

	});
}
