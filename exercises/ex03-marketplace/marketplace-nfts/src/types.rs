use crate::{Config, Vec};
use codec::{Decode, Encode};
use frame_support::RuntimeDebug;
use scale_info::TypeInfo;

pub type UniqueAssetId = u128;

#[derive(Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct UniqueAssetDetails<T: Config> {
	creator: T::AccountId,
	metadata: Vec<u8>,
	pub supply: u128,
}

impl<T: Config> UniqueAssetDetails<T> {
	pub fn new(creator: T::AccountId, metadata: Vec<u8>, supply: u128) -> Self {
		UniqueAssetDetails {
			creator,
			metadata,
			supply,
		}
	}

	pub fn creator(&self) -> T::AccountId {
		self.creator.clone()
	}

	pub fn metadata(&self) -> Vec<u8> {
		self.metadata.clone()
	}
}
