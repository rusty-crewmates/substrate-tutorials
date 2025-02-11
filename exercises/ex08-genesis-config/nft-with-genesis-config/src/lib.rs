#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod tests;
pub mod types;

use frame_support::{ensure, BoundedVec};
use types::*;

type GenesisAssetList<T> = Vec<(
	<T as frame_system::Config>::AccountId,              // creator
	Vec<u8>,                                             // metadata
	u128,                                                // supply
	Vec<(<T as frame_system::Config>::AccountId, u128)>, // (owner, amount)
)>;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + scale_info::TypeInfo {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		#[pallet::constant]
		type MaxLength: Get<u32>;
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub genesis_asset_list: GenesisAssetList<T>,
	}

	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				genesis_asset_list: Default::default(),
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			// TODO
			// iterate over the `GenesisAssetList` and:
			// 1) mint each asset
			// 2) transfer the correct amount of this asset to each account in the inner vec
		}
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn unique_asset)]
	pub(super) type UniqueAsset<T: Config> =
		StorageMap<_, Blake2_128Concat, UniqueAssetId, UniqueAssetDetails<T, T::MaxLength>>;

	#[pallet::storage]
	#[pallet::getter(fn account)]
	/// The holdings of a specific account for a specific asset.
	pub(super) type Account<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		UniqueAssetId,
		Blake2_128Concat,
		T::AccountId,
		u128,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn nonce)]
	/// Nonce for id of the next created asset
	pub(super) type Nonce<T: Config> = StorageValue<_, UniqueAssetId, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New unique asset created
		Created {
			creator: T::AccountId,
			asset_id: UniqueAssetId,
		},
		/// Some assets have been burned
		Burned {
			asset_id: UniqueAssetId,
			owner: T::AccountId,
			total_supply: u128,
		},
		/// Some assets have been transferred
		Transferred {
			asset_id: UniqueAssetId,
			from: T::AccountId,
			to: T::AccountId,
			amount: u128,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The asset ID is unknown
		UnknownAssetId,
		/// The signing account does not own any amount of this asset
		NotOwned,
		/// Supply must be positive
		NoSupply,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn mint(
			origin: OriginFor<T>,
			metadata: BoundedVec<u8, T::MaxLength>,
			supply: u128,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;

			Self::inner_mint(origin, metadata, supply)?;

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn burn(origin: OriginFor<T>, asset_id: UniqueAssetId, amount: u128) -> DispatchResult {
			let origin = ensure_signed(origin)?;

			ensure!(
				Self::unique_asset(asset_id).is_some(),
				Error::<T>::UnknownAssetId
			);
			Self::ensure_own_some(asset_id, origin.clone())?;

			let mut total_supply = 0;

			UniqueAsset::<T>::try_mutate(asset_id, |maybe_details| -> DispatchResult {
				let details = maybe_details.as_mut().ok_or(Error::<T>::UnknownAssetId)?;

				let mut burned_amount = 0;
				Account::<T>::mutate(asset_id, origin.clone(), |balance| {
					let old_balance = *balance;
					*balance = balance.saturating_sub(amount);
					burned_amount = old_balance - *balance;
				});

				details.supply -= burned_amount;
				total_supply = details.supply;

				Ok(())
			})?;

			Self::deposit_event(Event::<T>::Burned {
				asset_id,
				owner: origin,
				total_supply,
			});

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn transfer(
			origin: OriginFor<T>,
			asset_id: UniqueAssetId,
			amount: u128,
			to: T::AccountId,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;

			Self::inner_transfer(asset_id, origin, to, amount)?;

			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	fn ensure_own_some(asset_id: UniqueAssetId, account: T::AccountId) -> Result<(), Error<T>> {
		let owned = Self::account(asset_id, account);

		ensure!(owned > 0, Error::<T>::NotOwned);

		Ok(())
	}

	fn inner_mint(
		creator: T::AccountId,
		metadata: BoundedVec<u8, T::MaxLength>,
		supply: u128,
	) -> Result<(), Error<T>> {
		ensure!(supply > 0, Error::<T>::NoSupply);

		let id = Self::nonce();
		let details = UniqueAssetDetails::new(creator.clone(), metadata, supply);
		UniqueAsset::<T>::insert(id, details);
		Account::<T>::insert(id, creator.clone(), supply);
		Nonce::<T>::set(id.saturating_add(1));

		Self::deposit_event(Event::<T>::Created {
			creator,
			asset_id: id,
		});

		Ok(())
	}

	fn inner_transfer(
		asset_id: UniqueAssetId,
		from: T::AccountId,
		to: T::AccountId,
		amount: u128,
	) -> Result<(), Error<T>> {
		ensure!(
			Self::unique_asset(asset_id).is_some(),
			Error::<T>::UnknownAssetId
		);
		Self::ensure_own_some(asset_id, from.clone())?;

		let mut transfered_amount = 0;
		Account::<T>::mutate(asset_id, from.clone(), |balance| {
			let old_balance = *balance;
			*balance = balance.saturating_sub(amount);
			transfered_amount = old_balance - *balance;
		});

		Account::<T>::mutate(asset_id, to.clone(), |balance| {
			*balance = balance.saturating_add(transfered_amount);
		});

		Self::deposit_event(Event::<T>::Transferred {
			asset_id,
			from,
			to,
			amount: transfered_amount,
		});

		Ok(())
	}
}
