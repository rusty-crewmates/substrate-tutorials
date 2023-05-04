mod error;

use error::OffchainWorkerError;

use crate::{Call, Config, Pallet, String, Vec};

use frame_support::sp_runtime::offchain::http;

use serde::Deserialize;
use sp_arithmetic::{FixedI64, FixedPointNumber};

#[derive(Debug, Deserialize)]
struct PairBuyPrice {
	base: String,
	currency: String,
	amount: String,
}

#[derive(Debug, Deserialize)]
struct CoinbaseResponseBody {
	data: PairBuyPrice,
}

pub(crate) fn fetch_btc_price() -> Result<FixedI64, OffchainWorkerError> {
	// TODO:
	// - do an http get request to `"https://api.coinbase.com/v2/prices/BTC-USD/buy`
	// - extract the price form the response body
	// - convert it to `FixedI64` before returning it

	Ok(Default::default())
}

impl<T: Config> Pallet<T> {
	pub(crate) fn fetch_btc_price_and_send_unsigned_transaction() -> Result<(), String> {
		// Todo: call `fetch_btc_price` and use the return to submit an unsigned transaction
		// containing a call to `set_btc_price`

		Ok(())
	}
}

// FixedI64::from_float is only available in `std` mode.
// This is a copy-paste of it's implementation, which as shown by the test bellow,
// works just fine for the values and precision we are working with
//
// Feel free to use!
fn f64_to_fixed_i64(n: f64) -> FixedI64 {
	FixedI64::from_inner((n * (<FixedI64 as FixedPointNumber>::DIV as f64)) as i64)
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn f64_to_fixed_i64_ok() {
		let mut x: f64 = 0.00;
		while x < 100_000.00 {
			assert_eq!(FixedI64::from_float(x), f64_to_fixed_i64(x));
			x += 0.01;
		}
	}
}
