// vim: expandtab shiftwidth=4 tabstop=4:

/* This will be my implementation of FEAL-4 in Rust. Primarily my goal here
 * is to try out what's described in this YouTube video: https://www.youtube.com/watch?v=xav-GUO_o4s
 * FEAL on Wikipedia: https://en.wikipedia.org/wiki/FEAL
 * Actually, "Applied Cryptography" by Bruce Schneier has a section on FEAL in Chapter 13.4 (pp 308 on my copy).
 */

use rand::Rng;
pub mod feal;

fn main() {
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

/*
    let mut rng = rand::thread_rng();
    let mut keyplaintextpairs: [(u16, u32); 256] = [(0, 0); 256];
    for (key, plaintext) in keyplaintextpairs.iter_mut() {
        *key = rng.gen_range(0..=u16::MAX);
        *plaintext = rng.gen_range(0..=u32::MAX);
    }

    for (key, plaintext) in keyplaintextpairs.iter() {
        let xoredplaintext = *plaintext ^ 0x80808080;
        let cipher = feal::f(*key, *plaintext);
        let xoredcipher = feal::f(*key, xoredplaintext);

        println!("{:08x} {:08x}", *plaintext ^ xoredplaintext, cipher ^ xoredcipher);
    }
*/

}
