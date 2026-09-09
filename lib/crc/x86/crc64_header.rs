// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * CRC64 using [V]PCLMULQDQ instructions
 *
 * Copyright 2025 Google LLC
 */

// The declarations and macros supplied by crc-pclmul-template.h are external
// dependencies of this translation unit.

extern "C" {
    static mut have_pclmulqdq: StaticKey;

    fn crc64_be_generic(crc: u64, p: *const u8, len: usize) -> u64;
    fn crc64_nvme_generic(crc: u64, p: *const u8, len: usize) -> u64;

    fn boot_cpu_has(feature: i32) -> bool;
    fn static_branch_enable(key: *mut StaticKey);
    fn have_vpclmul() -> bool;
    fn have_avx512() -> bool;
    fn static_call_update(call: *const (), func: *const ());

    static crc64_msb_pclmul: ();
    static crc64_lsb_pclmul: ();
    static crc64_msb_vpclmul_avx512: ();
    static crc64_lsb_vpclmul_avx512: ();
    static crc64_msb_vpclmul_avx2: ();
    static crc64_lsb_vpclmul_avx2: ();
}

// Opaque type supplied by the kernel dependency.
#[allow(non_camel_case_types)]
pub enum StaticKey {}

// DECLARE_CRC_PCLMUL_FUNCS(crc64_msb, u64);
// DECLARE_CRC_PCLMUL_FUNCS(crc64_lsb, u64);

#[inline]
pub unsafe fn crc64_be_arch(mut crc: u64, p: *const u8, len: usize) -> u64 {
    // CRC_PCLMUL(crc, p, len, crc64_msb,
    //            crc64_msb_0x42f0e1eba9ea3693_consts, have_pclmulqdq);
    crc = crc64_be_generic(crc, p, len);
    crc
}

#[inline]
pub unsafe fn crc64_nvme_arch(mut crc: u64, p: *const u8, len: usize) -> u64 {
    // CRC_PCLMUL(crc, p, len, crc64_lsb,
    //            crc64_lsb_0x9a6c9329ac4bc9b5_consts, have_pclmulqdq);
    crc = crc64_nvme_generic(crc, p, len);
    crc
}

// #define crc64_mod_init_arch crc64_mod_init_arch
#[inline]
pub unsafe fn crc64_mod_init_arch() {
    if boot_cpu_has(X86_FEATURE_PCLMULQDQ) {
        static_branch_enable(&mut have_pclmulqdq);
        if have_vpclmul() {
            if have_avx512() {
                static_call_update(
                    &crc64_msb_pclmul as *const (),
                    &crc64_msb_vpclmul_avx512 as *const (),
                );
                static_call_update(
                    &crc64_lsb_pclmul as *const (),
                    &crc64_lsb_vpclmul_avx512 as *const (),
                );
            } else {
                static_call_update(
                    &crc64_msb_pclmul as *const (),
                    &crc64_msb_vpclmul_avx2 as *const (),
                );
                static_call_update(
                    &crc64_lsb_pclmul as *const (),
                    &crc64_lsb_vpclmul_avx2 as *const (),
                );
            }
        }
    }
}

// X86_FEATURE_PCLMULQDQ is supplied by the x86 dependency.
extern "C" {
    static X86_FEATURE_PCLMULQDQ: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
