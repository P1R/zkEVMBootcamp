//use ark_ff::{Field, PrimeField, FpParameters, BigInteger};
use ark_ff::{Field, PrimeField, BigInteger};
use ark_bls12_381::Fq as F; // Prime Field
use ark_std::{One, Zero, UniformRand};


fn main() {
    let mut rng = ark_std::rand::thread_rng();
    // select a random value from the field
    let a = F::rand(&mut rng);
   
    let modulus = <F as PrimeField>::MODULUS;


    // show that 1 + 1 = 2
    let b: num_bigint::BigUint = F::one().into();
    let c: num_bigint::BigUint = F::one().into();
    let d: num_bigint::BigUint = (F::one()+ F::one()).into();
    
    assert_eq!(b+c,d);
    
    // show that the multiplicative inverse of a number multipled by itself equals one. 

    let inverse = a.inverse().unwrap();
    assert_eq!(a*inverse, F::one().into());

    // show that a value raised to the power of the modulus is equal to itself
    // use the `pow` function to raise to a power
    assert_eq!(a.pow(&modulus), a);
}
