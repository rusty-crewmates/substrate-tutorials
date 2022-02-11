use crate::{Config, Vec};
use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use scale_info::TypeInfo;

pub type AssetId = u128;

#[derive(Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct AssetDetails<T: Config> {
	pub owner: T::AccountId,
	pub supply: u128,
}

impl<T: Config> AssetDetails<T> {
	pub fn new(owner: T::AccountId) -> Self {
		AssetDetails { owner, supply: 0 }
	}
}

#[derive(Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct AssetMetadata {
	pub name: Vec<u8>,
	pub symbol: Vec<u8>,
}

impl AssetMetadata {
	pub fn new(name: Vec<u8>, symbol: Vec<u8>) -> Self {
		AssetMetadata { name, symbol }
	}
}
