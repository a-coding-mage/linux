// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

/* Dependencies from the original C includes:
 * <test_progs.h>
 * "test_endian.skel.h"
 */

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct test_endian {
    pub bss: *mut test_endian__bss,
}

#[repr(C)]
pub struct test_endian__bss {
    pub in16: u16,
    pub in32: u32,
    pub in64: u64,
    pub out16: u16,
    pub out32: u32,
    pub out64: u64,
    pub const16: u16,
    pub const32: u32,
    pub const64: u64,
}

unsafe extern "C" {
    fn test_endian__open_and_load() -> *mut test_endian;
    fn test_endian__attach(skel: *mut test_endian) -> c_int;
    fn test_endian__destroy(skel: *mut test_endian);
    fn usleep(usec: u32) -> c_int;

    fn CHECK(condition: bool, name: *const c_char, format: *const c_char, ...) -> bool;
}

static mut duration: c_int = 0;

const IN16: u16 = 0x1234;
const IN32: u32 = 0x12345678u32;
const IN64: u64 = 0x123456789abcdef0u64;

const OUT16: u16 = 0x3412;
const OUT32: u32 = 0x78563412u32;
const OUT64: u64 = 0xf0debc9a78563412u64;

pub unsafe extern "C" fn test_endian() {
    let mut skel: *mut test_endian;
    let bss: *mut test_endian__bss;
    let err: c_int;

    skel = test_endian__open_and_load();
    if CHECK(
        skel.is_null(),
        c"skel_open".as_ptr(),
        c"failed to open skeleton\n".as_ptr(),
    ) {
        return;
    }
    bss = (*skel).bss;

    (*bss).in16 = IN16;
    (*bss).in32 = IN32;
    (*bss).in64 = IN64;

    err = test_endian__attach(skel);
    if CHECK(
        err != 0,
        c"skel_attach".as_ptr(),
        c"skeleton attach failed: %d\n".as_ptr(),
        err,
    ) {
        test_endian__destroy(skel);
        return;
    }

    usleep(1);

    CHECK(
        (*bss).out16 != OUT16,
        c"out16".as_ptr(),
        c"got 0x%llx != exp 0x%llx\n".as_ptr(),
        (*bss).out16 as u64,
        OUT16 as u64,
    );
    CHECK(
        (*bss).out32 != OUT32,
        c"out32".as_ptr(),
        c"got 0x%llx != exp 0x%llx\n".as_ptr(),
        (*bss).out32 as u64,
        OUT32 as u64,
    );
    CHECK(
        (*bss).out64 != OUT64,
        c"out16".as_ptr(),
        c"got 0x%llx != exp 0x%llx\n".as_ptr(),
        (*bss).out64 as u64,
        OUT64 as u64,
    );

    CHECK(
        (*bss).const16 != OUT16,
        c"const16".as_ptr(),
        c"got 0x%llx != exp 0x%llx\n".as_ptr(),
        (*bss).const16 as u64,
        OUT16 as u64,
    );
    CHECK(
        (*bss).const32 != OUT32,
        c"const32".as_ptr(),
        c"got 0x%llx != exp 0x%llx\n".as_ptr(),
        (*bss).const32 as u64,
        OUT32 as u64,
    );
    CHECK(
        (*bss).const64 != OUT64,
        c"const64".as_ptr(),
        c"got 0x%llx != exp 0x%llx\n".as_ptr(),
        (*bss).const64 as u64,
        OUT64 as u64,
    );

    test_endian__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
