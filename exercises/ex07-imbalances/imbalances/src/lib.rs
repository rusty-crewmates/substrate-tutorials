#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

pub use pallet::*;

use sp_std::vec::Vec;

use frame_support::{
	sp_runtime::traits::{CheckedConversion, CheckedMul},
	traits::{Currency, Imbalance, TryDrop},
	transactional,
};

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
pub type NegativeBalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{ExistenceRequirement, WithdrawReasons},
	};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + scale_info::TypeInfo {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId>;

		#[pallet::constant]
		type TreasuryAccount: Get<Self::AccountId>;
		#[pallet::constant]
		type TreasuryFlatCut: Get<BalanceOf<Self>>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {
		AccountDoesNotExist,
		ImbalanceOffsetFailed,
		WithdrawalFailed,
		Overflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn mint_to(
			origin: OriginFor<T>,
			amount: BalanceOf<T>,
			beneficiary: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;

			let amount_to_distribute = T::Currency::issue(amount);
			T::Currency::resolve_into_existing(&beneficiary, amount_to_distribute)
				.map_err(|_| Error::<T>::AccountDoesNotExist)?;

			Ok(())
		}

		#[pallet::weight(0)]
		pub fn slash(
			origin: OriginFor<T>,
			amount: BalanceOf<T>,
			target: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;

			let (negative_imbalance, _) = T::Currency::slash(&target, amount);
			let (to_treasury, to_burn) = negative_imbalance.ration(1, 2);

			T::Currency::resolve_creating(&T::TreasuryAccount::get(), to_treasury);
			let amount_to_burn = to_burn.peek();
			let burned = T::Currency::burn(amount_to_burn);
			burned
				.offset(to_burn)
				.try_drop()
				.map_err(|_| Error::<T>::ImbalanceOffsetFailed)?;

			Ok(())
		}

		#[pallet::weight(0)]
		#[transactional]
		pub fn sack(
			origin: OriginFor<T>,
			sacked_accounts: Vec<T::AccountId>,
			beneficiary: T::AccountId,
		) -> DispatchResult {
			ensure_root(origin)?;
			let n_accounts = sacked_accounts.len();

			let amount_collected = sacked_accounts.into_iter().try_fold(
				NegativeBalanceOf::<T>::zero(),
				|acc, account| {
					let free_balance = T::Currency::free_balance(&account);
					T::Currency::withdraw(
						&account,
						free_balance - T::Currency::minimum_balance(),
						WithdrawReasons::TRANSFER,
						ExistenceRequirement::KeepAlive,
					)
					.map(|v| acc.merge(v))
				},
			)?;

			let amount_to_treasury = T::TreasuryFlatCut::get()
				.checked_mul(&n_accounts.checked_into().ok_or(Error::<T>::Overflow)?)
				.ok_or(Error::<T>::Overflow)?;

			let (to_treasury, to_recipient) = amount_collected.split(amount_to_treasury);
			T::Currency::resolve_creating(&T::TreasuryAccount::get(), to_treasury);
			T::Currency::resolve_into_existing(&beneficiary, to_recipient)
				.map_err(|_| Error::<T>::AccountDoesNotExist)?;

			Ok(())
		}
	}
}
