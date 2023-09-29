// vim: expandtab shiftwidth=4 tabstop=4:

/* This will be my implementation of FEAL-4 in Rust. Primarily my goal here
 * is to try out what's described in this YouTube video: https://www.youtube.com/watch?v=xav-GUO_o4s
 * FEAL on Wikipedia: https://en.wikipedia.org/wiki/FEAL
 * Actually, "Applied Cryptography" by Bruce Schneier has a section on FEAL in Chapter 13.4 (pp 308 on my copy).
 */

pub fn u64tou32(inp: u64) -> (u32, u32) {
    let a = ((inp >> 32) & 0x00ffffffff) as u32;
    let b = ((inp >>  0) & 0x00ffffffff) as u32;
    (a, b)
}

pub fn u32tou64(a: u32, b:u32) -> u64 {
    let number = ((a as u64) << 32) | b as u64;
    number
}

pub fn u16tou64(a: u16, b:u16, c: u16, d: u16) -> u64 {
    let number = ((a as u64) << 48) | ((b as u64) << 32) | ((c as u64) << 16) | d as u64;
    number
}

pub fn u32tou8(inp: u32) -> (u8, u8, u8, u8) {
    let a = ((inp >> 24) & 0x00ff) as u8;
    let b = ((inp >> 16) & 0x00ff) as u8;
    let c = ((inp >>  8) & 0x00ff) as u8;
    let d = ((inp >>  0) & 0x00ff) as u8;

    (a, b, c, d)
}

pub fn u8tou32(a: u8, b: u8, c: u8, d: u8) -> u32 {
    let number = ((a as u32) << 24) | ((b as u32) << 16) | ((c as u32) << 8) | d as u32;
    number
}

pub fn u32tou16(inp: u32) -> (u16, u16) {
    let a = ((inp >> 16) & 0x00ffff) as u16;
    let b = ((inp >>  0) & 0x00ffff) as u16;
    (a, b)
}

pub fn u16tou8(inp: u16) -> (u8, u8) {
    let a = ((inp >> 8) & 0x00ff) as u8;
    let b = ((inp >> 0) & 0x00ff) as u8;
    (a, b)
}

pub fn gx(x: u8, a: u8, b: u8) -> u8 {
    // Compared against feal-8 from https://www.schneier.com/wp-content/uploads/2015/03/FEAL8-WI-2.zip
    // and verified to be working.
    // gx(a, b) = rotate left two bits((a+b+x) mod 256)
    // gx corresponds to Sd in FEAL8
    let int = a.wrapping_add(b).wrapping_add(x);
    let rot = ((int << 2) & 0xfc) | ((int & 0xc0) >> 6);
    rot
}

pub fn g0(inp1: u8, inp2: u8) -> u8 { gx(0, inp1, inp2) }
pub fn g1(inp1: u8, inp2: u8) -> u8 { gx(1, inp1, inp2) }

pub fn fyoutube(a: u8, b: u8, c: u8, d: u8) -> (u8, u8, u8, u8) {
    // "Applied Cryptography" Bruce Schneier 13.4 Figure 13.4
    // This is _actually_ from this YouTube video: https://www.youtube.com/watch?v=xav-GUO_o4s#t=965
    // To really translate this from Bruce Schneier's diagram:
    // a = a0
    // b = b0 ^ a1
    // c = b1 ^ a2
    // d = a3
    // Where b0 and b1 are the "keys":
    //  "Function f takes the 32 bits of data and 16 bits of key material and mixes them together"
    // The reason for the difference between the YouTube video and what's in Bruce Schneier's diagram
    // is that the YouTube video is illustrating differential cryptanalysis, and in differential
    // cryptanalyis one is passing in _pairs_ of inputs, that differ in some specified way, while
    // keeping the keys constant. If you have constant inputs, say k0 and k1 and k0 == k1 then
    // k0 ^ k1 = 0 and 0 ^ a1 just becomes a1. (and similarly for 0 ^ a2 just becomes a2.)
    let v1 = a ^ b;
    let v2 = c ^ d;
    let v3 = g1(v1, v2);
    let v4 = g0(v2, v3);
    let v5 = g0(a, v3);
    let v6 = g1(d, v4);
    let ap = v5;
    let bp = v3;
    let cp = v4;
    let dp = v6;
    (ap, bp, cp, dp)
}

pub fn f(b: u16, a: u32) -> u32 {
    // "Applied Cryptography" Bruce Schneier 13.4 Figure 13.4
    // b0, b1 are the "keys"
    // a0, a1, a2, a3 are something. (Look at the diagram)
    // Compared against feal-8 from https://www.schneier.com/wp-content/uploads/2015/03/FEAL8-WI-2.zip
    // and verified to be working.
    //
    // This function (f) corresponds to f in that code as well.
    //  This function _and_ fyoutube in our code are what's in their f function.
    //  A corresponds to a in our code. (32 bits)
    //  Y corresponds to b in our code. (16 bits)
    //  U corresponds to our return value in our code. (32 bits)
    // Warning: Remember that _their_ code has _A_ first, and _then_ Y, and _our_ code has B first and _then_ A.

    let (b0, b1) = u16tou8(b);
    let (a0, a1, a2, a3) = u32tou8(a);

    let (ap, bp, cp, dp) = fyoutube(a0, b0 ^ a1, b1 ^ a2, a3);

    u8tou32(ap, bp, cp, dp)
}

pub fn fk(a0: u8, a1: u8, a2: u8, a3: u8, b0: u8, b1: u8, b2: u8, b3: u8) -> (u8, u8, u8, u8) {
    // "Applied Cryptography" Bruce Schneier 13.4 Figure 13.6
    let v1 = a0 ^ a1;
    let v2 = a2 ^ a3;
    let v3 = v2 ^ b0;
    let v4 = g1(v1, v3);
    let v5 = v4 ^ b1;
    let v6 = g0(v2, v5);
    let v7 = v6 ^ b3;
    let v8 = v4 ^ b2;
    let f1 = g0(a0, v8);
    let f2 = v4;
    let f3 = v6;
    let f4 = g1(a3, v7);
    (f1, f2, f3, f4)
}

pub fn fk32(a: u32, b: u32) -> u32 {
    // Compared against feal-8 from https://www.schneier.com/wp-content/uploads/2015/03/FEAL8-WI-2.zip
    // and verified to be working.
    // This function (fk32) corresponds to fK in that code.
    let (a0, a1, a2, a3) = u32tou8(a);
    let (b0, b1, b2, b3) = u32tou8(b);

    let (o0, o1, o2, o3) = fk(a0, a1, a2, a3, b0, b1, b2, b3);
    u8tou32(o0, o1, o2, o3)
}

pub fn keyround(a0: u32, b0: u32, d0: u32) -> (u16, u16, u32, u32, u32) {
    let v = b0 ^ d0;
    let k01 = fk32(a0, v);
    let (k0, k1) = u32tou16(k01);
    (k0, k1, b0, k01, a0)
}

pub fn keygen(a: u32, b: u32) -> [u16; 16] {
    // Key generation seems to differ between
    // https://www.schneier.com/wp-content/uploads/2015/03/FEAL8-WI-2.zip
    // and here.
    // "Applied Cryptography" Bruce Schneier 13.4 Figure 13.5
    // Compared against feal-8 from https://www.schneier.com/wp-content/uploads/2015/03/FEAL8-WI-2.zip
    // and verified to be working!
    // This function (keygen) corresponds to FEAL_key_schedule in that code.
    let mut d: u32 = 0;
    let mut subkeys: [u16; 16] = [0_u16; 16];

    let (mut a, mut b) = (a, b);
    for idx in 0..8 {
        let (k0, k1, ap, bp, dp) = keyround(a, b, d);

        // a in our code is U1 in the reference.
        // b in our code is U0 in the reference
        // bp in our code is V in the reference.
        // d in our code is U2 in the reference.
        // k01 in our code is U in the reference.
        subkeys[idx*2 + 0] = k0;
        subkeys[idx*2 + 1] = k1;
        // U2, U1, U0 = U1, U0, U
        // d, a, b = U1, U0, U
        (a, b, d) = (ap, bp, dp);
    }

    [
        subkeys[0],
        subkeys[1],
        subkeys[2],
        subkeys[3],
        subkeys[4],
        subkeys[5],
        subkeys[6],
        subkeys[7],
        subkeys[8],
        subkeys[9],
        subkeys[10],
        subkeys[11],
        subkeys[12],
        subkeys[13],
        subkeys[14],
        subkeys[15]
    ]
}

pub fn single_round_encrypt(k: u16, left: u32, right: u32) -> (u32, u32) {
    let (left, right) = (left, right);
    let intermediate = f(k, right);
    let (newleft, newright) = (right, left ^ intermediate);
    (newleft, newright)
}

pub fn feal4_raw(k: [u16; 16], input: u64) -> u64 {
    // "Applied Cryptography" Bruce Schneier 13.4 Figure 13.3
    // input: output
    let v1 = input ^ u16tou64(k[8], k[9], k[10], k[11]); // combined
    let (mut left, mut right) = u64tou32(v1);

    right = left ^ right;

    for x in 0..8 {
        (left, right) = single_round_encrypt(k[x], left, right);
    }
    left = left ^ right;
    let combined = u32tou64(right, left); // v1
    let output = combined ^ u16tou64(k[12], k[13], k[14], k[15]); // input

    output
}

pub fn encrypt(keybits: u64, plaintext: u64) -> u64 {
    let (ka, kb) = u64tou32(keybits);
    let k = keygen(ka, kb);
    feal4_raw(k, plaintext)
}

pub fn decrypt(keybits: u64, ciphertext: u64) -> u64 {
    let (ka, kb) = u64tou32(keybits);
    let kraw = keygen(ka, kb);
// k00 k01 k02 k03 k04 k05 k06 k07 k08 k09 k10 k11 k12 k13 k14 k15
// k07 k06 k05 k04 k03 k02 k01 k00 k12 k13 k14 k15 k08 k09 k10 k11
    let mapping : [usize; 16] = [7, 6, 5, 4, 3, 2, 1, 0, 12, 13, 14, 15, 8, 9, 10, 11];
    let mut k : [u16; 16] = [0_u16; 16];
    for (dstidx, srcidx) in mapping.iter().enumerate() {
        k[dstidx] = kraw[*srcidx];
    }
    feal4_raw(k, ciphertext)
}