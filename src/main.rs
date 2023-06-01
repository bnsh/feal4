// vim: expandtab shiftwidth=4 tabstop=4:

/* This will be my implementation of FEAL-4 in Rust. Primarily my goal here
 * is to try out what's described in this YouTube video: https://www.youtube.com/watch?v=xav-GUO_o4s
 * FEAL on Wikipedia: https://en.wikipedia.org/wiki/FEAL
 * Actually, "Applied Cryptography" by Bruce Schneier has a section on FEAL in Chapter 13.4 (pp 308 on my copy).
 */

fn gx(x: u8, a: u8, b: u8) -> u8 {
    // gx(a, b) = rotate left two bits((a+b+x) mod 256)
    let int = a.wrapping_add(b).wrapping_add(x);
    let rot = ((int << 2) & 0xfc) | ((int & 0xc0) >> 6);
    rot
}

fn g0(inp1: u8, inp2: u8) -> u8 { gx(0, inp1, inp2) }
fn g1(inp1: u8, inp2: u8) -> u8 { gx(1, inp1, inp2) }

fn f(a: u8, b: u8, c: u8, d: u8) {
    // "Applied Cryptography" Bruce Schneier 13.4 Figure 13.4
    // To really translate this from Bruce Schneier's diagram:
    // a = a0
    // b = b0 ^ a1
    // c = b1 ^ a2
    // d = a3
    // Where b0 and b1 are the "keys":
    //  "Function f takes the 32 bits of data and 16 bits of key material and mixes them together"
    let v1 = a ^ b;
    let v2 = c ^ d;
    let v3 = g1(v1, v2);
    let v4 = g0(v3, v4);
    let v5 = g0(a, v3);
    let v6 = g1(d, v4);
    let ap = v5;
    let bp = v3;
    let cp = v4;
    let dp = v6;
    (ap, bp, cp, dp)
}

fn fk(a0: u8, a1: u8, a2: u8, a3: u8, b0: u8, b1: u8, b2: u8, b3: u8) {
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

fn main() {
}
