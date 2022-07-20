use crate::{tests::mock::*, Error};
use frame_support::{assert_noop, assert_ok, error::BadOrigin, BoundedVec};

mod mint {
	use super::*;

	#[test]
	fn ok() {
		new_test_ext().execute_with(|| {
			let metadata: BoundedVec<u8, <TestRuntime as crate::pallet::Config>::MaxLength> =
				"Some metadata".as_bytes().to_vec().try_into().unwrap();
			assert_ok!(NFTs::mint(Origin::signed(ALICE), metadata.clone(), 5));

			assert_eq!(NFTs::nonce(), 1);

			let details = NFTs::unique_asset(0).unwrap();
			assert_eq!(details.creator(), ALICE);
			assert_eq!(details.metadata(), metadata);
			assert_eq!(details.supply, 5);
		})
	}

	#[test]
	fn must_be_signed() {
		new_test_ext().execute_with(|| {
			assert_noop!(
				NFTs::mint(
					Origin::none(),
					"Some metadata".as_bytes().to_vec().try_into().unwrap(),
					5
				),
				BadOrigin
			);
		})
	}

	#[test]
	fn must_have_positive_supply() {
		new_test_ext().execute_with(|| {
			assert_noop!(
				NFTs::mint(
					Origin::signed(ALICE),
					"Some metadata".as_bytes().to_vec().try_into().unwrap(),
					0
				),
				Error::<TestRuntime>::NoSupply
			);
		})
	}
}

mod transfer {
	use super::*;

	#[test]
	fn ok() {
		new_test_ext().execute_with(|| {
			let minted_amount = 5;
			let transfered_amount = 2;
			assert_ok!(NFTs::mint(
				Origin::signed(ALICE),
				"Some metadata".as_bytes().to_vec().try_into().unwrap(),
				minted_amount
			));
			assert_ok!(NFTs::transfer(
				Origin::signed(ALICE),
				0,
				transfered_amount,
				BOB
			));

			assert_eq!(NFTs::account(0, ALICE), minted_amount - transfered_amount);
			assert_eq!(NFTs::account(0, BOB), transfered_amount);
		})
	}

	#[test]
	fn ok_saturating() {
		new_test_ext().execute_with(|| {
			let minted_amount = 5;
			let transfered_amount = 10;
			assert_ok!(NFTs::mint(
				Origin::signed(ALICE),
				"Some metadata".as_bytes().to_vec().try_into().unwrap(),
				minted_amount
			));
			assert_ok!(NFTs::transfer(
				Origin::signed(ALICE),
				0,
				transfered_amount,
				BOB
			));

			assert_eq!(NFTs::account(0, ALICE), 0);
			assert_eq!(NFTs::account(0, BOB), minted_amount);
		})
	}

	#[test]
	fn must_be_signed() {
		new_test_ext().execute_with(|| {
			assert_noop!(
				NFTs::mint(
					Origin::none(),
					"Some metadata".as_bytes().to_vec().try_into().unwrap(),
					5
				),
				BadOrigin
			);
		})
	}

	#[test]
	fn must_exist() {
		new_test_ext().execute_with(|| {
			assert_noop!(
				NFTs::transfer(Origin::signed(ALICE), 0, 100, BOB),
				Error::<TestRuntime>::UnknownAssetId
			);
		})
	}

	#[test]
	fn must_own_some() {
		new_test_ext().execute_with(|| {
			assert_ok!(NFTs::mint(
				Origin::signed(ALICE),
				"Some metadata".as_bytes().to_vec().try_into().unwrap(),
				5
			));
			assert_noop!(
				NFTs::transfer(Origin::signed(BOB), 0, 2, ALICE),
				Error::<TestRuntime>::NotOwned
			);
		})
	}
}

mod burn {
	use super::*;

	#[test]
	fn ok() {
		new_test_ext().execute_with(|| {
			let minted_amount = 5;
			let burned_amount = 2;
			assert_ok!(NFTs::mint(
				Origin::signed(ALICE),
				"Some metadata".as_bytes().to_vec().try_into().unwrap(),
				minted_amount
			));
			assert_ok!(NFTs::burn(Origin::signed(ALICE), 0, burned_amount));

			assert_eq!(
				NFTs::unique_asset(0).unwrap().supply,
				minted_amount - burned_amount
			);
			assert_eq!(NFTs::account(0, ALICE), minted_amount - burned_amount);
		})
	}

	#[test]
	fn ok_saturating() {
		new_test_ext().execute_with(|| {
			let minted_amount = 5;
			let burned_amount = 10;
			assert_ok!(NFTs::mint(
				Origin::signed(ALICE),
				"Some metadata".as_bytes().to_vec().try_into().unwrap(),
				minted_amount
			));

			assert_ok!(NFTs::burn(Origin::signed(ALICE), 0, burned_amount));

			assert_eq!(
				NFTs::unique_asset(0).unwrap().supply,
				0
			);
			assert_eq!(NFTs::account(0, ALICE), 0);
		})
	}

	#[test]
	fn must_be_signed() {
		new_test_ext().execute_with(|| {
			assert_noop!(
				NFTs::burn(
					Origin::none(),
					0,
					5
				),
				BadOrigin
			);
		})
	}

	#[test]
	fn must_exist() {
		new_test_ext().execute_with(|| {
			assert_noop!(
				NFTs::burn(Origin::signed(ALICE), 0, 100),
				Error::<TestRuntime>::UnknownAssetId
			);
		})
	}

	#[test]
	fn must_own_some() {
		new_test_ext().execute_with(|| {
			assert_ok!(NFTs::mint(
				Origin::signed(ALICE),
				"Some metadata".as_bytes().to_vec().try_into().unwrap(),
				5
			));
			assert_noop!(
				NFTs::burn(Origin::signed(BOB), 0, 2),
				Error::<TestRuntime>::NotOwned
			);
		})
	}
}
