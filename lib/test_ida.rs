// SPDX-License-Identifier: GPL-2.0+
/*
 * test_ida.c: Test the IDA API
 * Copyright (c) 2016-2018 Microsoft Corporation
 * Copyright (c) 2018 Oracle Corporation
 * Author: Matthew Wilcox <willy@infradead.org>
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong};

#[repr(C)]
pub struct ida {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn ida_alloc(ida: *mut ida, gfp: c_uint) -> c_int;
    fn ida_alloc_min(ida: *mut ida, min: c_ulong, gfp: c_uint) -> c_int;
    fn ida_free(ida: *mut ida, id: c_ulong);
    fn ida_destroy(ida: *mut ida);
    fn ida_is_empty(ida: *const ida) -> bool;
    fn ida_exists(ida: *const ida, id: c_ulong) -> bool;
    fn ida_find_first(ida: *const ida) -> c_int;
    fn ida_find_first_range(ida: *const ida, min: c_ulong, max: c_ulong) -> c_int;
    fn dump_stack();
    fn printk(fmt: *const c_char, ...);
}

static mut TESTS_RUN: c_uint = 0;
static mut TESTS_PASSED: c_uint = 0;

const GFP_KERNEL: c_uint = 0;
const IDA_BITMAP_BITS: c_ulong = 128;
const BITS_PER_LONG: c_ulong = 64;
const ENOSPC: c_int = 28;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const INT_MAX: c_ulong = 2_147_483_647;

unsafe fn ida_dump(_ida: *mut ida) {}

unsafe fn ida_bug_on(ida: *mut ida, condition: bool) {
    TESTS_RUN += 1;
    if condition {
        ida_dump(ida);
        dump_stack();
    } else {
        TESTS_PASSED += 1;
    }
}

unsafe fn ida_check_alloc(ida: *mut ida) {
    let mut i: c_ulong;
    let mut id: c_int;
    i = 0;
    while i < 10000 {
        ida_bug_on(ida, ida_alloc(ida, GFP_KERNEL) != i as c_int);
        i += 1;
    }
    ida_free(ida, 20);
    ida_free(ida, 21);
    i = 0;
    while i < 3 {
        id = ida_alloc(ida, GFP_KERNEL);
        ida_bug_on(ida, id < 0);
        if i == 2 { ida_bug_on(ida, id != 10000); }
        i += 1;
    }
    i = 0;
    while i < 5000 { ida_free(ida, i); i += 1; }
    ida_bug_on(ida, ida_alloc_min(ida, 5000, GFP_KERNEL) != 10001);
    ida_destroy(ida);
    ida_bug_on(ida, !ida_is_empty(ida));
}

unsafe fn ida_check_destroy_1(ida: *mut ida, base: c_ulong) {
    ida_bug_on(ida, ida_alloc_min(ida, base, GFP_KERNEL) as c_ulong != base);
    ida_bug_on(ida, !ida_is_empty(ida));
    ida_destroy(ida);
    ida_bug_on(ida, !ida_is_empty(ida));
}

unsafe fn ida_check_destroy(ida: *mut ida) {
    ida_bug_on(ida, !ida_is_empty(ida));
    ida_destroy(ida);
    ida_bug_on(ida, !ida_is_empty(ida));
    ida_check_destroy_1(ida, 0); ida_check_destroy_1(ida, 1);
    ida_check_destroy_1(ida, 1023); ida_check_destroy_1(ida, 1024);
    ida_check_destroy_1(ida, 12345678);
}

unsafe fn ida_check_leaf(ida: *mut ida, base: c_ulong) {
    let mut i = 0;
    while i < IDA_BITMAP_BITS {
        ida_bug_on(ida, ida_alloc_min(ida, base, GFP_KERNEL) as c_ulong != base + i);
        i += 1;
    }
    ida_destroy(ida); ida_bug_on(ida, !ida_is_empty(ida));
    ida_bug_on(ida, ida_alloc(ida, GFP_KERNEL) != 0);
    ida_bug_on(ida, !ida_is_empty(ida)); ida_free(ida, 0);
    ida_bug_on(ida, !ida_is_empty(ida));
}

unsafe fn ida_check_max(ida: *mut ida) {
    let mut j = 1;
    while j < 65537 {
        let base = (1u64 << 31) - j;
        let mut i = 0;
        while i < j { ida_bug_on(ida, ida_alloc_min(ida, base, GFP_KERNEL) as u64 != base + i); i += 1; }
        ida_bug_on(ida, ida_alloc_min(ida, base, GFP_KERNEL) != -ENOSPC);
        ida_destroy(ida); ida_bug_on(ida, !ida_is_empty(ida)); j *= 2;
    }
}

unsafe fn ida_check_conv(ida: *mut ida) {
    let mut i = 0;
    while i < IDA_BITMAP_BITS * 2 {
        ida_bug_on(ida, ida_alloc_min(ida, i + 1, GFP_KERNEL) as c_ulong != i + 1);
        ida_bug_on(ida, ida_alloc_min(ida, i + BITS_PER_LONG, GFP_KERNEL) as c_ulong != i + BITS_PER_LONG);
        ida_free(ida, i + 1); ida_free(ida, i + BITS_PER_LONG); ida_bug_on(ida, !ida_is_empty(ida)); i += IDA_BITMAP_BITS;
    }
    i = 0; while i < IDA_BITMAP_BITS * 2 { ida_bug_on(ida, ida_alloc(ida, GFP_KERNEL) != i as c_int); i += 1; }
    i = IDA_BITMAP_BITS * 2; while i > 0 { i -= 1; ida_free(ida, i); } ida_bug_on(ida, !ida_is_empty(ida));
    i = 0; while i < IDA_BITMAP_BITS + BITS_PER_LONG - 4 { ida_bug_on(ida, ida_alloc(ida, GFP_KERNEL) != i as c_int); i += 1; }
    while i > 0 { i -= 1; ida_free(ida, i); } ida_bug_on(ida, !ida_is_empty(ida));
}

unsafe fn ida_check_bad_free(ida: *mut ida) {
    printk(b"vvv Ignore \"not allocated\" warnings\0".as_ptr() as *const c_char);
    ida_free(ida, 0); let mut i = 0; while i < 31 { ida_free(ida, 1 << i); i += 1; }
    ida_bug_on(ida, ida_alloc_min(ida, 3, GFP_KERNEL) != 3); ida_free(ida, 0); i = 0; while i < 31 { ida_free(ida, 1 << i); i += 1; }
    ida_bug_on(ida, ida_alloc_min(ida, 1023, GFP_KERNEL) != 1023); ida_free(ida, 0); i = 0; while i < 31 { ida_free(ida, 1 << i); i += 1; }
    ida_bug_on(ida, ida_alloc_min(ida, (1 << 20) - 1, GFP_KERNEL) != (1 << 20) - 1); ida_free(ida, 0); i = 0; while i < 31 { ida_free(ida, 1 << i); i += 1; }
    printk(b"^^^ \"not allocated\" warnings over\0".as_ptr() as *const c_char);
    ida_free(ida, 3); ida_free(ida, 1023); ida_free(ida, (1 << 20) - 1); ida_bug_on(ida, !ida_is_empty(ida));
}

unsafe fn ida_check_find_first(ida: *mut ida) {
    for x in [0,3,63,1023,(1<<20)-1] { ida_bug_on(ida, ida_exists(ida,x)); }
    ida_bug_on(ida, ida_alloc_min(ida,3,GFP_KERNEL)!=3); ida_bug_on(ida,ida_exists(ida,0)); ida_bug_on(ida,!ida_exists(ida,3)); ida_bug_on(ida,ida_exists(ida,63)); ida_bug_on(ida,ida_exists(ida,1023)); ida_bug_on(ida,ida_exists(ida,(1<<20)-1));
    ida_bug_on(ida, ida_alloc_min(ida,63,GFP_KERNEL)!=63); ida_bug_on(ida,ida_exists(ida,0)); ida_bug_on(ida,!ida_exists(ida,3)); ida_bug_on(ida,!ida_exists(ida,63)); ida_bug_on(ida,ida_exists(ida,1023)); ida_bug_on(ida,ida_exists(ida,(1<<20)-1));
    ida_bug_on(ida, ida_alloc_min(ida,1023,GFP_KERNEL)!=1023); ida_bug_on(ida,ida_exists(ida,0)); ida_bug_on(ida,!ida_exists(ida,3)); ida_bug_on(ida,!ida_exists(ida,63)); ida_bug_on(ida,!ida_exists(ida,1023)); ida_bug_on(ida,ida_exists(ida,(1<<20)-1));
    ida_bug_on(ida, ida_alloc_min(ida,(1<<20)-1,GFP_KERNEL)!=(1<<20)-1); ida_bug_on(ida,ida_exists(ida,0)); ida_bug_on(ida,!ida_exists(ida,3)); ida_bug_on(ida,!ida_exists(ida,63)); ida_bug_on(ida,!ida_exists(ida,1023)); ida_bug_on(ida,!ida_exists(ida,(1<<20)-1));
    ida_bug_on(ida,ida_find_first(ida)!=3); ida_bug_on(ida,ida_find_first_range(ida,-1i64 as c_ulong,2)!=-EINVAL); ida_bug_on(ida,ida_find_first_range(ida,0,2)!=-ENOENT); ida_bug_on(ida,ida_find_first_range(ida,0,3)!=3); ida_bug_on(ida,ida_find_first_range(ida,1,3)!=3); ida_bug_on(ida,ida_find_first_range(ida,3,3)!=3); ida_bug_on(ida,ida_find_first_range(ida,2,4)!=3); ida_bug_on(ida,ida_find_first_range(ida,4,3)!=-ENOENT); ida_bug_on(ida,ida_find_first_range(ida,4,60)!=-ENOENT); ida_bug_on(ida,ida_find_first_range(ida,4,64)!=63); ida_bug_on(ida,ida_find_first_range(ida,63,63)!=63); ida_bug_on(ida,ida_find_first_range(ida,64,1026)!=1023); ida_bug_on(ida,ida_find_first_range(ida,1023,1023)!=1023); ida_bug_on(ida,ida_find_first_range(ida,1023,(1<<20)-1)!=1023); ida_bug_on(ida,ida_find_first_range(ida,1024,(1<<20)-1)!=(1<<20)-1); ida_bug_on(ida,ida_find_first_range(ida,1<<20,INT_MAX)!=-ENOENT);
    ida_free(ida,3); ida_free(ida,63); ida_free(ida,1023); ida_free(ida,(1<<20)-1); ida_bug_on(ida,!ida_is_empty(ida));
}

static mut IDA: ida = ida { _private: [] };

pub unsafe fn ida_checks() -> c_int {
    ida_bug_on(&raw mut IDA,!ida_is_empty(&raw const IDA)); ida_check_alloc(&raw mut IDA); ida_check_destroy(&raw mut IDA); ida_check_leaf(&raw mut IDA,0); ida_check_leaf(&raw mut IDA,1024); ida_check_leaf(&raw mut IDA,1024*64); ida_check_max(&raw mut IDA); ida_check_conv(&raw mut IDA); ida_check_bad_free(&raw mut IDA); ida_check_find_first(&raw mut IDA);
    printk(b"IDA: %u of %u tests passed\n\0".as_ptr() as *const c_char, TESTS_PASSED, TESTS_RUN); if TESTS_RUN != TESTS_PASSED { 0 } else { -EINVAL }
}

pub unsafe fn ida_exit() {}

// C module metadata: module_init(ida_checks), module_exit(ida_exit).
// MODULE_AUTHOR("Matthew Wilcox <willy@infradead.org>");
// MODULE_DESCRIPTION("Test the IDA API");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
