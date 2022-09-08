#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

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
	}

	#[pallet::storage]
	#[pallet::getter(fn event_counter)]
	pub type EventCounter<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::unbounded]
	#[pallet::getter(fn reminders)]
	pub type Reminders<T: Config> =
		StorageMap<_, Blake2_256, T::BlockNumber, Vec<Vec<u8>>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ReminderSet(T::BlockNumber, Vec<u8>),
		Reminder(Vec<u8>),
		RemindersExecuteds(u32),
	}

	#[pallet::error]
	pub enum Error<T> {
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(n: T::BlockNumber) -> Weight {
			let reminders = Self::reminders(n);
			for reminder in reminders {
				Self::deposit_event(Event::Reminder(reminder.clone()));
				<EventCounter<T>>::mutate(|value| *value += 1);
			}
			<Reminders<T>>::remove(n);
			0
		}

		fn on_idle(_: T::BlockNumber, _: Weight) -> Weight {
			let count = Self::event_counter();
			if count > 0 {
				Self::deposit_event(Event::RemindersExecuteds(count));
			}
			0
		}

		fn on_finalize(_: T::BlockNumber) {
			if Self::event_counter() > 0 {
				<EventCounter<T>>::put(0);
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().reads(1))]
		pub fn schedule_reminder(
			origin: OriginFor<T>,
			at: T::BlockNumber,
			message: Vec<u8>,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			<Reminders<T>>::mutate(at, |reminders| reminders.push(message.clone()));
			Self::deposit_event(Event::ReminderSet(at, message));

			Ok(())
		}
	}
}
