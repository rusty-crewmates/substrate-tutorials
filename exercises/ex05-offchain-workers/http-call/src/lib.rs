#![cfg_attr(not(feature = "std"),no_std)]
#[cfg(test)]
mod tests;

pub use pallet::*;
#[frame_support::pallet]
pub mod pallet{
	use frame_system::pallet_prelude::*;
	use frame_support::pallet_prelude::*;
	use sp_runtime::offchain::{http};
	use log;
	use lite_json;
	use lite_json::JsonValue;

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(_);


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
