use crate::{tests::mock::*, Error};
use frame_support::{assert_noop, assert_ok, error::BadOrigin};

fn create_nft(amount: u128) {
	let metadata = "Some metadata".as_bytes().to_vec().try_into().unwrap();
	NFTs::mint(Origin::signed(ALICE), metadata, amount).unwrap();
}

fn amount_owned(resource_id: u128, address: u64) -> u128 {
	use pallet_marketplace_nfts::types::Sellable;

	<TestRuntime as crate::pallet::Config>::Resource::amount_owned(resource_id, address)
}

mod set_sale {
	use super::*;

	#[test]
	fn ok() {
		ExtBuilder::default().balances(vec![]).build().execute_with(|| {
			create_nft(5);
			let price = 1000;
			let amount = 2;

			assert_ok!(Marketplace::set_sale(
				Origin::signed(ALICE),
				0,
				price,
				amount
			));

			let sale = Marketplace::resource_for_sale(0, ALICE);
			assert_eq!(sale.price, price);
			assert_eq!(sale.amount, amount);
		})
	}

	#[test]
	fn nft_does_not_exist() {
		ExtBuilder::default().balances(vec![]).build().execute_with(|| {
			assert_noop!(
				Marketplace::set_sale(Origin::signed(ALICE), 0, 1000, 2),
				Error::<TestRuntime>::NotEnoughOwned
			);
		})
	}

	#[test]
	fn zero_amount() {
		ExtBuilder::default().balances(vec![]).build().execute_with(|| {
			create_nft(5);
			assert_noop!(
				Marketplace::set_sale(Origin::signed(ALICE), 0, 1000, 0),
				Error::<TestRuntime>::ZeroAmount
			);
		})
	}

	#[test]
	fn not_enough_owned() {
		ExtBuilder::default().balances(vec![]).build().execute_with(|| {
			create_nft(5);
			assert_noop!(
				Marketplace::set_sale(Origin::signed(ALICE), 0, 1000, 10),
				Error::<TestRuntime>::NotEnoughOwned
			);
		})
	}

	#[test]
	fn must_be_signed() {
		ExtBuilder::default().balances(vec![]).build().execute_with(|| {
			assert_noop!(Marketplace::set_sale(Origin::none(), 0, 1000, 2), BadOrigin);
		})
	}
}

mod buy {
	use super::*;

	#[test]
	fn ok() {
		let bob_funds = 2000;
		ExtBuilder::default().balances(vec![(BOB, bob_funds)]).build().execute_with(|| {
			create_nft(5);
			let price = 1000;
			assert_ok!(Marketplace::set_sale(Origin::signed(ALICE), 0, price, 2));

			assert_ok!(Marketplace::buy(Origin::signed(BOB), 0, ALICE, 1));

			assert_eq!(bob_funds - price, Balances::free_balance(BOB));
			assert_eq!(price, Balances::free_balance(ALICE));
			assert_eq!(amount_owned(0, ALICE), 4);
			assert_eq!(amount_owned(0, BOB), 1);
		})
	}

	#[test]
	fn ok_multiple() {
		let bob_funds = 2000;
		ExtBuilder::default().balances(vec![(BOB, bob_funds)]).build().execute_with(|| {
			create_nft(5);
			let price = 1000;
			assert_ok!(Marketplace::set_sale(Origin::signed(ALICE), 0, price, 2));

			let amount_buy = 2;
			let total_price = amount_buy * price;
			assert_ok!(Marketplace::buy(Origin::signed(BOB), 0, ALICE, amount_buy));

			assert_eq!(bob_funds - total_price, Balances::free_balance(BOB));
			assert_eq!(total_price, Balances::free_balance(ALICE));
			assert_eq!(amount_owned(0, ALICE), 3);
			assert_eq!(amount_owned(0, BOB), 2);
		})
	}

	#[test]
	fn not_enough_in_sale() {
		ExtBuilder::default().balances(vec![]).build().execute_with(|| {
			create_nft(5);
			assert_ok!(Marketplace::set_sale(Origin::signed(ALICE), 0, 1000, 2));

			assert_noop!(
				Marketplace::buy(Origin::signed(BOB), 0, ALICE, 5),
				Error::<TestRuntime>::NotEnoughInSale
			);
		})
	}

	#[test]
	fn not_enough_owned() {
		ExtBuilder::default().balances(vec![]).build().execute_with(|| {
			create_nft(5);
			assert_ok!(Marketplace::set_sale(Origin::signed(ALICE), 0, 1000, 2));

			assert_ok!(NFTs::transfer(Origin::signed(ALICE), 0, 4, 0));

			assert_noop!(
				Marketplace::buy(Origin::signed(BOB), 0, ALICE, 2),
				Error::<TestRuntime>::NotEnoughOwned
			);
		})
	}

	#[test]
	fn must_be_signed() {
		ExtBuilder::default().balances(vec![]).build().execute_with(|| {
			assert_noop!(Marketplace::buy(Origin::none(), 0, ALICE, 1), BadOrigin);
		})
	}
}
