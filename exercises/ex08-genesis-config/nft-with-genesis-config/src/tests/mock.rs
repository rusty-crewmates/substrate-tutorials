use crate::{self as pallet_nft, GenesisConfig};
use frame_support::{parameter_types, traits::GenesisBuild};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, BuildStorage,
};

type Block = frame_system::mocking::MockBlock<TestRuntime>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum TestRuntime {
		System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
		NFTs: pallet_nft::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for TestRuntime {
	type AccountData = ();
	type AccountId = u64;
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockHashCount = BlockHashCount;
	type BlockLength = ();
	type BlockWeights = ();
	type DbWeight = ();
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Lookup = IdentityLookup<Self::AccountId>;
	type MaxConsumers = frame_support::traits::ConstU32<16>;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type OnSetCode = ();
	type PalletInfo = PalletInfo;
	type SS58Prefix = SS58Prefix;
	type SystemWeightInfo = ();
	type Version = ();

	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type RuntimeTask = ();
	type Nonce = u64;
	type Block = Block;
	type SingleBlockMigrations = ();
	type MultiBlockMigrator = ();
	type PreInherents = ();
	type PostInherents = ();
	type PostTransactions = ();
}

parameter_types! {
	pub const MaxLength: u32 = 20;
}

impl pallet_nft::Config for TestRuntime {
	type RuntimeEvent = RuntimeEvent;
	type MaxLength = MaxLength;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let storage = <GenesisConfig<TestRuntime> as BuildStorage>::build_storage(&GenesisConfig::default()).unwrap();
	let mut ext = sp_io::TestExternalities::new(storage);
	// In order to emit events the block number must be more than 0
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub fn new_test_ext_with_genesis_config(
	genesis_config: pallet_nft::GenesisConfig<TestRuntime>,
) -> sp_io::TestExternalities {
	let mut storage = <pallet_nft::GenesisConfig<TestRuntime> as GenesisBuild<TestRuntime>>::build_storage(&genesis_config).unwrap();

	BuildStorage::assimilate_storage(&genesis_config, &mut storage).unwrap();

	let mut ext = sp_io::TestExternalities::new(storage);
	// In order to emit events the block number must be more than 0
	ext.execute_with(|| System::set_block_number(1));
	ext
}

// Mock users AccountId
pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
