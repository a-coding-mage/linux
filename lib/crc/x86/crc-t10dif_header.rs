// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * CRC-T10DIF using [V]PCLMULQDQ instructions
 *
 * Copyright 2024 Google LLC
 */

// Dependency intent: declarations and macros supplied by "crc-pclmul-template.h".

// The C header declares a static read-only-after-init static key.
extern "C" {
    static mut have_pclmulqdq: StaticKeyFalse;
}

// Opaque dependency type corresponding to the kernel's static key type.
#[repr(C)]
pub struct StaticKeyFalse {
    _private: [u8; 0],
}

// DECLARE_CRC_PCLMUL_FUNCS(crc16_msb, u16);
extern "C" {
    fn crc16_msb_pclmul(crc: u16, p: *const u8, len: usize) -> u16;
    fn crc16_msb_vpclmul_avx512(crc: u16, p: *const u8, len: usize) -> u16;
    fn crc16_msb_vpclmul_avx2(crc: u16, p: *const u8, len: usize) -> u16;
    fn crc_t10dif_generic(crc: u16, p: *const u8, len: usize) -> u16;
    fn boot_cpu_has(feature: u32) -> bool;
    fn static_branch_enable(key: *mut StaticKeyFalse);
    fn have_vpclmul() -> bool;
    fn have_avx512() -> bool;
    fn static_call_update(
        call: unsafe extern "C" fn(u16, *const u8, usize) -> u16,
        target: unsafe extern "C" fn(u16, *const u8, usize) -> u16,
    );
}

// Dependency intent: X86_FEATURE_PCLMULQDQ is supplied by the x86 feature definitions.
extern "C" {
    static X86_FEATURE_PCLMULQDQ: u32;
}

// Dependency intent: crc16_msb_0x8bb7_consts is supplied by the CRC PCLMUL template.
extern "C" {
    static crc16_msb_0x8bb7_consts: u8;
}

#[inline]
pub unsafe fn crc_t10dif_arch(crc: u16, p: *const u8, len: usize) -> u16 {
    // CRC_PCLMUL(crc, p, len, crc16_msb, crc16_msb_0x8bb7_consts,
    //            have_pclmulqdq);
    crc_t10dif_generic(crc, p, len)
}

// #define crc_t10dif_mod_init_arch crc_t10dif_mod_init_arch
pub unsafe fn crc_t10dif_mod_init_arch() {
    if boot_cpu_has(X86_FEATURE_PCLMULQDQ) {
        static_branch_enable(&raw mut have_pclmulqdq);
        if have_vpclmul() {
            if have_avx512() {
                static_call_update(crc16_msb_pclmul, crc16_msb_vpclmul_avx512);
            } else {
                static_call_update(crc16_msb_pclmul, crc16_msb_vpclmul_avx2);
            }
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
