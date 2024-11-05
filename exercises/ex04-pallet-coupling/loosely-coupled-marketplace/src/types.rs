use crate::{BalanceOf, Config};
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::RuntimeDebug;

#[derive(Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[codec(mel_bound())]
pub struct SaleData<T: Config> {
	pub price: BalanceOf<T>,
	pub amount: u128,
}

impl<T: Config> Default for SaleData<T> {
	fn default() -> Self {
		Self {
			price: Default::default(),
			amount: Default::default(),
		}
	}
}
