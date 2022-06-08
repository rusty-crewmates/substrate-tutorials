use crate::Config;
use codec::{Decode, Encode};
use frame_support::{
	pallet_prelude::{BoundedVec, MaxEncodedLen},
	traits::Get,
	RuntimeDebug,
};
use scale_info::TypeInfo;

pub type UniqueAssetId = u128;

#[derive(Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(S))]
#[codec(mel_bound())]
pub struct UniqueAssetDetails<T: Config, S: Get<u32>> {
	creator: T::AccountId,
	metadata: BoundedVec<u8, S>,
	pub supply: u128,
}

impl<T: Config, S: Get<u32>> UniqueAssetDetails<T, S> {
	pub fn new(creator: T::AccountId, metadata: BoundedVec<u8, S>, supply: u128) -> Self {
		UniqueAssetDetails {
			creator,
			metadata,
			supply,
		}
	}

	pub fn creator(&self) -> T::AccountId {
		self.creator.clone()
	}

	pub fn metadata(&self) -> BoundedVec<u8, S> {
		self.metadata.clone()
	}
}
