#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
    fn on_initialize(m: u32, p: u32, ) -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    // Storage: UpwardMessages MessageQueue (r:1 w:1)
    // Storage: System Digest (r:1 w:1)
    fn on_initialize(m: u32, p: u32, ) -> Weight {
        (107_697_000 as Weight)
            // Standard Error: 723_000
            .saturating_add((37_406_000 as Weight).saturating_mul(m as Weight))
            // Standard Error: 21_000
            .saturating_add((1_204_000 as Weight).saturating_mul(p as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
}