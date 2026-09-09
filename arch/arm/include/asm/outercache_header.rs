/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/include/asm/outercache.h
 *
 * Copyright (C) 2010 ARM Ltd.
 * Written by Catalin Marinas <catalin.marinas@arm.com>
 */

// Dependency supplied externally by the translated kernel sources:
// use linux_types::phys_addr_t;

#[repr(C)]
pub struct l2x0_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct outer_cache_fns {
    pub inv_range: Option<unsafe extern "C" fn(start: libc::c_ulong, end: libc::c_ulong)>,
    pub clean_range: Option<unsafe extern "C" fn(start: libc::c_ulong, end: libc::c_ulong)>,
    pub flush_range: Option<unsafe extern "C" fn(start: libc::c_ulong, end: libc::c_ulong)>,
    pub flush_all: Option<unsafe extern "C" fn()>,
    pub disable: Option<unsafe extern "C" fn()>,
    // Preserves the CONFIG_OUTER_CACHE_SYNC conditional declaration.
    #[cfg(feature = "CONFIG_OUTER_CACHE_SYNC")]
    pub sync: Option<unsafe extern "C" fn()>,
    pub resume: Option<unsafe extern "C" fn()>,

    /* This is an ARM L2C thing */
    pub write_sec: Option<unsafe extern "C" fn(sect: libc::c_ulong, value: libc::c_uint)>,
    pub configure: Option<unsafe extern "C" fn(regs: *const l2x0_regs)>,
}

extern "C" {
    pub static mut outer_cache: outer_cache_fns;
}

// The following items are enabled when CONFIG_OUTER_CACHE is defined.
#[cfg(feature = "CONFIG_OUTER_CACHE")]
pub unsafe fn outer_inv_range(start: phys_addr_t, end: phys_addr_t) {
    if let Some(f) = outer_cache.inv_range {
        f(start as libc::c_ulong, end as libc::c_ulong);
    }
}

#[cfg(feature = "CONFIG_OUTER_CACHE")]
pub unsafe fn outer_clean_range(start: phys_addr_t, end: phys_addr_t) {
    if let Some(f) = outer_cache.clean_range {
        f(start as libc::c_ulong, end as libc::c_ulong);
    }
}

#[cfg(feature = "CONFIG_OUTER_CACHE")]
pub unsafe fn outer_flush_range(start: phys_addr_t, end: phys_addr_t) {
    if let Some(f) = outer_cache.flush_range {
        f(start as libc::c_ulong, end as libc::c_ulong);
    }
}

#[cfg(feature = "CONFIG_OUTER_CACHE")]
pub unsafe fn outer_flush_all() {
    if let Some(f) = outer_cache.flush_all {
        f();
    }
}

#[cfg(feature = "CONFIG_OUTER_CACHE")]
extern "C" {
    pub fn outer_disable();
}

#[cfg(feature = "CONFIG_OUTER_CACHE")]
pub unsafe fn outer_resume() {
    if let Some(f) = outer_cache.resume {
        f();
    }
}

// Empty definitions preserve the no-outer-cache configuration.
#[cfg(not(feature = "CONFIG_OUTER_CACHE"))]
pub unsafe fn outer_inv_range(_start: phys_addr_t, _end: phys_addr_t) {}
#[cfg(not(feature = "CONFIG_OUTER_CACHE"))]
pub unsafe fn outer_clean_range(_start: phys_addr_t, _end: phys_addr_t) {}
#[cfg(not(feature = "CONFIG_OUTER_CACHE"))]
pub unsafe fn outer_flush_range(_start: phys_addr_t, _end: phys_addr_t) {}
#[cfg(not(feature = "CONFIG_OUTER_CACHE"))]
pub unsafe fn outer_flush_all() {}
#[cfg(not(feature = "CONFIG_OUTER_CACHE"))]
pub unsafe fn outer_disable() {}
#[cfg(not(feature = "CONFIG_OUTER_CACHE"))]
pub unsafe fn outer_resume() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
