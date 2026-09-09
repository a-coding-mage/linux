/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ARM64 accelerated implementation of NH
 *
 * Copyright 2018 Google LLC
 */

// C dependencies: asm/hwcap.h, asm/simd.h, and linux/cpufeature.h.

// __ro_after_init DEFINE_STATIC_KEY_FALSE(have_neon)
static mut HAVE_NEON: bool = false;

extern "C" {
    fn nh_neon(
        key: *const u32,
        message: *const u8,
        message_len: usize,
        hash: *mut __le64,
    );
}

unsafe fn nh_arch(
    key: *const u32,
    message: *const u8,
    message_len: usize,
    hash: *mut __le64,
) -> bool {
    // static_branch_likely(&have_neon) && may_use_simd()
    if HAVE_NEON && message_len >= 64 && may_use_simd() {
        // scoped_ksimd()
        nh_neon(key, message, message_len, hash);
        return true;
    }
    false
}

// #define nh_mod_init_arch nh_mod_init_arch
unsafe fn nh_mod_init_arch() {
    if cpu_have_named_feature(ASIMD) {
        HAVE_NEON = true;
    }
}

// External symbols supplied by the surrounding kernel translation unit.
extern "C" {
    fn may_use_simd() -> bool;
    fn cpu_have_named_feature(feature: u32) -> bool;
    static ASIMD: u32;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
