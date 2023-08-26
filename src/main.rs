// vim: expandtab shiftwidth=4 tabstop=4:

/* This will be my implementation of FEAL-4 in Rust. Primarily my goal here
 * is to try out what's described in this YouTube video: https://www.youtube.com/watch?v=xav-GUO_o4s
 * FEAL on Wikipedia: https://en.wikipedia.org/wiki/FEAL
 * Actually, "Applied Cryptography" by Bruce Schneier has a section on FEAL in Chapter 13.4 (pp 308 on my copy).
 */

use rand::Rng;
pub mod feal;

fn main() {
/*
    let mode = 0;
    if mode == 0 {
        let key = 0x0123456789abcdef;
        let plaintext = 0x0;
        let ciphertext = feal::encrypt(key, plaintext);
        let decrypted = feal::decrypt(key, ciphertext);

        println!(" plaintext={plaintext:016x}");
        println!("ciphertext={ciphertext:016x}");
        println!(" decrypted={decrypted:016x}");
    }
    else {
        let mut rng = rand::thread_rng();
        let mut random_numbers: [(u64, u64); 16] = [(0, 0); 16];

        for (key0, key1) in random_numbers.iter_mut() {
            *key0 = rng.gen_range(0..=u64::MAX);
            *key1 = rng.gen_range(0..=u64::MAX);
        }

        println!("#define KEYCOUNT 16");
        println!("  testset testsets[KEYCOUNT] = {{");
        for (key, plaintext) in random_numbers.iter() {
            let ciphertext = feal::encrypt(*key, *plaintext);
            println!("    {{ 0x{key:016x}, 0x{plaintext:016x}, 0x{ciphertext:016x} }},");
        }
        println!("  }};");
    }
*/

/* */
    let mut rng = rand::thread_rng();
    let mut keyplaintextpairs: [(u16, u32); 256] = [(0, 0); 256];
    for (key, plaintext) in keyplaintextpairs.iter_mut() {
        *key = rng.gen_range(0..=u16::MAX);
        *plaintext = rng.gen_range(0..=u32::MAX);
    }

    for (key, plaintext) in keyplaintextpairs.iter() {
        let xoredplaintext = *plaintext ^ 0x80808080;
        // let xoredplaintext = *plaintext ^ 0x40404040;
        // let xoredplaintext = *plaintext ^ 0x20202020;
        let cipher = feal::f(*key, *plaintext);
        let xoredcipher = feal::f(*key, xoredplaintext);

        println!("{:08x} {:08x}", *plaintext ^ xoredplaintext, cipher ^ xoredcipher);
    }
/* */

/*
    for key in 0..=255 {
        let plain_xor = 0x40;
        for plain1 in 0..=255 {
            let plain2 = plain1 ^ plain_xor;
            let cipher1 = feal::g0(plain1, key);
            let cipher2 = feal::g0(plain2, key);
            let cipher_xor = cipher1 ^ cipher2;
            println!("key={:02x}, p1={:02x}, p2={:02x}, {:02x}", key, plain1, plain2, cipher_xor);
        }
    }
*/

/*
    let mut rng = rand::thread_rng();
    let mut keyplaintextpairs: [(u16, u32); 256] = [(0, 0); 256];
    for (bval, aval) in keyplaintextpairs.iter_mut() {
        *bval = rng.gen_range(0..=u16::MAX);
        *aval = rng.gen_range(0..=u32::MAX);
    }

    for (bval, aval) in keyplaintextpairs.iter() {
        let actual = feal::f(*bval, *aval);

        println!("    test_f(0x{bval:04x}, 0x{aval:08x}, 0x{actual:08x})")
    }
 */

/*
    let mut rng = rand::thread_rng();
    let mut random_numbers: [(u64, u64); 16] = [(0, 0); 16];

    for (key, plaintext) in random_numbers.iter_mut() {
        *key = rng.gen_range(0..=u64::MAX);
        *plaintext = rng.gen_range(0..=u64::MAX);
    }

    for (key, plaintext) in random_numbers.iter() {
        let (ka, kb) = feal::u64tou32(*key);
        let subkeys = feal::keygen(ka, kb);
        let ciphertext = feal::feal4_raw(subkeys, *plaintext);

        print!("    test_feal8(0x{plaintext:016x}, [");
        for subkey in subkeys.iter() {
            print!("0x{subkey:04x}, ")
        }
        println!("], 0x{ciphertext:016x})");
    }
 */
}
