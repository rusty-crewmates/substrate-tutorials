use super::mock::*;
use crate::Error;
use frame_support::{assert_noop, assert_ok};

#[test]
fn set_value_ok() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(FlipperModule::set_value(Origin::signed(1), true));
		// Read pallet storage and assert the value has been set as expected.
		assert_eq!(FlipperModule::value(), Some(true));
	});
}

#[test]
fn set_value_err_already_set() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(FlipperModule::set_value(Origin::signed(1), true));
		// Ensure the expected error is thrown when value is already present.
		assert_noop!(
			FlipperModule::set_value(Origin::signed(1), true),
			Error::<Test>::AlreadySet
		);
	});
}

#[test]
fn flip_value_ok() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(FlipperModule::set_value(Origin::signed(1), true));
		// Read pallet storage and assert the value has been set as expected.
		assert_eq!(FlipperModule::value(), Some(true));
		// Flip de value.
		assert_ok!(FlipperModule::flip_value(Origin::signed(1)));
		// Read pallet storage and assert the value flipped as expected.
		assert_eq!(FlipperModule::value(), Some(false));
	});
}

#[test]
fn flip_value_err_not_set() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			FlipperModule::flip_value(Origin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}
