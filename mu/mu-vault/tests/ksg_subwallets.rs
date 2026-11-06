//! P11c (Spec S5 §3.2, T6 and T8): one sub-wallet per capsule, derived inside the
//! vault from the owner's main wallet key and the capsule's identifier. The history of
//! one payer must not tie the anchors of one capsule to another's, and no key leaves
//! the vault — only public keys and signatures do.
#![cfg(all(feature = "softvault", feature = "ksg"))]

use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use mu_vault::backend::SoftVault;
use mu_vault::Vault;

const MAIN: [u8; 32] = [7u8; 32];

fn vault() -> SoftVault {
    SoftVault::for_test([1u8; 32], [2u8; 32], MAIN)
}

fn verifies(public: [u8; 32], msg: &[u8], sig: [u8; 64]) -> bool {
    VerifyingKey::from_bytes(&public)
        .map(|vk| vk.verify(msg, &Signature::from_bytes(&sig)).is_ok())
        .unwrap_or(false)
}

#[test]
fn per_capsule_wallet() {
    let v = vault();
    let main = v.tx_signer_ed25519().expect("the main wallet");
    let a = v.capsule_signer_ed25519("1220aaaa").expect("capsule a");
    let b = v.capsule_signer_ed25519("1220bbbb").expect("capsule b");
    let a_again = v.capsule_signer_ed25519("1220aaaa").expect("capsule a again");

    // Two capsules, two wallets, neither is the main one; one capsule keeps its wallet.
    assert_ne!(a.public_key(), b.public_key(), "two capsules share one payer");
    assert_ne!(a.public_key(), main.public_key(), "a sub-wallet is not the main wallet");
    assert_ne!(b.public_key(), main.public_key());
    assert_eq!(a.public_key(), a_again.public_key(), "a capsule's wallet is stable");

    // A's signature is A's alone: it does not verify under B's address or the main one.
    let sig = a.sign_message(b"anchor of capsule a");
    assert!(verifies(a.public_key(), b"anchor of capsule a", sig));
    assert!(!verifies(b.public_key(), b"anchor of capsule a", sig));
    assert!(!verifies(main.public_key(), b"anchor of capsule a", sig));

    // Another owner's vault (another main key) has other sub-wallets for the same capsule.
    let other = SoftVault::for_test([1u8; 32], [2u8; 32], [8u8; 32]);
    assert_ne!(
        other.capsule_signer_ed25519("1220aaaa").expect("other").public_key(),
        a.public_key()
    );
}

#[test]
fn a_capsule_identifier_cannot_be_split_to_collide() {
    // The identifier is length-prefixed in the derivation: "ab"+"c" is not "a"+"bc".
    let v = vault();
    let x = v.capsule_signer_ed25519("abc").expect("x").public_key();
    let y = v.capsule_signer_ed25519("ab").expect("y").public_key();
    assert_ne!(x, y);
    assert!(v.capsule_signer_ed25519("").is_err(), "no capsule, no sub-wallet");
}

#[test]
fn agent_never_sees_wallet_key() {
    let v = vault();
    let a = v.capsule_signer_ed25519("1220aaaa").expect("capsule a");
    // What leaves the handle: a public key and signatures. Neither carries the
    // main key. The handle has no Debug and no Clone by construction (printing or
    // copying it would not compile); what can be seen is checked here.
    let public = a.public_key();
    let sig = a.sign_message(b"x");
    let has = |hay: &[u8], needle: &[u8]| hay.windows(needle.len()).any(|w| w == needle);
    assert!(!has(&public, &MAIN) && !has(&sig, &MAIN), "the main key is in the output");
    assert_eq!(public.len(), 32);
}
