/* SPDX-License-Identifier: GPL-2.0 */
/*
 * ARM32 accelerated implementation of NH
 *
 * Copyright 2018 Google LLC
 */

// The following symbols and types are supplied by the surrounding kernel
// translation unit.

pub type __le64 = u64;

// Equivalent storage for DEFINE_STATIC_KEY_FALSE(have_neon), with the
// __ro_after_init qualifier retained as source-level intent.
#[allow(non_upper_case_globals)]
static mut have_neon: bool = false;

extern "C" {
    pub fn nh_neon(
        key: *const u32,
        message: *const u8,
        message_len: usize,
        hash: *mut __le64,
    );

    fn static_branch_likely(key: *const bool) -> bool;
    fn may_use_simd() -> bool;
    fn static_branch_enable(key: *mut bool);
    static elf_hwcap: usize;
}

// The self-referential C macro `#define nh_mod_init_arch nh_mod_init_arch`
// preserves the architecture-specific initializer name.
#[allow(non_snake_case)]
pub unsafe fn nh_arch(
    key: *const u32,
    message: *const u8,
    message_len: usize,
    hash: *mut __le64,
) -> bool {
    if static_branch_likely(&raw const have_neon)
        && message_len >= 64
        && may_use_simd()
    {
        // scoped_ksimd() surrounds this call in the C implementation.
        nh_neon(key, message, message_len, hash);
        return true;
    }
    false
}

pub unsafe fn nh_mod_init_arch() {
    // HWCAP_NEON is supplied by the architecture-specific dependency.
    if elf_hwcap & HWCAP_NEON != 0 {
        static_branch_enable(&raw mut have_neon);
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
