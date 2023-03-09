use crate::tests::mock::*;
use frame_support::BoundedVec;

#[test]
fn ok() {
	let genesis_config = crate::pallet::GenesisConfig {
		genesis_asset_list: vec![
			(
				ALICE,
				"Some metadata".as_bytes().to_vec(),
				5,
				vec![(BOB, 2)],
			),
			(
				BOB,
				"Some other metadata".as_bytes().to_vec(),
				3,
				vec![(ALICE, 1)],
			),
		],
	};

	new_test_ext_with_genesis_config(genesis_config.into()).execute_with(|| {
		let expected_metadata_1: BoundedVec<u8, <TestRuntime as crate::pallet::Config>::MaxLength> =
			"Some metadata".as_bytes().to_vec().try_into().unwrap();
		let expected_metadata_2: BoundedVec<u8, <TestRuntime as crate::pallet::Config>::MaxLength> =
			"Some other metadata".as_bytes().to_vec().try_into().unwrap();

		assert_eq!(NFTs::nonce(), 2);

		let first_asset_id = 0;
		let details = NFTs::unique_asset(first_asset_id).unwrap();
		assert_eq!(details.creator(), ALICE);
		assert_eq!(details.metadata(), expected_metadata_1);
		assert_eq!(details.supply, 5);
		assert_eq!(NFTs::account(first_asset_id, ALICE), 3);
		assert_eq!(NFTs::account(first_asset_id, BOB), 2);

		let second_asset_id = 1;
		let details = NFTs::unique_asset(second_asset_id).unwrap();
		assert_eq!(details.creator(), BOB);
		assert_eq!(details.metadata(), expected_metadata_2);
		assert_eq!(details.supply, 3);
		assert_eq!(NFTs::account(second_asset_id, ALICE), 1);
		assert_eq!(NFTs::account(second_asset_id, BOB), 2);

		// No events were emited because everything happened during block initialization
		assert!(frame_system::Pallet::<TestRuntime>::events().pop().is_none())
	})
}

#[test]
#[should_panic]
fn no_supply() {
	let genesis_config = crate::pallet::GenesisConfig {
		genesis_asset_list: vec![(
			ALICE,
			"Some metadata".as_bytes().to_vec(),
			0,
			vec![(BOB, 0)],
		)],
	};

	let _ = new_test_ext_with_genesis_config(genesis_config.into());
}

#[test]
#[should_panic]
fn metadata_too_long() {
	let genesis_config = crate::pallet::GenesisConfig {
		genesis_asset_list: vec![(
			ALICE,
			"A longer than twenty characters string".as_bytes().to_vec(),
			5,
			vec![(BOB, 2)],
		)],
	};

	let _ = new_test_ext_with_genesis_config(genesis_config.into());
}
