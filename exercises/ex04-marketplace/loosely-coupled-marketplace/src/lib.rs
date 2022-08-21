#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod tests;
pub mod types;

use frame_support::{
	ensure,
	sp_runtime::traits::{CheckedConversion, CheckedMul},
	traits::{Currency, ExistenceRequirement::KeepAlive},
};
// use support::Sellable;
use types::*;

use pallet_marketplace_nfts::types::Sellable;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::{ensure_signed, pallet_prelude::*};

	#[pallet::config]
	pub trait Config: frame_system::Config + scale_info::TypeInfo {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId>;
		// Here are types that allow for the pallet coupling.
		// Resource must be a type that implements transferable (remember that pallets are types).
		// ResourceId is used to have a fully generic resource, can be int, uint, string, hash
		// or about anything.
		// It's entirely up to the coupled pallet to choose the type of the ID,
		// it will still be compatible with this one.
		type ResourceId: Parameter + Copy + MaxEncodedLen;
		type Resource: Sellable<Self::AccountId, Self::ResourceId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Resource has been listed for sale (resource_id, seller, price, amount)
		ListedForSale(T::ResourceId, T::AccountId, BalanceOf<T>, u128),
		// Resource has been sold (resource_id, seller, buyer, amount)
		Sold(T::ResourceId, T::AccountId, T::AccountId, u128),
	}

	#[pallet::error]
	pub enum Error<T> {
		ZeroAmount,
		NotEnoughInSale,
		NotEnoughOwned,
		SaleNotFound,
		Overflow,
	}

	#[pallet::storage]
	#[pallet::getter(fn resource_for_sale)]
	pub type ResourcesForSale<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::ResourceId,
		Blake2_128Concat,
		T::AccountId,
		SaleData<T>,
		ValueQuery,
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn set_sale(
			origin: OriginFor<T>,
			nft_id: T::ResourceId,
			price: BalanceOf<T>,
			amount: u128,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;

			ensure!(amount > 0, Error::<T>::ZeroAmount);
			let owned: u128 = todo!(
				"get the amount of this specific NFT owned by the seller, through the Resource type and its Sellable trait"
			);
			ensure!(owned >= amount, Error::<T>::NotEnoughOwned);

			ResourcesForSale::<T>::insert(nft_id, origin.clone(), SaleData { price, amount });

			Self::deposit_event(Event::<T>::ListedForSale(nft_id, origin, price, amount));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn buy(
			origin: OriginFor<T>,
			nft_id: T::ResourceId,
			seller: T::AccountId,
			amount: u128,
		) -> DispatchResult {
			let buyer = ensure_signed(origin)?;

			let sale_data = ResourcesForSale::<T>::get(nft_id, seller.clone());
			let owned = todo!(
				"get the amount of this specific NFT owned by the seller, through the Resource type and its Sellable trait"
			);

			ensure!(amount <= sale_data.amount, Error::<T>::NotEnoughInSale);
			ensure!(sale_data.amount <= owned, Error::<T>::NotEnoughOwned);

			let total_to_pay = sale_data
				.price
				.checked_mul(&amount.checked_into().ok_or(Error::<T>::Overflow)?)
				.ok_or(Error::<T>::Overflow)?;

			todo!("transfer amount of nft_id from the seller to the buyer");

			todo!("transfer the amount of currency owed from the buyer to the seller");

			T::Resource::transfer(nft_id, seller.clone(), buyer.clone(), amount);

			if amount == sale_data.amount {
				ResourcesForSale::<T>::remove(nft_id, seller.clone());
			} else {
				ResourcesForSale::<T>::mutate(nft_id, seller.clone(), |data| data.amount -= amount);
			}

			Self::deposit_event(Event::<T>::Sold(nft_id, seller, buyer, amount));

			Ok(())
		}
	}
}
