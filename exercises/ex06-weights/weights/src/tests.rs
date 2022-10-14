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

// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
// 	});
// }
