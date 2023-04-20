use crate::{tests::mock::*, Error, PriceOracle};
use frame_support::{assert_err, assert_noop, assert_ok, error::BadOrigin};

#[test]
fn ok() {
	new_test_ext().execute_with(|| {
		let balance_before = Balances::free_balance(ALICE);

		assert_ok!(PalletToMock::mint(Origin::signed(ALICE)));

		let balance_after = Balances::free_balance(ALICE);

		let price = <TestRuntime as crate::Config>::SomePriceOracle::get_price().unwrap();
		let value_to_mint = <TestRuntime as crate::Config>::ValueToMint::get();
		let amount_minted = value_to_mint / price;

		assert_eq!(balance_before + amount_minted, balance_after);
	})
}

#[test]
fn must_be_signed() {
	new_test_ext().execute_with(|| {
		assert_noop!(PalletToMock::mint(Origin::none()), BadOrigin);
	})
}

#[test]
fn callet_should_exist() {
	new_test_ext().execute_with(|| {
		assert_err!(
			PalletToMock::mint(Origin::signed(2)),
			Error::<TestRuntime>::CallerShouldExist
		);
	})
}
