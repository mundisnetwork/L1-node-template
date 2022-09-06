#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
    fn set_history_depth(b: u32, m: u32, ) -> Weight;
    fn force_set_era_payout() -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    // Storage: CurrentEra (r:1 w:0)
    // Storage: HistoryDepth (r:1 w:1)
    // Storage: ErasValidatorReward (r:0 w:1)
    // Storage: ErasRewardPoints (r:0 w:1)
    // Storage: ErasStakers (r:0 w:1)
    // Storage: ErasTotalStake (r:0 w:1)
    // Storage: ErasStartSessionIndex (r:0 w:1)
    fn set_history_depth(_b: u32, _m: u32, ) -> Weight {
        (453_275_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }

    // Storage: EraPayout (r:0 w:1)
    fn force_set_era_payout() -> Weight {
        (81_743_000 as Weight)
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
}
