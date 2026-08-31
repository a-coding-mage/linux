// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

/* Original C dependencies:
 * #define _GNU_SOURCE
 * #include <string.h>
 * #include <byteswap.h>
 * #include <test_progs.h>
 * #include <bpf/btf.h>
 */

use core::ffi::{c_char, c_int, c_void};

type __u16 = u16;
type __u32 = u32;

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_header {
    pub magic: __u16,
}

#[repr(C)]
pub struct btf_type {
    pub name_off: __u32,
    pub info: __u32,
    pub size: __u32,
    pub type_: __u32,
}

#[repr(C)]
pub struct btf_var {
    pub linkage: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btf_endianness {
    BTF_LITTLE_ENDIAN = 0,
    BTF_BIG_ENDIAN = 1,
}

const BTF_MAGIC: __u16 = 0xeB9F;
const BTF_VAR_GLOBAL_ALLOCATED: __u32 = 1;

unsafe extern "C" {
    fn btf__parse_elf(path: *const c_char, opts: *const c_void) -> *mut btf;
    fn btf__endianness(btf: *const btf) -> btf_endianness;
    fn btf__set_endianness(btf: *mut btf, endian: btf_endianness);
    fn btf__raw_data(btf: *const btf, size: *mut __u32) -> *const c_void;
    fn btf__new(data: *const c_void, size: __u32) -> *mut btf;
    fn btf__type_cnt(btf: *const btf) -> c_int;
    fn btf__add_var(
        btf: *mut btf,
        name: *const c_char,
        linkage: __u32,
        type_id: __u32,
    ) -> c_int;
    fn btf__free(btf: *mut btf);
    fn btf__type_by_id(btf: *const btf, type_id: c_int) -> *const btf_type;
    fn btf__str_by_offset(btf: *const btf, offset: __u32) -> *const c_char;
    fn btf_var(t: *const btf_type) -> *const btf_var;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ<T>(actual: T, expected: T, name: *const c_char);
    fn ASSERT_OK(ret: c_int, name: *const c_char);
    fn ASSERT_GT<T>(actual: T, expected: T, name: *const c_char);
    fn ASSERT_STREQ(actual: *const c_char, expected: *const c_char, name: *const c_char);

    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
}

#[inline]
const fn bswap_16(x: __u16) -> __u16 {
    x.swap_bytes()
}

pub unsafe fn test_btf_endian() {
    /*
     * C selected this with:
     * #if __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__
     * #elif __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__
     * #else
     * #error "Unrecognized __BYTE_ORDER__"
     * #endif
     */
    #[cfg(target_endian = "little")]
    let endian: btf_endianness = btf_endianness::BTF_LITTLE_ENDIAN;
    #[cfg(target_endian = "big")]
    let endian: btf_endianness = btf_endianness::BTF_BIG_ENDIAN;

    let swap_endian: btf_endianness = if 1 - endian as c_int == 0 {
        btf_endianness::BTF_LITTLE_ENDIAN
    } else {
        btf_endianness::BTF_BIG_ENDIAN
    };
    let mut btf: *mut btf = core::ptr::null_mut();
    let mut swap_btf: *mut btf = core::ptr::null_mut();
    let mut raw_data: *const c_void;
    let mut swap_raw_data: *const c_void;
    let mut t: *const btf_type;
    let mut hdr: *const btf_header;
    let mut raw_sz: __u32 = 0;
    let mut swap_raw_sz: __u32 = 0;
    let mut var_id: c_int;

    /* Load BTF in native endianness */
    btf = btf__parse_elf(
        c"btf_dump_test_case_syntax.bpf.o".as_ptr(),
        core::ptr::null(),
    );
    if !ASSERT_OK_PTR(btf as *const c_void, c"parse_native_btf".as_ptr()) {
        goto_err_out(btf, swap_btf);
        return;
    }

    ASSERT_EQ(btf__endianness(btf), endian, c"endian".as_ptr());
    btf__set_endianness(btf, swap_endian);
    ASSERT_EQ(btf__endianness(btf), swap_endian, c"endian".as_ptr());

    /* Get raw BTF data in non-native endianness... */
    raw_data = btf__raw_data(btf, &mut raw_sz);
    if !ASSERT_OK_PTR(raw_data, c"raw_data_inverted".as_ptr()) {
        goto_err_out(btf, swap_btf);
        return;
    }

    /* ...and open it as a new BTF instance */
    swap_btf = btf__new(raw_data, raw_sz);
    if !ASSERT_OK_PTR(swap_btf as *const c_void, c"parse_swap_btf".as_ptr()) {
        goto_err_out(btf, swap_btf);
        return;
    }

    ASSERT_EQ(btf__endianness(swap_btf), swap_endian, c"endian".as_ptr());
    ASSERT_EQ(btf__type_cnt(swap_btf), btf__type_cnt(btf), c"nr_types".as_ptr());

    swap_raw_data = btf__raw_data(swap_btf, &mut swap_raw_sz);
    if !ASSERT_OK_PTR(swap_raw_data, c"swap_raw_data".as_ptr()) {
        goto_err_out(btf, swap_btf);
        return;
    }

    /* both raw data should be identical (with non-native endianness) */
    ASSERT_OK(
        memcmp(raw_data, swap_raw_data, raw_sz as usize),
        c"mem_identical".as_ptr(),
    );

    /* make sure that at least BTF header data is really swapped */
    hdr = swap_raw_data as *const btf_header;
    ASSERT_EQ(
        bswap_16((*hdr).magic),
        BTF_MAGIC,
        c"btf_magic_swapped".as_ptr(),
    );
    ASSERT_EQ(raw_sz, swap_raw_sz, c"raw_sizes".as_ptr());

    /* swap it back to native endianness */
    btf__set_endianness(swap_btf, endian);
    swap_raw_data = btf__raw_data(swap_btf, &mut swap_raw_sz);
    if !ASSERT_OK_PTR(swap_raw_data, c"swap_raw_data".as_ptr()) {
        goto_err_out(btf, swap_btf);
        return;
    }

    /* now header should have native BTF_MAGIC */
    hdr = swap_raw_data as *const btf_header;
    ASSERT_EQ((*hdr).magic, BTF_MAGIC, c"btf_magic_native".as_ptr());
    ASSERT_EQ(raw_sz, swap_raw_sz, c"raw_sizes".as_ptr());

    /* now modify original BTF */
    var_id = btf__add_var(btf, c"some_var".as_ptr(), BTF_VAR_GLOBAL_ALLOCATED, 1);
    ASSERT_GT(var_id, 0, c"var_id".as_ptr());

    btf__free(swap_btf);
    swap_btf = core::ptr::null_mut();

    btf__set_endianness(btf, swap_endian);
    raw_data = btf__raw_data(btf, &mut raw_sz);
    if !ASSERT_OK_PTR(raw_data, c"raw_data_inverted".as_ptr()) {
        goto_err_out(btf, swap_btf);
        return;
    }

    /* and re-open swapped raw data again */
    swap_btf = btf__new(raw_data, raw_sz);
    if !ASSERT_OK_PTR(swap_btf as *const c_void, c"parse_swap_btf".as_ptr()) {
        goto_err_out(btf, swap_btf);
        return;
    }

    ASSERT_EQ(btf__endianness(swap_btf), swap_endian, c"endian".as_ptr());
    ASSERT_EQ(btf__type_cnt(swap_btf), btf__type_cnt(btf), c"nr_types".as_ptr());

    /* the type should appear as if it was stored in native endianness */
    t = btf__type_by_id(swap_btf, var_id);
    ASSERT_STREQ(
        btf__str_by_offset(swap_btf, (*t).name_off),
        c"some_var".as_ptr(),
        c"var_name".as_ptr(),
    );
    ASSERT_EQ(
        (*btf_var(t)).linkage,
        BTF_VAR_GLOBAL_ALLOCATED,
        c"var_linkage".as_ptr(),
    );
    ASSERT_EQ((*t).type_, 1, c"var_type".as_ptr());

    goto_err_out(btf, swap_btf);
}

unsafe fn goto_err_out(btf: *mut btf, swap_btf: *mut btf) {
    btf__free(btf);
    btf__free(swap_btf);
}
