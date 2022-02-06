#![allow(unused_imports)]

use ark_ec::{ProjectiveCurve, AffineCurve};
use ark_ff::{PrimeField, Field};

use ark_bls12_381::{G1Projective as G, G1Affine as GAffine, Fr as ScalarField};
use ark_std::{Zero, UniformRand};

fn main() {
    let alice = generate_key_pair();
    let bob = generate_key_pair();

    let alice_shared_secret = bob.public_key.into_affine().mul(alice.secret_key);
    let bob_shared_secret = alice.public_key.into_affine().mul(bob.secret_key);

    assert_eq!(alice_shared_secret, bob_shared_secret);

    assert_eq!(eval(&[2, 3, 1], 1), 6);
    assert_eq!(eval(&[2, 3, 1], 2), 15);

}

// assume p is in high degree -> low degree
fn eval(p: &[i32], x: i32) -> i32 {
    let mut result = 0;
    for i in p.iter() {
        result *= x;
        result += i;
    }
    return result
}

struct KeyPair {
    secret_key: ScalarField,
    public_key: G,
}

fn generate_key_pair() -> KeyPair {
    let mut rng = ark_std::test_rng();
     
    // Wikipedia says that d needs to be in [0, n - 1]. Is this true? Still confused regarding 
    // the parameters.

    let d = ScalarField::rand(&mut rng);
    let q = GAffine::prime_subgroup_generator().mul(d);

    let r = KeyPair {secret_key: d, public_key: q};
    return r;
}
