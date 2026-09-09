/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2024 ARM Ltd.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

pub const RSI_PDEV_NAME: &str = "arm-cca-dev";

extern "C" {
    pub static rsi_present: StaticKey;

    pub fn arm64_rsi_init();

    pub fn arm64_rsi_is_protected(base: phys_addr_t, size: usize) -> bool;

    pub fn static_branch_unlikely(key: *const StaticKey) -> bool;

    pub fn rsi_set_addr_range_state(
        start: phys_addr_t,
        end: phys_addr_t,
        state: ripas,
        flags: core::ffi::c_ulong,
        top: *mut phys_addr_t,
    ) -> core::ffi::c_ulong;
}

pub const RSI_RIPAS_RAM: ripas = RSI_RIPAS_RAM_VALUE;
pub const RSI_RIPAS_EMPTY: ripas = RSI_RIPAS_EMPTY_VALUE;
pub const RSI_CHANGE_DESTROYED: core::ffi::c_ulong = RSI_CHANGE_DESTROYED_VALUE;
pub const RSI_NO_CHANGE_DESTROYED: core::ffi::c_ulong = RSI_NO_CHANGE_DESTROYED_VALUE;

#[inline]
pub unsafe fn is_realm_world() -> bool {
    static_branch_unlikely(&rsi_present as *const StaticKey)
}

#[inline]
pub unsafe fn rsi_set_memory_range(
    mut start: phys_addr_t,
    end: phys_addr_t,
    state: ripas,
    flags: core::ffi::c_ulong,
) -> core::ffi::c_int {
    let mut ret: core::ffi::c_ulong;
    let mut top: phys_addr_t;

    while start != end {
        ret = rsi_set_addr_range_state(start, end, state, flags, &mut top);
        if ret != 0 || top < start || top > end {
            return -EINVAL;
        }
        start = top;
    }

    0
}

/*
 * Convert the specified range to RAM. Do not use this if you rely on the
 * contents of a page that may already be in RAM state.
 */
#[inline]
pub unsafe fn rsi_set_memory_range_protected(
    start: phys_addr_t,
    end: phys_addr_t,
) -> core::ffi::c_int {
    rsi_set_memory_range(start, end, RSI_RIPAS_RAM, RSI_CHANGE_DESTROYED)
}

/*
 * Convert the specified range to RAM. Do not convert any pages that may have
 * been DESTROYED, without our permission.
 */
#[inline]
pub unsafe fn rsi_set_memory_range_protected_safe(
    start: phys_addr_t,
    end: phys_addr_t,
) -> core::ffi::c_int {
    rsi_set_memory_range(start, end, RSI_RIPAS_RAM, RSI_NO_CHANGE_DESTROYED)
}

#[inline]
pub unsafe fn rsi_set_memory_range_shared(
    start: phys_addr_t,
    end: phys_addr_t,
) -> core::ffi::c_int {
    rsi_set_memory_range(start, end, RSI_RIPAS_EMPTY, RSI_CHANGE_DESTROYED)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
