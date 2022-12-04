use crate::{tests::mock::*, Error};
use frame_support::{assert_noop, error::BadOrigin};

#[test]
fn ok() {
	ExtBuilder::default().balances(vec![(ALICE, 100)]).build().execute_with(|| {
		let total_issuance = Balances::total_issuance();
		let alice_balance = Balances::free_balance(ALICE);

		let amount_to_mint = 500;

		Imbalances::mint_to(Origin::root(), amount_to_mint, ALICE).unwrap();

		assert_eq!(Balances::total_issuance(), total_issuance + amount_to_mint);
		assert_eq!(
			Balances::free_balance(ALICE),
			alice_balance + amount_to_mint
		);
	})
}

#[test]
fn must_be_signed() {
	ExtBuilder::default().balances(vec![(ALICE, 100)]).build().execute_with(|| {
		assert_noop!(Imbalances::mint_to(Origin::none(), 500, ALICE), BadOrigin);
	})
}

#[test]
fn must_be_root() {
	ExtBuilder::default().balances(vec![(ALICE, 100)]).build().execute_with(|| {
		assert_noop!(
			Imbalances::mint_to(Origin::signed(ALICE), 500, ALICE),
			BadOrigin
		);
	})
}

#[test]
fn recipent_must_exist() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			Imbalances::mint_to(Origin::root(), 500, ALICE),
			Error::<TestRuntime>::AccountDoesNotExist
		);
	})
}
