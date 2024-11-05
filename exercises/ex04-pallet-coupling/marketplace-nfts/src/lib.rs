#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod tests;
pub mod types;

use frame_support::ensure;
use sp_runtime::traits::{AtLeast32BitUnsigned, One, Saturating};
use types::*;

#[allow(unexpected_cfgs)] // skip warning "unexpected `cfg` condition value: `try-runtime`"
#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::config]
	pub trait Config: frame_system::Config + scale_info::TypeInfo {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type NFTId: Parameter + AtLeast32BitUnsigned + Default + Copy + MaxEncodedLen;

		#[pallet::constant]
		type MaxLength: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn unique_asset)]
	pub(super) type UniqueAsset<T: Config> =
		StorageMap<_, Blake2_128Concat, T::NFTId, UniqueAssetDetails<T, T::MaxLength>>;

	#[pallet::storage]
	#[pallet::getter(fn account)]
	/// The holdings of a specific account for a specific asset.
	pub(super) type Account<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::NFTId,
		Blake2_128Concat,
		T::AccountId,
		u128,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn nonce)]
	/// Nonce for id of the next created asset
	pub(super) type Nonce<T: Config> = StorageValue<_, T::NFTId, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New unique asset created
		Created {
			creator: T::AccountId,
			asset_id: T::NFTId,
		},
		/// Some assets have been burned
		Burned {
			asset_id: T::NFTId,
			owner: T::AccountId,
			total_supply: u128,
		},
		/// Some assets have been transferred
		Transferred {
			asset_id: T::NFTId,
			from: T::AccountId,
			to: T::AccountId,
			amount: u128,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The asset ID is unknown
		Unknown,
		/// The signing account does not own any amount of this asset
		NotOwned,
		/// Supply must be positive
		NoSupply,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight({0})]
		pub fn mint(
			origin: OriginFor<T>,
			metadata: BoundedVec<u8, T::MaxLength>,
			supply: u128,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;

			ensure!(supply > 0, Error::<T>::NoSupply);

			let id = Self::nonce();
			let details = UniqueAssetDetails::new(origin.clone(), metadata, supply);
			UniqueAsset::<T>::insert(id, details);
			Account::<T>::insert(id, origin.clone(), supply);
			Nonce::<T>::set(id.saturating_add(T::NFTId::one()));

			Self::deposit_event(Event::<T>::Created {
				creator: origin,
				asset_id: id,
			});

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight({0})]
		pub fn burn(origin: OriginFor<T>, asset_id: T::NFTId, amount: u128) -> DispatchResult {
			let origin = ensure_signed(origin)?;

			ensure!(Self::unique_asset(asset_id).is_some(), Error::<T>::Unknown);
			Self::ensure_own_some(asset_id, origin.clone())?;

			let mut total_supply = 0;

			UniqueAsset::<T>::try_mutate(asset_id, |maybe_details| -> DispatchResult {
				let details = maybe_details.as_mut().ok_or(Error::<T>::Unknown)?;

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

		#[pallet::call_index(2)]
		#[pallet::weight({0})]
		pub fn transfer(
			origin: OriginFor<T>,
			asset_id: T::NFTId,
			amount: u128,
			to: T::AccountId,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;

			ensure!(Self::unique_asset(asset_id).is_some(), Error::<T>::Unknown);
			Self::ensure_own_some(asset_id, from.clone())?;

			Self::unchecked_transfer(asset_id, from, to, amount);

			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	fn ensure_own_some(asset_id: T::NFTId, account: T::AccountId) -> Result<(), Error<T>> {
		let owned = Self::account(asset_id, account);

		ensure!(owned > 0, Error::<T>::NotOwned);

		Ok(())
	}

	// Note that it must be public to be callable from pallet_tight_marketplace
	pub fn unchecked_transfer(
		nft_id: T::NFTId,
		from: T::AccountId,
		to: T::AccountId,
		amount: u128,
	) -> u128 {
		let mut transfered_amount = 0;
		Account::<T>::mutate(nft_id, from.clone(), |balance| {
			let old_balance = *balance;
			*balance = balance.saturating_sub(amount);
			transfered_amount = old_balance - *balance;
		});

		Account::<T>::mutate(nft_id, to.clone(), |balance| {
			*balance = balance.saturating_add(transfered_amount);
		});

		Self::deposit_event(Event::<T>::Transferred {
			asset_id: nft_id,
			from,
			to,
			amount: transfered_amount,
		});

		transfered_amount
	}
}

impl<T: Config> Sellable<T::AccountId, T::NFTId> for Pallet<T> {
	fn amount_owned(nft_id: T::NFTId, account: T::AccountId) -> u128 {
		todo!("return the amount of nft_id owned by account")
	}

	fn transfer(nft_id: T::NFTId, from: T::AccountId, to: T::AccountId, amount: u128) -> u128 {
		todo!("do the transfer")
	}
}
