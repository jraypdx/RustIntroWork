mod lib;

/// Fixed RSA encryption exponent.
pub const EXP: u64 = 65_537;
use std::convert::{TryInto};

/// Generate a pair of primes in the range `2**30..2**31`
/// suitable for RSA encryption with exponent
/// `EXP`. Warning: this routine has unbounded runtime; it
/// works by generate-and-test, generating pairs of primes
/// `p` `q` and testing that they satisfy `λ(pq) <= EXP` and
/// that `λ(pq)` has no common factors with `EXP`.
pub fn genkey() -> (u32, u32) {
    let mut p = lib::rsa_prime() as u32;
    let mut q = lib::rsa_prime() as u32;
    while p == q || (EXP >= lib::lcm(p as u64 - 1, q as u64 - 1) && lib::gcd(EXP, lib::lcm(p as u64 - 1, q as u64 - 1)) != 1) {
        p = lib::rsa_prime() as u32;
        q = lib::rsa_prime() as u32;
    }
    println!("{} {}", p, q);
    (p,q)
}

/// Encrypt the plaintext `msg` using the RSA public `key`
/// and return the ciphertext.
pub fn encrypt(key: u64, msg: u32) -> u64 {
    let o = lib::modexp(msg as u64, EXP, key);
    o
    //let mut m = msg as u64;
    //m = m.pow(EXP.try_into().unwrap()) % key;
    //let o = u64::try_from(m).unwrap();
    //m
}

/// Decrypt the cipertext `msg` using the RSA private `key`
/// and return the resulting plaintext.
pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {
    let l = lib::lcm((key.0 - 1) as u64, (key.1 - 1) as u64);
    let d = lib::modinverse(EXP, l);
    let m = lib::modexp(msg, d, key.0 as u64 * key.1 as u64);
    let o: u32 = m.try_into().unwrap();
    o
    //let k = (key.0 * key.1) as u64;
    //let d = 1 / (EXP % k);
    //let o = msg.pow(d.try_into().unwrap()) % k;
    //o as u32
}

fn main() {
    let message: u32 = rand::random();
    let (p, q) = genkey();
    let key = p as u64 * q as u64;
    let encrypted = encrypt(key, message);
    let decrypted = decrypt((p, q), encrypted);
    println!("Message before encryption: {}\nMessage after encryption: {}\nMessage after decryption:{}", message, encrypted, decrypted);
}