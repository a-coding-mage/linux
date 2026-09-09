// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * KUnit tests and benchmark for ML-DSA
 *
 * Copyright 2025 Google LLC
 */
// Dependencies: crypto/mldsa.h, kunit/test.h, linux/random.h,
// linux/unaligned.h, and test-utils.h.

const Q: i32 = 8380417; /* The prime q = 2^23 - 2^13 + 1 */

/* ML-DSA parameters that the tests use */
#[repr(C)]
struct Params {
    sig_len: i32,
    pk_len: i32,
    k: i32,
    lambda: i32,
    gamma1: i32,
    beta: i32,
    omega: i32,
}

static PARAMS: [Params; 3] = [
    Params { sig_len: MLDSA44_SIGNATURE_SIZE, pk_len: MLDSA44_PUBLIC_KEY_SIZE, k: 4, lambda: 128, gamma1: 1 << 17, beta: 78, omega: 80 },
    Params { sig_len: MLDSA65_SIGNATURE_SIZE, pk_len: MLDSA65_PUBLIC_KEY_SIZE, k: 6, lambda: 192, gamma1: 1 << 19, beta: 196, omega: 55 },
    Params { sig_len: MLDSA87_SIGNATURE_SIZE, pk_len: MLDSA87_PUBLIC_KEY_SIZE, k: 8, lambda: 256, gamma1: 1 << 19, beta: 120, omega: 75 },
];

// Declarations supplied by the kernel and mldsa-testvecs.h.
extern "C" {
    fn mldsa_verify(alg: i32, sig: *const u8, sig_len: i32, msg: *const u8, msg_len: i32, pk: *const u8, pk_len: i32) -> i32;
    fn mldsa_use_hint(h: u8, r: i32, gamma2: i32) -> i32;
    fn get_random_u32_below(n: i32) -> u32;
    fn ktime_get_ns() -> u64;
    fn div64_u64(a: u64, b: u64) -> u64;
}

#[repr(C)]
struct Kunit;
#[repr(C)]
struct MldsaTestvector { alg: i32, sig: *const u8, sig_len: i32, msg: *const u8, msg_len: i32, pk: *const u8, pk_len: i32 }

extern "C" {
    static mldsa44_testvector: MldsaTestvector;
    static mldsa65_testvector: MldsaTestvector;
    static mldsa87_testvector: MldsaTestvector;
    fn memdup_buf(test: *mut Kunit, buf: *const u8, len: i32) -> *mut u8;
    fn get_unaligned_le32(p: *const u8) -> u32;
    fn put_unaligned_le32(v: u32, p: *mut u8);
}

unsafe fn do_mldsa_and_assert_success(test: *mut Kunit, tv: *const MldsaTestvector) {
    let err = mldsa_verify((*tv).alg, (*tv).sig, (*tv).sig_len, (*tv).msg, (*tv).msg_len, (*tv).pk, (*tv).pk_len);
    kunit_assert_eq!(test, err, 0);
}

/* Test that changing coefficients in a valid signature's z vector results in the expected verifier errors. */
unsafe fn test_mldsa_z_range(test: *mut Kunit, tv: *const MldsaTestvector) {
    let sig = memdup_buf(test, (*tv).sig, (*tv).sig_len);
    let lambda = PARAMS[(*tv).alg as usize].lambda;
    let gamma1 = PARAMS[(*tv).alg as usize].gamma1;
    let beta = PARAMS[(*tv).alg as usize].beta;
    let z_ptr = sig.add((lambda / 4) as usize);
    let z_data = get_unaligned_le32(z_ptr);
    let mask = ((gamma1 << 1) - 1) as u32;
    let out_of_range_coeffs = [-gamma1 + 1, -(gamma1 - beta), gamma1, gamma1 - beta];
    let in_range_coeffs = [-(gamma1 - beta - 1), 0, gamma1 - beta - 1];
    do_mldsa_and_assert_success(test, tv);
    for &c in &out_of_range_coeffs {
        put_unaligned_le32((z_data & !mask) | (mask & (gamma1 - c) as u32), z_ptr);
        kunit_assert_eq!(test, -EBADMSG, mldsa_verify((*tv).alg, sig, (*tv).sig_len, (*tv).msg, (*tv).msg_len, (*tv).pk, (*tv).pk_len));
    }
    for &c in &in_range_coeffs {
        put_unaligned_le32((z_data & !mask) | (mask & (gamma1 - c) as u32), z_ptr);
        kunit_assert_eq!(test, -EKEYREJECTED, mldsa_verify((*tv).alg, sig, (*tv).sig_len, (*tv).msg, (*tv).msg_len, (*tv).pk, (*tv).pk_len));
    }
}

unsafe fn test_mldsa_bad_hints(test: *mut Kunit, tv: *const MldsaTestvector) {
    let omega = PARAMS[(*tv).alg as usize].omega;
    let k = PARAMS[(*tv).alg as usize].k;
    let sig = memdup_buf(test, (*tv).sig, (*tv).sig_len);
    let hintvec = sig.add((*tv).sig_len as usize - omega as usize - k as usize);
    do_mldsa_and_assert_success(test, tv);
    core::ptr::copy_nonoverlapping((*tv).sig, sig, (*tv).sig_len as usize);
    *hintvec.add((omega + k - 1) as usize) = (omega + 1) as u8;
    kunit_assert_eq!(test, -EBADMSG, mldsa_verify((*tv).alg, sig, (*tv).sig_len, (*tv).msg, (*tv).msg_len, (*tv).pk, (*tv).pk_len));
    core::ptr::copy_nonoverlapping((*tv).sig, sig, (*tv).sig_len as usize);
    kunit_assert_ge!(test, *hintvec.add((omega + k - 2) as usize), 1);
    *hintvec.add((omega + k - 1) as usize) = hintvec.add((omega + k - 2) as usize).read().wrapping_sub(1);
    kunit_assert_eq!(test, -EBADMSG, mldsa_verify((*tv).alg, sig, (*tv).sig_len, (*tv).msg, (*tv).msg_len, (*tv).pk, (*tv).pk_len));
    core::ptr::copy_nonoverlapping((*tv).sig, sig, (*tv).sig_len as usize);
    kunit_assert_ge!(test, *hintvec.add(omega as usize), 2);
    let h = *hintvec; *hintvec = *hintvec.add(1); *hintvec.add(1) = h;
    kunit_assert_eq!(test, -EBADMSG, mldsa_verify((*tv).alg, sig, (*tv).sig_len, (*tv).msg, (*tv).msg_len, (*tv).pk, (*tv).pk_len));
    core::ptr::copy_nonoverlapping((*tv).sig, sig, (*tv).sig_len as usize);
    kunit_assert_lt!(test, *hintvec.add((omega + k - 1) as usize), omega);
    *hintvec.add((omega - 1) as usize) = 0xff;
    kunit_assert_eq!(test, -EBADMSG, mldsa_verify((*tv).alg, sig, (*tv).sig_len, (*tv).msg, (*tv).msg_len, (*tv).pk, (*tv).pk_len));
}

unsafe fn test_mldsa_mutation(test: *mut Kunit, tv: *const MldsaTestvector) {
    let sig_len = (*tv).sig_len; let msg_len = (*tv).msg_len; let pk_len = (*tv).pk_len;
    let sig = memdup_buf(test, (*tv).sig, sig_len); let msg = memdup_buf(test, (*tv).msg, msg_len); let pk = memdup_buf(test, (*tv).pk, pk_len);
    do_mldsa_and_assert_success(test, tv);
    for _ in 0..200 { let pos = get_random_u32_below(sig_len) as usize; let b = 1u8 << get_random_u32_below(8); *sig.add(pos) ^= b; kunit_assert_ne!(test, 0, mldsa_verify((*tv).alg, sig, sig_len, msg, msg_len, pk, pk_len)); *sig.add(pos) ^= b; }
    for _ in 0..200 { let pos = get_random_u32_below(msg_len) as usize; let b = 1u8 << get_random_u32_below(8); *msg.add(pos) ^= b; kunit_assert_ne!(test, 0, mldsa_verify((*tv).alg, sig, sig_len, msg, msg_len, pk, pk_len)); *msg.add(pos) ^= b; }
    for _ in 0..200 { let pos = get_random_u32_below(pk_len) as usize; let b = 1u8 << get_random_u32_below(8); *pk.add(pos) ^= b; kunit_assert_ne!(test, 0, mldsa_verify((*tv).alg, sig, sig_len, msg, msg_len, pk, pk_len)); *pk.add(pos) ^= b; }
    kunit_assert_eq!(test, 0, mldsa_verify((*tv).alg, sig, sig_len, msg, msg_len, pk, pk_len));
}

unsafe fn test_mldsa(test: *mut Kunit, tv: *const MldsaTestvector) {
    kunit_assert_eq!(test, (*tv).sig_len, PARAMS[(*tv).alg as usize].sig_len); kunit_assert_eq!(test, (*tv).pk_len, PARAMS[(*tv).alg as usize].pk_len); do_mldsa_and_assert_success(test, tv);
    kunit_assert_eq!(test, -EBADMSG, mldsa_verify((*tv).alg, (*tv).sig, (*tv).sig_len - 1, (*tv).msg, (*tv).msg_len, (*tv).pk, (*tv).pk_len));
    kunit_assert_eq!(test, -EBADMSG, mldsa_verify((*tv).alg, (*tv).sig, (*tv).sig_len + 1, (*tv).msg, (*tv).msg_len, (*tv).pk, (*tv).pk_len));
    kunit_assert_eq!(test, -EBADMSG, mldsa_verify((*tv).alg, (*tv).sig, (*tv).sig_len, (*tv).msg, (*tv).msg_len, (*tv).pk, (*tv).pk_len - 1));
    kunit_assert_eq!(test, -EBADMSG, mldsa_verify((*tv).alg, (*tv).sig, (*tv).sig_len, (*tv).msg, (*tv).msg_len, (*tv).pk, (*tv).pk_len + 1));
    kunit_assert_eq!(test, -EKEYREJECTED, mldsa_verify((*tv).alg, (*tv).sig, (*tv).sig_len, (*tv).msg, (*tv).msg_len - 1, (*tv).pk, (*tv).pk_len));
    test_mldsa_z_range(test, tv); test_mldsa_bad_hints(test, tv); test_mldsa_mutation(test, tv);
}
unsafe fn test_mldsa44(test: *mut Kunit) { test_mldsa(test, &mldsa44_testvector); }
unsafe fn test_mldsa65(test: *mut Kunit) { test_mldsa(test, &mldsa65_testvector); }
unsafe fn test_mldsa87(test: *mut Kunit) { test_mldsa(test, &mldsa87_testvector); }

unsafe fn mod_(mut a: i32, m: i32) -> i32 { a %= m; if a < 0 { a += m; } a }
unsafe fn symmetric_mod(mut a: i32, m: i32) -> i32 { a = mod_(a, m); if a > m / 2 { a -= m; } a }
unsafe fn decompose_ref(r: i32, gamma2: i32, r0: *mut i32, r1: *mut i32) { let rplus = mod_(r, Q); *r0 = symmetric_mod(rplus, 2 * gamma2); if rplus - *r0 == Q - 1 { *r1 = 0; *r0 -= 1; } else { *r1 = (rplus - *r0) / (2 * gamma2); } }
unsafe fn use_hint_ref(h: u8, r: i32, gamma2: i32) -> i32 { let m = (Q - 1) / (2 * gamma2); let mut r0 = 0; let mut r1 = 0; decompose_ref(r, gamma2, &mut r0, &mut r1); if h == 1 && r0 > 0 { return mod_(r1 + 1, m); } if h == 1 && r0 <= 0 { return mod_(r1 - 1, m); } r1 }

unsafe fn test_mldsa_use_hint(test: *mut Kunit) { for i in 0..2 { let gamma2 = (Q - 1) / if i == 0 { 88 } else { 32 }; for h in 0..2u8 { for r in 0..Q { kunit_assert_eq!(test, mldsa_use_hint(h, r, gamma2), use_hint_ref(h, r, gamma2)); } } } }

// KUnit registration and benchmark declarations are supplied by the kernel build environment.
// MODULE_DESCRIPTION("KUnit tests and benchmark for ML-DSA");
// MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
