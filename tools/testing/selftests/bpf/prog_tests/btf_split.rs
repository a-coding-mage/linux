// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
/* Translated from C. External test/libbpf/libc declarations are dependencies. */

use core::ffi::{c_char, c_int, c_void};

type size_t = usize;
type ssize_t = isize;
type __u32 = u32;
type bool_ = bool;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct va_list {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_dump {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_type {
    pub name_off: __u32,
}

extern "C" {
    static mut errno: c_int;

    fn vfprintf(stream: *mut FILE, format: *const c_char, ap: va_list) -> c_int;
    fn mkstemp(template: *mut c_char) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn open_memstream(ptr: *mut *mut c_char, sizeloc: *mut size_t) -> *mut FILE;
    fn fflush(stream: *mut FILE) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn free(ptr: *mut c_void);
    fn unlink(pathname: *const c_char) -> c_int;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;

    fn btf__new_empty() -> *mut btf;
    fn btf__new_empty_split(base_btf: *mut btf) -> *mut btf;
    fn btf__set_pointer_size(btf: *mut btf, ptr_sz: c_int) -> c_int;
    fn btf__pointer_size(btf: *const btf) -> c_int;
    fn btf__add_int(btf: *mut btf, name: *const c_char, byte_sz: c_int, encoding: c_int) -> c_int;
    fn btf__add_ptr(btf: *mut btf, type_id: c_int) -> c_int;
    fn btf__add_struct(btf: *mut btf, name: *const c_char, sz: __u32) -> c_int;
    fn btf__add_union(btf: *mut btf, name: *const c_char, sz: __u32) -> c_int;
    fn btf__add_field(
        btf: *mut btf,
        name: *const c_char,
        type_id: c_int,
        bit_offset: __u32,
        bit_size: __u32,
    ) -> c_int;
    fn btf__find_str(btf: *const btf, s: *const c_char) -> c_int;
    fn btf__type_by_id(btf: *const btf, type_id: __u32) -> *const btf_type;
    fn btf__str_by_offset(btf: *const btf, offset: __u32) -> *const c_char;
    fn btf__type_cnt(btf: *const btf) -> __u32;
    fn btf__raw_data(btf: *const btf, size: *mut __u32) -> *const c_void;
    fn btf__parse(path: *const c_char, opts: *mut c_void) -> *mut btf;
    fn btf__parse_split(path: *const c_char, base_btf: *mut btf) -> *mut btf;
    fn btf__free(btf: *mut btf);

    fn btf_is_int(t: *const btf_type) -> bool_;
    fn btf_is_struct(t: *const btf_type) -> bool_;
    fn btf_is_union(t: *const btf_type) -> bool_;
    fn btf_vlen(t: *const btf_type) -> __u32;

    fn btf_dump__new(
        btf: *const btf,
        printf_fn: unsafe extern "C" fn(*mut c_void, *const c_char, va_list),
        ctx: *mut c_void,
        opts: *mut c_void,
    ) -> *mut btf_dump;
    fn btf_dump__dump_type(d: *mut btf_dump, id: __u32) -> c_int;
    fn btf_dump__free(d: *mut btf_dump);

    fn ASSERT_GE(actual: ssize_t, expected: ssize_t, name: *const c_char) -> bool_;
    fn ASSERT_OK_PTR<T>(ptr: *const T, name: *const c_char) -> bool_;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool_;
    fn ASSERT_EQ<T: Copy + PartialEq>(actual: T, expected: T, name: *const c_char) -> bool_;
    fn ASSERT_NEQ<T: Copy + PartialEq>(actual: T, expected: T, name: *const c_char) -> bool_;
    fn ASSERT_STREQ(actual: *const c_char, expected: *const c_char, name: *const c_char) -> bool_;
    fn ASSERT_NULL<T>(ptr: *const T, name: *const c_char) -> bool_;
    fn test__start_subtest(name: *const c_char) -> bool_;
}

const BTF_INT_SIGNED: c_int = 1;
const ENOENT: c_int = 2;
const EINVAL: c_int = 22;

static mut dump_buf: *mut c_char = core::ptr::null_mut();
static mut dump_buf_sz: size_t = 0;
static mut dump_buf_file: *mut FILE = core::ptr::null_mut();

unsafe extern "C" fn btf_dump_printf(ctx: *mut c_void, fmt: *const c_char, args: va_list) {
    vfprintf(ctx as *mut FILE, fmt, args);
}

/* Write raw BTF to file, return number of bytes written or negative errno */
unsafe fn btf_raw_write(btf: *mut btf, file: *mut c_char) -> ssize_t {
    let mut written: ssize_t = 0;
    let data: *const c_void;
    let mut size: __u32 = 0;
    let fd: c_int;
    let mut ret: ssize_t;

    fd = mkstemp(file);
    if !ASSERT_GE(fd as ssize_t, 0, b"create_file\0".as_ptr() as *const c_char) {
        return -(errno as ssize_t);
    }

    data = btf__raw_data(btf, &mut size);
    if !ASSERT_OK_PTR(data, b"btf__raw_data\0".as_ptr() as *const c_char) {
        close(fd);
        return -(EINVAL as ssize_t);
    }
    while written < size as ssize_t {
        ret = write(
            fd,
            (data as *const u8).offset(written) as *const c_void,
            (size as ssize_t - written) as size_t,
        );
        if !ASSERT_GE(ret, 0, b"write succeeded\0".as_ptr() as *const c_char) {
            close(fd);
            return -(errno as ssize_t);
        }
        written += ret;
    }
    close(fd);
    written
}

unsafe fn __test_btf_split(multi: bool_) {
    let mut multisplit_btf_file = *b"/tmp/test_btf_multisplit.XXXXXX\0";
    let mut split_btf_file = *b"/tmp/test_btf_split.XXXXXX\0";
    let mut base_btf_file = *b"/tmp/test_btf_base.XXXXXX\0";
    let mut multisplit_btf_sz: ssize_t = 0;
    let mut split_btf_sz: ssize_t = 0;
    let mut base_btf_sz: ssize_t = 0;
    let mut d: *mut btf_dump = core::ptr::null_mut();
    let mut t: *const btf_type;
    let mut ot: *const btf_type;
    let mut btf1: *mut btf = core::ptr::null_mut();
    let mut btf2: *mut btf = core::ptr::null_mut();
    let mut btf3: *mut btf = core::ptr::null_mut();
    let mut btf4: *mut btf = core::ptr::null_mut();
    let mut btf5: *mut btf = core::ptr::null_mut();
    let mut btf6: *mut btf = core::ptr::null_mut();
    let mut str_off: c_int;
    let mut i: __u32;
    let mut err: c_int;

    btf1 = btf__new_empty();
    if !ASSERT_OK_PTR(btf1, b"empty_main_btf\0".as_ptr() as *const c_char) {
        return;
    }

    btf__set_pointer_size(btf1, 8); /* enforce 64-bit arch */

    btf__add_int(btf1, b"int\0".as_ptr() as *const c_char, 4, BTF_INT_SIGNED); /* [1] int */
    btf__add_ptr(btf1, 1); /* [2] ptr to int */

    btf__add_struct(btf1, b"s1\0".as_ptr() as *const c_char, 4); /* [3] struct s1 { */
    btf__add_field(btf1, b"f1\0".as_ptr() as *const c_char, 1, 0, 0); /*      int f1; */
                                                                              /* } */

    btf2 = btf__new_empty_split(btf1);
    if !ASSERT_OK_PTR(btf2, b"empty_split_btf\0".as_ptr() as *const c_char) {
        goto_cleanup(
            dump_buf_file,
            &mut dump_buf,
            d,
            btf1,
            btf2,
            btf3,
            btf4,
            btf5,
            btf6,
            base_btf_sz,
            split_btf_sz,
            multisplit_btf_sz,
            base_btf_file.as_mut_ptr() as *mut c_char,
            split_btf_file.as_mut_ptr() as *mut c_char,
            multisplit_btf_file.as_mut_ptr() as *mut c_char,
        );
        return;
    }

    /* pointer size should be "inherited" from main BTF */
    ASSERT_EQ(
        btf__pointer_size(btf2),
        8,
        b"inherit_ptr_sz\0".as_ptr() as *const c_char,
    );

    str_off = btf__find_str(btf2, b"int\0".as_ptr() as *const c_char);
    ASSERT_NEQ(
        str_off,
        -ENOENT,
        b"str_int_missing\0".as_ptr() as *const c_char,
    );

    t = btf__type_by_id(btf2, 1);
    if !ASSERT_OK_PTR(t, b"int_type\0".as_ptr() as *const c_char) {
        cleanup!(
            dump_buf_file,
            d,
            btf1,
            btf2,
            btf3,
            btf4,
            btf5,
            btf6,
            base_btf_sz,
            split_btf_sz,
            multisplit_btf_sz,
            base_btf_file,
            split_btf_file,
            multisplit_btf_file
        );
        return;
    }
    ASSERT_EQ(btf_is_int(t), true, b"int_kind\0".as_ptr() as *const c_char);
    ASSERT_STREQ(
        btf__str_by_offset(btf2, (*t).name_off),
        b"int\0".as_ptr() as *const c_char,
        b"int_name\0".as_ptr() as *const c_char,
    );

    btf__add_struct(btf2, b"s2\0".as_ptr() as *const c_char, 16); /* [4] struct s2 {	*/
    btf__add_field(btf2, b"f1\0".as_ptr() as *const c_char, 3, 0, 0); /*      struct s1 f1;	*/
    btf__add_field(btf2, b"f2\0".as_ptr() as *const c_char, 1, 32, 0); /*      int f2;		*/
    btf__add_field(btf2, b"f3\0".as_ptr() as *const c_char, 2, 64, 0); /*      int *f3;	*/
                                                                              /* } */

    t = btf__type_by_id(btf1, 4);
    ASSERT_NULL(t, b"split_type_in_main\0".as_ptr() as *const c_char);

    t = btf__type_by_id(btf2, 4);
    if !ASSERT_OK_PTR(t, b"split_struct_type\0".as_ptr() as *const c_char) {
        cleanup!(
            dump_buf_file,
            d,
            btf1,
            btf2,
            btf3,
            btf4,
            btf5,
            btf6,
            base_btf_sz,
            split_btf_sz,
            multisplit_btf_sz,
            base_btf_file,
            split_btf_file,
            multisplit_btf_file
        );
        return;
    }
    ASSERT_EQ(
        btf_is_struct(t),
        true,
        b"split_struct_kind\0".as_ptr() as *const c_char,
    );
    ASSERT_EQ(
        btf_vlen(t),
        3,
        b"split_struct_vlen\0".as_ptr() as *const c_char,
    );
    ASSERT_STREQ(
        btf__str_by_offset(btf2, (*t).name_off),
        b"s2\0".as_ptr() as *const c_char,
        b"split_struct_name\0".as_ptr() as *const c_char,
    );

    if multi {
        btf3 = btf__new_empty_split(btf2);
        if !ASSERT_OK_PTR(btf3, b"multi_split_btf\0".as_ptr() as *const c_char) {
            cleanup!(
                dump_buf_file,
                d,
                btf1,
                btf2,
                btf3,
                btf4,
                btf5,
                btf6,
                base_btf_sz,
                split_btf_sz,
                multisplit_btf_sz,
                base_btf_file,
                split_btf_file,
                multisplit_btf_file
            );
            return;
        }
    } else {
        btf3 = btf2;
    }

    btf__add_union(btf3, b"u1\0".as_ptr() as *const c_char, 16); /* [5] union u1 {	*/
    btf__add_field(btf3, b"f1\0".as_ptr() as *const c_char, 4, 0, 0); /*	struct s2 f1;	*/
    btf__add_field(btf3, b"uf2\0".as_ptr() as *const c_char, 1, 0, 0); /*	int f2;		*/
                                                                                /* } */

    if multi {
        t = btf__type_by_id(btf2, 5);
        ASSERT_NULL(
            t,
            b"multisplit_type_in_first_split\0".as_ptr() as *const c_char,
        );
    }

    t = btf__type_by_id(btf3, 5);
    if !ASSERT_OK_PTR(t, b"split_union_type\0".as_ptr() as *const c_char) {
        cleanup!(
            dump_buf_file,
            d,
            btf1,
            btf2,
            btf3,
            btf4,
            btf5,
            btf6,
            base_btf_sz,
            split_btf_sz,
            multisplit_btf_sz,
            base_btf_file,
            split_btf_file,
            multisplit_btf_file
        );
        return;
    }
    ASSERT_EQ(
        btf_is_union(t),
        true,
        b"split_union_kind\0".as_ptr() as *const c_char,
    );
    ASSERT_EQ(
        btf_vlen(t),
        2,
        b"split_union_vlen\0".as_ptr() as *const c_char,
    );
    ASSERT_STREQ(
        btf__str_by_offset(btf3, (*t).name_off),
        b"u1\0".as_ptr() as *const c_char,
        b"split_union_name\0".as_ptr() as *const c_char,
    );
    ASSERT_EQ(
        btf__type_cnt(btf3),
        6,
        b"split_type_cnt\0".as_ptr() as *const c_char,
    );

    t = btf__type_by_id(btf3, 1);
    if !ASSERT_OK_PTR(t, b"split_base_type\0".as_ptr() as *const c_char) {
        cleanup!(
            dump_buf_file,
            d,
            btf1,
            btf2,
            btf3,
            btf4,
            btf5,
            btf6,
            base_btf_sz,
            split_btf_sz,
            multisplit_btf_sz,
            base_btf_file,
            split_btf_file,
            multisplit_btf_file
        );
        return;
    }
    ASSERT_EQ(
        btf_is_int(t),
        true,
        b"split_base_int\0".as_ptr() as *const c_char,
    );
    ASSERT_STREQ(
        btf__str_by_offset(btf3, (*t).name_off),
        b"int\0".as_ptr() as *const c_char,
        b"split_base_type_name\0".as_ptr() as *const c_char,
    );

    /* BTF-to-C dump of split BTF */
    dump_buf_file = open_memstream(&mut dump_buf, &mut dump_buf_sz);
    if !ASSERT_OK_PTR(dump_buf_file, b"dump_memstream\0".as_ptr() as *const c_char) {
        return;
    }
    d = btf_dump__new(
        btf3,
        btf_dump_printf,
        dump_buf_file as *mut c_void,
        core::ptr::null_mut(),
    );
    if !ASSERT_OK_PTR(d, b"btf_dump__new\0".as_ptr() as *const c_char) {
        cleanup!(
            dump_buf_file,
            d,
            btf1,
            btf2,
            btf3,
            btf4,
            btf5,
            btf6,
            base_btf_sz,
            split_btf_sz,
            multisplit_btf_sz,
            base_btf_file,
            split_btf_file,
            multisplit_btf_file
        );
        return;
    }
    i = 1;
    while i < btf__type_cnt(btf3) {
        err = btf_dump__dump_type(d, i);
        ASSERT_OK(err, b"dump_type_ok\0".as_ptr() as *const c_char);
        i += 1;
    }
    fflush(dump_buf_file);
    *dump_buf.add(dump_buf_sz) = 0; /* some libc implementations don't do this */
    ASSERT_STREQ(
        dump_buf,
        b"struct s1 {\n\
\tint f1;\n\
};\n\n\
struct s2 {\n\
\tstruct s1 f1;\n\
\tint f2;\n\
\tint *f3;\n\
};\n\n\
union u1 {\n\
\tstruct s2 f1;\n\
\tint uf2;\n\
};\n\n\0"
            .as_ptr() as *const c_char,
        b"c_dump\0".as_ptr() as *const c_char,
    );

    /* write base, split BTFs to files and ensure parsing succeeds */
    base_btf_sz = btf_raw_write(btf1, base_btf_file.as_mut_ptr() as *mut c_char);
    if base_btf_sz < 0 {
        cleanup!(
            dump_buf_file,
            d,
            btf1,
            btf2,
            btf3,
            btf4,
            btf5,
            btf6,
            base_btf_sz,
            split_btf_sz,
            multisplit_btf_sz,
            base_btf_file,
            split_btf_file,
            multisplit_btf_file
        );
        return;
    }
    split_btf_sz = btf_raw_write(btf2, split_btf_file.as_mut_ptr() as *mut c_char);
    if split_btf_sz < 0 {
        cleanup!(
            dump_buf_file,
            d,
            btf1,
            btf2,
            btf3,
            btf4,
            btf5,
            btf6,
            base_btf_sz,
            split_btf_sz,
            multisplit_btf_sz,
            base_btf_file,
            split_btf_file,
            multisplit_btf_file
        );
        return;
    }
    btf4 = btf__parse(base_btf_file.as_ptr() as *const c_char, core::ptr::null_mut());
    if !ASSERT_OK_PTR(btf4, b"parse_base\0".as_ptr() as *const c_char) {
        cleanup!(
            dump_buf_file,
            d,
            btf1,
            btf2,
            btf3,
            btf4,
            btf5,
            btf6,
            base_btf_sz,
            split_btf_sz,
            multisplit_btf_sz,
            base_btf_file,
            split_btf_file,
            multisplit_btf_file
        );
        return;
    }
    btf5 = btf__parse_split(split_btf_file.as_ptr() as *const c_char, btf4);
    if !ASSERT_OK_PTR(btf5, b"parse_split\0".as_ptr() as *const c_char) {
        cleanup!(
            dump_buf_file,
            d,
            btf1,
            btf2,
            btf3,
            btf4,
            btf5,
            btf6,
            base_btf_sz,
            split_btf_sz,
            multisplit_btf_sz,
            base_btf_file,
            split_btf_file,
            multisplit_btf_file
        );
        return;
    }
    if multi {
        multisplit_btf_sz =
            btf_raw_write(btf3, multisplit_btf_file.as_mut_ptr() as *mut c_char);
        if multisplit_btf_sz < 0 {
            cleanup!(
                dump_buf_file,
                d,
                btf1,
                btf2,
                btf3,
                btf4,
                btf5,
                btf6,
                base_btf_sz,
                split_btf_sz,
                multisplit_btf_sz,
                base_btf_file,
                split_btf_file,
                multisplit_btf_file
            );
            return;
        }
        btf6 = btf__parse_split(multisplit_btf_file.as_ptr() as *const c_char, btf5);
        if !ASSERT_OK_PTR(btf6, b"parse_multisplit\0".as_ptr() as *const c_char) {
            cleanup!(
                dump_buf_file,
                d,
                btf1,
                btf2,
                btf3,
                btf4,
                btf5,
                btf6,
                base_btf_sz,
                split_btf_sz,
                multisplit_btf_sz,
                base_btf_file,
                split_btf_file,
                multisplit_btf_file
            );
            return;
        }
    } else {
        btf6 = btf5;
    }

    if !ASSERT_EQ(
        btf__type_cnt(btf3),
        btf__type_cnt(btf6),
        b"cmp_type_cnt\0".as_ptr() as *const c_char,
    ) {
        cleanup!(
            dump_buf_file,
            d,
            btf1,
            btf2,
            btf3,
            btf4,
            btf5,
            btf6,
            base_btf_sz,
            split_btf_sz,
            multisplit_btf_sz,
            base_btf_file,
            split_btf_file,
            multisplit_btf_file
        );
        return;
    }

    /* compare parsed to original BTF */
    i = 1;
    while i < btf__type_cnt(btf6) {
        t = btf__type_by_id(btf6, i);
        if !ASSERT_OK_PTR(t, b"type_in_parsed_btf\0".as_ptr() as *const c_char) {
            cleanup!(
                dump_buf_file,
                d,
                btf1,
                btf2,
                btf3,
                btf4,
                btf5,
                btf6,
                base_btf_sz,
                split_btf_sz,
                multisplit_btf_sz,
                base_btf_file,
                split_btf_file,
                multisplit_btf_file
            );
            return;
        }
        ot = btf__type_by_id(btf3, i);
        if !ASSERT_OK_PTR(ot, b"type_in_orig_btf\0".as_ptr() as *const c_char) {
            cleanup!(
                dump_buf_file,
                d,
                btf1,
                btf2,
                btf3,
                btf4,
                btf5,
                btf6,
                base_btf_sz,
                split_btf_sz,
                multisplit_btf_sz,
                base_btf_file,
                split_btf_file,
                multisplit_btf_file
            );
            return;
        }
        if !ASSERT_EQ(
            memcmp(
                t as *const c_void,
                ot as *const c_void,
                core::mem::size_of_val(&*ot),
            ),
            0,
            b"cmp_parsed_orig_btf\0".as_ptr() as *const c_char,
        ) {
            cleanup!(
                dump_buf_file,
                d,
                btf1,
                btf2,
                btf3,
                btf4,
                btf5,
                btf6,
                base_btf_sz,
                split_btf_sz,
                multisplit_btf_sz,
                base_btf_file,
                split_btf_file,
                multisplit_btf_file
            );
            return;
        }
        i += 1;
    }

    cleanup!(
        dump_buf_file,
        d,
        btf1,
        btf2,
        btf3,
        btf4,
        btf5,
        btf6,
        base_btf_sz,
        split_btf_sz,
        multisplit_btf_sz,
        base_btf_file,
        split_btf_file,
        multisplit_btf_file
    );
}

macro_rules! cleanup {
    (
        $dump_buf_file:expr,
        $d:expr,
        $btf1:expr,
        $btf2:expr,
        $btf3:expr,
        $btf4:expr,
        $btf5:expr,
        $btf6:expr,
        $base_btf_sz:expr,
        $split_btf_sz:expr,
        $multisplit_btf_sz:expr,
        $base_btf_file:expr,
        $split_btf_file:expr,
        $multisplit_btf_file:expr
    ) => {{
        if !$dump_buf_file.is_null() {
            fclose($dump_buf_file);
        }
        free(dump_buf as *mut c_void);
        btf_dump__free($d);
        btf__free($btf1);
        btf__free($btf2);
        if $btf2 != $btf3 {
            btf__free($btf3);
        }
        btf__free($btf4);
        btf__free($btf5);
        if $btf5 != $btf6 {
            btf__free($btf6);
        }
        if $base_btf_sz > 0 {
            unlink($base_btf_file.as_ptr() as *const c_char);
        }
        if $split_btf_sz > 0 {
            unlink($split_btf_file.as_ptr() as *const c_char);
        }
        if $multisplit_btf_sz > 0 {
            unlink($multisplit_btf_file.as_ptr() as *const c_char);
        }
    }};
}

unsafe fn goto_cleanup(
    local_dump_buf_file: *mut FILE,
    _local_dump_buf: *mut *mut c_char,
    d: *mut btf_dump,
    btf1: *mut btf,
    btf2: *mut btf,
    btf3: *mut btf,
    btf4: *mut btf,
    btf5: *mut btf,
    btf6: *mut btf,
    base_btf_sz: ssize_t,
    split_btf_sz: ssize_t,
    multisplit_btf_sz: ssize_t,
    base_btf_file: *mut c_char,
    split_btf_file: *mut c_char,
    multisplit_btf_file: *mut c_char,
) {
    if !local_dump_buf_file.is_null() {
        fclose(local_dump_buf_file);
    }
    free(dump_buf as *mut c_void);
    btf_dump__free(d);
    btf__free(btf1);
    btf__free(btf2);
    if btf2 != btf3 {
        btf__free(btf3);
    }
    btf__free(btf4);
    btf__free(btf5);
    if btf5 != btf6 {
        btf__free(btf6);
    }
    if base_btf_sz > 0 {
        unlink(base_btf_file);
    }
    if split_btf_sz > 0 {
        unlink(split_btf_file);
    }
    if multisplit_btf_sz > 0 {
        unlink(multisplit_btf_file);
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_btf_split() {
    if test__start_subtest(b"single_split\0".as_ptr() as *const c_char) {
        __test_btf_split(false);
    }
    if test__start_subtest(b"multi_split\0".as_ptr() as *const c_char) {
        __test_btf_split(true);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
