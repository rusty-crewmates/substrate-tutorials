
use sp_runtime::offchain::KeyTypeId;

pub const KEY_TEST:KeyTypeId = KeyTypeId(*b"test");

pub use ocw_test::*;
pub mod ocw_test{
	use super::KEY_TEST;
	use sp_runtime::{
		testing::{UintAuthorityId, TestSignature},
		app_crypto::{app_crypto, sr25519},
		traits::Verify
	};
	pub struct Authority;

	impl frame_system::offchain::AppCrypto<UintAuthorityId, TestSignature > for Authority {
		type RuntimeAppPublic = UintAuthorityId;
		type GenericSignature = TestSignature;
		type GenericPublic = UintAuthorityId;
	}

}
