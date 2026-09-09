// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 *
 * Based on public domain code from Daniel J. Bernstein and Peter Schwabe. This
 * began from SUPERCOP's curve25519/neon2/scalarmult.s, but has subsequently been
 * manually reworked for use in kernel space.
 */

// C dependencies supplied by the surrounding kernel translation unit:
// asm/hwcap.h, asm/neon.h, asm/simd.h, crypto/internal/simd.h,
// linux/types.h, and linux/jump_label.h.

extern "C" {
    fn curve25519_neon(
        mypublic: *mut u8,
        secret: *const u8,
        basepoint: *const u8,
    );

    fn curve25519_generic(
        out: *mut u8,
        scalar: *const u8,
        point: *const u8,
    );

    fn static_branch_likely(key: *const StaticKey) -> bool;
    fn crypto_simd_usable() -> bool;
    fn static_branch_enable(key: *mut StaticKey);

    static elf_hwcap: usize;
    static curve25519_base_point: [u8; CURVE25519_KEY_SIZE];
}

// Opaque kernel jump-label type supplied by the surrounding translation unit.
#[repr(C)]
pub struct StaticKey {
    _private: [u8; 0],
}

// Equivalent to __ro_after_init DEFINE_STATIC_KEY_FALSE(have_neon).
#[no_mangle]
static mut have_neon: StaticKey = StaticKey { _private: [] };

pub const CURVE25519_KEY_SIZE: usize = 32;
pub const HWCAP_NEON: usize = 1 << 12;

unsafe fn curve25519_arch(
    out: *mut u8,
    scalar: *const u8,
    point: *const u8,
) {
    if static_branch_likely(&have_neon) && crypto_simd_usable() {
        // Equivalent to scoped_ksimd() around the NEON call.
        curve25519_neon(out, scalar, point);
    } else {
        curve25519_generic(out, scalar, point);
    }
}

unsafe fn curve25519_base_arch(pub_: *mut u8, secret: *const u8) {
    curve25519_arch(pub_, secret, curve25519_base_point.as_ptr());
}

// #define curve25519_mod_init_arch curve25519_mod_init_arch
#[allow(non_upper_case_globals)]
pub const curve25519_mod_init_arch_macro: &str = "curve25519_mod_init_arch";

unsafe fn curve25519_mod_init_arch() {
    if elf_hwcap & HWCAP_NEON != 0 {
        static_branch_enable(&mut have_neon);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
