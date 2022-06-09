use crate::{tests::mock::*, Error};
use frame_support::{assert_noop, assert_ok, error::BadOrigin, BoundedVec};

mod create {
	use super::*;

	#[test]
	fn ok() {
		new_test_ext().execute_with(|| {
			// The execution went through without error.
			assert_ok!(Assets::create(Origin::signed(ALICE)));

			// The nonce was increased.
			assert!(Assets::nonce() == 1);

			// The asset id is 0;
			let details = Assets::asset(0).unwrap();

			// The sender is the owner of the asset.
			assert!(details.owner == ALICE);

			// The supply is still 0.
			assert!(details.supply == 0);
		})
	}

	#[test]
	fn must_be_signed() {
		new_test_ext().execute_with(|| {
			assert_noop!(Assets::create(Origin::none()), BadOrigin);
		})
	}
}

mod set_metadata {

	use super::*;

	#[test]
	fn ok() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::create(Origin::signed(ALICE)));

			let name: BoundedVec<u8, <TestRuntime as crate::pallet::Config>::MaxLength> =
				BoundedVec::try_from("TestAsset".as_bytes().to_vec()).unwrap();
			let symbol: BoundedVec<u8, <TestRuntime as crate::pallet::Config>::MaxLength> =
				BoundedVec::try_from("TASS".as_bytes().to_vec()).unwrap();

			assert_ok!(Assets::set_metadata(
				Origin::signed(ALICE),
				0,
				name.clone(),
				symbol.clone()
			));

			let metadata = Assets::metadata(0).unwrap();

			// Metadata has been set
			assert_eq!(metadata.name, name);
			assert_eq!(metadata.symbol, symbol);
		})
	}

	#[test]
	fn must_be_signed() {
		new_test_ext().execute_with(|| {
			let name: BoundedVec<u8, <TestRuntime as crate::pallet::Config>::MaxLength> =
				BoundedVec::try_from("TestAsset".as_bytes().to_vec()).unwrap();
			let symbol: BoundedVec<u8, <TestRuntime as crate::pallet::Config>::MaxLength> =
				BoundedVec::try_from("TASS".as_bytes().to_vec()).unwrap();
			assert_ok!(Assets::create(Origin::signed(ALICE)));
			assert_noop!(
				Assets::set_metadata(Origin::none(), 0, name, symbol),
				BadOrigin
			);
		})
	}

	#[test]
	fn must_exist() {
		new_test_ext().execute_with(|| {
			let name: BoundedVec<u8, <TestRuntime as crate::pallet::Config>::MaxLength> =
				BoundedVec::try_from("TestAsset".as_bytes().to_vec()).unwrap();
			let symbol: BoundedVec<u8, <TestRuntime as crate::pallet::Config>::MaxLength> =
				BoundedVec::try_from("TASS".as_bytes().to_vec()).unwrap();
			assert_noop!(
				Assets::set_metadata(Origin::signed(ALICE), 0, name, symbol,),
				Error::<TestRuntime>::Unknown
			);
		})
	}

	#[test]
	fn must_be_owner() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::create(Origin::signed(ALICE)));

			let name: BoundedVec<u8, <TestRuntime as crate::pallet::Config>::MaxLength> =
				BoundedVec::try_from("TestAsset".as_bytes().to_vec()).unwrap();
			let symbol: BoundedVec<u8, <TestRuntime as crate::pallet::Config>::MaxLength> =
				BoundedVec::try_from("TASS".as_bytes().to_vec()).unwrap();
			assert_noop!(
				Assets::set_metadata(Origin::signed(BOB), 0, name, symbol,),
				Error::<TestRuntime>::NoPermission
			);
		})
	}
}

mod mint {
	use super::*;

	#[test]
	fn ok() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::create(Origin::signed(ALICE)));

			let mut total_supply = Assets::asset(0).unwrap().supply;
			assert_eq!(total_supply, 0);
			// Initial owned amount is 0.
			assert_eq!(Assets::account(0, ALICE), 0);
			assert_eq!(Assets::account(0, BOB), 0);

			let amount = 100;

			// Can mint to itself.
			assert_ok!(Assets::mint(Origin::signed(ALICE), 0, amount, ALICE));
			total_supply += amount;
			// Total supply has been increased.
			assert_eq!(Assets::asset(0).unwrap().supply, total_supply);
			// User account has been credited.
			assert_eq!(Assets::account(0, ALICE), amount);

			// Can mint to somebody else.
			assert_ok!(Assets::mint(Origin::signed(ALICE), 0, amount, BOB));
			total_supply += amount;
			assert_eq!(Assets::asset(0).unwrap().supply, total_supply);
			assert_eq!(Assets::account(0, BOB), amount);
		})
	}

	#[test]
	fn ok_saturating() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::create(Origin::signed(ALICE)));

			let first_mint_amount = std::u128::MAX - 50;
			let second_mint_amount = 100;
			assert_ok!(Assets::mint(
				Origin::signed(ALICE),
				0,
				first_mint_amount,
				ALICE
			));
			assert_ok!(Assets::mint(
				Origin::signed(ALICE),
				0,
				second_mint_amount,
				BOB
			));

			assert_eq!(Assets::asset(0).unwrap().supply, std::u128::MAX);
			assert_eq!(Assets::account(0, BOB), 50);
		})
	}

	#[test]
	fn must_be_signed() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::create(Origin::signed(ALICE)));
			assert_noop!(Assets::mint(Origin::none(), 0, 100, BOB), BadOrigin);
		})
	}

	#[test]
	fn must_exist() {
		new_test_ext().execute_with(|| {
			assert_noop!(
				Assets::mint(Origin::signed(ALICE), 0, 100, BOB),
				Error::<TestRuntime>::Unknown
			);
		})
	}

	#[test]
	fn must_be_owner() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::create(Origin::signed(ALICE)));

			assert_noop!(
				Assets::mint(Origin::signed(BOB), 0, 100, BOB),
				Error::<TestRuntime>::NoPermission
			);
		})
	}
}

mod burn {
	use super::*;

	#[test]
	fn ok() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::create(Origin::signed(ALICE)));

			let mint_amount = 100;
			let burn_amount = 50;
			assert_ok!(Assets::mint(Origin::signed(ALICE), 0, mint_amount, BOB));

			let mut total_supply = Assets::asset(0).unwrap().supply;

			assert_ok!(Assets::burn(Origin::signed(BOB), 0, burn_amount));
			total_supply -= burn_amount;
			// Total supply and account have been reduced by burn_amount.
			assert_eq!(Assets::asset(0).unwrap().supply, total_supply);
			assert_eq!(Assets::account(0, BOB), mint_amount - burn_amount);
		})
	}

	#[test]
	fn ok_saturating() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::create(Origin::signed(ALICE)));

			let mint_amount = 100;
			let burn_amount = mint_amount + 1;
			assert_ok!(Assets::mint(Origin::signed(ALICE), 0, mint_amount, ALICE));
			assert_ok!(Assets::mint(Origin::signed(ALICE), 0, mint_amount, BOB));

			let total_supply = Assets::asset(0).unwrap().supply;

			assert_ok!(Assets::burn(Origin::signed(BOB), 0, burn_amount));
			// Total supply and account have been reduced by mint_amount.
			assert_eq!(Assets::asset(0).unwrap().supply, total_supply - mint_amount);
			assert_eq!(Assets::account(0, BOB), 0);
		})
	}

	#[test]
	fn must_be_signed() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::create(Origin::signed(ALICE)));
			assert_noop!(Assets::burn(Origin::none(), 0, 100), BadOrigin);
		})
	}

	#[test]
	fn must_exist() {
		new_test_ext().execute_with(|| {
			assert_noop!(
				Assets::burn(Origin::signed(ALICE), 0, 100),
				Error::<TestRuntime>::Unknown
			);
		})
	}
}

mod transfer {
	use super::*;

	#[test]
	fn ok() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::create(Origin::signed(ALICE)));

			let mint_amount = 100;
			let transfer_amount = 50;
			assert_ok!(Assets::mint(Origin::signed(ALICE), 0, mint_amount, BOB));

			let total_supply = Assets::asset(0).unwrap().supply;

			assert_ok!(Assets::transfer(
				Origin::signed(BOB),
				0,
				transfer_amount,
				ALICE
			));
			// Total supply is still the same.
			assert_eq!(Assets::asset(0).unwrap().supply, total_supply);
			// BOB's account has been reduced by transfer_amount.
			assert_eq!(Assets::account(0, BOB), mint_amount - transfer_amount);
			// Alice's account has been increased by transfer_amount.
			assert_eq!(Assets::account(0, ALICE), transfer_amount);
		})
	}

	#[test]
	fn ok_saturating() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::create(Origin::signed(ALICE)));

			let mint_amount = 100;
			let transfer_amount = mint_amount + 1;
			assert_ok!(Assets::mint(Origin::signed(ALICE), 0, mint_amount, BOB));

			assert_ok!(Assets::transfer(
				Origin::signed(BOB),
				0,
				transfer_amount,
				ALICE
			));
			// Account has only been modified by mint_amount.
			assert_eq!(Assets::account(0, BOB), 0);
			assert_eq!(Assets::account(0, ALICE), mint_amount);
		})
	}

	#[test]
	fn must_be_signed() {
		new_test_ext().execute_with(|| {
			assert_ok!(Assets::create(Origin::signed(ALICE)));
			assert_noop!(Assets::transfer(Origin::none(), 0, 100, BOB), BadOrigin);
		})
	}

	#[test]
	fn must_exist() {
		new_test_ext().execute_with(|| {
			assert_noop!(
				Assets::transfer(Origin::signed(ALICE), 0, 100, BOB),
				Error::<TestRuntime>::Unknown
			);
		})
	}
}
