#![cfg_attr(not(feature = "std"), no_std)]

use borsh::{BorshDeserialize, BorshSerialize};
use codec::{Decode, Encode};
use scale_info::{prelude::string::String, TypeInfo};
use sp_runtime::{RuntimeDebug, KeyTypeId};
use sp_std::prelude::*;
use frame_support::dispatch::DispatchError;

#[allow(dead_code)]
pub(crate) const LOG_TARGET: &'static str = "runtime::mundis-common";

// syntactic sugar for logging.
#[macro_export]
macro_rules! log {
	($level:tt, $patter:expr $(, $values:expr)* $(,)?) => {
		log::$level!(
			target: crate::LOG_TARGET,
			concat!("[{:?}] 🐙 ", $patter), <frame_system::Pallet<T>>::block_number() $(, $values)*
		)
	};
}


#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum PayloadType {
	Lock,
	BurnAsset,
	PlanNewEra,
	EraPayout,
	LockNft,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct LockPayload {
	pub sender: String,
	pub receiver_id: String,
	pub amount: u128,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct BurnAssetPayload {
	pub token_id: String,
	pub sender: String,
	pub receiver_id: String,
	pub amount: u128,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct PlanNewEraPayload {
	pub new_era: u32,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Offender {
	pub kind: String,
	pub who: String,
	pub offences: u32,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct EraPayoutPayload {
	pub end_era: u32,
	pub excluded_validators: Vec<String>,
	pub offenders: Vec<Offender>,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct LockNftPayload {
	pub sender: String,
	pub receiver_id: String,
	pub class: u128,
	pub instance: u128,
	pub metadata: AxisTokenMetadata,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct AxisTokenMetadata {
    // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
	pub title: Option<String>,
    // free-form description
	pub description: Option<String>,
}

pub trait MinichainInterface {
	fn is_activated() -> bool;
	fn next_set_id() -> u32;
}

/// Something that can provide a set of validators for the next era.
pub trait ValidatorsProvider<AccountId> {
	/// A new set of validators.
	fn validators() -> Vec<(AccountId, u128)>;
}


pub trait TokenIdAndAssetIdProvider<AssetId> {
	type Err;

	fn try_get_asset_id(token_id: impl AsRef<[u8]>) -> Result<AssetId, Self::Err>;
	fn try_get_token_id(asset_id: AssetId) -> Result<Vec<u8>, Self::Err>;
}

pub trait LposInterface<AccountId> {
	fn is_active_validator(id: KeyTypeId, key_data: &[u8]) -> Option<AccountId>;
	fn active_stake_of(who: &AccountId) -> u128;
	fn active_total_stake() -> Option<u128>;
}

pub trait UpwardMessagesInterface<AccountId> {
	fn submit(who: Option<AccountId>, payload_type: PayloadType, payload: &[u8]) -> Result<u64, DispatchError>;
}

pub trait ConvertIntoL0Token {
	type CollectionId;
	type ItemId;

	fn convert_into_l0_token_metadata(class: Self::CollectionId, instance: Self::ItemId) -> Option<AxisTokenMetadata>;
}

impl ConvertIntoL0Token for () {
	type CollectionId = u128;
	type ItemId = u128;

	fn convert_into_l0_token_metadata(_class: Self::CollectionId, _instance: Self::ItemId) -> Option<AxisTokenMetadata> {
		None
	}
}
