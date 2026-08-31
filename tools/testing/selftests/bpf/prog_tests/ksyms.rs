// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook */

// C dependencies translated as external declarations:
// <test_progs.h>, "test_ksyms.skel.h", <sys/stat.h>

use core::ffi::{c_char, c_int, c_uint, c_void};

type __u64 = u64;
type useconds_t = c_uint;

const EINVAL: c_int = 22;
const ENOENT: c_int = 2;

#[repr(C)]
pub struct stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: c_uint,
    pub st_uid: c_uint,
    pub st_gid: c_uint,
    pub __pad0: c_int,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atime: i64,
    pub st_atime_nsec: i64,
    pub st_mtime: i64,
    pub st_mtime_nsec: i64,
    pub st_ctime: i64,
    pub st_ctime_nsec: i64,
    pub __glibc_reserved: [i64; 3],
}

#[repr(C)]
pub struct test_ksyms {
    pub data: *mut test_ksyms__data,
}

#[repr(C)]
pub struct test_ksyms__data {
    pub out__bpf_link_fops: __u64,
    pub out__bpf_link_fops1: __u64,
    pub out__btf_size: __u64,
    pub out__per_cpu_start: __u64,
}

extern "C" {
    fn kallsyms_find(sym: *const c_char, addr: *mut __u64) -> c_int;
    fn stat(path: *const c_char, buf: *mut stat) -> c_int;
    fn usleep(usec: useconds_t) -> c_int;

    fn test_ksyms__open_and_load() -> *mut test_ksyms;
    fn test_ksyms__attach(skel: *mut test_ksyms) -> c_int;
    fn test_ksyms__destroy(skel: *mut test_ksyms);

    fn ASSERT_NEQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(actual: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: __u64, expected: __u64, name: *const c_char) -> bool;
}

#[no_mangle]
pub unsafe extern "C" fn test_ksyms() {
    let btf_path: *const c_char = b"/sys/kernel/btf/vmlinux\0".as_ptr() as *const c_char;
    let mut skel: *mut test_ksyms;
    let mut data: *mut test_ksyms__data;
    let mut link_fops_addr: __u64 = 0;
    let mut per_cpu_start_addr: __u64 = 0;
    let mut st: stat = core::mem::zeroed();
    let btf_size: __u64;
    let mut err: c_int;

    err = kallsyms_find(
        b"bpf_link_fops\0".as_ptr() as *const c_char,
        &mut link_fops_addr,
    );
    if !ASSERT_NEQ(
        err,
        -EINVAL,
        b"bpf_link_fops: kallsyms_fopen\0".as_ptr() as *const c_char,
    ) {
        return;
    }
    if !ASSERT_NEQ(
        err,
        -ENOENT,
        b"bpf_link_fops: ksym_find\0".as_ptr() as *const c_char,
    ) {
        return;
    }

    err = kallsyms_find(
        b"__per_cpu_start\0".as_ptr() as *const c_char,
        &mut per_cpu_start_addr,
    );
    if !ASSERT_NEQ(
        err,
        -EINVAL,
        b"__per_cpu_start: kallsyms_fopen\0".as_ptr() as *const c_char,
    ) {
        return;
    }
    if !ASSERT_NEQ(
        err,
        -ENOENT,
        b"__per_cpu_start: ksym_find\0".as_ptr() as *const c_char,
    ) {
        return;
    }

    if !ASSERT_OK(stat(btf_path, &mut st), b"stat_btf\0".as_ptr() as *const c_char) {
        return;
    }
    btf_size = st.st_size as __u64;

    skel = test_ksyms__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *const c_void,
        b"test_ksyms__open_and_load\0".as_ptr() as *const c_char,
    ) {
        return;
    }

    err = test_ksyms__attach(skel);
    if !ASSERT_OK(
        err,
        b"test_ksyms__attach\0".as_ptr() as *const c_char,
    ) {
        test_ksyms__destroy(skel);
        return;
    }

    /* trigger tracepoint */
    usleep(1);

    data = (*skel).data;
    ASSERT_EQ(
        (*data).out__bpf_link_fops,
        link_fops_addr,
        b"bpf_link_fops\0".as_ptr() as *const c_char,
    );
    ASSERT_EQ(
        (*data).out__bpf_link_fops1,
        0,
        b"bpf_link_fops1\0".as_ptr() as *const c_char,
    );
    ASSERT_EQ(
        (*data).out__btf_size,
        btf_size,
        b"btf_size\0".as_ptr() as *const c_char,
    );
    ASSERT_EQ(
        (*data).out__per_cpu_start,
        per_cpu_start_addr,
        b"__per_cpu_start\0".as_ptr() as *const c_char,
    );

    test_ksyms__destroy(skel);
}
