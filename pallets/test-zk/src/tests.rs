use super::*;
use crate::mock::*;
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::Hash;

#[test]
fn register_verification_key_works() {
    new_test_ext().execute_with(|| {
        let vk = vec![1, 2, 3];
        let vk_hash = <Test as pallet_simple_zk::Config>::Hashing::hash(&vk);
        
        // Register a new verification key
        assert_ok!(SimpleZK::register_verification_key(
            RuntimeOrigin::signed(1),
            vk.clone()
        ));
        
        // Check event emitted
        System::assert_last_event(
            Event::SimpleZK(pallet_simple_zk::Event::VerificationKeyRegistered { vk_hash }).into()
        );
        
        // Check storage updated
        assert_eq!(SimpleZK::verification_keys(vk_hash), Some(vk));
        
        // Try to register same key again - should fail
        assert_noop!(
            SimpleZK::register_verification_key(RuntimeOrigin::signed(1), vk),
            pallet_simple_zk::Error::<Test>::VerificationKeyAlreadyExists
        );
    });
}

#[test]
fn verify_proof_works() {
    new_test_ext().execute_with(|| {
        let vk = vec![1, 2, 3];
        let vk_hash = <Test as pallet_simple_zk::Config>::Hashing::hash(&vk);
        let proof = vec![4, 5, 6];
        let inputs = vec![7, 8, 9];
        
        // Register verification key first
        assert_ok!(SimpleZK::register_verification_key(
            RuntimeOrigin::signed(1),
            vk
        ));
        
        // Verify proof
        assert_ok!(SimpleZK::verify_proof(
            RuntimeOrigin::signed(1),
            vk_hash,
            proof,
            inputs
        ));
        
        // Check event emitted
        System::assert_last_event(
            Event::SimpleZK(pallet_simple_zk::Event::ProofVerified { vk_hash }).into()
        );
        
        // Try with non-existent VK - should fail
        let bad_hash = <Test as pallet_simple_zk::Config>::Hashing::hash(&[0]);
        assert_noop!(
            SimpleZK::verify_proof(
                RuntimeOrigin::signed(1),
                bad_hash,
                vec![],
                vec![]
            ),
            pallet_simple_zk::Error::<Test>::