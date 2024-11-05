use codec::{Decode, Encode};
use frame_support::{
	pallet_prelude::{BoundedVec, MaxEncodedLen},
	traits::Get,
};
use scale_info::TypeInfo;
use sp_core::RuntimeDebug;

pub type AssetId = u128;

#[derive(Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct AssetDetails<AccountId> {
	pub owner: AccountId,
	pub supply: u128,
}

impl<AccountId> AssetDetails<AccountId> {
	pub fn new(owner: AccountId) -> Self {
		AssetDetails { owner, supply: 0 }
	}
}

#[derive(Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(S))]
#[codec(mel_bound())]
pub struct AssetMetadata<S: Get<u32>> {
	pub name: BoundedVec<u8, S>,
	pub symbol: BoundedVec<u8, S>,
}

impl<S: Get<u32>> AssetMetadata<S> {
	pub fn new(name: BoundedVec<u8, S>, symbol: BoundedVec<u8, S>) -> Self {
		AssetMetadata { name, symbol }
	}
}
