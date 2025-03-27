#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_simple_zk::WeightInfo for WeightInfo<T> {
    fn register_verification_key() -> Weight {
        Weight::from_parts(10_000, 0)
            .saturating_add(T::DbWeight::get().writes(1))
    }
    
    fn verify_proof() -> Weight {
        Weight::from_parts(20_000, 0)
            .saturating_add(T::DbWeight::get().reads(1))
    }
}