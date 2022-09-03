use crate as pallet_http_call;
use frame_support::traits::{ConstU16, ConstU32, ConstU64};
use sp_runtime::{
	testing::{Header},
	traits::{Extrinsic, IdentifyAccount, Verify}
};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
};
use sp_core::{
	offchain::{testing, OffchainWorkerExt, TransactionPoolExt},
	sr25519::Signature,
};
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
		HttpCall: pallet_http_call,
	}
);


impl frame_system::Config for TestRuntime {
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

impl pallet_http_call::Config for TestRuntime {}

#[test]
fn test_htt_call(){
	let mut test_ext = sp_io::TestExternalities::default();
	let (ocw, ocw_state) = testing::TestOffchainExt::new();
	let ocw_ext = OffchainWorkerExt::new(ocw);
	test_ext.register_extension(ocw_ext);

	ocw_state.write().expect_request(testing::PendingRequest {
		method: "GET".into(),
		uri: "https://api.fda.gov/food/enforcement.json?limit=0".into(),
		response: Some(br#"
		    {
			  "meta": {
				"disclaimer": "Do not rely on openFDA to make decisions regarding medical care. While we make every effort to ensure that data is accurate, you should assume all results are unvalidated. We may limit or otherwise restrict your access to the API in line with our Terms of Service.",
				"terms": "https://open.fda.gov/terms/",
				"license": "https://open.fda.gov/license/",
				"last_updated": "2022-08-31",
				"results": {
				  "skip": 0,
				  "limit": 0,
				  "total": 22840
				}
			  },
              "results": []
            }
		"#.to_vec()),
		sent: true,
		..Default::default()
	});


	test_ext.execute_with(||{
		let total_result = HttpCall::get_external_data().unwrap();
		assert_eq!(total_result,22840);
	})
}
