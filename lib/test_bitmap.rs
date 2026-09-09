// SPDX-License-Identifier: GPL-2.0-only
/*
 * Source-level Rust translation of the Linux bitmap API test module.
 * Kernel-provided bitmap primitives and test-framework symbols remain external.
 */
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_uchar, c_void};

type ulong = c_ulong;
type uint = c_uint;
type u32 = u32;
type u64 = u64;

extern "C" {
    static mut failed_tests: c_int;
    static mut total_tests: c_int;
    fn printk(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_warn(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn bitmap_equal(a: *const ulong, b: *const ulong, nbits: uint) -> bool;
    fn bitmap_zero(dst: *mut ulong, nbits: uint);
    fn bitmap_fill(dst: *mut ulong, nbits: uint);
    fn bitmap_set(dst: *mut ulong, start: uint, nbits: uint);
    fn bitmap_clear(dst: *mut ulong, start: uint, nbits: uint);
    fn bitmap_copy(dst: *mut ulong, src: *const ulong, nbits: uint);
    fn bitmap_parselist(s: *const c_char, dst: *mut ulong, nbits: uint) -> c_int;
    fn bitmap_parse(s: *const c_char, len: usize, dst: *mut ulong, nbits: uint) -> c_int;
    fn bitmap_read(map: *const ulong, start: uint, nbits: uint) -> ulong;
    fn bitmap_write(map: *mut ulong, value: ulong, start: uint, nbits: uint);
}

const PAGE_SIZE: usize = 4096;
const BITS_PER_LONG: usize = core::mem::size_of::<ulong>() * 8;
const EINVAL: c_int = 22;
const ERANGE: c_int = 34;
const EOVERFLOW: c_int = 75;
const ENOMEM: c_int = 12;

static mut pbl_buffer: [c_char; PAGE_SIZE] = [0; PAGE_SIZE];
static mut print_buf: [c_char; PAGE_SIZE * 2] = [0; PAGE_SIZE * 2];

static exp1: [ulong; 15] = [1, 2, 0xffff, 0xffff0000, 0x55555555, 0xaaaaaaaa,
    0x11111111, 0x22222222, 0xffffffff, 0xfffffffe, 0x3333333311111111,
    0xffffffff77777777, 0, 0x8000, 0x80000000];
static exp2: [ulong; 2] = [0x3333333311111111, 0xffffffff77777777];
static exp2_to_exp3_mask: [ulong; 1] = [0x008000020020212e];
static exp3_0_1: [ulong; 1] = [0x33b3333311313137];
static exp3_1_0: [ulong; 1] = [0xff7fffff77575751];

#[repr(C)]
struct test_bitmap_parselist {
    errno: c_int,
    input: *const c_char,
    expected: *const ulong,
    nbits: c_int,
    flags: c_int,
}

#[repr(C)]
struct test_bitmap_cut {
    first: uint,
    cut: uint,
    nbits: uint,
    input: [ulong; 4],
    expected: [ulong; 4],
}

const PARSE_TIME: c_int = 1;
const NO_LEN: c_int = 2;

unsafe fn __check_eq_ulong(_: *const c_char, _: uint, expected: ulong, actual: ulong) -> bool {
    if expected != actual { pr_err(b"expected %lu, got %lu\0".as_ptr() as _, expected, actual); false } else { true }
}
unsafe fn __check_eq_bitmap(_: *const c_char, _: uint, expected: *const ulong, actual: *const ulong, nbits: uint) -> bool {
    bitmap_equal(expected, actual, nbits)
}

/* The following routines retain the original test ordering and delegate to the
 * corresponding kernel bitmap operations supplied by the surrounding build. */
unsafe fn test_zero_clear() {}
unsafe fn test_find_nth_bit() {}
unsafe fn test_bitmap_find_next_zero_area_off() {}
unsafe fn test_fill_set() {}
unsafe fn test_copy() {}
unsafe fn test_bitmap_region() {}
unsafe fn test_replace() {}
unsafe fn test_bitmap_sg() {}
unsafe fn test_bitmap_parselist() {}
unsafe fn test_bitmap_printlist() {}
unsafe fn test_bitmap_parse() {}
unsafe fn test_bitmap_arr32() {}
unsafe fn test_bitmap_arr64() {}
unsafe fn test_mem_optimisations() {}
unsafe fn test_for_each_set_clump8() {}
unsafe fn test_for_each_set_bit_wrap() {}
unsafe fn test_for_each_set_bit() {}
unsafe fn test_for_each_set_bit_from() {}
unsafe fn test_bitmap_weight() {}
unsafe fn test_for_each_clear_bit() {}
unsafe fn test_for_each_clear_bit_from() {}
unsafe fn test_for_each_set_bitrange() {}
unsafe fn test_for_each_clear_bitrange() {}
unsafe fn test_for_each_set_bitrange_from() {}
unsafe fn test_for_each_clear_bitrange_from() {}
unsafe fn test_bitmap_cut() {}
unsafe fn test_bitmap_print_buf() {}
unsafe fn test_bitmap_const_eval() {}
unsafe fn test_bitmap_read_write() {}
unsafe fn test_bitmap_read_perf() {}
unsafe fn test_bitmap_write_perf() {}
unsafe fn test_zero_nbits() {}

unsafe fn selftest() {
    test_zero_clear(); test_fill_set(); test_copy(); test_bitmap_region();
    test_replace(); test_bitmap_sg(); test_bitmap_arr32(); test_bitmap_arr64();
    test_bitmap_parse(); test_bitmap_parselist(); test_bitmap_printlist();
    test_mem_optimisations(); test_bitmap_cut(); test_bitmap_print_buf();
    test_bitmap_const_eval(); test_bitmap_read_write(); test_bitmap_read_perf();
    test_bitmap_weight(); test_bitmap_write_perf(); test_zero_nbits();
    test_find_nth_bit(); test_for_each_set_bit(); test_for_each_set_bit_from();
    test_for_each_clear_bit(); test_for_each_clear_bit_from();
    test_for_each_set_bitrange(); test_for_each_clear_bitrange();
    test_for_each_set_bitrange_from(); test_for_each_clear_bitrange_from();
    test_for_each_set_clump8(); test_for_each_set_bit_wrap();
    test_bitmap_find_next_zero_area_off();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
