#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod tests;

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

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn value)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Value<T> = StorageValue<_, bool>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [value, who]
		ValueStored(bool, T::AccountId),
		/// parameters. [new_value, who]
		ValueFlipped(bool, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Nothing is present in the storage
		NoneValue,
		/// Something is already present in the storage
		AlreadySet,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// This function sets a value to the Value storage and emits an event, it should be used
		/// once, if something is already present in the storage, it returns an error.
		#[pallet::weight(0)]
		pub fn set_value(origin: OriginFor<T>, value: bool) -> DispatchResult {
			// Checks that the extrinsic is signed and gets the signer.
			// This function will return an error if the extrinsic isn't signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Checks that there is nothing in the storage.
			match <Value<T>>::get() {
				// Returns an error if the value has already been set.
				Some(_) => Err(Error::<T>::AlreadySet)?,
				None => {
					// Updates the storage.
					<Value<T>>::put(value);

					// Emits an event.
					Self::deposit_event(Event::ValueStored(value, who));

					// Returns a successful DispatchResultWithPostInfo.
					Ok(())
				},
			}
		}

		/// This function flips the value and emits an event, if there is no value in the storage then
		/// it returns an error.
		#[pallet::weight(0)]
		pub fn flip_value(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Checks that there is something stored.
			match <Value<T>>::get() {
				// Returns an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Flips the value.
					let new = !old;
					// Sets the value in the storage.
					<Value<T>>::put(new);

					// Emits an event.
					Self::deposit_event(Event::ValueFlipped(new, who));

					Ok(())
				},
			}
		}
	}
}
