use core::str::FromStr;

use frame_support::sp_runtime::offchain::{http, HttpError};

#[derive(Debug)]
pub enum OffchainWorkerError {
	Http(HttpError),
	Request(http::Error),
	Json(serde_json::Error),
	ParsePrice(<f64 as FromStr>::Err),
	WrongPair,
}

impl core::fmt::Display for OffchainWorkerError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			OffchainWorkerError::Http(e) => write!(f, "Http error occured: {:?}", e),
			OffchainWorkerError::Request(e) => write!(f, "Http error occured: {:?}", e),
			OffchainWorkerError::Json(e) =>
				write!(f, "Json deserialization error occured: {:?}", e),
			OffchainWorkerError::ParsePrice(e) => write!(f, "f64 parsing error occured: {:?}", e),
			OffchainWorkerError::WrongPair => write!(f, "Wrong pair price"),
		}
	}
}

#[cfg(feature = "std")]
impl std::error::Error for OffchainWorkerError {}

impl From<HttpError> for OffchainWorkerError {
	fn from(value: HttpError) -> Self {
		Self::Http(value)
	}
}

impl From<http::Error> for OffchainWorkerError {
	fn from(value: http::Error) -> Self {
		Self::Request(value)
	}
}

impl From<serde_json::Error> for OffchainWorkerError {
	fn from(value: serde_json::Error) -> Self {
		Self::Json(value)
	}
}
