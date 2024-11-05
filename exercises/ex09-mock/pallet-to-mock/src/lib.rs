#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use frame_support::{
	sp_runtime::traits::{CheckedConversion, CheckedDiv},
	traits::Currency,
};

// TODO: Uncomment the following lines and fill the tests/mock.rs file
// in order for the tests to compile and run successfully
// #[cfg(test)]
// mod tests;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type Currency: Currency<Self::AccountId>;

		#[pallet::constant]
		type ValueToMint: Get<BalanceOf<Self>>;

		type SomePriceOracle: PriceOracle;
	}

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Minted(BalanceOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		Overflow,
		OracleFailedToReturnPrice,
		CallerShouldExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn mint(origin: OriginFor<T>) -> DispatchResult {
			let origin = ensure_signed(origin)?;

			let value_to_mint = <T as Config>::ValueToMint::get();
			let price = <T as Config>::SomePriceOracle::get_price()
				.map_err(|_| Error::<T>::OracleFailedToReturnPrice)?;

			let amount_to_mint = value_to_mint
				.checked_div(&price.checked_into().ok_or(Error::<T>::Overflow)?)
				.ok_or(Error::<T>::Overflow)?;

			<T as Config>::Currency::deposit_into_existing(&origin, amount_to_mint)
				.map_err(|_| Error::<T>::CallerShouldExist)?;

			Self::deposit_event(Event::<T>::Minted(amount_to_mint));

			Ok(())
		}
	}
}

pub trait PriceOracle {
	type Error;

	fn get_price() -> Result<u64, Self::Error>;
}
