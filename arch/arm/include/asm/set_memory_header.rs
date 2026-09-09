/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 1999-2002 Russell King
 */

// C header guard: _ASMARM_SET_MEMORY_H

#[cfg(CONFIG_MMU)]
extern "C" {
    pub fn set_memory_ro(addr: core::ffi::c_ulong, numpages: core::ffi::c_int) -> core::ffi::c_int;
    pub fn set_memory_rw(addr: core::ffi::c_ulong, numpages: core::ffi::c_int) -> core::ffi::c_int;
    pub fn set_memory_x(addr: core::ffi::c_ulong, numpages: core::ffi::c_int) -> core::ffi::c_int;
    pub fn set_memory_nx(addr: core::ffi::c_ulong, numpages: core::ffi::c_int) -> core::ffi::c_int;
    pub fn set_memory_valid(
        addr: core::ffi::c_ulong,
        numpages: core::ffi::c_int,
        enable: core::ffi::c_int,
    ) -> core::ffi::c_int;
}

#[cfg(not(CONFIG_MMU))]
#[inline]
pub unsafe fn set_memory_ro(
    _addr: core::ffi::c_ulong,
    _numpages: core::ffi::c_int,
) -> core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_MMU))]
#[inline]
pub unsafe fn set_memory_rw(
    _addr: core::ffi::c_ulong,
    _numpages: core::ffi::c_int,
) -> core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_MMU))]
#[inline]
pub unsafe fn set_memory_x(
    _addr: core::ffi::c_ulong,
    _numpages: core::ffi::c_int,
) -> core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_MMU))]
#[inline]
pub unsafe fn set_memory_nx(
    _addr: core::ffi::c_ulong,
    _numpages: core::ffi::c_int,
) -> core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
