use codec::Decode;
use sp_arithmetic::FixedI64;
use sp_core::offchain::{
	testing::{OffchainState, PendingRequest, TestOffchainExt, TestTransactionPoolExt},
	OffchainWorkerExt, TransactionPoolExt,
};

use super::mock::*;
use crate::offchain_worker::fetch_btc_price;

use super::mock::{Call, PriceOracle};

fn price_oracle_response(state: &mut OffchainState) {
	state.expect_request(PendingRequest {
		method: "GET".into(),
		uri: "https://api.coinbase.com/v2/prices/BTC-USD/buy".into(),
		response: Some(br#"{"data":{"base":"BTC","currency":"USD","amount":"29021.47"}}"#.to_vec()),
		sent: true,
		..Default::default()
	});
}

#[test]
fn fetch_btc_price_ok() {
	let (offchain, state) = TestOffchainExt::new();
	let mut t = sp_io::TestExternalities::default();
	t.register_extension(OffchainWorkerExt::new(offchain));

	price_oracle_response(&mut state.write());

	t.execute_with(|| {
		// when
		let price = fetch_btc_price().unwrap();
		// then
		assert_eq!(price, FixedI64::from_float(29021.47));
	});
}

#[test]
fn offchain_worker_submit_unsigned_transaction_ok() {
	let (offchain, offchain_state) = TestOffchainExt::new();
	let (pool, pool_state) = TestTransactionPoolExt::new();

	let mut t = sp_io::TestExternalities::default();
	t.register_extension(OffchainWorkerExt::new(offchain));
	t.register_extension(TransactionPoolExt::new(pool));

	price_oracle_response(&mut offchain_state.write());

	t.execute_with(|| {
		// when
		PriceOracle::fetch_btc_price_and_send_unsigned_transaction().unwrap();
		// then
		let tx = pool_state.write().transactions.pop().unwrap();
		assert!(pool_state.read().transactions.is_empty());

		let tx = Extrinsic::decode(&mut &*tx).unwrap();
		assert!(tx.signature.is_none());

		assert_eq!(
			tx.call,
			Call::PriceOracle(crate::Call::set_btc_price {
				btc_price: FixedI64::from_float(29021.47)
			})
		);
	});
}
