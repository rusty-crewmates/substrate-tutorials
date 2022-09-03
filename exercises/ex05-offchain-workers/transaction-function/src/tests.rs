#[allow(unused_imports)]
use super::*;
use crate as pallet_transaction_function;
use frame_support::{assert_ok,assert_err,assert_noop};
use frame_support::traits::{ConstU16, ConstU32, ConstU64};
use frame_system as system;
use frame_system::offchain::{AppCrypto,SigningTypes, CreateSignedTransaction, SendTransactionTypes};
use sp_core::H256;
use sp_runtime::{
	testing::{Header,TestSignature, TestXt},
	traits::{BlakeTwo256, IdentityLookup},
};
use sp_core::{
	offchain::{testing, OffchainWorkerExt, TransactionPoolExt},
	sr25519::Signature,
};
use sp_core::offchain::testing::TestTransactionPoolExt;

use sp_runtime::{
	traits::{Extrinsic, IdentifyAccount, Verify}
};
use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStore};
use sp_runtime::DispatchError::BadOrigin;
use crate::Event::ValueStored;
use crate::ocw_test_mod::ocw_test;
use crate::offchain;


type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum TestRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		TxnFunction: pallet_transaction_function,
	}
);

impl system::Config for TestRuntime {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

//Implementing CreateSignedTransaction trait and other dependency traits
type AccountId = <<TestSignature as Verify>::Signer as IdentifyAccount>::AccountId;
type TestExtrinsic = TestXt<Call,()>;

impl<T> CreateSignedTransaction<T> for TestRuntime where Call: From<T>,{
	fn create_transaction<C: AppCrypto<Self::Public, Self::Signature>>
	(call: Call,
	 _public:<TestSignature as Verify>::Signer,
	 _account: AccountId,
	 nonce: u64
	) -> Option<(Call, <TestExtrinsic as Extrinsic>::SignaturePayload)> {
		Some((call,(nonce,())))
	}
}

impl<L> SendTransactionTypes<L> for TestRuntime where Call: From<L>{
	type Extrinsic = TestExtrinsic;
	type OverarchingCall = Call;
}
impl SigningTypes for TestRuntime {
	type Public = <TestSignature as Verify>::Signer;
	type Signature = TestSignature;
}


 impl pallet_transaction_function::Config for TestRuntime {
	 type Event = Event;
 	 type Authority = ocw_test::Authority;
 }


#[test]
fn test_set_value_call(){
	let mut test_ext = sp_io::TestExternalities::default();

	test_ext.execute_with(||{
		assert_ok!(TxnFunction::set_value(Origin::signed(1),230));
		assert_eq!(TxnFunction::get_value(),230);
		//Should Fail
		assert_noop!(TxnFunction::set_value(Origin::root(),203), BadOrigin);

	})
}

#[test]
fn test_send_signed_transaction(){
	let mut test_ext = sp_io::TestExternalities::default();

	test_ext.execute_with(||{

		assert_eq!(TxnFunction::send_signed_transaction().unwrap(),());

	})
}
