// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
/* Copyright (c) 2026 Meta Platforms, Inc. and affiliates. */

// C header guard / #pragma once omitted in Rust.

pub const ST_PAGES: i32 = 64;

// External declarations supplied by the surrounding translated test harness.
extern "C" {
    static mut asan_violated: u64;

    fn ASAN_GRANULE(addr: *mut core::ffi::c_void) -> u32;
    fn asan_shadow_set(addr: *mut core::ffi::c_void) -> bool;
    fn arena_stdout(fmt: *const core::ffi::c_char, ...);
}

pub type s8 = i8;

pub const EINVAL: i32 = 22;

#[repr(transparent)]
pub struct VolatileU8(core::cell::UnsafeCell<u8>);

#[inline]
pub unsafe fn print_asan_map_state(addr: *mut core::ffi::c_void) {
    unsafe {
        arena_stdout(
            c"%s:%d ASAN %p -> (val: %x gran: %x set: [%s])".as_ptr(),
            c"print_asan_map_state".as_ptr(),
            line!() as i32,
            addr,
            *(addr as *mut s8) as i32,
            ASAN_GRANULE(addr) as i32,
            if asan_shadow_set(addr) {
                c"yes".as_ptr()
            } else {
                c"no".as_ptr()
            },
        );
    }
}

/*
 * Emit an error and force the current function to exit if the ASAN
 * violation state is unexpected. Reset the violation state after.
 */
#[inline]
pub unsafe fn asan_validate_addr(cond: bool, addr: *mut core::ffi::c_void) -> i32 {
    unsafe {
        if ((asan_violated != 0) == cond) {
            asan_violated = 0;
            return 0;
        }

        arena_stdout(
            c"%s:%d ASAN asan_violated %lx".as_ptr(),
            c"asan_validate_addr".as_ptr(),
            line!() as i32,
            asan_violated as u64,
        );
        print_asan_map_state(addr);

        asan_violated = 0;

        -EINVAL
    }
}

#[inline]
pub unsafe fn asan_validate() -> i32 {
    unsafe {
        if asan_violated == 0 {
            return 0;
        }

        arena_stdout(
            c"%s:%d Found ASAN violation at %lx".as_ptr(),
            c"asan_validate".as_ptr(),
            line!() as i32,
            asan_violated,
        );

        asan_violated = 0;

        -EINVAL
    }
}

#[repr(C)]
pub struct blob {
    pub mem: [VolatileU8; 59],
    pub oob: u8,
}
