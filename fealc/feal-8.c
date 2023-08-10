/*
  Author:  Pate Williams (c) 1997

  FEAL-8. Fast Data Encipherment Algorithm.
  See "Handbook of Applied Cryptography" by
  Alfred J. Menezes et al 7.5 Section pages
  259 - 262.
*/

#include <stdio.h>

long Sd(long d, long x, long y)
{
  long sum = ((x + y + d) % 256) << 2;

  return (sum | ((sum & 768) >> 8)) & 255;
}

void f(const long *A, const long *Y, long *U)
{
  long t1 = (A[0] ^ A[1]) ^ Y[0];
  long t2 = (A[2] ^ A[3]) ^ Y[1];

  U[1] = Sd(1, t1, t2);
  U[2] = Sd(0, t2, U[1]);
  U[0] = Sd(0, A[0], U[1]);
  U[3] = Sd(1, A[3], U[2]);
}

void fK(const long *A, const long *B, long *U)
{
  long t1 = A[0] ^ A[1];
  long t2 = A[2] ^ A[3];

  U[1] = Sd(1, t1, t2 ^ B[0]);
  U[2] = Sd(0, t2, U[1] ^ B[1]);
  U[0] = Sd(0, A[0], U[1] ^ B[2]);
  U[3] = Sd(1, A[3], U[2] ^ B[3]);
}

void FEAL_key_schedule(long key0, long key1, long *K)
{
  long i, j, i2, U[4], U0[4], U1[4], U2[4], V[4];

  for (i = 0; i < 4; i++) U2[i] = 0;
  U1[0] = key0 >> 24;
  U1[1] = key0 >> 16;
  U1[2] = key0 >>  8;
  U1[3] = key0 & 255;
  U0[0] = key1 >> 24;
  U0[1] = key1 >> 16;
  U0[2] = key1 >>  8;
  U0[3] = key1 & 255;
  for (i = 1; i <= 8; i++) {
    for (j = 0; j < 4; j++)
      V[j] = U0[j] ^ U2[j];
    fK(U1, V, U);


    i2 = 2 * i;
    K[i2 - 2] = (U[0] << 8) | U[1];
    K[i2 - 1] = (U[2] << 8) | U[3];
    // U2, U1, U0 = U1, U0, U
    for (j = 0; j < 4; j++) U2[j] = U1[j];
    for (j = 0; j < 4; j++) U1[j] = U0[j];
    for (j = 0; j < 4; j++) U0[j] = U[j];
  }
}

void FEAL_encryption(long M0, long M1,
                     long *C0, long *C1, const long *K)
{
  long i, j, L, L8, ML, MR, R, R8;
  long L0[4], L1[4], R0[4], R1[4], U[4], Y[2];

  ML = M0, MR = M1;
  L = ML ^ ((K[8]  << 16) | K[9]);
  R = MR ^ ((K[10] << 16) | K[11]);
  R ^= L;
  L0[0] = L >> 24;
  L0[1] = L >> 16;
  L0[2] = L >>  8;
  L0[3] = L & 255;
  R0[0] = R >> 24;
  R0[1] = R >> 16;
  R0[2] = R >>  8;
  R0[3] = R & 255;
  for (i = 0; i < 8; i++) {
    for (j = 0; j < 4; j++) L1[j] = R0[j];
    Y[0] = K[i] >> 8;
    Y[1] = K[i] & 255;
    f(R0, Y, U);
    for (j = 0; j < 4; j++) {
      R1[j] = (L0[j] ^ U[j]) & 255;
      L0[j] = L1[j];
      R0[j] = R1[j];
    }
  }
  L8  = L1[0] << 24;
  L8 |= L1[1] << 16;
  L8 |= L1[2] <<  8;
  L8 |= L1[3] & 255;
  R8  = R1[0] << 24;
  R8 |= R1[1] << 16;
  R8 |= R1[2] <<  8;
  R8 |= R1[3] & 255;
  L8 ^= R8;
  L8 ^= (K[14] << 16) | K[15];
  R8 ^= (K[12] << 16) | K[13];
  *C0 = R8;
  *C1 = L8;
}

void FEAL_decryption(long C0, long C1,
                     long *M0, long *M1, const long *K)
{
  long i, j, L, L8, ML, MR, R, R8;
  long L0[4], L1[4], R0[4], R1[4], U[4], Y[2];

  ML = C0, MR = C1;
  L = ML ^ ((K[12] << 16) | K[13]);
  R = MR ^ ((K[14] << 16) | K[15]);
  R ^= L;
  L0[0] = L >> 24;
  L0[1] = L >> 16;
  L0[2] = L >>  8;
  L0[3] = L & 255;
  R0[0] = R >> 24;
  R0[1] = R >> 16;
  R0[2] = R >>  8;
  R0[3] = R & 255;
  for (i = 7; i >= 0; i--) {
    for (j = 0; j < 4; j++) L1[j] = R0[j];
    Y[0] = K[i] >> 8;
    Y[1] = K[i] & 255;
    f(R0, Y, U);
    for (j = 0; j < 4; j++) {
      R1[j] = (L0[j] ^ U[j]) & 255;
      L0[j] = L1[j];
      R0[j] = R1[j];
    }
  }
  L8  = L1[0] << 24;
  L8 |= L1[1] << 16;
  L8 |= L1[2] <<  8;
  L8 |= L1[3] & 255;
  R8  = R1[0] << 24;
  R8 |= R1[1] << 16;
  R8 |= R1[2] <<  8;
  R8 |= R1[3] & 255;
  L8 ^= R8;
  R8 ^= (K[8]  << 16) | K[9];
  L8 ^= (K[10] << 16) | K[11];
  *M0 = R8;
  *M1 = L8;
}

typedef struct {
  long key, plaintext, ciphertext;
} testset;

#define KEYCOUNT 16
int main(void)
{
  testset testsets[KEYCOUNT] = {
    { 0x3cbd13f22023727c, 0xedc8b33d6344c781, 0x3d654171ab30b1bb },
    { 0x6bac3915a1b88add, 0x76d8bf1519b68cae, 0xe007894f2e751901 },
    { 0xcf8a9d8c494b0a65, 0xfed43845a77a7f74, 0xc678d98aad11154c },
    { 0x6455d0cc4ea18240, 0xd8736669126af6d5, 0x961071da242819f2 },
    { 0xe30c78de8f7689c3, 0x5fde2d0149690466, 0x88708d97b2e1420d },
    { 0xbdb9d378bd744fbb, 0x43532e649b146e0e, 0x70942299f547e446 },
    { 0x4ac608fc50a1a4b8, 0xcc0d288306b6461e, 0x635384869518f634 },
    { 0xe3836d1b3af096ef, 0x9a55a847ed07e367, 0x4ed9a279bc33bf4e },
    { 0xc8510a1c1c48d70b, 0xcf709bb3fdd682f6, 0x1221cff9cd814903 },
    { 0x659b423b15149925, 0x9e3653fe032580c1, 0x345ad2605d0c88b4 },
    { 0x16e3e6c552b82e35, 0xb0219c72cd262ca3, 0xf30c2fe6dc25d0b4 },
    { 0x1431fe1f9624057f, 0xefa7a3e0d1335a2c, 0x891d2350c9435c24 },
    { 0xc583c4d39033f8c2, 0xbd62d107c7244a70, 0xb40726856129a3b6 },
    { 0xf9f7eb2415a714f9, 0x2ccdcb66fc39b76a, 0x841c2631f3337d8a },
    { 0x13b36ad74dd823aa, 0x08b162231d39825a, 0xb7fcf75254a7b7a3 },
    { 0x8f8972bbadcdbef7, 0x217f68e23fda4895, 0x86ece13815a1eb6e },
  };

  for (int i = 0; i < KEYCOUNT; ++i) {
    long key0 = (testsets[i].key >> 32) & 0x00ffffffffL;
    long key1 =  testsets[i].key & 0x00ffffffffL;
    long M0 = (testsets[i].plaintext >> 32) & 0x00ffffffffL;
    long M1 =  testsets[i].plaintext & 0x00ffffffffL;
    long E0 = (testsets[i].ciphertext >> 32) & 0x00ffffffffL;
    long E1 =  testsets[i].ciphertext & 0x00ffffffffL;

    long C0, C1, D0, D1;
    long K[16];
    FEAL_key_schedule(key0, key1, K);
    FEAL_encryption(M0, M1, &C0, &C1, K);
    FEAL_decryption(C0, C1, &D0, &D1, K);
    printf("decrypt(encrypt(0x%lx%lx) -> 0x%lx%lx) -> 0x%lx%lx %s\n",
      M0, M1, C0, C1, D0, D1,
      (C0 == E0 && C1 == E1 && D0 == M0 && D1 == M1) ? "success": "fail");
  }
}
