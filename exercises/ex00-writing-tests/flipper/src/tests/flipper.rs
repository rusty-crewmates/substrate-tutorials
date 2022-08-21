use super::mock::*;
use crate::Error;
use frame_support::{assert_noop, assert_ok};

#[test]
fn set_value_ok() {
	new_test_ext().execute_with(|| {
		assert_ok!(Flipper::set_value(Origin::signed(ALICE), false));
		assert_eq!(Flipper::value(), Some(false));
	});
}

#[test]
fn set_value_err_already_set() {
	new_test_ext().execute_with(|| {
		assert_ok!(Flipper::set_value(Origin::signed(1), true));
		assert_noop!(
			Flipper::set_value(Origin::signed(1), true),
			Error::<TestRuntime>::AlreadySet
		);
	});
}

#[test]
fn flip_value_ok() {
	new_test_ext().execute_with(|| {
		assert_ok!(Flipper::set_value(Origin::signed(1), true));
		assert_eq!(Flipper::value(), Some(true));
		assert_ok!(Flipper::flip_value(Origin::signed(1)));
		assert_eq!(Flipper::value(), Some(false));
	});
}

#[test]
fn flip_value_ko() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Flipper::flip_value(Origin::signed(1)),
			Error::<TestRuntime>::NoneValue
		);
	});
}
