#![cfg_attr(not(feature ="std"),no_std)]
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
	use sp_runtime::offchain::{http};
	use lite_json;
	use lite_json::JsonValue;
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

	#[pallet::hooks]
	impl<T:Config> Hooks<BlockNumberFor<T>> for Pallet<T>{
		fn offchain_worker(_n: BlockNumberFor<T>) {
			Self::send_signed_transaction().map_err(|_|log::info!("Failed Function"));
		}
	}

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
			let value = Self::get_external_data().unwrap();
			let result = signer.send_signed_transaction(|account|{
				Call::set_value{value}
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

	impl<T: Config> Pallet<T> {
		pub fn get_external_data() -> Result<u32, http::Error> {

			let request = http::Request::get("https://api.fda.gov/food/enforcement.json?limit=0");
			let pending = request.send().map_err(|_|http::Error::Unknown)?;
			let result = pending.wait();

			let response = result.map_err(|_| http::Error::IoError)?;

			if response.code != 200 {
				log::info!("Failed Response Code {}",response.code);
				return Err(http::Error::IoError)
			}
			log::info!("fetched success");

			let body = response.body().collect::<Vec<u8>>();
			let body_str = sp_std::str::from_utf8(&body[..])
				.map_err(|_| http::Error::Unknown)?;
			let body_json = lite_json::json_parser::parse_json(body_str)
				.map_err(|_|http::Error::Unknown)?;
			let result_total = Self::parse_to_int(body_json).unwrap();
			Ok(result_total)

		}

		pub fn parse_to_int(body: JsonValue) -> Option<u32>{
			let result = match body {
				JsonValue::Object(obj) => {
					let (_,val) = obj.into_iter().find(|(k,_)| k.iter().copied().eq("meta".chars()))?;
					match val {
						JsonValue::Object(obj) => {
							let (_,val) = obj.into_iter().find(|(k,_)|k.iter().copied().eq("results".chars()))?;
							match val {
								JsonValue::Object(obj) => {
									let (_,val) = obj.into_iter().find(|(k,_)|k.iter().copied().eq("total".chars()))?;
									match val  {
										JsonValue::Number(num) => Some(num),
										_=> None
									}
								},
								_=> None
							}
						},
						_=> None
					}
				},
				_=> None
			};
			Some(result.unwrap().integer as u32)
		}
	}

}
