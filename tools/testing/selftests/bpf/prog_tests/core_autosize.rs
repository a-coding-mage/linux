// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

// C dependencies: <test_progs.h>, <bpf/btf.h>, and "test_core_autosize.skel.h".

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type size_t = usize;
type __u32 = u32;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_object_open_opts {
    pub btf_custom_path: *mut c_char,
}

#[repr(C)]
pub struct test_core_autosize_links {
    pub handle_samesize: *mut bpf_link,
    pub handle_downsize: *mut bpf_link,
    pub handle_probed: *mut bpf_link,
}

#[repr(C)]
pub struct test_core_autosize {
    pub obj: *mut bpf_object,
    pub links: test_core_autosize_links,
}

/* real layout and sizes according to test's (32-bit) BTF
 * needs to be defined before skeleton is included */
#[repr(C)]
pub struct test_struct___real {
    pub ptr: c_uint, /* can't use `void *`, it is always 8 byte in BPF target */
    pub val2: c_uint,
    pub val1: u64,
    pub val3: u16,
    pub val4: u8,
    pub _pad: u8,
}

#[repr(C)]
pub struct out_t {
    pub ptr_samesized: u64,
    pub val1_samesized: u64,
    pub val2_samesized: u64,
    pub val3_samesized: u64,
    pub val4_samesized: u64,
    pub output_samesized: test_struct___real,

    pub ptr_downsized: u64,
    pub val1_downsized: u64,
    pub val2_downsized: u64,
    pub val3_downsized: u64,
    pub val4_downsized: u64,
    pub output_downsized: test_struct___real,

    pub ptr_probed: u64,
    pub val1_probed: u64,
    pub val2_probed: u64,
    pub val3_probed: u64,
    pub val4_probed: u64,

    pub ptr_signed: u64,
    pub val1_signed: u64,
    pub val2_signed: u64,
    pub val3_signed: u64,
    pub val4_signed: u64,
    pub output_signed: test_struct___real,
}

static mut duration: c_int = 0;

static mut out: out_t = out_t {
    ptr_samesized: 0,
    val1_samesized: 0,
    val2_samesized: 0,
    val3_samesized: 0,
    val4_samesized: 0,
    output_samesized: test_struct___real {
        ptr: 0,
        val2: 0,
        val1: 0,
        val3: 0,
        val4: 0,
        _pad: 0,
    },
    ptr_downsized: 0,
    val1_downsized: 0,
    val2_downsized: 0,
    val3_downsized: 0,
    val4_downsized: 0,
    output_downsized: test_struct___real {
        ptr: 0,
        val2: 0,
        val1: 0,
        val3: 0,
        val4: 0,
        _pad: 0,
    },
    ptr_probed: 0,
    val1_probed: 0,
    val2_probed: 0,
    val3_probed: 0,
    val4_probed: 0,
    ptr_signed: 0,
    val1_signed: 0,
    val2_signed: 0,
    val3_signed: 0,
    val4_signed: 0,
    output_signed: test_struct___real {
        ptr: 0,
        val2: 0,
        val1: 0,
        val3: 0,
        val4: 0,
        _pad: 0,
    },
};

extern "C" {
    static mut errno: c_int;

    fn btf__new_empty() -> *mut btf;
    fn btf__set_pointer_size(btf: *mut btf, ptr_sz: c_uint);
    fn btf__add_int(btf: *mut btf, name: *const c_char, sz: c_uint, encoding: c_uint) -> c_int;
    fn btf__add_ptr(btf: *mut btf, type_id: c_int) -> c_int;
    fn btf__add_struct(btf: *mut btf, name: *const c_char, sz: c_uint) -> c_int;
    fn btf__add_field(
        btf: *mut btf,
        name: *const c_char,
        type_id: c_int,
        bit_offset: c_uint,
        bit_size: c_uint,
    ) -> c_int;
    fn btf__raw_data(btf: *const btf, size: *mut __u32) -> *const c_void;
    fn btf__free(btf: *mut btf);

    fn mkstemp(template: *mut c_char) -> c_int;
    fn fdopen(fd: c_int, mode: *const c_char) -> *mut FILE;
    fn fwrite(ptr: *const c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    fn fflush(stream: *mut FILE) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn remove(pathname: *const c_char) -> c_int;
    fn usleep(usec: c_uint) -> c_int;

    fn test_core_autosize__open_opts(
        opts: *const bpf_object_open_opts,
    ) -> *mut test_core_autosize;
    fn test_core_autosize__load(skel: *mut test_core_autosize) -> c_int;
    fn test_core_autosize__destroy(skel: *mut test_core_autosize);

    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool);
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_program__attach(prog: *mut bpf_program) -> *mut bpf_link;
    fn bpf_object__find_map_by_name(obj: *mut bpf_object, name: *const c_char) -> *mut bpf_map;
    fn bpf_map__lookup_elem(
        map: *const bpf_map,
        key: *const c_void,
        key_sz: size_t,
        value: *mut c_void,
        value_sz: size_t,
        flags: u64,
    ) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: u64, expected: u64, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_ERR(err: c_int, name: *const c_char) -> bool;
    fn CHECK(condition: bool, name: *const c_char, fmt: *const c_char, ...) -> bool;
}

#[no_mangle]
pub unsafe extern "C" fn test_core_autosize() {
    let mut btf_file = *b"/tmp/core_autosize.btf.XXXXXX\0";
    let mut err: c_int;
    let mut fd: c_int = -1;
    let zero: c_int = 0;
    let mut char_id: c_int;
    let mut short_id: c_int;
    let mut int_id: c_int;
    let mut long_long_id: c_int;
    let mut void_ptr_id: c_int;
    let mut id: c_int;
    let mut open_opts = bpf_object_open_opts {
        btf_custom_path: core::ptr::null_mut(),
    };
    let mut skel: *mut test_core_autosize = core::ptr::null_mut();
    let mut prog: *mut bpf_program;
    let mut bss_map: *mut bpf_map;
    let mut btf: *mut btf = core::ptr::null_mut();
    let mut written: size_t;
    let mut raw_data: *const c_void;
    let mut raw_sz: __u32 = 0;
    let mut f: *mut FILE = core::ptr::null_mut();

    btf = btf__new_empty();
    if !ASSERT_OK_PTR(btf as *const c_void, b"empty_btf\0".as_ptr() as *const c_char) {
        return;
    }
    /* Emit the following struct with 32-bit pointer size:
     *
     * struct test_struct {
     *     void *ptr;
     *     unsigned long val2;
     *     unsigned long long val1;
     *     unsigned short val3;
     *     unsigned char val4;
     *     char: 8;
     * };
     *
     * This struct is going to be used as the "kernel BTF" for this test.
     * It's equivalent memory-layout-wise to test_struct__real above.
     */

    /* force 32-bit pointer size */
    btf__set_pointer_size(btf, 4);

    char_id = btf__add_int(btf, b"unsigned char\0".as_ptr() as *const c_char, 1, 0);
    ASSERT_EQ(char_id as u64, 1, b"char_id\0".as_ptr() as *const c_char);
    short_id = btf__add_int(btf, b"unsigned short\0".as_ptr() as *const c_char, 2, 0);
    ASSERT_EQ(short_id as u64, 2, b"short_id\0".as_ptr() as *const c_char);
    /* "long unsigned int" of 4 byte size tells BTF that sizeof(void *) == 4 */
    int_id = btf__add_int(
        btf,
        b"long unsigned int\0".as_ptr() as *const c_char,
        4,
        0,
    );
    ASSERT_EQ(int_id as u64, 3, b"int_id\0".as_ptr() as *const c_char);
    long_long_id = btf__add_int(
        btf,
        b"unsigned long long\0".as_ptr() as *const c_char,
        8,
        0,
    );
    ASSERT_EQ(
        long_long_id as u64,
        4,
        b"long_long_id\0".as_ptr() as *const c_char,
    );
    void_ptr_id = btf__add_ptr(btf, 0);
    ASSERT_EQ(
        void_ptr_id as u64,
        5,
        b"void_ptr_id\0".as_ptr() as *const c_char,
    );

    id = btf__add_struct(btf, b"test_struct\0".as_ptr() as *const c_char, 20 /* bytes */);
    ASSERT_EQ(id as u64, 6, b"struct_id\0".as_ptr() as *const c_char);
    err = btf__add_field(btf, b"ptr\0".as_ptr() as *const c_char, void_ptr_id, 0, 0);
    if err == 0 {
        err = btf__add_field(btf, b"val2\0".as_ptr() as *const c_char, int_id, 32, 0);
    }
    if err == 0 {
        err = btf__add_field(
            btf,
            b"val1\0".as_ptr() as *const c_char,
            long_long_id,
            64,
            0,
        );
    }
    if err == 0 {
        err = btf__add_field(btf, b"val3\0".as_ptr() as *const c_char, short_id, 128, 0);
    }
    if err == 0 {
        err = btf__add_field(btf, b"val4\0".as_ptr() as *const c_char, char_id, 144, 0);
    }
    ASSERT_OK(err, b"struct_fields\0".as_ptr() as *const c_char);

    fd = mkstemp(btf_file.as_mut_ptr() as *mut c_char);
    if CHECK(
        fd < 0,
        b"btf_tmp\0".as_ptr() as *const c_char,
        b"failed to create file: %d\n\0".as_ptr() as *const c_char,
        fd,
    ) {
        goto_cleanup(fd, f, btf_file.as_ptr() as *const c_char, btf, skel);
        return;
    }
    f = fdopen(fd, b"w\0".as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(f as *const c_void, b"btf_fdopen\0".as_ptr() as *const c_char) {
        goto_cleanup(fd, f, btf_file.as_ptr() as *const c_char, btf, skel);
        return;
    }

    raw_data = btf__raw_data(btf, &mut raw_sz);
    if !ASSERT_OK_PTR(raw_data, b"raw_data\0".as_ptr() as *const c_char) {
        goto_cleanup(fd, f, btf_file.as_ptr() as *const c_char, btf, skel);
        return;
    }
    written = fwrite(raw_data, 1, raw_sz as size_t, f);
    if CHECK(
        written != raw_sz as size_t,
        b"btf_write\0".as_ptr() as *const c_char,
        b"written: %zu, errno: %d\n\0".as_ptr() as *const c_char,
        written,
        errno,
    ) {
        goto_cleanup(fd, f, btf_file.as_ptr() as *const c_char, btf, skel);
        return;
    }
    fflush(f);
    fclose(f);
    f = core::ptr::null_mut();
    close(fd);
    fd = -1;

    /* open and load BPF program with custom BTF as the kernel BTF */
    open_opts.btf_custom_path = btf_file.as_mut_ptr() as *mut c_char;
    skel = test_core_autosize__open_opts(&open_opts);
    if !ASSERT_OK_PTR(skel as *const c_void, b"skel_open\0".as_ptr() as *const c_char) {
        goto_cleanup(fd, f, btf_file.as_ptr() as *const c_char, btf, skel);
        return;
    }

    /* disable handle_signed() for now */
    prog = bpf_object__find_program_by_name(
        (*skel).obj,
        b"handle_signed\0".as_ptr() as *const c_char,
    );
    if !ASSERT_OK_PTR(prog as *const c_void, b"prog_find\0".as_ptr() as *const c_char) {
        goto_cleanup(fd, f, btf_file.as_ptr() as *const c_char, btf, skel);
        return;
    }
    bpf_program__set_autoload(prog, false);

    err = bpf_object__load((*skel).obj);
    if !ASSERT_OK(err, b"prog_load\0".as_ptr() as *const c_char) {
        goto_cleanup(fd, f, btf_file.as_ptr() as *const c_char, btf, skel);
        return;
    }

    prog = bpf_object__find_program_by_name(
        (*skel).obj,
        b"handle_samesize\0".as_ptr() as *const c_char,
    );
    if !ASSERT_OK_PTR(prog as *const c_void, b"prog_find\0".as_ptr() as *const c_char) {
        goto_cleanup(fd, f, btf_file.as_ptr() as *const c_char, btf, skel);
        return;
    }
    (*skel).links.handle_samesize = bpf_program__attach(prog);
    if !ASSERT_OK_PTR(
        (*skel).links.handle_samesize as *const c_void,
        b"prog_attach\0".as_ptr() as *const c_char,
    ) {
        goto_cleanup(fd, f, btf_file.as_ptr() as *const c_char, btf, skel);
        return;
    }

    prog = bpf_object__find_program_by_name(
        (*skel).obj,
        b"handle_downsize\0".as_ptr() as *const c_char,
    );
    if !ASSERT_OK_PTR(prog as *const c_void, b"prog_find\0".as_ptr() as *const c_char) {
        goto_cleanup(fd, f, btf_file.as_ptr() as *const c_char, btf, skel);
        return;
    }
    (*skel).links.handle_downsize = bpf_program__attach(prog);
    if !ASSERT_OK_PTR(
        (*skel).links.handle_downsize as *const c_void,
        b"prog_attach\0".as_ptr() as *const c_char,
    ) {
        goto_cleanup(fd, f, btf_file.as_ptr() as *const c_char, btf, skel);
        return;
    }

    prog = bpf_object__find_program_by_name(
        (*skel).obj,
        b"handle_probed\0".as_ptr() as *const c_char,
    );
    if !ASSERT_OK_PTR(prog as *const c_void, b"prog_find\0".as_ptr() as *const c_char) {
        goto_cleanup(fd, f, btf_file.as_ptr() as *const c_char, btf, skel);
        return;
    }
    (*skel).links.handle_probed = bpf_program__attach(prog);
    if !ASSERT_OK_PTR(
        (*skel).links.handle_probed as *const c_void,
        b"prog_attach\0".as_ptr() as *const c_char,
    ) {
        goto_cleanup(fd, f, btf_file.as_ptr() as *const c_char, btf, skel);
        return;
    }

    usleep(1);

    bss_map = bpf_object__find_map_by_name((*skel).obj, b".bss\0".as_ptr() as *const c_char);
    if !ASSERT_OK_PTR(bss_map as *const c_void, b"bss_map_find\0".as_ptr() as *const c_char) {
        goto_cleanup(fd, f, btf_file.as_ptr() as *const c_char, btf, skel);
        return;
    }

    err = bpf_map__lookup_elem(
        bss_map,
        &zero as *const c_int as *const c_void,
        core::mem::size_of_val(&zero),
        &mut out as *mut out_t as *mut c_void,
        core::mem::size_of_val(&out),
        0,
    );
    if !ASSERT_OK(err, b"bss_lookup\0".as_ptr() as *const c_char) {
        goto_cleanup(fd, f, btf_file.as_ptr() as *const c_char, btf, skel);
        return;
    }

    ASSERT_EQ(out.ptr_samesized, 0x01020304, b"ptr_samesized\0".as_ptr() as *const c_char);
    ASSERT_EQ(out.val1_samesized, 0x1020304050607080, b"val1_samesized\0".as_ptr() as *const c_char);
    ASSERT_EQ(out.val2_samesized, 0x0a0b0c0d, b"val2_samesized\0".as_ptr() as *const c_char);
    ASSERT_EQ(out.val3_samesized, 0xfeed, b"val3_samesized\0".as_ptr() as *const c_char);
    ASSERT_EQ(out.val4_samesized, 0xb9, b"val4_samesized\0".as_ptr() as *const c_char);
    ASSERT_EQ(out.output_samesized.ptr as u64, 0x01020304, b"ptr_samesized\0".as_ptr() as *const c_char);
    ASSERT_EQ(out.output_samesized.val1, 0x1020304050607080, b"val1_samesized\0".as_ptr() as *const c_char);
    ASSERT_EQ(out.output_samesized.val2 as u64, 0x0a0b0c0d, b"val2_samesized\0".as_ptr() as *const c_char);
    ASSERT_EQ(out.output_samesized.val3 as u64, 0xfeed, b"val3_samesized\0".as_ptr() as *const c_char);
    ASSERT_EQ(out.output_samesized.val4 as u64, 0xb9, b"val4_samesized\0".as_ptr() as *const c_char);

    ASSERT_EQ(out.ptr_downsized, 0x01020304, b"ptr_downsized\0".as_ptr() as *const c_char);
    ASSERT_EQ(out.val1_downsized, 0x1020304050607080, b"val1_downsized\0".as_ptr() as *const c_char);
    ASSERT_EQ(out.val2_downsized, 0x0a0b0c0d, b"val2_downsized\0".as_ptr() as *const c_char);
    ASSERT_EQ(out.val3_downsized, 0xfeed, b"val3_downsized\0".as_ptr() as *const c_char);
    ASSERT_EQ(out.val4_downsized, 0xb9, b"val4_downsized\0".as_ptr() as *const c_char);
    ASSERT_EQ(out.output_downsized.ptr as u64, 0x01020304, b"ptr_downsized\0".as_ptr() as *const c_char);
    ASSERT_EQ(out.output_downsized.val1, 0x1020304050607080, b"val1_downsized\0".as_ptr() as *const c_char);
    ASSERT_EQ(out.output_downsized.val2 as u64, 0x0a0b0c0d, b"val2_downsized\0".as_ptr() as *const c_char);
    ASSERT_EQ(out.output_downsized.val3 as u64, 0xfeed, b"val3_downsized\0".as_ptr() as *const c_char);
    ASSERT_EQ(out.output_downsized.val4 as u64, 0xb9, b"val4_downsized\0".as_ptr() as *const c_char);

    ASSERT_EQ(out.ptr_probed, 0x01020304, b"ptr_probed\0".as_ptr() as *const c_char);
    ASSERT_EQ(out.val1_probed, 0x1020304050607080, b"val1_probed\0".as_ptr() as *const c_char);
    ASSERT_EQ(out.val2_probed, 0x0a0b0c0d, b"val2_probed\0".as_ptr() as *const c_char);
    ASSERT_EQ(out.val3_probed, 0xfeed, b"val3_probed\0".as_ptr() as *const c_char);
    ASSERT_EQ(out.val4_probed, 0xb9, b"val4_probed\0".as_ptr() as *const c_char);

    test_core_autosize__destroy(skel);
    skel = core::ptr::null_mut();

    /* now re-load with handle_signed() enabled, it should fail loading */
    open_opts.btf_custom_path = btf_file.as_mut_ptr() as *mut c_char;
    skel = test_core_autosize__open_opts(&open_opts);
    if !ASSERT_OK_PTR(skel as *const c_void, b"skel_open\0".as_ptr() as *const c_char) {
        goto_cleanup(fd, f, btf_file.as_ptr() as *const c_char, btf, skel);
        return;
    }

    err = test_core_autosize__load(skel);
    if !ASSERT_ERR(err, b"skel_load\0".as_ptr() as *const c_char) {
        goto_cleanup(fd, f, btf_file.as_ptr() as *const c_char, btf, skel);
        return;
    }

    goto_cleanup(fd, f, btf_file.as_ptr() as *const c_char, btf, skel);
}

unsafe fn goto_cleanup(
    fd: c_int,
    f: *mut FILE,
    btf_file: *const c_char,
    btf: *mut btf,
    skel: *mut test_core_autosize,
) {
    if !f.is_null() {
        fclose(f);
    }
    if fd >= 0 {
        close(fd);
    }
    remove(btf_file);
    btf__free(btf);
    test_core_autosize__destroy(skel);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
