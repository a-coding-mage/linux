/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _ASM_POWERPC_SET_MEMORY_H

pub const SET_MEMORY_RO: i32 = 0;
pub const SET_MEMORY_RW: i32 = 1;
pub const SET_MEMORY_NX: i32 = 2;
pub const SET_MEMORY_X: i32 = 3;
pub const SET_MEMORY_NP: i32 = 4; // Set memory non present
pub const SET_MEMORY_P: i32 = 5; // Set memory present
pub const SET_MEMORY_ROX: i32 = 6;

unsafe extern "C" {
    pub fn change_memory_attr(
        addr: ::core::ffi::c_ulong,
        numpages: ::core::ffi::c_int,
        action: ::core::ffi::c_long,
    ) -> ::core::ffi::c_int;
}

#[must_use]
#[inline]
pub unsafe fn set_memory_ro(
    addr: ::core::ffi::c_ulong,
    numpages: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe { change_memory_attr(addr, numpages, SET_MEMORY_RO as ::core::ffi::c_long) }
}

#[must_use]
#[inline]
pub unsafe fn set_memory_rw(
    addr: ::core::ffi::c_ulong,
    numpages: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe { change_memory_attr(addr, numpages, SET_MEMORY_RW as ::core::ffi::c_long) }
}

#[must_use]
#[inline]
pub unsafe fn set_memory_nx(
    addr: ::core::ffi::c_ulong,
    numpages: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe { change_memory_attr(addr, numpages, SET_MEMORY_NX as ::core::ffi::c_long) }
}

#[must_use]
#[inline]
pub unsafe fn set_memory_x(
    addr: ::core::ffi::c_ulong,
    numpages: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe { change_memory_attr(addr, numpages, SET_MEMORY_X as ::core::ffi::c_long) }
}

#[must_use]
#[inline]
pub unsafe fn set_memory_np(
    addr: ::core::ffi::c_ulong,
    numpages: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe { change_memory_attr(addr, numpages, SET_MEMORY_NP as ::core::ffi::c_long) }
}

#[must_use]
#[inline]
pub unsafe fn set_memory_p(
    addr: ::core::ffi::c_ulong,
    numpages: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe { change_memory_attr(addr, numpages, SET_MEMORY_P as ::core::ffi::c_long) }
}

#[must_use]
#[inline]
pub unsafe fn set_memory_rox(
    addr: ::core::ffi::c_ulong,
    numpages: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe { change_memory_attr(addr, numpages, SET_MEMORY_ROX as ::core::ffi::c_long) }
}

// C self-referential macro: #define set_memory_rox set_memory_rox

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
