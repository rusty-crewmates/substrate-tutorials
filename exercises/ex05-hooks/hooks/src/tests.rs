use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

use frame_support::traits::{OnFinalize, OnInitialize};

#[test]
fn authorized() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		<Hooks as OnInitialize<u64>>::on_initialize(0);
		assert_ok!(Hooks::transfer_funds(Origin::signed(ALICE), BOB, 100_000));
	})
}

#[test]
fn unauthorized() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		<Hooks as OnFinalize<u64>>::on_finalize(0);
		<Hooks as OnInitialize<u64>>::on_initialize(1);
		assert_noop!(
			Hooks::transfer_funds(Origin::signed(ALICE), BOB, 100_000),
			Error::<TestRuntime>::Unauthorized
		);
	})
}

#[test]
fn correctly_set_back_to_false() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		<Hooks as OnInitialize<u64>>::on_initialize(0);
		<Hooks as OnFinalize<u64>>::on_finalize(0);
		<Hooks as OnInitialize<u64>>::on_initialize(1);
		assert_noop!(
			Hooks::transfer_funds(Origin::signed(ALICE), BOB, 100_000),
			Error::<TestRuntime>::Unauthorized
		);
	})
}

#[test]
fn set_false_on_finalize_and_not_on_initialize() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		<Hooks as OnInitialize<u64>>::on_initialize(0);
		<Hooks as OnFinalize<u64>>::on_finalize(0);
		assert_noop!(
			Hooks::transfer_funds(Origin::signed(ALICE), BOB, 100_000),
			Error::<TestRuntime>::Unauthorized
		);
	})
}
