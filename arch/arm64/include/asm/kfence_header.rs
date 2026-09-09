/* SPDX-License-Identifier: GPL-2.0 */
/*
 * arm64 KFENCE support.
 *
 * Copyright (C) 2020, Google LLC.
 */

// Dependency supplied by the corresponding asm/set_memory interface.
unsafe extern "C" {
    pub fn set_memory_valid(addr: usize, pages: usize, valid: bool);
}

pub unsafe fn kfence_protect_page(addr: usize, protect: bool) -> bool {
    unsafe {
        set_memory_valid(addr, 1, !protect);
    }

    true
}

// CONFIG_KFENCE is a build-time configuration condition from the C header.
#[cfg(feature = "CONFIG_KFENCE")]
unsafe extern "C" {
    pub static mut kfence_early_init: bool;
}

#[cfg(feature = "CONFIG_KFENCE")]
pub unsafe fn arm64_kfence_can_set_direct_map() -> bool {
    unsafe { !kfence_early_init }
}

#[cfg(feature = "CONFIG_KFENCE")]
unsafe extern "C" {
    pub fn arch_kfence_init_pool() -> bool;
}

#[cfg(not(feature = "CONFIG_KFENCE"))]
pub fn arm64_kfence_can_set_direct_map() -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
