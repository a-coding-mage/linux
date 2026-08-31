// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2020 ARM Limited

// Translated from C implementation source. C includes and _GNU_SOURCE are
// dependency/build intent supplied by the surrounding selftest tree.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

type size_t = usize;
type ssize_t = isize;
type uintptr_t = usize;

const TEST_NAME_MAX: usize = 100;

// External constants supplied by the surrounding kselftest/MTE headers.
extern "C" {
    static KSFT_PASS: c_int;
    static KSFT_FAIL: c_int;
    static MTE_ALLOW_NON_ZERO_TAG: c_int;
    static MTE_SYNC_ERR: c_int;
    static MTE_ASYNC_ERR: c_int;
    static MAP_SHARED: c_int;
    static MAP_PRIVATE: c_int;
    static MT_GRANULE_SIZE: c_int;
    static USE_MMAP: c_int;
}

#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: size_t,
}

#[repr(C)]
struct mte_context {
    fault_valid: bool,
}

extern "C" {
    static mut cur_mte_cxt: mte_context;

    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn readv(fd: c_int, iov: *const iovec, iovcnt: c_int) -> ssize_t;
    fn writev(fd: c_int, iov: *const iovec, iovcnt: c_int) -> ssize_t;
    fn lseek(fd: c_int, offset: c_long, whence: c_int) -> c_long;
    fn close(fd: c_int) -> c_int;
    fn getpagesize() -> c_int;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;

    fn ksft_print_header();
    fn ksft_print_msg(msg: *const c_char, ...);
    fn ksft_set_plan(plan: c_int);
    fn ksft_print_cnts();
    fn ksft_get_fail_cnt() -> c_int;
    fn evaluate_test(res: c_int, name: *const c_char);

    fn mte_switch_mode(mode: c_int, tag_mask: c_int, include: bool);
    fn create_temp_file() -> c_int;
    fn mte_allocate_memory(
        len: size_t,
        mem_type: c_int,
        mapping: c_int,
        tag: bool,
    ) -> *mut c_void;
    fn check_allocated_memory(ptr: *mut c_void, len: size_t, mem_type: c_int, tag: bool) -> c_int;
    fn mte_initialize_current_context(mode: c_int, ptr: uintptr_t, len: size_t);
    fn mte_wait_after_trig();
    fn mte_insert_new_tag(ptr: *mut c_void) -> *mut c_void;
    fn mte_set_tag_address_range(ptr: *mut c_void, len: size_t);
    fn mte_free_memory(ptr: *mut c_void, len: size_t, mem_type: c_int, tag: bool);
    fn mte_default_setup() -> c_int;
    fn mte_default_handler(sig: c_int, si: *mut c_void, uc: *mut c_void);
    fn mte_register_signal(sig: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void, *mut c_void), flags: bool);
    fn mte_restore_setup();
}

static mut page_sz: size_t = 0;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum test_type {
    READ_TEST,
    WRITE_TEST,
    READV_TEST,
    WRITEV_TEST,
    LAST_TEST,
}

unsafe fn check_usermem_access_fault(
    mem_type: c_int,
    mode: c_int,
    mapping: c_int,
    tag_offset: c_int,
    mut tag_len: c_int,
    test_type: test_type,
) -> c_int {
    let mut fd: c_int;
    let mut i: ssize_t;
    let val: c_char = b'A' as c_char;
    let len: ssize_t;
    let mut syscall_len: ssize_t;
    let ptr: *mut c_void;
    let mut ptr_next: *mut c_void;
    let mut fileoff: c_int;
    let mut ptroff: c_int;
    let mut size: c_int;
    let sizes: [size_t; 8] = [1, 2, 3, 8, 16, 32, 4096, page_sz];
    let mut err: c_int;

    err = KSFT_PASS;
    len = (2usize * page_sz) as ssize_t;
    mte_switch_mode(mode, MTE_ALLOW_NON_ZERO_TAG, false);
    fd = create_temp_file();
    if fd == -1 {
        return KSFT_FAIL;
    }
    i = 0;
    while i < len {
        if write(fd, &val as *const c_char as *const c_void, core::mem::size_of_val(&val)) != core::mem::size_of_val(&val) as ssize_t {
            return KSFT_FAIL;
        }
        i += 1;
    }
    lseek(fd, 0, 0);
    ptr = mte_allocate_memory(len as size_t, mem_type, mapping, true);
    if check_allocated_memory(ptr, len as size_t, mem_type, true) != KSFT_PASS {
        close(fd);
        return KSFT_FAIL;
    }
    mte_initialize_current_context(mode, ptr as uintptr_t, len as size_t);
    /* Copy from file into buffer with valid tag */
    syscall_len = read(fd, ptr, len as size_t);
    mte_wait_after_trig();
    if cur_mte_cxt.fault_valid || syscall_len < len {
        err = KSFT_FAIL;
        mte_free_memory(ptr, len as size_t, mem_type, true);
        close(fd);
        return err;
    }
    /* Verify same pattern is read */
    i = 0;
    while i < len {
        if *((ptr as *mut c_char).add(i as usize)) != val {
            break;
        }
        i += 1;
    }
    if i < len {
        err = KSFT_FAIL;
        mte_free_memory(ptr, len as size_t, mem_type, true);
        close(fd);
        return err;
    }

    if tag_len == 0 {
        tag_len = len as c_int - tag_offset;
    }
    /* Tag a part of memory with different value */
    ptr_next = (ptr as c_ulong).wrapping_add(tag_offset as c_ulong) as *mut c_void;
    ptr_next = mte_insert_new_tag(ptr_next);
    mte_set_tag_address_range(ptr_next, tag_len as size_t);

    fileoff = 0;
    while fileoff < 16 {
        ptroff = 0;
        while ptroff < 16 {
            i = 0;
            while (i as usize) < sizes.len() {
                size = sizes[i as usize] as c_int;
                lseek(fd, 0, 0);

                /* perform file operation on buffer with invalid tag */
                match test_type {
                    test_type::READ_TEST => {
                        syscall_len = read(fd, (ptr as *mut c_char).add(ptroff as usize) as *mut c_void, size as size_t);
                    }
                    test_type::WRITE_TEST => {
                        syscall_len = write(fd, (ptr as *mut c_char).add(ptroff as usize) as *const c_void, size as size_t);
                    }
                    test_type::READV_TEST => {
                        let mut iov: [iovec; 1] = [iovec {
                            iov_base: core::ptr::null_mut(),
                            iov_len: 0,
                        }];
                        iov[0].iov_base = (ptr as *mut c_char).add(ptroff as usize) as *mut c_void;
                        iov[0].iov_len = size as size_t;
                        syscall_len = readv(fd, iov.as_ptr(), 1);
                    }
                    test_type::WRITEV_TEST => {
                        let mut iov: [iovec; 1] = [iovec {
                            iov_base: core::ptr::null_mut(),
                            iov_len: 0,
                        }];
                        iov[0].iov_base = (ptr as *mut c_char).add(ptroff as usize) as *mut c_void;
                        iov[0].iov_len = size as size_t;
                        syscall_len = writev(fd, iov.as_ptr(), 1);
                    }
                    test_type::LAST_TEST => {
                        err = KSFT_FAIL;
                        mte_free_memory(ptr, len as size_t, mem_type, true);
                        close(fd);
                        return err;
                    }
                }

                mte_wait_after_trig();
                /*
                 * Accessing user memory in kernel with invalid tag should fail in sync
                 * mode without fault but may not fail in async mode as per the
                 * implemented MTE userspace support in Arm64 kernel.
                 */
                if cur_mte_cxt.fault_valid {
                    err = KSFT_FAIL;
                    mte_free_memory(ptr, len as size_t, mem_type, true);
                    close(fd);
                    return err;
                }
                if mode == MTE_SYNC_ERR && syscall_len < len {
                    /* test passed */
                } else if mode == MTE_ASYNC_ERR && syscall_len == size as ssize_t {
                    /* test passed */
                } else {
                    err = KSFT_FAIL;
                    mte_free_memory(ptr, len as size_t, mem_type, true);
                    close(fd);
                    return err;
                }
                i += 1;
            }
            ptroff += 1;
        }
        fileoff += 1;
    }

    mte_free_memory(ptr, len as size_t, mem_type, true);
    close(fd);
    err
}

unsafe fn format_test_name(
    name: *mut c_char,
    name_len: c_int,
    type_: c_int,
    sync: c_int,
    map: c_int,
    len: c_int,
    offset: c_int,
) {
    let test_type: *const c_char;
    let mte_type: *const c_char;
    let map_type: *const c_char;

    match type_ {
        x if x == test_type::READ_TEST as c_int => {
            test_type = b"read\0".as_ptr() as *const c_char;
        }
        x if x == test_type::WRITE_TEST as c_int => {
            test_type = b"write\0".as_ptr() as *const c_char;
        }
        x if x == test_type::READV_TEST as c_int => {
            test_type = b"readv\0".as_ptr() as *const c_char;
        }
        x if x == test_type::WRITEV_TEST as c_int => {
            test_type = b"writev\0".as_ptr() as *const c_char;
        }
        _ => {
            assert!(false);
            test_type = core::ptr::null();
        }
    }

    match sync {
        x if x == MTE_SYNC_ERR => {
            mte_type = b"MTE_SYNC_ERR\0".as_ptr() as *const c_char;
        }
        x if x == MTE_ASYNC_ERR => {
            mte_type = b"MTE_ASYNC_ERR\0".as_ptr() as *const c_char;
        }
        _ => {
            assert!(false);
            mte_type = core::ptr::null();
        }
    }

    match map {
        x if x == MAP_SHARED => {
            map_type = b"MAP_SHARED\0".as_ptr() as *const c_char;
        }
        x if x == MAP_PRIVATE => {
            map_type = b"MAP_PRIVATE\0".as_ptr() as *const c_char;
        }
        _ => {
            assert!(false);
            map_type = core::ptr::null();
        }
    }

    snprintf(
        name,
        name_len as size_t,
        b"test type: %s, %s, %s, tag len: %d, tag offset: %d\n\0".as_ptr() as *const c_char,
        test_type,
        mte_type,
        map_type,
        len,
        offset,
    );
}

unsafe fn main_0(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut err: c_int;
    let mut t: c_int;
    let mut s: c_int;
    let mut m: c_int;
    let mut l: c_int;
    let mut o: c_int;
    let mte_sync: [c_int; 2] = [MTE_SYNC_ERR, MTE_ASYNC_ERR];
    let maps: [c_int; 2] = [MAP_SHARED, MAP_PRIVATE];
    let tag_lens: [c_int; 2] = [0, MT_GRANULE_SIZE];
    let tag_offsets: [c_int; 2] = [page_sz as c_int, MT_GRANULE_SIZE];
    let mut test_name: [c_char; TEST_NAME_MAX] = [0; TEST_NAME_MAX];

    ksft_print_header();

    page_sz = getpagesize() as size_t;
    if page_sz == 0 {
        ksft_print_msg(b"ERR: Unable to get page size\n\0".as_ptr() as *const c_char);
        return KSFT_FAIL;
    }
    err = mte_default_setup();
    if err != 0 {
        return err;
    }

    /* Register signal handlers */
    mte_register_signal(11, mte_default_handler, false);

    /* Set test plan */
    ksft_set_plan(64);

    t = 0;
    while t < test_type::LAST_TEST as c_int {
        s = 0;
        while (s as usize) < mte_sync.len() {
            m = 0;
            while (m as usize) < maps.len() {
                l = 0;
                while (l as usize) < tag_lens.len() {
                    o = 0;
                    while (o as usize) < tag_offsets.len() {
                        let sync: c_int = mte_sync[s as usize];
                        let map: c_int = maps[m as usize];
                        let offset: c_int = tag_offsets[o as usize];
                        let tag_len: c_int = tag_lens[l as usize];
                        let res: c_int = check_usermem_access_fault(
                            USE_MMAP,
                            sync,
                            map,
                            offset,
                            tag_len,
                            core::mem::transmute::<c_int, test_type>(t),
                        );
                        format_test_name(
                            test_name.as_mut_ptr(),
                            TEST_NAME_MAX as c_int,
                            t,
                            sync,
                            map,
                            tag_len,
                            offset,
                        );
                        evaluate_test(res, test_name.as_ptr());
                        o += 1;
                    }
                    l += 1;
                }
                m += 1;
            }
            s += 1;
        }
        t += 1;
    }

    mte_restore_setup();
    ksft_print_cnts();
    if ksft_get_fail_cnt() == 0 {
        KSFT_PASS
    } else {
        KSFT_FAIL
    }
}

fn main() {
    unsafe {
        let code = main_0(0, core::ptr::null_mut());
        std::process::exit(code);
    }
}
