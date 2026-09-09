// SPDX-License-Identifier: GPL-2.0
/*
 * imr_selftest.c -- Intel Isolated Memory Region self-test driver
 *
 * Copyright(c) 2013 Intel Corporation.
 * Copyright(c) 2015 Bryan O'Donoghue <pure.logic@nexus-software.ie>
 *
 * IMR self test. The purpose of this module is to run a set of tests on the
 * IMR API to validate its sanity. We check for overlapping, reserved
 * addresses and setup/teardown sanity.
 */

// C dependencies supplied by the surrounding kernel translation.

use core::ffi::{c_char, c_int, c_void};

type PhysAddr = usize;

#[repr(C)]
pub struct X86CpuId {
    _private: [u8; 0],
}

extern "C" {
    static _text: c_void;
    static __end_rodata: c_void;

    fn virt_to_phys(addr: *const c_void) -> PhysAddr;
    fn __va(addr: PhysAddr) -> *mut c_void;
    fn imr_add_range(base: PhysAddr, size: usize, rmask: u32, wmask: u32) -> c_int;
    fn imr_remove_range(base: PhysAddr, size: usize) -> c_int;
    fn x86_match_cpu(ids: *const X86CpuId) -> *const X86CpuId;
    fn pr_info(fmt: *const c_char, ...);
    fn vprintk(fmt: *const c_char, args: *mut c_void);
    fn warn(condition: bool, fmt: *const c_char, ...);
}

// These values are provided by <asm/imr.h> and the architecture build.
extern "C" {
    static IMR_ALIGN: usize;
    static IMR_CPU: u32;
    static IMR_READ_ACCESS_ALL: u32;
    static IMR_WRITE_ACCESS_ALL: u32;
}

/*
 * imr_self_test_result - Print result string for self test.
 *
 * @res: result code - true if test passed false otherwise.
 * @fmt: format string.
 * ...  variadic argument list.
 */
unsafe fn imr_self_test_result(res: c_int, fmt: *const c_char, mut args: ...) {
    /* Print pass/fail. */
    if res != 0 {
        pr_info(b"imr_selftest: pass \0".as_ptr() as *const c_char);
    } else {
        pr_info(b"imr_selftest: fail \0".as_ptr() as *const c_char);
    }

    /* Print variable string. */
    vprintk(fmt, &mut args as *mut _ as *mut c_void);

    /* Optional warning. */
    warn(res == 0, b"test failed\0".as_ptr() as *const c_char);
}

/*
 * imr_self_test - perform the IMR self-test
 *
 * Verify IMR self_test with some simple tests to verify overlap,
 * zero sized allocations and 1 KiB sized areas.
 */
unsafe fn imr_self_test() {
    let mut base: PhysAddr = virt_to_phys(&_text as *const _ as *const c_void);
    let mut size: usize = virt_to_phys(&__end_rodata as *const _ as *const c_void) - base;
    let fmt_over = b"overlapped IMR @ (0x%08lx - 0x%08lx)\n\0";
    let mut ret: c_int;

    /* Test zero zero. */
    ret = imr_add_range(0, 0, 0, 0);
    imr_self_test_result((ret < 0) as c_int, b"zero sized IMR\n\0".as_ptr() as *const c_char);

    /* Test exact overlap. */
    ret = imr_add_range(base, size, IMR_CPU, IMR_CPU);
    imr_self_test_result((ret < 0) as c_int, fmt_over.as_ptr() as *const c_char,
                         __va(base), __va(base + size));

    /* Test overlap with base inside of existing. */
    base = base.wrapping_add(size).wrapping_sub(IMR_ALIGN);
    ret = imr_add_range(base, size, IMR_CPU, IMR_CPU);
    imr_self_test_result((ret < 0) as c_int, fmt_over.as_ptr() as *const c_char,
                         __va(base), __va(base + size));

    /* Test overlap with end inside of existing. */
    base = base.wrapping_sub(size).wrapping_sub(IMR_ALIGN.wrapping_mul(2));
    ret = imr_add_range(base, size, IMR_CPU, IMR_CPU);
    imr_self_test_result((ret < 0) as c_int, fmt_over.as_ptr() as *const c_char,
                         __va(base), __va(base + size));

    /* Test that a 1 KiB IMR @ zero with read/write all will bomb out. */
    ret = imr_add_range(0, IMR_ALIGN, IMR_READ_ACCESS_ALL, IMR_WRITE_ACCESS_ALL);
    imr_self_test_result((ret < 0) as c_int,
                         b"1KiB IMR @ 0x00000000 - access-all\n\0".as_ptr() as *const c_char);

    /* Test that a 1 KiB IMR @ zero with CPU only will work. */
    ret = imr_add_range(0, IMR_ALIGN, IMR_CPU, IMR_CPU);
    imr_self_test_result((ret >= 0) as c_int,
                         b"1KiB IMR @ 0x00000000 - cpu-access\n\0".as_ptr() as *const c_char);
    if ret >= 0 {
        ret = imr_remove_range(0, IMR_ALIGN);
        imr_self_test_result((ret == 0) as c_int, b"teardown - cpu-access\n\0".as_ptr() as *const c_char);
    }

    /* Test 2 KiB works. */
    size = IMR_ALIGN.wrapping_mul(2);
    ret = imr_add_range(0, size, IMR_READ_ACCESS_ALL, IMR_WRITE_ACCESS_ALL);
    imr_self_test_result((ret >= 0) as c_int, b"2KiB IMR @ 0x00000000\n\0".as_ptr() as *const c_char);
    if ret >= 0 {
        ret = imr_remove_range(0, size);
        imr_self_test_result((ret == 0) as c_int, b"teardown 2KiB\n\0".as_ptr() as *const c_char);
    }
}

static IMR_IDS: [X86CpuId; 2] = [
    // X86_MATCH_VFM(INTEL_QUARK_X1000, NULL),
    X86CpuId { _private: [] },
    X86CpuId { _private: [] },
];

/* imr_self_test_init - entry point for IMR driver. */
unsafe fn imr_self_test_init() -> c_int {
    if !x86_match_cpu(IMR_IDS.as_ptr()).is_null() {
        imr_self_test();
    }
    0
}

// device_initcall(imr_self_test_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
