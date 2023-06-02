// vim: expandtab shiftwidth=4 tabstop=4:

/* This will be my implementation of FEAL-4 in Rust. Primarily my goal here
 * is to try out what's described in this YouTube video: https://www.youtube.com/watch?v=xav-GUO_o4s
 * FEAL on Wikipedia: https://en.wikipedia.org/wiki/FEAL
 * Actually, "Applied Cryptography" by Bruce Schneier has a section on FEAL in Chapter 13.4 (pp 308 on my copy).
 */

fn u32tou8(inp: u32) -> (u8, u8, u8, u8) {
    let a = ((inp >> 24) & 0x00ff) as u8;
    let b = ((inp >> 16) & 0x00ff) as u8;
    let c = ((inp >>  8) & 0x00ff) as u8;
    let d = ((inp >>  0) & 0x00ff) as u8;

    (a, b, c, d)
}

fn u8tou32(a: u8, b: u8, c: u8, d: u8) -> u32 {
    let number = ((a as u32) << 24) | ((b as u32) << 16) | ((c as u32) << 8) | d as u32;
    number
}

fn u32tou16(inp: u32) -> (u16, u16) {
    let a = ((inp >> 16) & 0x00ffff) as u16;
    let b = ((inp >>  0) & 0x00ffff) as u16;
    (a, b)
}

fn u16tou32(a: u16, b: u16) -> u32 {
    let number = ((a as u32) << 24) | b as u32;
    number
}

fn gx(x: u8, a: u8, b: u8) -> u8 {
    // gx(a, b) = rotate left two bits((a+b+x) mod 256)
    let int = a.wrapping_add(b).wrapping_add(x);
    let rot = ((int << 2) & 0xfc) | ((int & 0xc0) >> 6);
    rot
}

fn g0(inp1: u8, inp2: u8) -> u8 { gx(0, inp1, inp2) }
fn g1(inp1: u8, inp2: u8) -> u8 { gx(1, inp1, inp2) }

fn f(a: u8, b: u8, c: u8, d: u8) -> (u8, u8, u8, u8) {
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
    let v4 = g0(v2, v3);
    let v5 = g0(a, v3);
    let v6 = g1(d, v4);
    let ap = v5;
    let bp = v3;
    let cp = v4;
    let dp = v6;
    (ap, bp, cp, dp)
}

fn fk(a0: u8, a1: u8, a2: u8, a3: u8, b0: u8, b1: u8, b2: u8, b3: u8) -> (u8, u8, u8, u8) {
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

fn fk32(a: u32, b: u32) -> u32 {
    let (a0, a1, a2, a3) = u32tou8(a);
    let (b0, b1, b2, b3) = u32tou8(b);

    let (o0, o1, o2, o3) = fk(a0, a1, a2, a3, b0, b1, b2, b3);
    u8tou32(o0, o1, o2, o3)
}

fn keyround(a0: u32, b0: u32, d0: u32) -> (u16, u16, u32, u32, u32) {
    let k01 = fk32(a0, b0);
    let (k0, k1) = u32tou16(k01);
    (k0, k1, b0, k01, a0)
}

fn keygen(a: u32, b: u32) -> (u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16, u16) {
    let d: u32 = 0;
    let mut subkeys: [u16; 16] = [0_u16; 16];

    for idx in 0..8 {
        let (k0, k1, a1, b1, d) = keyround(a, b, d);
        subkeys[idx*2 + 0] = k0;
        subkeys[idx*2 + 1] = k1;
    }

    (
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
    )
}

fn main() {
}
