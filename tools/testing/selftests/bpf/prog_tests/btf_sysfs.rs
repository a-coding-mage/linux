// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
/* Copyright (c) 2025 Isovalent */

// C dependencies:
// #include <test_progs.h>
// #include <bpf/btf.h>
// #include <sys/stat.h>
// #include <sys/mman.h>
// #include <fcntl.h>
// #include <unistd.h>

use core::ffi::{c_char, c_int, c_long, c_void};
use core::ptr;

type __u8 = u8;
type __u64 = u64;

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stat {
    pub st_size: i64,
}

const _SC_PAGESIZE: c_int = 30;
const O_RDONLY: c_int = 0;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const PROT_EXEC: c_int = 0x4;
const MAP_PRIVATE: c_int = 0x02;
const MAP_SHARED: c_int = 0x01;

const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

unsafe extern "C" {
    fn sysconf(name: c_int) -> c_long;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    fn mprotect(addr: *mut c_void, len: usize, prot: c_int) -> c_int;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn close(fd: c_int) -> c_int;

    fn btf__new_split(data: *const c_void, size: usize, base_btf: *mut btf) -> *mut btf;
    fn btf__free(btf: *mut btf);

    fn ASSERT_GE(actual: c_long, expected: c_long, name: *const c_char) -> bool;
    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: isize, expected: isize, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn PRINT_FAIL(fmt: *const c_char, ...);
}

unsafe fn test_btf_mmap_sysfs(path: *const c_char, base: *mut btf) {
    let mut st: stat = core::mem::zeroed();
    let mut btf_size: __u64;
    let mut end: __u64;
    let mut raw_data: *mut c_void = ptr::null_mut();
    let mut fd: c_int = -1;
    let page_size: c_long;
    let mut btf: *mut btf = ptr::null_mut();

    page_size = sysconf(_SC_PAGESIZE);
    if !ASSERT_GE(page_size, 0, c"get_page_size".as_ptr()) {
        goto_cleanup(btf, raw_data, btf_size_if_initialized(false, 0), fd);
        return;
    }

    if !ASSERT_OK(stat(path, &mut st), c"stat_btf".as_ptr()) {
        goto_cleanup(btf, raw_data, btf_size_if_initialized(false, 0), fd);
        return;
    }

    btf_size = st.st_size as __u64;
    end = (btf_size + page_size as __u64 - 1) / page_size as __u64 * page_size as __u64;

    fd = open(path, O_RDONLY);
    if !ASSERT_GE(fd as c_long, 0, c"open_btf".as_ptr()) {
        goto_cleanup(btf, raw_data, btf_size, fd);
        return;
    }

    raw_data = mmap(
        ptr::null_mut(),
        btf_size as usize,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE,
        fd,
        0,
    );
    if !ASSERT_EQ(
        raw_data as isize,
        MAP_FAILED as isize,
        c"mmap_btf_writable".as_ptr(),
    ) {
        goto_cleanup(btf, raw_data, btf_size, fd);
        return;
    }

    raw_data = mmap(
        ptr::null_mut(),
        btf_size as usize,
        PROT_READ,
        MAP_SHARED,
        fd,
        0,
    );
    if !ASSERT_EQ(
        raw_data as isize,
        MAP_FAILED as isize,
        c"mmap_btf_shared".as_ptr(),
    ) {
        goto_cleanup(btf, raw_data, btf_size, fd);
        return;
    }

    raw_data = mmap(
        ptr::null_mut(),
        (end + 1) as usize,
        PROT_READ,
        MAP_PRIVATE,
        fd,
        0,
    );
    if !ASSERT_EQ(
        raw_data as isize,
        MAP_FAILED as isize,
        c"mmap_btf_invalid_size".as_ptr(),
    ) {
        goto_cleanup(btf, raw_data, btf_size, fd);
        return;
    }

    raw_data = mmap(
        ptr::null_mut(),
        end as usize,
        PROT_READ,
        MAP_PRIVATE,
        fd,
        0,
    );
    if !ASSERT_OK_PTR(raw_data, c"mmap_btf".as_ptr()) {
        goto_cleanup(btf, raw_data, btf_size, fd);
        return;
    }

    if !ASSERT_EQ(
        mprotect(raw_data, btf_size as usize, PROT_READ | PROT_WRITE) as isize,
        -1,
        c"mprotect_writable".as_ptr(),
    ) {
        goto_cleanup(btf, raw_data, btf_size, fd);
        return;
    }

    if !ASSERT_EQ(
        mprotect(raw_data, btf_size as usize, PROT_READ | PROT_EXEC) as isize,
        -1,
        c"mprotect_executable".as_ptr(),
    ) {
        goto_cleanup(btf, raw_data, btf_size, fd);
        return;
    }

    /* Check padding is zeroed */
    let mut i: c_int = btf_size as c_int;
    while (i as __u64) < end {
        if *(raw_data as *mut __u8).offset(i as isize) != 0 {
            PRINT_FAIL(
                c"tail of BTF is not zero at page offset %d\n".as_ptr(),
                i,
            );
            goto_cleanup(btf, raw_data, btf_size, fd);
            return;
        }
        i += 1;
    }

    btf = btf__new_split(raw_data, btf_size as usize, base);
    if !ASSERT_OK_PTR(btf as *const c_void, c"parse_btf".as_ptr()) {
        goto_cleanup(btf, raw_data, btf_size, fd);
        return;
    }

    goto_cleanup(btf, raw_data, btf_size, fd);
}

unsafe fn btf_size_if_initialized(initialized: bool, value: __u64) -> __u64 {
    if initialized {
        value
    } else {
        0
    }
}

unsafe fn goto_cleanup(btf: *mut btf, raw_data: *mut c_void, btf_size: __u64, fd: c_int) {
    btf__free(btf);
    if !raw_data.is_null() && raw_data != MAP_FAILED {
        munmap(raw_data, btf_size as usize);
    }
    if fd >= 0 {
        close(fd);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_btf_sysfs() {
    test_btf_mmap_sysfs(c"/sys/kernel/btf/vmlinux".as_ptr(), ptr::null_mut());
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
