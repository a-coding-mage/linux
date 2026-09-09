/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependency supplied by the kernel jump-label implementation.

extern "C" {
    pub static mut stress_slb_enabled: bool;
    pub static mut stress_slb_key: StaticKeyFalse;
}

#[inline]
pub unsafe fn stress_slb() -> bool {
    static_branch_unlikely(&stress_slb_key)
}

extern "C" {
    pub static mut stress_hpt_enabled: bool;
    pub static mut stress_hpt_key: StaticKeyFalse;
}

#[inline]
pub unsafe fn stress_hpt() -> bool {
    static_branch_unlikely(&stress_hpt_key)
}

extern "C" {
    pub static mut no_slb_preload: bool;
    pub static mut no_slb_preload_key: StaticKeyFalse;
}

#[inline]
pub unsafe fn slb_preload_disabled() -> bool {
    static_branch_unlikely(&no_slb_preload_key)
}

extern "C" {
    pub fn hpt_do_stress(ea: ::core::ffi::c_ulong, hpte_group: ::core::ffi::c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
