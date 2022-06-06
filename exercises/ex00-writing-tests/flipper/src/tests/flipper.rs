use super::mock::*;
use crate::Error;
use frame_support::{assert_noop, assert_ok};

#[test]
fn set_value_ok() {
	new_test_ext().execute_with(|| todo!("ensure the good behaviour of set_value() function."));
}

#[test]
fn set_value_err_already_set() {
	new_test_ext().execute_with(|| todo!("verify if the function returns the expected error."));
}

#[test]
fn flip_value_ok() {
	new_test_ext().execute_with(|| todo!("ensure the good behaviour of flip_value() function."));
}

todo!("Make another test to check the behaviour in the case where an error occured in the flip_function().");
