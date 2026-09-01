// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Google LLC. */

// Dependencies in the original C source:
// <test_progs.h>
// "test_snprintf.skel.h"
// "test_snprintf_single.skel.h"

use core::ffi::{c_char, c_int, c_void};

const EINVAL: c_int = 22;

const EXP_NUM_OUT: &[u8] = b"-8 9 96 -424242 1337 DABBAD00\0";
const EXP_NUM_RET: usize = EXP_NUM_OUT.len();

const EXP_IP_OUT: &[u8] =
    b"127.000.000.001 0000:0000:0000:0000:0000:0000:0000:0001\0";
const EXP_IP_RET: usize = EXP_IP_OUT.len();

/* The third specifier, %pB, depends on compiler inlining so don't check it */
const EXP_SYM_OUT: &[u8] = b"schedule schedule+0x0/\0";
const MIN_SYM_RET: usize = EXP_SYM_OUT.len();

/* The third specifier, %p, is a hashed pointer which changes on every reboot */
const EXP_ADDR_OUT: &[u8] = b"0000000000000000 ffff00000add4e55 \0";
const EXP_ADDR_RET: usize = b"0000000000000000 ffff00000add4e55 unknownhashedptr\0".len();

const EXP_STR_OUT: &[u8] = b"str1         a  b c      d e longstr\0";
const EXP_STR_RET: usize = EXP_STR_OUT.len();

const EXP_OVER_OUT: &[u8] = b"%over\0";
const EXP_OVER_RET: c_int = 10;

const EXP_PAD_OUT: &[u8] = b"    4 000\0";
const EXP_PAD_RET: c_int = 900007;

const EXP_NO_ARG_OUT: &[u8] = b"simple case\0";
const EXP_NO_ARG_RET: c_int = 12;

const EXP_NO_BUF_RET: c_int = 29;

#[repr(C)]
pub struct test_snprintf_bss {
    pub pid: c_int,
    pub num_out: [c_char; 0],
    pub num_ret: usize,
    pub ip_out: [c_char; 0],
    pub ip_ret: usize,
    pub sym_out: [c_char; 0],
    pub sym_ret: usize,
    pub addr_out: [c_char; 0],
    pub addr_ret: usize,
    pub str_out: [c_char; 0],
    pub str_ret: usize,
    pub over_out: [c_char; 0],
    pub over_ret: c_int,
    pub pad_out: [c_char; 0],
    pub pad_ret: c_int,
    pub noarg_out: [c_char; 0],
    pub noarg_ret: c_int,
    pub nobuf_ret: c_int,
}

#[repr(C)]
pub struct test_snprintf {
    pub bss: *mut test_snprintf_bss,
}

#[repr(C)]
pub struct test_snprintf_single_rodata {
    pub fmt: [c_char; 0],
}

#[repr(C)]
pub struct test_snprintf_single {
    pub rodata: *mut test_snprintf_single_rodata,
}

unsafe extern "C" {
    fn test_snprintf__open_and_load() -> *mut test_snprintf;
    fn test_snprintf__attach(skel: *mut test_snprintf) -> c_int;
    fn test_snprintf__destroy(skel: *mut test_snprintf);

    fn test_snprintf_single__open() -> *mut test_snprintf_single;
    fn test_snprintf_single__load(skel: *mut test_snprintf_single) -> c_int;
    fn test_snprintf_single__destroy(skel: *mut test_snprintf_single);

    fn test__start_subtest(name: *const c_char) -> bool;
    fn getpid() -> c_int;
    fn usleep(usec: u32) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;

    fn ASSERT_OK_PTR(ptr: *mut test_snprintf, name: *const c_char) -> bool;
    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_STREQ(actual: *const c_char, expected: *const c_char, name: *const c_char) -> bool;
    fn ASSERT_EQ<T>(actual: T, expected: T, name: *const c_char) -> bool;
    fn ASSERT_LT<T>(left: T, right: T, name: *const c_char) -> bool;
}

unsafe fn test_snprintf_positive() {
    let exp_addr_out = *EXP_ADDR_OUT;
    let exp_sym_out = *EXP_SYM_OUT;
    let mut skel: *mut test_snprintf;

    skel = test_snprintf__open_and_load();
    if !ASSERT_OK_PTR(skel, c"skel_open".as_ptr()) {
        return;
    }

    (*(*skel).bss).pid = getpid();

    if !ASSERT_OK(test_snprintf__attach(skel), c"skel_attach".as_ptr()) {
        goto_cleanup(skel);
        return;
    }

    /* trigger tracepoint */
    usleep(1);

    ASSERT_STREQ((*(*skel).bss).num_out.as_ptr(), EXP_NUM_OUT.as_ptr().cast(), c"num_out".as_ptr());
    ASSERT_EQ((*(*skel).bss).num_ret, EXP_NUM_RET, c"num_ret".as_ptr());

    ASSERT_STREQ((*(*skel).bss).ip_out.as_ptr(), EXP_IP_OUT.as_ptr().cast(), c"ip_out".as_ptr());
    ASSERT_EQ((*(*skel).bss).ip_ret, EXP_IP_RET, c"ip_ret".as_ptr());

    ASSERT_OK(
        memcmp(
            (*(*skel).bss).sym_out.as_ptr().cast(),
            exp_sym_out.as_ptr().cast(),
            exp_sym_out.len() - 1,
        ),
        c"sym_out".as_ptr(),
    );
    ASSERT_LT(MIN_SYM_RET, (*(*skel).bss).sym_ret, c"sym_ret".as_ptr());

    ASSERT_OK(
        memcmp(
            (*(*skel).bss).addr_out.as_ptr().cast(),
            exp_addr_out.as_ptr().cast(),
            exp_addr_out.len() - 1,
        ),
        c"addr_out".as_ptr(),
    );
    ASSERT_EQ((*(*skel).bss).addr_ret, EXP_ADDR_RET, c"addr_ret".as_ptr());

    ASSERT_STREQ((*(*skel).bss).str_out.as_ptr(), EXP_STR_OUT.as_ptr().cast(), c"str_out".as_ptr());
    ASSERT_EQ((*(*skel).bss).str_ret, EXP_STR_RET, c"str_ret".as_ptr());

    ASSERT_STREQ((*(*skel).bss).over_out.as_ptr(), EXP_OVER_OUT.as_ptr().cast(), c"over_out".as_ptr());
    ASSERT_EQ((*(*skel).bss).over_ret, EXP_OVER_RET, c"over_ret".as_ptr());

    ASSERT_STREQ((*(*skel).bss).pad_out.as_ptr(), EXP_PAD_OUT.as_ptr().cast(), c"pad_out".as_ptr());
    ASSERT_EQ((*(*skel).bss).pad_ret, EXP_PAD_RET, c"pad_ret".as_ptr());

    ASSERT_STREQ(
        (*(*skel).bss).noarg_out.as_ptr(),
        EXP_NO_ARG_OUT.as_ptr().cast(),
        c"no_arg_out".as_ptr(),
    );
    ASSERT_EQ((*(*skel).bss).noarg_ret, EXP_NO_ARG_RET, c"no_arg_ret".as_ptr());

    ASSERT_EQ((*(*skel).bss).nobuf_ret, EXP_NO_BUF_RET, c"no_buf_ret".as_ptr());

    goto_cleanup(skel);
}

unsafe fn goto_cleanup(skel: *mut test_snprintf) {
    test_snprintf__destroy(skel);
}

/* Loads an eBPF object calling bpf_snprintf with up to 10 characters of fmt */
unsafe fn load_single_snprintf(fmt: *mut c_char) -> c_int {
    let skel: *mut test_snprintf_single;
    let ret: c_int;

    skel = test_snprintf_single__open();
    if skel.is_null() {
        return -EINVAL;
    }

    memcpy(
        (*(*skel).rodata).fmt.as_mut_ptr().cast(),
        fmt.cast(),
        core::cmp::min(strlen(fmt) + 1, 10),
    );

    ret = test_snprintf_single__load(skel);
    test_snprintf_single__destroy(skel);

    ret
}

unsafe fn test_snprintf_negative() {
    ASSERT_OK(load_single_snprintf(c"valid %d".as_ptr() as *mut c_char), c"valid usage".as_ptr());

    ASSERT_ERR(load_single_snprintf(c"0123456789".as_ptr() as *mut c_char), c"no terminating zero".as_ptr());
    ASSERT_ERR(load_single_snprintf(c"%d %d".as_ptr() as *mut c_char), c"too many specifiers".as_ptr());
    ASSERT_ERR(load_single_snprintf(c"%pi5".as_ptr() as *mut c_char), c"invalid specifier 1".as_ptr());
    ASSERT_ERR(load_single_snprintf(c"%a".as_ptr() as *mut c_char), c"invalid specifier 2".as_ptr());
    ASSERT_ERR(load_single_snprintf(c"%".as_ptr() as *mut c_char), c"invalid specifier 3".as_ptr());
    ASSERT_ERR(load_single_snprintf(c"%12345678".as_ptr() as *mut c_char), c"invalid specifier 4".as_ptr());
    ASSERT_ERR(load_single_snprintf(c"%--------".as_ptr() as *mut c_char), c"invalid specifier 5".as_ptr());
    ASSERT_ERR(load_single_snprintf(c"%lc".as_ptr() as *mut c_char), c"invalid specifier 6".as_ptr());
    ASSERT_ERR(load_single_snprintf(c"%llc".as_ptr() as *mut c_char), c"invalid specifier 7".as_ptr());
    ASSERT_OK(load_single_snprintf(b"\x80\0".as_ptr() as *mut c_char), c"non ascii plain text".as_ptr());
    ASSERT_ERR(load_single_snprintf(b"%\x80\0".as_ptr() as *mut c_char), c"non ascii in specifier".as_ptr());
    ASSERT_ERR(load_single_snprintf(b"\x01\0".as_ptr() as *mut c_char), c"non printable character".as_ptr());
    ASSERT_ERR(load_single_snprintf(c"%p%".as_ptr() as *mut c_char), c"invalid specifier 8".as_ptr());
    ASSERT_ERR(load_single_snprintf(c"%s%".as_ptr() as *mut c_char), c"invalid specifier 9".as_ptr());
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_snprintf() {
    if test__start_subtest(c"snprintf_positive".as_ptr()) {
        test_snprintf_positive();
    }
    if test__start_subtest(c"snprintf_negative".as_ptr()) {
        test_snprintf_negative();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
