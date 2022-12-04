use crate::tests::mock::*;
use frame_support::{assert_noop, error::BadOrigin};

#[test]
fn ok() {
	ExtBuilder::default().balances(vec![(ALICE, 1000)]).build().execute_with(|| {
		let total_issuance = Balances::total_issuance();
		let alice_balance = Balances::free_balance(ALICE);

		let amount_to_slash = 500;

		Imbalances::slash(Origin::root(), amount_to_slash, ALICE).unwrap();

		let treasury_part = amount_to_slash / 3;
		let burned_part = amount_to_slash - treasury_part;

		assert_eq!(Balances::total_issuance(), total_issuance - burned_part);
		assert_eq!(
			Balances::free_balance(ALICE),
			alice_balance - amount_to_slash
		);
		assert_eq!(Balances::free_balance(TREASURY), treasury_part);
	})
}

#[test]
fn ok_not_enough_funds() {
	ExtBuilder::default().balances(vec![(ALICE, 1000)]).build().execute_with(|| {
		let total_issuance = Balances::total_issuance();
		let alice_balance = Balances::free_balance(ALICE);

		let amount_to_slash = 5000;

		Imbalances::slash(Origin::root(), amount_to_slash, ALICE).unwrap();

		let treasury_part = alice_balance / 3;
		let burned_part = alice_balance - treasury_part;

		assert_eq!(Balances::total_issuance(), total_issuance - burned_part);
		assert_eq!(Balances::free_balance(ALICE), 0);
		assert_eq!(Balances::free_balance(TREASURY), treasury_part);
	})
}

#[test]
fn must_be_signed() {
	ExtBuilder::default().balances(vec![(ALICE, 100)]).build().execute_with(|| {
		assert_noop!(Imbalances::slash(Origin::none(), 500, ALICE), BadOrigin);
	})
}

#[test]
fn must_be_root() {
	ExtBuilder::default().balances(vec![(ALICE, 100)]).build().execute_with(|| {
		assert_noop!(
			Imbalances::slash(Origin::signed(ALICE), 500, ALICE),
			BadOrigin
		);
	})
}
