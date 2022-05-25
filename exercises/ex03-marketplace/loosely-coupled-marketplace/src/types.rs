use crate::{BalanceOf, Config};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::RuntimeDebug;
use scale_info::TypeInfo;

#[derive(Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
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
