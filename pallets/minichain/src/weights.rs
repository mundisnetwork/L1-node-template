#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;


pub trait WeightInfo {
    fn force_set_is_activated() -> Weight;
    fn force_set_next_set_id(b: u32, ) -> Weight;
    fn force_set_planned_validators(b: u32, ) -> Weight;
    fn lock() -> Weight;
    fn mint_asset() -> Weight;
    fn burn_asset() -> Weight;
    fn set_asset_name() -> Weight;
    fn tranfer_from_pallet_account() -> Weight;
    fn lock_nft() -> Weight;
    fn delete_token_id() -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    // Storage: Minichain IsActivated (r:0 w:1)
    fn force_set_is_activated() -> Weight {
        (44_694_000 as Weight)
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Minichain NextSetId (r:0 w:1)
    fn force_set_next_set_id(_b: u32, ) -> Weight {
        (352_705_000 as Weight)
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Minichain PlannedValidators (r:0 w:1)
    fn force_set_planned_validators(_b: u32, ) -> Weight {
        (56_770_000 as Weight)
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Minichain IsActivated (r:1 w:0)
    // Storage: UpwardMessages MessageQueue (r:1 w:1)
    // Storage: UpwardMessages Nonce (r:1 w:1)
    fn lock() -> Weight {
        (870_434_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(3 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:1 w:1)
    fn mint_asset() -> Weight {
        (1_531_204_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Minichain IsActivated (r:1 w:0)
    // Storage: Minichain AssetIdByTokenId (r:2 w:0)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:1 w:1)
    // Storage: UpwardMessages MessageQueue (r:1 w:1)
    // Storage: UpwardMessages Nonce (r:1 w:1)
    fn burn_asset() -> Weight {
        (2_211_922_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(7 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    // Storage: Minichain IsActivated (r:1 w:0)
    // Storage: Minichain AssetIdByTokenId (r:3 w:1)
    fn set_asset_name() -> Weight {
        (694_313_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: System Account (r:1 w:1)
    fn tranfer_from_pallet_account() -> Weight {
        (1_835_074_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Minichain IsActivated (r:1 w:0)
    // Storage: Uniques ClassMetadataOf (r:1 w:0)
    // Storage: Uniques InstanceMetadataOf (r:1 w:0)
    // Storage: Uniques Class (r:1 w:0)
    // Storage: Uniques Asset (r:1 w:1)
    // Storage: UpwardMessages Nonce (r:1 w:1)
    // Storage: UpwardMessages MessageQueue (r:1 w:1)
    // Storage: Uniques Account (r:0 w:2)
    fn lock_nft() -> Weight {
        (2_447_795_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(7 as Weight))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
    }
    // Storage: Minichain AssetIdByTokenId (r:1 w:1)
    fn delete_token_id() -> Weight {
        (207_420_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
}