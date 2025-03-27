#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::*,
    traits::StorageVersion,
};
use frame_system::pallet_prelude::*;
use sp_runtime::traits::Hash;
use sp_std::vec::Vec;

// Storage version
const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    /// Pallet configuration
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        
        /// Hashing algorithm used for verification keys
        type Hashing: Hash<Output = <Self as frame_system::Config>::Hash>;
    }

    /// Storage for registered verification keys
    #[pallet::storage]
    pub type VerificationKeys<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash, // Key hash
        Vec<u8>, // Verification key bytes
    >;

    /// Events for the pallet
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A new verification key was registered
        VerificationKeyRegistered { vk_hash: T::Hash },
        /// A proof was successfully verified
        ProofVerified { vk_hash: T::Hash },
    }

    /// Errors for the pallet
    #[pallet::error]
    pub enum Error<T> {
        /// Verification key already exists
        VerificationKeyAlreadyExists,
        /// Proof verification failed
        VerificationFailed,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register a new verification key
        #[pallet::weight(10_000)]
        pub fn register_verification_key(
            origin: OriginFor<T>,
            vk: Vec<u8>,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            let vk_hash = T::Hashing::hash(&vk);
            
            ensure!(
                !VerificationKeys::<T>::contains_key(vk_hash),
                Error::<T>::VerificationKeyAlreadyExists
            );

            VerificationKeys::<T>::insert(vk_hash, vk);
            Self::deposit_event(Event::VerificationKeyRegistered { vk_hash });

            Ok(())
        }

        /// Verify a proof against a registered verification key
        #[pallet::weight(20_000)]
        pub fn verify_proof(
            origin: OriginFor<T>,
            vk_hash: T::Hash,
            proof: Vec<u8>,
            public_inputs: Vec<u8>,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            // In a real implementation, this would perform actual ZKP verification
            // For our simple example, we just check if the VK exists
            ensure!(
                VerificationKeys::<T>::contains_key(vk_hash),
                Error::<T>::VerificationFailed
            );

            // Simulate successful verification
            Self::deposit_event(Event::ProofVerified { vk_hash });

            Ok(())
        }
    }
}