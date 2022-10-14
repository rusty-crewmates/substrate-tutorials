#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// uncomment the following lines to include the benchmarking.rs file in the module tree, if the
// runtime-benchmarks feature is activated
//
// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn acc)]
	pub type Acc<T: Config> = StorageValue<_, T::AccountId>;

	#[pallet::storage]
	#[pallet::unbounded]
	pub type VecDup<T: Config> = StorageValue<_, Vec<u32>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		IsRoot(T::AccountId),
		SomethingStored(u32, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		Invalid,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		//TODO give this exctrinsic an arbitrary weight !
		#[pallet::weight(0)]
		pub fn verify_address(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;
			ensure_root(origin)?;

			// we do a read, this should be seen in the weight
			let address = Self::acc();

			if address == Some(who.clone()) {
				Self::deposit_event(Event::IsRoot(who));
			} else {
				return Err(Error::<T>::Invalid.into())
			}

			Ok(())
		}

		//TODO write a benchmark for this extrinsic in benchmarking.rs
		#[pallet::weight(0)]
		pub fn duplicate_and_store(origin: OriginFor<T>, elem: u32, count: u32) -> DispatchResult {
			ensure_signed(origin)?;

			let mut vec = Vec::new();
			for _ in 0..count {
				vec.push(elem);
			}

			VecDup::<T>::put(vec);
			Ok(())
		}
	}
}
