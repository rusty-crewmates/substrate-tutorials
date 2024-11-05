use crate as pallet_imbalance;
use frame_support::parameter_types;
use frame_system::GenesisConfig;
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, 
	BuildStorage,
};

type Block = frame_system::mocking::MockBlock<TestRuntime>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum TestRuntime {
		System: frame_system,
		Balances: pallet_balances,

		Imbalances: pallet_imbalance::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for TestRuntime {
	type AccountData = pallet_balances::AccountData<u128>;
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

pub const EXISTENTIAL_DEPOSIT: u128 = 10;

parameter_types! {
	pub const ExistentialDeposit: u128 = EXISTENTIAL_DEPOSIT;
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for TestRuntime {
	type AccountStore = System;
	type Balance = u128;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
	
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ();
}

pub const TREASURY_FLAT_CUT: u128 = 50;

parameter_types! {
	pub const TreasuryAccount: u64 = TREASURY;
	pub const TreasuryFlatCut: u128= TREASURY_FLAT_CUT;
}

impl pallet_imbalance::Config for TestRuntime {
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type TreasuryAccount = TreasuryAccount;
	type TreasuryFlatCut = TreasuryFlatCut;
}

// Mock users AccountId
pub const TREASURY: u64 = 42;
pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub const CHARLY: u64 = 3;

#[derive(Default)]
pub struct ExtBuilder {
	caps_endowed_accounts: Vec<(u64, u128)>,
}

impl ExtBuilder {
	pub fn balances(mut self, accounts: Vec<(u64, u128)>) -> Self {
		for account in accounts {
			self.caps_endowed_accounts.push(account);
		}
		self
	}

	pub fn build(self) -> sp_io::TestExternalities {
		let mut storage = <GenesisConfig<TestRuntime> as BuildStorage>::build_storage(&GenesisConfig::default()).unwrap();

		pallet_balances::GenesisConfig::<TestRuntime> {
			balances: self.caps_endowed_accounts,
		}
		.assimilate_storage(&mut storage)
		.unwrap();

		let mut ext = sp_io::TestExternalities::new(storage);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}
