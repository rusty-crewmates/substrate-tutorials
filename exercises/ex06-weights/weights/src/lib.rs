#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use sp_io::hashing::blake2_256;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// uncomment the following lines to include the benchmarking.rs file in the module tree, if the
// runtime-benchmarks feature is activated
//
// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

use sp_std::vec::Vec;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type WeightInfo;
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

	#[pallet::storage]
	#[pallet::unbounded]
	pub type Data<T: Config> = StorageValue<_, Vec<u8>>;

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
		/////////////////////// Part 1 - arbitrary weights ///////////////////////
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
				return Err(Error::<T>::Invalid.into());
			}

			Ok(())
		}

		/////////////////////// Part 2 - benchmarks ///////////////////////
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

		/////////////////////// Part 3.A - conditional arbitrary weight ///////////////////////
		//TODO give this extrinsic a weight of 100_000 if `hash` is true, or 10_000 otherwise
		#[pallet::weight(0)]
		pub fn store_maybe_hashed(
			origin: OriginFor<T>,
			data: Vec<u8>,
			hash: bool,
		) -> DispatchResult {
			ensure_signed(origin)?;

			if hash {
				let hash = blake2_256(&data);
				Data::<T>::put(hash.as_ref().to_vec());
			} else {
				Data::<T>::put(data);
			}

			Ok(())
		}

		/////////////////////// Part 3.B - conditional benchmark ///////////////////////
		//TODO write two benchmarks for this extrinsic in benchmarking.rs, and then choose the
		//corresponding one depending on the value of `hash`
		//hint: look at this pallet's weights macros: https://github.com/paritytech/substrate/blob/master/frame/utility/src/lib.rs
		#[pallet::weight(0)]
		pub fn benchmarked_store_maybe_hashed(
			origin: OriginFor<T>,
			data: Vec<u8>,
			hash: bool,
		) -> DispatchResult {
			ensure_signed(origin)?;

			if hash {
				let hash = blake2_256(&data);
				Data::<T>::put(hash.as_ref().to_vec());
			} else {
				Data::<T>::put(data);
			}

			Ok(())
		}
	}
}
