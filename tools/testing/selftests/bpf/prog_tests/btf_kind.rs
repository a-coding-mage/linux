// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026, Oracle and/or its affiliates. */

/* Translated from C. External test/libbpf/libc symbols are dependencies. */

type __u32 = u32;
type __s32 = i32;
type size_t = usize;
type ssize_t = isize;

const BTF_INT_CHAR: __u32 = 2;
const BTF_KIND_TYPEDEF: __u32 = 8;
const NR_BTF_KINDS: usize = 26;
const ENOENT: i32 = 2;

#[repr(C)]
struct btf;

#[repr(C)]
struct btf_new_opts {
    sz: size_t,
    add_layout: bool,
}

#[repr(C)]
struct btf_header {
    magic: u16,
    version: u8,
    flags: u8,
    hdr_len: __u32,
    type_off: __u32,
    type_len: __u32,
    str_off: __u32,
    str_len: __u32,
    layout_off: __u32,
    layout_len: __u32,
}

#[repr(C)]
struct btf_layout {
    info_sz: __u32,
    elem_sz: __u32,
    flags: __u32,
}

#[repr(C)]
struct btf_type {
    name_off: __u32,
    info: __u32,
    size: __u32,
}

unsafe extern "C" {
    fn btf__new_empty_opts(opts: *const btf_new_opts) -> *mut btf;
    fn btf__raw_data(btf: *const btf, size: *mut __u32) -> *const core::ffi::c_void;
    fn btf__free(btf: *mut btf);
    fn btf__add_int(btf: *mut btf, name: *const i8, byte_sz: __u32, encoding: __u32) -> __s32;
    fn btf__add_typedef(btf: *mut btf, name: *const i8, ref_type_id: __s32) -> __s32;
    fn btf__parse_raw(path: *const i8) -> *mut btf;
    fn btf__find_by_name(btf: *const btf, type_name: *const i8) -> __s32;
    fn btf__find_by_name_kind(btf: *const btf, type_name: *const i8, kind: __u32) -> __s32;

    fn test__start_subtest(name: *const i8) -> bool;
    fn ASSERT_OK_PTR<T>(ptr: *const T, name: *const i8) -> bool;
    fn ASSERT_OK_FD(fd: i32, name: *const i8) -> bool;
    fn ASSERT_EQ<T: Copy>(actual: T, expected: T, name: *const i8) -> bool;
    fn ASSERT_GT<T: Copy>(actual: T, expected: T, name: *const i8) -> bool;

    fn mkstemp(template: *mut i8) -> i32;
    fn write(fd: i32, buf: *const core::ffi::c_void, count: size_t) -> ssize_t;
    fn close(fd: i32) -> i32;
    fn calloc(nmemb: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn memcpy(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memmove(
        dest: *mut core::ffi::c_void,
        src: *const core::ffi::c_void,
        n: size_t,
    ) -> *mut core::ffi::c_void;
    fn free(ptr: *mut core::ffi::c_void);
    fn unlink(pathname: *const i8) -> i32;
}

/* Verify kind encoding exists for each kind */
unsafe fn test_btf_kind_encoding() {
    let mut opts = btf_new_opts {
        sz: core::mem::size_of::<btf_new_opts>(),
        add_layout: false,
    };
    let mut hdr: *const btf_header;
    let mut raw_btf: *const core::ffi::c_void;
    let mut btf: *mut btf;
    let mut raw_size: __u32 = 0;

    opts.add_layout = true;
    btf = btf__new_empty_opts(&opts);
    if !ASSERT_OK_PTR(btf, c"btf_new".as_ptr()) {
        return;
    }

    raw_btf = btf__raw_data(btf, &mut raw_size);
    if !ASSERT_OK_PTR(raw_btf, c"btf__raw_data".as_ptr()) {
        return;
    }

    hdr = raw_btf as *const btf_header;

    ASSERT_EQ((*hdr).layout_off % 4, 0, c"layout_aligned".as_ptr());
    ASSERT_EQ(
        (*hdr).layout_len as usize,
        core::mem::size_of::<btf_layout>() * NR_BTF_KINDS,
        c"layout_len".as_ptr(),
    );
    ASSERT_EQ(
        (*hdr).str_off,
        (*hdr).layout_off + (*hdr).layout_len,
        c"str_after_layout".as_ptr(),
    );
    btf__free(btf);

    opts.add_layout = false;
    btf = btf__new_empty_opts(&opts);
    if !ASSERT_OK_PTR(btf, c"btf_new".as_ptr()) {
        return;
    }

    raw_btf = btf__raw_data(btf, &mut raw_size);
    if !ASSERT_OK_PTR(raw_btf, c"btf__raw_data".as_ptr()) {
        return;
    }

    hdr = raw_btf as *const btf_header;

    ASSERT_EQ((*hdr).layout_off, 0, c"no_layout_off".as_ptr());
    ASSERT_EQ((*hdr).layout_len, 0, c"no_layout_len".as_ptr());
    ASSERT_EQ(
        (*hdr).str_off,
        (*hdr).type_off + (*hdr).type_len,
        c"strs_after_types".as_ptr(),
    );
    btf__free(btf);
}

unsafe fn write_raw_btf(raw_btf: *mut core::ffi::c_void, raw_size: size_t, file: *mut i8) -> i32 {
    let fd: i32 = mkstemp(file);
    let n: ssize_t;

    if !ASSERT_OK_FD(fd, c"open_raw_btf".as_ptr()) {
        return -1;
    }
    n = write(fd, raw_btf as *const core::ffi::c_void, raw_size);
    close(fd);
    if !ASSERT_EQ(n, raw_size as ssize_t, c"write_raw_btf".as_ptr()) {
        return -1;
    }
    0
}

/*
 * Fabricate an unrecognized kind at BTF_KIND_MAX + 1, and after adding
 * the appropriate struct/typedefs to the BTF such that it recognizes
 * this kind, ensure that parsing of BTF containing the unrecognized kind
 * can succeed.
 */
pub unsafe fn test_btf_kind_decoding() {
    let mut btf_kind_file1 = *b"/tmp/test_btf_kind.XXXXXX\0";
    let mut btf_kind_file2 = *b"/tmp/test_btf_kind.XXXXXX\0";
    let mut btf_kind_file3 = *b"/tmp/test_btf_kind.XXXXXX\0";
    let mut btf: *mut btf = core::ptr::null_mut();
    let mut new_btf: *mut btf = core::ptr::null_mut();
    let int_id: __s32;
    let unrec_id: __s32;
    let id: __s32;
    let id2: __s32;
    let mut opts = btf_new_opts {
        sz: core::mem::size_of::<btf_new_opts>(),
        add_layout: false,
    };
    let mut l: *mut btf_layout;
    let mut hdr: *mut btf_header;
    let raw_btf: *const core::ffi::c_void;
    let mut t: *mut btf_type;
    let new_raw_btf: *mut core::ffi::c_void;
    let str_data: *mut core::ffi::c_void;
    let mut raw_size: __u32 = 0;

    opts.add_layout = true;
    btf = btf__new_empty_opts(&opts);
    if !ASSERT_OK_PTR(btf, c"btf_new".as_ptr()) {
        return;
    }

    int_id = btf__add_int(btf, c"test_char".as_ptr(), 1, BTF_INT_CHAR);
    if !ASSERT_GT(int_id, 0, c"add_int_id".as_ptr()) {
        return;
    }

    /*
     * Create our type with unrecognized kind by adding a typedef kind
     * we will overwrite it with our unrecognized kind value.
     */
    unrec_id = btf__add_typedef(btf, c"unrec_kind".as_ptr(), int_id);
    if !ASSERT_GT(unrec_id, 0, c"add_unrec_id".as_ptr()) {
        return;
    }

    /*
     * Add an id after it that we will look up to verify we can parse
     * beyond unrecognized kinds.
     */
    id = btf__add_typedef(btf, c"test_lookup".as_ptr(), int_id);
    if !ASSERT_GT(id, 0, c"add_test_lookup_id".as_ptr()) {
        return;
    }
    id2 = btf__add_typedef(btf, c"test_lookup2".as_ptr(), int_id);
    if !ASSERT_GT(id2, 0, c"add_test_lookup_id2".as_ptr()) {
        return;
    }

    raw_btf = btf__raw_data(btf, &mut raw_size);
    if !ASSERT_OK_PTR(raw_btf, c"btf__raw_data".as_ptr()) {
        return;
    }

    new_raw_btf = calloc(1, raw_size as size_t + core::mem::size_of::<btf_layout>());
    if !ASSERT_OK_PTR(new_raw_btf, c"calloc_raw_btf".as_ptr()) {
        return;
    }
    memcpy(new_raw_btf, raw_btf, raw_size as size_t);

    hdr = new_raw_btf as *mut btf_header;

    /* Move strings to make space for one new layout description */
    raw_size += core::mem::size_of::<btf_layout>() as __u32;
    str_data = (new_raw_btf as *mut u8)
        .add((*hdr).hdr_len as usize + (*hdr).str_off as usize)
        as *mut core::ffi::c_void;
    memmove(
        (str_data as *mut u8).add(core::mem::size_of::<btf_layout>()) as *mut core::ffi::c_void,
        str_data,
        (*hdr).str_len as size_t,
    );
    (*hdr).str_off += core::mem::size_of::<btf_layout>() as __u32;

    /* Add new layout description */
    (*hdr).layout_len += core::mem::size_of::<btf_layout>() as __u32;
    l = (new_raw_btf as *mut u8)
        .add((*hdr).hdr_len as usize + (*hdr).layout_off as usize)
        as *mut btf_layout;
    (*l.add(NR_BTF_KINDS)).info_sz = 0;
    (*l.add(NR_BTF_KINDS)).elem_sz = 0;
    (*l.add(NR_BTF_KINDS)).flags = 0;

    /* Now modify typedef added above to be an unrecognized kind. */
    t = (hdr as *mut u8)
        .add(
            (*hdr).hdr_len as usize
                + (*hdr).type_off as usize
                + core::mem::size_of::<btf_type>()
                + core::mem::size_of::<__u32>(),
        ) as *mut btf_type;
    (*t).info = (NR_BTF_KINDS as __u32) << 24;

    /* Write BTF to a raw file, ready for parsing. */
    if write_raw_btf(
        new_raw_btf,
        raw_size as size_t,
        btf_kind_file1.as_mut_ptr() as *mut i8,
    ) != 0
    {
        goto_out(new_btf, new_raw_btf, &mut btf_kind_file1, &mut btf_kind_file2, &mut btf_kind_file3, btf);
        return;
    }

    /*
     * Verify parsing succeeds, and that we can read type info past
     * the unrecognized kind.
     */
    new_btf = btf__parse_raw(btf_kind_file1.as_ptr() as *const i8);
    if ASSERT_OK_PTR(new_btf, c"btf__parse_raw".as_ptr()) {
        ASSERT_EQ(
            btf__find_by_name(new_btf, c"unrec_kind".as_ptr()),
            unrec_id,
            c"unrec_kind_found".as_ptr(),
        );
        ASSERT_EQ(
            btf__find_by_name_kind(new_btf, c"test_lookup".as_ptr(), BTF_KIND_TYPEDEF),
            id,
            c"verify_id_lookup".as_ptr(),
        );
        ASSERT_EQ(
            btf__find_by_name_kind(new_btf, c"test_lookup2".as_ptr(), BTF_KIND_TYPEDEF),
            id2,
            c"verify_id2_lookup".as_ptr(),
        );
    }
    btf__free(new_btf);
    new_btf = core::ptr::null_mut();

    /*
     * Next, change info_sz to equal sizeof(struct btf_type); this means the
     * "test_lookup" kind will be reinterpreted as a singular info element
     * following the unrecognized kind.
     */
    (*l.add(NR_BTF_KINDS)).info_sz = core::mem::size_of::<btf_type>() as __u32;
    if write_raw_btf(
        new_raw_btf,
        raw_size as size_t,
        btf_kind_file2.as_mut_ptr() as *mut i8,
    ) != 0
    {
        goto_out(new_btf, new_raw_btf, &mut btf_kind_file1, &mut btf_kind_file2, &mut btf_kind_file3, btf);
        return;
    }

    new_btf = btf__parse_raw(btf_kind_file2.as_ptr() as *const i8);
    if ASSERT_OK_PTR(new_btf, c"btf__parse_raw".as_ptr()) {
        ASSERT_EQ(
            btf__find_by_name_kind(new_btf, c"test_lookup".as_ptr(), BTF_KIND_TYPEDEF),
            -ENOENT,
            c"verify_id_not_found".as_ptr(),
        );
        /* id of "test_lookup2" will be id2 -1 as we have removed one type */
        ASSERT_EQ(
            btf__find_by_name_kind(new_btf, c"test_lookup2".as_ptr(), BTF_KIND_TYPEDEF),
            id2 - 1,
            c"verify_id_lookup2".as_ptr(),
        );
    }
    btf__free(new_btf);
    new_btf = core::ptr::null_mut();

    /*
     * Change elem_sz to equal sizeof(struct btf_type) and set vlen
     * associated with unrecognized type to 1; this allows us to verify
     * vlen-specified BTF can still be parsed.
     */
    (*l.add(NR_BTF_KINDS)).info_sz = 0;
    (*l.add(NR_BTF_KINDS)).elem_sz = core::mem::size_of::<btf_type>() as __u32;
    (*t).info |= 1;
    if write_raw_btf(
        new_raw_btf,
        raw_size as size_t,
        btf_kind_file3.as_mut_ptr() as *mut i8,
    ) != 0
    {
        goto_out(new_btf, new_raw_btf, &mut btf_kind_file1, &mut btf_kind_file2, &mut btf_kind_file3, btf);
        return;
    }

    new_btf = btf__parse_raw(btf_kind_file3.as_ptr() as *const i8);
    if ASSERT_OK_PTR(new_btf, c"btf__parse_raw".as_ptr()) {
        ASSERT_EQ(
            btf__find_by_name_kind(new_btf, c"test_lookup".as_ptr(), BTF_KIND_TYPEDEF),
            -ENOENT,
            c"verify_id_not_found".as_ptr(),
        );
        /* id of "test_lookup2" will be id2 -1 as we have removed one type */
        ASSERT_EQ(
            btf__find_by_name_kind(new_btf, c"test_lookup2".as_ptr(), BTF_KIND_TYPEDEF),
            id2 - 1,
            c"verify_id_lookup2".as_ptr(),
        );
    }

    goto_out(new_btf, new_raw_btf, &mut btf_kind_file1, &mut btf_kind_file2, &mut btf_kind_file3, btf);
}

unsafe fn goto_out(
    new_btf: *mut btf,
    new_raw_btf: *mut core::ffi::c_void,
    btf_kind_file1: &mut [u8; 27],
    btf_kind_file2: &mut [u8; 27],
    btf_kind_file3: &mut [u8; 27],
    btf: *mut btf,
) {
    btf__free(new_btf);
    free(new_raw_btf);
    unlink(btf_kind_file1.as_ptr() as *const i8);
    unlink(btf_kind_file2.as_ptr() as *const i8);
    unlink(btf_kind_file3.as_ptr() as *const i8);
    btf__free(btf);
}

pub unsafe fn test_btf_kind() {
    if test__start_subtest(c"btf_kind_encoding".as_ptr()) {
        test_btf_kind_encoding();
    }
    if test__start_subtest(c"btf_kind_decoding".as_ptr()) {
        test_btf_kind_decoding();
    }
}
