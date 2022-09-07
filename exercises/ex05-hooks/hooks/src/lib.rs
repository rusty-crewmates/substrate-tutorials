#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use frame_support::traits::Currency;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId>;
	}

	#[pallet::storage]
	#[pallet::getter(fn authorized)]
	pub type Authorized<T> = StorageValue<_, bool, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SuccessfullTransfer(T::AccountId, T::AccountId, BalanceOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The transaction is not authorized
		Unauthorized,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(n: T::BlockNumber) -> Weight {
			if n % 2u32.into() == 0u32.into() {
				<Authorized<T>>::put(true);
				T::DbWeight::get().writes(1)
			} else {
				0
			}
		}

		fn on_finalize(_: T::BlockNumber) {
			<Authorized<T>>::put(false);
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().reads(1))]
		pub fn transfer_funds(
			origin: OriginFor<T>,
			recipient: T::AccountId,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			if !Self::authorized() {
				return Err(Error::<T>::Unauthorized.into())
			}

			let _ = T::Currency::transfer(
				&who,
				&recipient,
				amount,
				frame_support::traits::ExistenceRequirement::AllowDeath,
			);

			Self::deposit_event(Event::SuccessfullTransfer(who, recipient, amount));
			Ok(())
		}
	}
}
