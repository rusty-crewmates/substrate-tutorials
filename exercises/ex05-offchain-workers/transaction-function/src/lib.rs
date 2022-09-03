#![cfg_attr(not(feature = "std"),no_std)]
#[cfg(test)]
mod tests;

#[cfg(test)]
mod ocw_test_mod;

use sp_runtime::offchain::KeyTypeId;
pub use offchain::*;
pub const KEY_TYPE:KeyTypeId = KeyTypeId(*b"test");
pub mod offchain {
	use super::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
		MultiSignature, MultiSigner
	};
	app_crypto!(sr25519,KEY_TYPE);
	pub struct Authority;

	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for Authority {
		type RuntimeAppPublic = sr25519::AppPublic;
		type GenericSignature = Sr25519Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}
pub use pallet::*;
#[frame_support::pallet]
pub mod pallet{
	use frame_system::pallet_prelude::*;
	use frame_support::pallet_prelude::*;
	use super::*;
	use log;
	use frame_system::offchain::{
		AppCrypto, CreateSignedTransaction, SendSignedTransaction,
		SignedPayload, Signer, SigningTypes, SubmitTransaction,
	};
	#[pallet::config]
	pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Authority: AppCrypto<Self::Public, Self::Signature>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T> {
		ValueStored
	}

	#[pallet::storage]
	#[pallet::getter(fn get_value)]
	pub type ResultValue<T> = StorageValue<_, u32, ValueQuery>;


	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10)]
		pub fn set_value(origin: OriginFor<T>, value: u32) ->DispatchResult {
			ensure_signed(origin)?;
			ResultValue::<T>::put(value);
			Self::deposit_event(Event::<T>::ValueStored);
			Ok(())

		}
	}

	impl<T: Config> Pallet<T>{

		pub fn send_signed_transaction()-> Result<(),&'static str> {
			let signer = Signer::<T, T::Authority>::all_accounts();
			let result = signer.send_signed_transaction(|account|{
				Call::set_value{value: 230}
			});
			for (acc, res) in result{
				match res {
					Ok(()) => log::info!("Success submitted by {:?}",acc.id),
					Err(()) => log::info!("Failed submitted by {:?}",acc.id)
				}
			};
			Ok(())
		}
	}

}
