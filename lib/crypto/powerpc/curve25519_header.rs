// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2024- IBM Corp.
 *
 * X25519 scalar multiplication with 51 bits limbs for PPC64le.
 *   Based on RFC7748 and AArch64 optimized implementation for X25519
 *     - Algorithm 1 Scalar multiplication of a variable point
 */

// C dependencies: linux/types.h, linux/jump_label.h, linux/kernel.h,
// linux/cpufeature.h, and linux/processor.h.

pub type fe51 = [u64; 5];

extern "C" {
    pub fn x25519_fe51_mul(h: *mut u64, f: *const u64, g: *const u64);
    pub fn x25519_fe51_sqr(h: *mut u64, f: *const u64);
    pub fn x25519_fe51_mul121666(h: *mut u64, f: *mut u64);
    pub fn x25519_fe51_sqr_times(h: *mut u64, f: *const u64, n: i32);
    pub fn x25519_fe51_frombytes(h: *mut u64, s: *const u8);
    pub fn x25519_fe51_tobytes(s: *mut u8, h: *const u64);
    pub fn x25519_cswap(p: *mut u64, q: *mut u64, bit: u32);
}

// #define fmul x25519_fe51_mul
// #define fsqr x25519_fe51_sqr
// #define fmul121666 x25519_fe51_mul121666
// #define fe51_tobytes x25519_fe51_tobytes

unsafe fn fadd(h: *mut u64, f: *const u64, g: *const u64) {
    for i in 0..5 { *h.add(i) = (*f.add(i)).wrapping_add(*g.add(i)); }
}

/*
 * Prime = 2 ** 255 - 19, 255 bits
 *    (0x7fffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffff ffffffed)
 *
 * Prime in 5 51-bit limbs
 */
static mut prime51: fe51 = [0x7ffffffffffed, 0x7ffffffffffff, 0x7ffffffffffff, 0x7ffffffffffff, 0x7ffffffffffff];

unsafe fn fsub(h: *mut u64, f: *const u64, g: *const u64) {
    for i in 0..5 { *h.add(i) = (*f.add(i)).wrapping_add(prime51[i].wrapping_mul(2)).wrapping_sub(*g.add(i)); }
}

unsafe fn fe51_frombytes(h: *mut u64, s: *const u8) {
    /* Make sure 64-bit aligned. */
    let mut sbuf = [0u8; 40];
    let sb = sbuf.as_mut_ptr().add((8 - (sbuf.as_mut_ptr() as usize & 7)) & 7);
    core::ptr::copy_nonoverlapping(s, sb, 32);
    x25519_fe51_frombytes(h, sb);
}

unsafe fn finv(o: *mut u64, i: *const u64) {
    let mut a0 = [0u64; 5]; let mut b = [0u64; 5]; let mut c = [0u64; 5]; let mut t00 = [0u64; 5];
    x25519_fe51_sqr(a0.as_mut_ptr(), i); x25519_fe51_sqr_times(t00.as_mut_ptr(), a0.as_ptr(), 2);
    x25519_fe51_mul(b.as_mut_ptr(), t00.as_ptr(), i); x25519_fe51_mul(a0.as_mut_ptr(), b.as_ptr(), a0.as_ptr());
    x25519_fe51_sqr(t00.as_mut_ptr(), a0.as_ptr()); x25519_fe51_mul(b.as_mut_ptr(), t00.as_ptr(), b.as_ptr());
    x25519_fe51_sqr_times(t00.as_mut_ptr(), b.as_ptr(), 5); x25519_fe51_mul(b.as_mut_ptr(), t00.as_ptr(), b.as_ptr());
    x25519_fe51_sqr_times(t00.as_mut_ptr(), b.as_ptr(), 10); x25519_fe51_mul(c.as_mut_ptr(), t00.as_ptr(), b.as_ptr());
    x25519_fe51_sqr_times(t00.as_mut_ptr(), c.as_ptr(), 20); x25519_fe51_mul(t00.as_mut_ptr(), t00.as_ptr(), c.as_ptr());
    x25519_fe51_sqr_times(t00.as_mut_ptr(), t00.as_ptr(), 10); x25519_fe51_mul(b.as_mut_ptr(), t00.as_ptr(), b.as_ptr());
    x25519_fe51_sqr_times(t00.as_mut_ptr(), b.as_ptr(), 50); x25519_fe51_mul(c.as_mut_ptr(), t00.as_ptr(), b.as_ptr());
    x25519_fe51_sqr_times(t00.as_mut_ptr(), c.as_ptr(), 100); x25519_fe51_mul(t00.as_mut_ptr(), t00.as_ptr(), c.as_ptr());
    x25519_fe51_sqr_times(t00.as_mut_ptr(), t00.as_ptr(), 50); x25519_fe51_mul(t00.as_mut_ptr(), t00.as_ptr(), b.as_ptr());
    x25519_fe51_sqr_times(t00.as_mut_ptr(), t00.as_ptr(), 5); x25519_fe51_mul(o, t00.as_ptr(), a0.as_ptr());
}

unsafe fn curve25519_fe51(out: *mut u8, scalar: *const u8, point: *const u8) {
    let mut x1 = [0u64;5]; let mut x2 = [0u64;5]; let mut z2 = [0u64;5]; let mut x3 = [0u64;5]; let mut z3 = [0u64;5];
    let mut s = [0u8;32]; core::ptr::copy_nonoverlapping(scalar, s.as_mut_ptr(), 32); s[0]&=0xf8; s[31]&=0x7f; s[31]|=0x40;
    fe51_frombytes(x1.as_mut_ptr(), point); x3=x1; x2[0]=1; z3[0]=1;
    let mut swap=0u32;
    for i in (0..=254).rev() { let k_t=1 & ((s[i/8] >> (i&7)) as u32); swap^=k_t; x25519_cswap(x2.as_mut_ptr(),x3.as_mut_ptr(),swap); x25519_cswap(z2.as_mut_ptr(),z3.as_mut_ptr(),swap); swap=k_t;
        let (mut a,mut b,mut c,mut d,mut e,mut da,mut cb,mut aa,mut bb,mut dacb_p,mut dacb_m)=([0u64;5],[0u64;5],[0u64;5],[0u64;5],[0u64;5],[0u64;5],[0u64;5],[0u64;5],[0u64;5],[0u64;5],[0u64;5]);
        fsub(b.as_mut_ptr(),x2.as_ptr(),z2.as_ptr()); fadd(a.as_mut_ptr(),x2.as_ptr(),z2.as_ptr()); fsub(d.as_mut_ptr(),x3.as_ptr(),z3.as_ptr()); fadd(c.as_mut_ptr(),x3.as_ptr(),z3.as_ptr());
        x25519_fe51_sqr(bb.as_mut_ptr(),b.as_ptr()); x25519_fe51_sqr(aa.as_mut_ptr(),a.as_ptr()); x25519_fe51_mul(da.as_mut_ptr(),d.as_ptr(),a.as_ptr()); x25519_fe51_mul(cb.as_mut_ptr(),c.as_ptr(),b.as_ptr()); fsub(e.as_mut_ptr(),aa.as_ptr(),bb.as_ptr()); x25519_fe51_mul(x2.as_mut_ptr(),aa.as_ptr(),bb.as_ptr()); fadd(dacb_p.as_mut_ptr(),da.as_ptr(),cb.as_ptr()); fsub(dacb_m.as_mut_ptr(),da.as_ptr(),cb.as_ptr()); x25519_fe51_mul121666(z3.as_mut_ptr(),e.as_mut_ptr()); x25519_fe51_sqr(z2.as_mut_ptr(),dacb_m.as_ptr()); x25519_fe51_sqr(x3.as_mut_ptr(),dacb_p.as_ptr()); fadd(b.as_mut_ptr(),bb.as_ptr(),z3.as_ptr()); x25519_fe51_mul(z3.as_mut_ptr(),x1.as_ptr(),z2.as_ptr()); x25519_fe51_mul(z2.as_mut_ptr(),e.as_ptr(),b.as_ptr()); }
    finv(z2.as_mut_ptr(),z2.as_ptr()); x25519_fe51_mul(x2.as_mut_ptr(),x2.as_ptr(),z2.as_ptr()); x25519_fe51_tobytes(out,x2.as_ptr());
}

unsafe fn curve25519_arch(mypublic: *mut u8, secret: *const u8, basepoint: *const u8) { curve25519_fe51(mypublic, secret, basepoint); }
unsafe fn curve25519_base_arch(pub_: *mut u8, secret: *const u8) { curve25519_fe51(pub_, secret, curve25519_base_point.as_ptr()); }

// Supplied by the surrounding kernel translation unit.
extern "C" { static curve25519_base_point: [u8; 32]; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
