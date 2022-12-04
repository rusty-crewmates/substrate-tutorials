use crate::{tests::mock::*, Error};
use frame_support::{assert_noop, error::BadOrigin};

#[test]
fn ok() {
	ExtBuilder::default()
		.balances(vec![(ALICE, 1000), (BOB, 1000), (CHARLY, 1000)])
		.build()
		.execute_with(|| {
			let total_issuance = Balances::total_issuance();
			let alice_balance = Balances::free_balance(ALICE);
			let bob_balance = Balances::free_balance(BOB);
			let charly_balance = Balances::free_balance(CHARLY);

			Imbalances::sack(Origin::root(), vec![ALICE, BOB], CHARLY).unwrap();

			assert_eq!(Balances::total_issuance(), total_issuance);
			assert_eq!(Balances::free_balance(ALICE), EXISTENTIAL_DEPOSIT);
			assert_eq!(Balances::free_balance(BOB), EXISTENTIAL_DEPOSIT);
			assert_eq!(
				Balances::free_balance(CHARLY),
				alice_balance + bob_balance + charly_balance
					- EXISTENTIAL_DEPOSIT * 2
					- TREASURY_FLAT_CUT * 2
			);
			assert_eq!(Balances::free_balance(TREASURY), TREASURY_FLAT_CUT * 2);
		})
}

#[test]
fn must_be_signed() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		assert_noop!(Imbalances::sack(Origin::none(), vec![], ALICE), BadOrigin);
	})
}

#[test]
fn must_be_root() {
	ExtBuilder::default().balances(vec![]).build().execute_with(|| {
		assert_noop!(
			Imbalances::sack(Origin::signed(ALICE), vec![], ALICE),
			BadOrigin
		);
	})
}

#[test]
fn recipient_must_exist() {
	ExtBuilder::default()
		.balances(vec![(ALICE, 1000), (BOB, 1000)])
		.build()
		.execute_with(|| {
			assert_noop!(
				Imbalances::sack(Origin::root(), vec![ALICE, BOB], CHARLY),
				Error::<TestRuntime>::AccountDoesNotExist
			);
		})
}
