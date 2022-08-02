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
use types::*;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::{ensure_signed, pallet_prelude::*};

	#[pallet::config]
	pub trait Config: frame_system::Config + scale_info::TypeInfo {
		todo!("add a dependency on pallet_marketplace_nft on the previous line");
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// NFT has been listed for sale (nft_id, seller, price, amount)
		ListedForSale(T::NFTId, T::AccountId, BalanceOf<T>, u128),
		// NFT has been sold (nft_id, seller, buyer, amount)
		Sold(T::NFTId, T::AccountId, T::AccountId, u128),
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
	#[pallet::getter(fn nft_for_sale)]
	pub type NFTsForSale<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::NFTId,
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
			nft_id: T::NFTId,
			price: BalanceOf<T>,
			amount: u128,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;

			ensure!(amount > 0, Error::<T>::ZeroAmount);
			let owned = todo!("get the amount owned from the pallet_nft account storage");
			ensure!(owned >= amount, Error::<T>::NotEnoughOwned);

			NFTsForSale::<T>::insert(nft_id, origin.clone(), SaleData { price, amount });

			Self::deposit_event(Event::<T>::ListedForSale(nft_id, origin, price, amount));

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn buy(
			origin: OriginFor<T>,
			nft_id: T::NFTId,
			seller: T::AccountId,
			amount: u128,
		) -> DispatchResult {
			let buyer = ensure_signed(origin)?;

			let sale_data = NFTsForSale::<T>::get(nft_id, seller.clone());
			let owned = todo!("get the amount owned from the pallet_nft account storage");

			ensure!(amount <= sale_data.amount, Error::<T>::NotEnoughInSale);
			ensure!(sale_data.amount <= owned, Error::<T>::NotEnoughOwned);

			let total_to_pay = sale_data
				.price
				.checked_mul(&amount.checked_into().ok_or(Error::<T>::Overflow)?)
				.ok_or(Error::<T>::Overflow)?;

			<T as pallet::Config>::Currency::transfer(&buyer, &seller, total_to_pay, KeepAlive)?;

			todo!("call the pallet_marketplace_nft transfer function");

			if amount == sale_data.amount {
				NFTsForSale::<T>::remove(nft_id, seller.clone());
			} else {
				NFTsForSale::<T>::mutate(nft_id, seller.clone(), |data| data.amount -= amount);
			}

			Self::deposit_event(Event::<T>::Sold(nft_id, seller, buyer, amount));

			Ok(())
		}
	}
}
