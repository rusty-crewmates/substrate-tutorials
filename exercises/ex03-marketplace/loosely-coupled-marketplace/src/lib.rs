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
		type RessourceId: Parameter + Copy + MaxEncodedLen;
		type Ressource: Sellable<Self::AccountId, Self::RessourceId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Ressource has been listed for sale (ressource_id, seller, price, amount)
		ListedForSale(T::RessourceId, T::AccountId, BalanceOf<T>, u128),
		// Ressource has been sold (ressource_id, seller, buyer, amount)
		Sold(T::RessourceId, T::AccountId, T::AccountId, u128),
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
	#[pallet::getter(fn ressource_for_sale)]
	pub type RessourcesForSale<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::RessourceId,
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
			nft_id: T::RessourceId,
			price: BalanceOf<T>,
			amount: u128,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;

			ensure!(amount > 0, Error::<T>::ZeroAmount);
			let owned = T::Ressource::amount_owned(nft_id, origin.clone());
			ensure!(owned >= amount, Error::<T>::NotEnoughOwned);

			RessourcesForSale::<T>::insert(nft_id, origin.clone(), SaleData { price, amount });

			Self::deposit_event(Event::<T>::ListedForSale(nft_id, origin, price, amount));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn buy(
			origin: OriginFor<T>,
			nft_id: T::RessourceId,
			seller: T::AccountId,
			amount: u128,
		) -> DispatchResult {
			let buyer = ensure_signed(origin)?;

			let sale_data = RessourcesForSale::<T>::get(nft_id, seller.clone());
			let owned = T::Ressource::amount_owned(nft_id, seller.clone());

			ensure!(amount <= sale_data.amount, Error::<T>::NotEnoughInSale);
			ensure!(sale_data.amount <= owned, Error::<T>::NotEnoughOwned);

			let total_to_pay = sale_data
				.price
				.checked_mul(&amount.checked_into().ok_or(Error::<T>::Overflow)?)
				.ok_or(Error::<T>::Overflow)?;

			T::Currency::transfer(&buyer, &seller, total_to_pay, KeepAlive)?;

			T::Ressource::transfer(nft_id, seller.clone(), buyer.clone(), amount);

			if amount == sale_data.amount {
				RessourcesForSale::<T>::remove(nft_id, seller.clone());
			} else {
				RessourcesForSale::<T>::mutate(nft_id, seller.clone(), |data| {
					data.amount -= amount
				});
			}

			Self::deposit_event(Event::<T>::Sold(nft_id, seller, buyer, amount));

			Ok(())
		}
	}
}
