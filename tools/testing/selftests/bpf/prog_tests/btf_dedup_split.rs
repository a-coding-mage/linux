// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
/* Dependencies from <test_progs.h>, <bpf/btf.h>, and "btf_helpers.h" are
 * expected to be supplied by the surrounding selftest harness.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_type {
    pub name_off: c_uint,
    pub info: c_uint,
    pub size: c_uint,
    pub type_: c_uint,
}

#[repr(C)]
pub struct btf_param {
    pub name_off: c_uint,
    pub type_: c_uint,
}

const BTF_INT_SIGNED: c_uint = 1;
const BTF_FWD_STRUCT: c_uint = 0;
const BTF_KIND_FUNC: c_uint = 12;
const ENOENT: c_int = 2;

static mut mod_funcs: [*const c_char; 3] = [
    b"bpf_testmod_test_write\0".as_ptr() as *const c_char,
    b"bpf_kfunc_call_test3\0".as_ptr() as *const c_char,
    b"bpf_kfunc_call_test_pass_ctx\0".as_ptr() as *const c_char,
];

unsafe extern "C" {
    fn btf__new_empty() -> *mut btf;
    fn btf__new_empty_split(base_btf: *mut btf) -> *mut btf;
    fn btf__set_pointer_size(btf: *mut btf, ptr_sz: c_uint);
    fn btf__pointer_size(btf: *mut btf) -> c_uint;
    fn btf__add_int(btf: *mut btf, name: *const c_char, byte_sz: c_uint, encoding: c_uint) -> c_int;
    fn btf__add_ptr(btf: *mut btf, ref_type_id: c_uint) -> c_int;
    fn btf__add_struct(btf: *mut btf, name: *const c_char, byte_sz: c_uint) -> c_int;
    fn btf__add_field(
        btf: *mut btf,
        name: *const c_char,
        type_id: c_uint,
        bit_offset: c_uint,
        bit_size: c_uint,
    ) -> c_int;
    fn btf__add_typedef(btf: *mut btf, name: *const c_char, ref_type_id: c_uint) -> c_int;
    fn btf__add_fwd(btf: *mut btf, name: *const c_char, fwd_kind: c_uint) -> c_int;
    fn btf__find_str(btf: *mut btf, s: *const c_char) -> c_int;
    fn btf__type_by_id(btf: *mut btf, type_id: c_uint) -> *const btf_type;
    fn btf__str_by_offset(btf: *mut btf, offset: c_uint) -> *const c_char;
    fn btf__dedup(btf: *mut btf, opts: *const c_void) -> c_int;
    fn btf__free(btf: *mut btf);
    fn btf_type_c_dump(btf: *mut btf) -> *const c_char;
    fn btf_is_int(t: *const btf_type) -> bool;
    fn btf_is_func_proto(t: *const btf_type) -> bool;
    fn btf_is_mod(t: *const btf_type) -> bool;
    fn btf_is_ptr(t: *const btf_type) -> bool;
    fn btf_is_typedef(t: *const btf_type) -> bool;
    fn btf_vlen(t: *const btf_type) -> c_uint;
    fn btf_params(t: *const btf_type) -> *const btf_param;
    fn btf__load_vmlinux_btf() -> *mut btf;
    fn btf__type_cnt(btf: *mut btf) -> c_int;
    fn btf__parse_split(path: *const c_char, base_btf: *mut btf) -> *mut btf;
    fn btf__find_by_name_kind(btf: *mut btf, name: *const c_char, kind: c_uint) -> c_uint;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_STREQ(actual: *const c_char, expected: *const c_char, name: *const c_char) -> bool;
    fn ASSERT_EQ_UINT(actual: c_uint, expected: c_uint, name: *const c_char) -> bool;
    fn ASSERT_EQ_BOOL(actual: bool, expected: bool, name: *const c_char) -> bool;
    fn ASSERT_NEQ_INT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT_INT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE_UINT(actual: c_uint, expected: c_uint, name: *const c_char) -> bool;
    fn ASSERT_LT_UINT(actual: c_uint, expected: c_uint, name: *const c_char) -> bool;
    fn VALIDATE_RAW_BTF(btf: *mut btf, ...);
}

unsafe fn test_split_simple() {
    let mut t: *const btf_type;
    let mut btf2: *mut btf = ptr::null_mut();
    let str_off: c_int;
    let err: c_int;

    let btf1 = btf__new_empty();
    if !ASSERT_OK_PTR(btf1 as *const c_void, b"empty_main_btf\0".as_ptr() as *const c_char) {
        return;
    }

    btf__set_pointer_size(btf1, 8); /* enforce 64-bit arch */

    btf__add_int(btf1, b"int\0".as_ptr() as *const c_char, 4, BTF_INT_SIGNED); /* [1] int */
    btf__add_ptr(btf1, 1); /* [2] ptr to int */
    btf__add_struct(btf1, b"s1\0".as_ptr() as *const c_char, 4); /* [3] struct s1 { */
    btf__add_field(btf1, b"f1\0".as_ptr() as *const c_char, 1, 0, 0); /*      int f1; */
    /* } */
    btf__add_typedef(btf1, b"t1\0".as_ptr() as *const c_char, 1); /* [4] typedef int */

    VALIDATE_RAW_BTF(
        btf1,
        b"[1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED\0".as_ptr() as *const c_char,
        b"[2] PTR '(anon)' type_id=1\0".as_ptr() as *const c_char,
        b"[3] STRUCT 's1' size=4 vlen=1\n\t'f1' type_id=1 bits_offset=0\0".as_ptr() as *const c_char,
        b"[4] TYPEDEF 't1' type_id=1\0".as_ptr() as *const c_char,
    );

    ASSERT_STREQ(
        btf_type_c_dump(btf1),
        b"struct s1 {\n\tint f1;\n};\n\ntypedef int t1;\n\n\0".as_ptr() as *const c_char,
        b"c_dump\0".as_ptr() as *const c_char,
    );

    'cleanup: {
        btf2 = btf__new_empty_split(btf1);
        if !ASSERT_OK_PTR(btf2 as *const c_void, b"empty_split_btf\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }

        /* pointer size should be "inherited" from main BTF */
        ASSERT_EQ_UINT(btf__pointer_size(btf2), 8, b"inherit_ptr_sz\0".as_ptr() as *const c_char);

        str_off = btf__find_str(btf2, b"int\0".as_ptr() as *const c_char);
        ASSERT_NEQ_INT(str_off, -ENOENT, b"str_int_missing\0".as_ptr() as *const c_char);

        t = btf__type_by_id(btf2, 1);
        if !ASSERT_OK_PTR(t as *const c_void, b"int_type\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }
        ASSERT_EQ_BOOL(btf_is_int(t), true, b"int_kind\0".as_ptr() as *const c_char);
        ASSERT_STREQ(btf__str_by_offset(btf2, (*t).name_off), b"int\0".as_ptr() as *const c_char, b"int_name\0".as_ptr() as *const c_char);

        btf__add_struct(btf2, b"s2\0".as_ptr() as *const c_char, 16); /* [5] struct s2 { */
        btf__add_field(btf2, b"f1\0".as_ptr() as *const c_char, 7, 0, 0); /*      struct s1 f1; */
        btf__add_field(btf2, b"f2\0".as_ptr() as *const c_char, 6, 32, 0); /*      int f2; */
        btf__add_field(btf2, b"f3\0".as_ptr() as *const c_char, 2, 64, 0); /*      int *f3; */
        /* } */

        /* duplicated int */
        btf__add_int(btf2, b"int\0".as_ptr() as *const c_char, 4, BTF_INT_SIGNED); /* [6] int */

        /* duplicated struct s1 */
        btf__add_struct(btf2, b"s1\0".as_ptr() as *const c_char, 4); /* [7] struct s1 { */
        btf__add_field(btf2, b"f1\0".as_ptr() as *const c_char, 6, 0, 0); /*      int f1; */
        /* } */

        /* duplicated typedef t1 */
        btf__add_typedef(btf2, b"t1\0".as_ptr() as *const c_char, 6); /* [8] typedef int */

        VALIDATE_RAW_BTF(
            btf2,
            b"[1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED\0".as_ptr() as *const c_char,
            b"[2] PTR '(anon)' type_id=1\0".as_ptr() as *const c_char,
            b"[3] STRUCT 's1' size=4 vlen=1\n\t'f1' type_id=1 bits_offset=0\0".as_ptr() as *const c_char,
            b"[4] TYPEDEF 't1' type_id=1\0".as_ptr() as *const c_char,
            b"[5] STRUCT 's2' size=16 vlen=3\n\t'f1' type_id=7 bits_offset=0\n\t'f2' type_id=6 bits_offset=32\n\t'f3' type_id=2 bits_offset=64\0".as_ptr() as *const c_char,
            b"[6] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED\0".as_ptr() as *const c_char,
            b"[7] STRUCT 's1' size=4 vlen=1\n\t'f1' type_id=6 bits_offset=0\0".as_ptr() as *const c_char,
            b"[8] TYPEDEF 't1' type_id=6\0".as_ptr() as *const c_char,
        );

        ASSERT_STREQ(
            btf_type_c_dump(btf2),
            b"struct s1 {\n\tint f1;\n};\n\ntypedef int t1;\n\nstruct s1___2 {\n\tint f1;\n};\n\nstruct s2 {\n\tstruct s1___2 f1;\n\tint f2;\n\tint *f3;\n};\n\ntypedef int t1___2;\n\n\0".as_ptr() as *const c_char,
            b"c_dump\0".as_ptr() as *const c_char,
        );

        err = btf__dedup(btf2, ptr::null());
        if !ASSERT_OK(err, b"btf_dedup\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }

        VALIDATE_RAW_BTF(
            btf2,
            b"[1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED\0".as_ptr() as *const c_char,
            b"[2] PTR '(anon)' type_id=1\0".as_ptr() as *const c_char,
            b"[3] STRUCT 's1' size=4 vlen=1\n\t'f1' type_id=1 bits_offset=0\0".as_ptr() as *const c_char,
            b"[4] TYPEDEF 't1' type_id=1\0".as_ptr() as *const c_char,
            b"[5] STRUCT 's2' size=16 vlen=3\n\t'f1' type_id=3 bits_offset=0\n\t'f2' type_id=1 bits_offset=32\n\t'f3' type_id=2 bits_offset=64\0".as_ptr() as *const c_char,
        );

        ASSERT_STREQ(
            btf_type_c_dump(btf2),
            b"struct s1 {\n\tint f1;\n};\n\ntypedef int t1;\n\nstruct s2 {\n\tstruct s1 f1;\n\tint f2;\n\tint *f3;\n};\n\n\0".as_ptr() as *const c_char,
            b"c_dump\0".as_ptr() as *const c_char,
        );
    }
    btf__free(btf2);
    btf__free(btf1);
}

unsafe fn test_split_fwd_resolve() {
    let mut btf2: *mut btf = ptr::null_mut();
    let err: c_int;

    let btf1 = btf__new_empty();
    if !ASSERT_OK_PTR(btf1 as *const c_void, b"empty_main_btf\0".as_ptr() as *const c_char) {
        return;
    }

    btf__set_pointer_size(btf1, 8); /* enforce 64-bit arch */

    btf__add_int(btf1, b"int\0".as_ptr() as *const c_char, 4, BTF_INT_SIGNED); /* [1] int */
    btf__add_ptr(btf1, 4); /* [2] ptr to struct s1 */
    btf__add_ptr(btf1, 5); /* [3] ptr to struct s2 */
    btf__add_struct(btf1, b"s1\0".as_ptr() as *const c_char, 16); /* [4] struct s1 { */
    btf__add_field(btf1, b"f1\0".as_ptr() as *const c_char, 2, 0, 0); /*      struct s1 *f1; */
    btf__add_field(btf1, b"f2\0".as_ptr() as *const c_char, 3, 64, 0); /*      struct s2 *f2; */
    /* } */
    btf__add_struct(btf1, b"s2\0".as_ptr() as *const c_char, 4); /* [5] struct s2 { */
    btf__add_field(btf1, b"f1\0".as_ptr() as *const c_char, 1, 0, 0); /*      int f1; */
    /* } */
    /* keep this not a part of type the graph to test btf_dedup_resolve_fwds */
    btf__add_struct(btf1, b"s3\0".as_ptr() as *const c_char, 4); /* [6] struct s3 { */
    btf__add_field(btf1, b"f1\0".as_ptr() as *const c_char, 1, 0, 0); /*      int f1; */
    /* } */

    VALIDATE_RAW_BTF(
        btf1,
        b"[1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED\0".as_ptr() as *const c_char,
        b"[2] PTR '(anon)' type_id=4\0".as_ptr() as *const c_char,
        b"[3] PTR '(anon)' type_id=5\0".as_ptr() as *const c_char,
        b"[4] STRUCT 's1' size=16 vlen=2\n\t'f1' type_id=2 bits_offset=0\n\t'f2' type_id=3 bits_offset=64\0".as_ptr() as *const c_char,
        b"[5] STRUCT 's2' size=4 vlen=1\n\t'f1' type_id=1 bits_offset=0\0".as_ptr() as *const c_char,
        b"[6] STRUCT 's3' size=4 vlen=1\n\t'f1' type_id=1 bits_offset=0\0".as_ptr() as *const c_char,
    );

    'cleanup: {
        btf2 = btf__new_empty_split(btf1);
        if !ASSERT_OK_PTR(btf2 as *const c_void, b"empty_split_btf\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }

        btf__add_int(btf2, b"int\0".as_ptr() as *const c_char, 4, BTF_INT_SIGNED); /* [7] int */
        btf__add_ptr(btf2, 11); /* [8] ptr to struct s1 */
        btf__add_fwd(btf2, b"s2\0".as_ptr() as *const c_char, BTF_FWD_STRUCT); /* [9] fwd for struct s2 */
        btf__add_ptr(btf2, 9); /* [10] ptr to fwd struct s2 */
        btf__add_struct(btf2, b"s1\0".as_ptr() as *const c_char, 16); /* [11] struct s1 { */
        btf__add_field(btf2, b"f1\0".as_ptr() as *const c_char, 8, 0, 0); /*      struct s1 *f1; */
        btf__add_field(btf2, b"f2\0".as_ptr() as *const c_char, 10, 64, 0); /*      struct s2 *f2; */
        /* } */
        btf__add_fwd(btf2, b"s3\0".as_ptr() as *const c_char, BTF_FWD_STRUCT); /* [12] fwd for struct s3 */
        btf__add_ptr(btf2, 12); /* [13] ptr to struct s1 */

        VALIDATE_RAW_BTF(
            btf2,
            b"[1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED\0".as_ptr() as *const c_char,
            b"[2] PTR '(anon)' type_id=4\0".as_ptr() as *const c_char,
            b"[3] PTR '(anon)' type_id=5\0".as_ptr() as *const c_char,
            b"[4] STRUCT 's1' size=16 vlen=2\n\t'f1' type_id=2 bits_offset=0\n\t'f2' type_id=3 bits_offset=64\0".as_ptr() as *const c_char,
            b"[5] STRUCT 's2' size=4 vlen=1\n\t'f1' type_id=1 bits_offset=0\0".as_ptr() as *const c_char,
            b"[6] STRUCT 's3' size=4 vlen=1\n\t'f1' type_id=1 bits_offset=0\0".as_ptr() as *const c_char,
            b"[7] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED\0".as_ptr() as *const c_char,
            b"[8] PTR '(anon)' type_id=11\0".as_ptr() as *const c_char,
            b"[9] FWD 's2' fwd_kind=struct\0".as_ptr() as *const c_char,
            b"[10] PTR '(anon)' type_id=9\0".as_ptr() as *const c_char,
            b"[11] STRUCT 's1' size=16 vlen=2\n\t'f1' type_id=8 bits_offset=0\n\t'f2' type_id=10 bits_offset=64\0".as_ptr() as *const c_char,
            b"[12] FWD 's3' fwd_kind=struct\0".as_ptr() as *const c_char,
            b"[13] PTR '(anon)' type_id=12\0".as_ptr() as *const c_char,
        );

        err = btf__dedup(btf2, ptr::null());
        if !ASSERT_OK(err, b"btf_dedup\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }

        VALIDATE_RAW_BTF(
            btf2,
            b"[1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED\0".as_ptr() as *const c_char,
            b"[2] PTR '(anon)' type_id=4\0".as_ptr() as *const c_char,
            b"[3] PTR '(anon)' type_id=5\0".as_ptr() as *const c_char,
            b"[4] STRUCT 's1' size=16 vlen=2\n\t'f1' type_id=2 bits_offset=0\n\t'f2' type_id=3 bits_offset=64\0".as_ptr() as *const c_char,
            b"[5] STRUCT 's2' size=4 vlen=1\n\t'f1' type_id=1 bits_offset=0\0".as_ptr() as *const c_char,
            b"[6] STRUCT 's3' size=4 vlen=1\n\t'f1' type_id=1 bits_offset=0\0".as_ptr() as *const c_char,
            b"[7] PTR '(anon)' type_id=6\0".as_ptr() as *const c_char,
        );
    }
    btf__free(btf2);
    btf__free(btf1);
}

unsafe fn test_split_struct_duped() {
    let mut btf2: *mut btf = ptr::null_mut();
    let err: c_int;

    let btf1 = btf__new_empty();
    if !ASSERT_OK_PTR(btf1 as *const c_void, b"empty_main_btf\0".as_ptr() as *const c_char) {
        return;
    }

    btf__set_pointer_size(btf1, 8); /* enforce 64-bit arch */

    btf__add_int(btf1, b"int\0".as_ptr() as *const c_char, 4, BTF_INT_SIGNED); /* [1] int */
    btf__add_ptr(btf1, 5); /* [2] ptr to struct s1 */
    btf__add_fwd(btf1, b"s2\0".as_ptr() as *const c_char, BTF_FWD_STRUCT); /* [3] fwd for struct s2 */
    btf__add_ptr(btf1, 3); /* [4] ptr to fwd struct s2 */
    btf__add_struct(btf1, b"s1\0".as_ptr() as *const c_char, 16); /* [5] struct s1 { */
    btf__add_field(btf1, b"f1\0".as_ptr() as *const c_char, 2, 0, 0); /*      struct s1 *f1; */
    btf__add_field(btf1, b"f2\0".as_ptr() as *const c_char, 4, 64, 0); /*      struct s2 *f2; */
    /* } */

    VALIDATE_RAW_BTF(
        btf1,
        b"[1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED\0".as_ptr() as *const c_char,
        b"[2] PTR '(anon)' type_id=5\0".as_ptr() as *const c_char,
        b"[3] FWD 's2' fwd_kind=struct\0".as_ptr() as *const c_char,
        b"[4] PTR '(anon)' type_id=3\0".as_ptr() as *const c_char,
        b"[5] STRUCT 's1' size=16 vlen=2\n\t'f1' type_id=2 bits_offset=0\n\t'f2' type_id=4 bits_offset=64\0".as_ptr() as *const c_char,
    );

    'cleanup: {
        btf2 = btf__new_empty_split(btf1);
        if !ASSERT_OK_PTR(btf2 as *const c_void, b"empty_split_btf\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }

        btf__add_int(btf2, b"int\0".as_ptr() as *const c_char, 4, BTF_INT_SIGNED); /* [6] int */
        btf__add_ptr(btf2, 10); /* [7] ptr to struct s1 */
        btf__add_fwd(btf2, b"s2\0".as_ptr() as *const c_char, BTF_FWD_STRUCT); /* [8] fwd for struct s2 */
        btf__add_ptr(btf2, 11); /* [9] ptr to struct s2 */
        btf__add_struct(btf2, b"s1\0".as_ptr() as *const c_char, 16); /* [10] struct s1 { */
        btf__add_field(btf2, b"f1\0".as_ptr() as *const c_char, 7, 0, 0); /*      struct s1 *f1; */
        btf__add_field(btf2, b"f2\0".as_ptr() as *const c_char, 9, 64, 0); /*      struct s2 *f2; */
        /* } */
        btf__add_struct(btf2, b"s2\0".as_ptr() as *const c_char, 40); /* [11] struct s2 { */
        btf__add_field(btf2, b"f1\0".as_ptr() as *const c_char, 7, 0, 0); /*      struct s1 *f1; */
        btf__add_field(btf2, b"f2\0".as_ptr() as *const c_char, 9, 64, 0); /*      struct s2 *f2; */
        btf__add_field(btf2, b"f3\0".as_ptr() as *const c_char, 6, 128, 0); /*      int f3; */
        btf__add_field(btf2, b"f4\0".as_ptr() as *const c_char, 10, 192, 0); /*      struct s1 f4; */
        /* } */
        btf__add_ptr(btf2, 8); /* [12] ptr to fwd struct s2 */
        btf__add_struct(btf2, b"s3\0".as_ptr() as *const c_char, 8); /* [13] struct s3 { */
        btf__add_field(btf2, b"f1\0".as_ptr() as *const c_char, 12, 0, 0); /*      struct s2 *f1; (fwd) */
        /* } */

        VALIDATE_RAW_BTF(
            btf2,
            b"[1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED\0".as_ptr() as *const c_char,
            b"[2] PTR '(anon)' type_id=5\0".as_ptr() as *const c_char,
            b"[3] FWD 's2' fwd_kind=struct\0".as_ptr() as *const c_char,
            b"[4] PTR '(anon)' type_id=3\0".as_ptr() as *const c_char,
            b"[5] STRUCT 's1' size=16 vlen=2\n\t'f1' type_id=2 bits_offset=0\n\t'f2' type_id=4 bits_offset=64\0".as_ptr() as *const c_char,
            b"[6] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED\0".as_ptr() as *const c_char,
            b"[7] PTR '(anon)' type_id=10\0".as_ptr() as *const c_char,
            b"[8] FWD 's2' fwd_kind=struct\0".as_ptr() as *const c_char,
            b"[9] PTR '(anon)' type_id=11\0".as_ptr() as *const c_char,
            b"[10] STRUCT 's1' size=16 vlen=2\n\t'f1' type_id=7 bits_offset=0\n\t'f2' type_id=9 bits_offset=64\0".as_ptr() as *const c_char,
            b"[11] STRUCT 's2' size=40 vlen=4\n\t'f1' type_id=7 bits_offset=0\n\t'f2' type_id=9 bits_offset=64\n\t'f3' type_id=6 bits_offset=128\n\t'f4' type_id=10 bits_offset=192\0".as_ptr() as *const c_char,
            b"[12] PTR '(anon)' type_id=8\0".as_ptr() as *const c_char,
            b"[13] STRUCT 's3' size=8 vlen=1\n\t'f1' type_id=12 bits_offset=0\0".as_ptr() as *const c_char,
        );

        err = btf__dedup(btf2, ptr::null());
        if !ASSERT_OK(err, b"btf_dedup\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }

        VALIDATE_RAW_BTF(
            btf2,
            b"[1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED\0".as_ptr() as *const c_char,
            b"[2] PTR '(anon)' type_id=5\0".as_ptr() as *const c_char,
            b"[3] FWD 's2' fwd_kind=struct\0".as_ptr() as *const c_char,
            b"[4] PTR '(anon)' type_id=3\0".as_ptr() as *const c_char,
            b"[5] STRUCT 's1' size=16 vlen=2\n\t'f1' type_id=2 bits_offset=0\n\t'f2' type_id=4 bits_offset=64\0".as_ptr() as *const c_char,
            b"[6] PTR '(anon)' type_id=8\0".as_ptr() as *const c_char,
            b"[7] PTR '(anon)' type_id=9\0".as_ptr() as *const c_char,
            b"[8] STRUCT 's1' size=16 vlen=2\n\t'f1' type_id=6 bits_offset=0\n\t'f2' type_id=7 bits_offset=64\0".as_ptr() as *const c_char,
            b"[9] STRUCT 's2' size=40 vlen=4\n\t'f1' type_id=6 bits_offset=0\n\t'f2' type_id=7 bits_offset=64\n\t'f3' type_id=1 bits_offset=128\n\t'f4' type_id=8 bits_offset=192\0".as_ptr() as *const c_char,
            b"[10] STRUCT 's3' size=8 vlen=1\n\t'f1' type_id=7 bits_offset=0\0".as_ptr() as *const c_char,
        );
    }
    btf__free(btf2);
    btf__free(btf1);
}

unsafe fn btf_add_dup_struct_in_cu(btf: *mut btf, start_id: c_int) {
    let id = |n: c_int| -> c_uint { (start_id + n) as c_uint };

    btf__set_pointer_size(btf, 8); /* enforce 64-bit arch */

    btf__add_int(btf, b"int\0".as_ptr() as *const c_char, 4, BTF_INT_SIGNED); /* [1] int */

    btf__add_struct(btf, b"s\0".as_ptr() as *const c_char, 8); /* [2] struct s { */
    btf__add_field(btf, b"a\0".as_ptr() as *const c_char, id(3), 0, 0); /*      struct anon a; */
    btf__add_field(btf, b"b\0".as_ptr() as *const c_char, id(4), 0, 0); /*      struct anon b; */
    /* } */

    btf__add_struct(btf, b"(anon)\0".as_ptr() as *const c_char, 8); /* [3] struct anon { */
    btf__add_field(btf, b"f1\0".as_ptr() as *const c_char, id(1), 0, 0); /*      int f1; */
    btf__add_field(btf, b"f2\0".as_ptr() as *const c_char, id(1), 32, 0); /*      int f2; */
    /* } */

    btf__add_struct(btf, b"(anon)\0".as_ptr() as *const c_char, 8); /* [4] struct anon { */
    btf__add_field(btf, b"f1\0".as_ptr() as *const c_char, id(1), 0, 0); /*      int f1; */
    btf__add_field(btf, b"f2\0".as_ptr() as *const c_char, id(1), 32, 0); /*      int f2; */
    /* } */
}

unsafe fn test_split_dup_struct_in_cu() {
    let mut btf2: *mut btf = ptr::null_mut();
    let err: c_int;

    /* generate the base data.. */
    let btf1 = btf__new_empty();
    if !ASSERT_OK_PTR(btf1 as *const c_void, b"empty_main_btf\0".as_ptr() as *const c_char) {
        return;
    }

    'cleanup: {
        btf_add_dup_struct_in_cu(btf1, 0);

        VALIDATE_RAW_BTF(
            btf1,
            b"[1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED\0".as_ptr() as *const c_char,
            b"[2] STRUCT 's' size=8 vlen=2\n\t'a' type_id=3 bits_offset=0\n\t'b' type_id=4 bits_offset=0\0".as_ptr() as *const c_char,
            b"[3] STRUCT '(anon)' size=8 vlen=2\n\t'f1' type_id=1 bits_offset=0\n\t'f2' type_id=1 bits_offset=32\0".as_ptr() as *const c_char,
            b"[4] STRUCT '(anon)' size=8 vlen=2\n\t'f1' type_id=1 bits_offset=0\n\t'f2' type_id=1 bits_offset=32\0".as_ptr() as *const c_char,
        );

        /* ..dedup them... */
        err = btf__dedup(btf1, ptr::null());
        if !ASSERT_OK(err, b"btf_dedup\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }

        VALIDATE_RAW_BTF(
            btf1,
            b"[1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED\0".as_ptr() as *const c_char,
            b"[2] STRUCT 's' size=8 vlen=2\n\t'a' type_id=3 bits_offset=0\n\t'b' type_id=3 bits_offset=0\0".as_ptr() as *const c_char,
            b"[3] STRUCT '(anon)' size=8 vlen=2\n\t'f1' type_id=1 bits_offset=0\n\t'f2' type_id=1 bits_offset=32\0".as_ptr() as *const c_char,
        );

        /* and add the same data on top of it */
        btf2 = btf__new_empty_split(btf1);
        if !ASSERT_OK_PTR(btf2 as *const c_void, b"empty_split_btf\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }

        btf_add_dup_struct_in_cu(btf2, 3);

        VALIDATE_RAW_BTF(
            btf2,
            b"[1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED\0".as_ptr() as *const c_char,
            b"[2] STRUCT 's' size=8 vlen=2\n\t'a' type_id=3 bits_offset=0\n\t'b' type_id=3 bits_offset=0\0".as_ptr() as *const c_char,
            b"[3] STRUCT '(anon)' size=8 vlen=2\n\t'f1' type_id=1 bits_offset=0\n\t'f2' type_id=1 bits_offset=32\0".as_ptr() as *const c_char,
            b"[4] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED\0".as_ptr() as *const c_char,
            b"[5] STRUCT 's' size=8 vlen=2\n\t'a' type_id=6 bits_offset=0\n\t'b' type_id=7 bits_offset=0\0".as_ptr() as *const c_char,
            b"[6] STRUCT '(anon)' size=8 vlen=2\n\t'f1' type_id=4 bits_offset=0\n\t'f2' type_id=4 bits_offset=32\0".as_ptr() as *const c_char,
            b"[7] STRUCT '(anon)' size=8 vlen=2\n\t'f1' type_id=4 bits_offset=0\n\t'f2' type_id=4 bits_offset=32\0".as_ptr() as *const c_char,
        );

        err = btf__dedup(btf2, ptr::null());
        if !ASSERT_OK(err, b"btf_dedup\0".as_ptr() as *const c_char) {
            break 'cleanup;
        }

        /* after dedup it should match the original data */
        VALIDATE_RAW_BTF(
            btf2,
            b"[1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED\0".as_ptr() as *const c_char,
            b"[2] STRUCT 's' size=8 vlen=2\n\t'a' type_id=3 bits_offset=0\n\t'b' type_id=3 bits_offset=0\0".as_ptr() as *const c_char,
            b"[3] STRUCT '(anon)' size=8 vlen=2\n\t'f1' type_id=1 bits_offset=0\n\t'f2' type_id=1 bits_offset=32\0".as_ptr() as *const c_char,
        );
    }
    btf__free(btf2);
    btf__free(btf1);
}

/* Ensure module split BTF dedup worked correctly; when dedup fails badly
 * core kernel types are in split BTF also, so ensure that references to
 * such types point at base - not split - BTF.
 *
 * bpf_testmod_test_write() has multiple core kernel type parameters;
 *
 * ssize_t
 * bpf_testmod_test_write(struct file *file, struct kobject *kobj,
 *                        struct bin_attribute *bin_attr,
 *                        char *buf, loff_t off, size_t len);
 *
 * Ensure each of the FUNC_PROTO params is a core kernel type.
 *
 * Do the same for
 *
 * __bpf_kfunc struct sock *bpf_kfunc_call_test3(struct sock *sk);
 *
 * ...and
 *
 * __bpf_kfunc void bpf_kfunc_call_test_pass_ctx(struct __sk_buff *skb);
 *
 */
unsafe fn test_split_module() {
    let mut btf1: *mut btf = ptr::null_mut();
    let nr_base_types: c_int;

    let vmlinux_btf = btf__load_vmlinux_btf();
    if !ASSERT_OK_PTR(vmlinux_btf as *const c_void, b"vmlinux_btf\0".as_ptr() as *const c_char) {
        return;
    }
    nr_base_types = btf__type_cnt(vmlinux_btf);
    if !ASSERT_GT_INT(nr_base_types, 0, b"nr_base_types\0".as_ptr() as *const c_char) {
        btf__free(btf1);
        btf__free(vmlinux_btf);
        return;
    }

    btf1 = btf__parse_split(b"/sys/kernel/btf/bpf_testmod\0".as_ptr() as *const c_char, vmlinux_btf);
    if !ASSERT_OK_PTR(btf1 as *const c_void, b"split_btf\0".as_ptr() as *const c_char) {
        return;
    }

    'cleanup: {
        let mut i: usize = 0;
        while i < mod_funcs.len() {
            let mut p: *const btf_param;
            let mut t: *const btf_type;
            let vlen: c_uint;
            let mut j: c_uint;
            let mut id: c_uint;

            id = btf__find_by_name_kind(btf1, mod_funcs[i], BTF_KIND_FUNC);
            if !ASSERT_GE_UINT(id, nr_base_types as c_uint, b"func_id\0".as_ptr() as *const c_char) {
                break 'cleanup;
            }
            t = btf__type_by_id(btf1, id);
            if !ASSERT_OK_PTR(t as *const c_void, b"func_id_type\0".as_ptr() as *const c_char) {
                break 'cleanup;
            }
            t = btf__type_by_id(btf1, (*t).type_);
            if !ASSERT_OK_PTR(t as *const c_void, b"func_proto_id_type\0".as_ptr() as *const c_char) {
                break 'cleanup;
            }
            if !ASSERT_EQ_BOOL(btf_is_func_proto(t), true, b"is_func_proto\0".as_ptr() as *const c_char) {
                break 'cleanup;
            }
            vlen = btf_vlen(t);

            j = 0;
            p = btf_params(t);
            while j < vlen {
                /* bpf_testmod uses resilient split BTF, so any
                 * reference types will be added to split BTF and their
                 * associated targets will be base BTF types; for example
                 * for a "struct sock *" the PTR will be in split BTF
                 * while the "struct sock" will be in base.
                 *
                 * In some cases like loff_t we have to resolve
                 * multiple typedefs hence the while() loop below.
                 *
                 * Note that resilient split BTF generation depends
                 * on pahole version, so we do not assert that
                 * reference types are in split BTF, as if pahole
                 * does not support resilient split BTF they will
                 * also be base BTF types.
                 */
                id = (*p).type_;
                loop {
                    t = btf__type_by_id(btf1, id);
                    if !ASSERT_OK_PTR(t as *const c_void, b"param_ref_type\0".as_ptr() as *const c_char) {
                        break 'cleanup;
                    }
                    if !btf_is_mod(t) && !btf_is_ptr(t) && !btf_is_typedef(t) {
                        break;
                    }
                    id = (*t).type_;
                }

                if !ASSERT_LT_UINT(id, nr_base_types as c_uint, b"verify_base_type\0".as_ptr() as *const c_char) {
                    break 'cleanup;
                }

                j += 1;
                p = p.add(1);
            }
            i += 1;
        }
    }
    btf__free(btf1);
    btf__free(vmlinux_btf);
}

#[no_mangle]
pub unsafe extern "C" fn test_btf_dedup_split() {
    if test__start_subtest(b"split_simple\0".as_ptr() as *const c_char) {
        test_split_simple();
    }
    if test__start_subtest(b"split_struct_duped\0".as_ptr() as *const c_char) {
        test_split_struct_duped();
    }
    if test__start_subtest(b"split_fwd_resolve\0".as_ptr() as *const c_char) {
        test_split_fwd_resolve();
    }
    if test__start_subtest(b"split_dup_struct_in_cu\0".as_ptr() as *const c_char) {
        test_split_dup_struct_in_cu();
    }
    if test__start_subtest(b"split_module\0".as_ptr() as *const c_char) {
        test_split_module();
    }
}
