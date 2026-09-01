// SPDX-License-Identifier: GPL-2.0

// C dependencies: linux/limits.h, sys/mman.h, stdio.h, stdlib.h, string.h,
// fcntl.h, kselftest.h, cgroup_util.h.

use std::ffi::{c_char, c_int, c_long, c_ulong, c_void};

const PATH_MAX: usize = 4096;
const O_RDONLY: c_int = 0;

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_HUGETLB: c_int = 0x40000;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const KSFT_PASS: c_int = 0;
const KSFT_FAIL: c_int = 1;
const KSFT_SKIP: c_int = 4;

const ADDR: *mut c_void = 0 as *mut c_void;
const FLAGS: c_int = MAP_PRIVATE | MAP_ANONYMOUS | MAP_HUGETLB;
/* mapping 8 MBs == 4 hugepages */
const LENGTH: c_ulong = 8 * 1024 * 1024;
const PROTECTION: c_int = PROT_READ | PROT_WRITE;

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: usize) -> c_int;
    fn free(ptr: *mut c_void);

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_exit_skip(fmt: *const c_char, ...) -> !;
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_finished();

    fn proc_mount_contains(option: *const c_char) -> c_int;
    fn cg_read_long(cgroup: *const c_char, control: *const c_char) -> c_long;
    fn values_close(a: c_long, b: c_long, err: c_int) -> bool;
    fn cg_name(root: *const c_char, name: *const c_char) -> *mut c_char;
    fn cg_create(cgroup: *const c_char) -> c_int;
    fn cg_write(cgroup: *const c_char, control: *const c_char, value: *const c_char) -> c_int;
    fn cg_run(
        cgroup: *const c_char,
        fn_: unsafe extern "C" fn(*const c_char, *mut c_void) -> c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn cg_destroy(cgroup: *const c_char);
    fn cg_find_unified_root(root: *mut c_char, len: usize, mount: *mut c_void) -> c_int;
    fn cg_read_strstr(cgroup: *const c_char, control: *const c_char, needle: *const c_char)
        -> c_int;
}

type c_uint = u32;

const fn MB(x: c_long) -> c_long {
    x * 1024 * 1024
}

/* borrowed from mm/hmm-tests.c */
unsafe fn get_hugepage_size() -> c_long {
    let fd: c_int;
    let mut buf = [0 as c_char; 2048];
    let len: c_int;
    let mut p: *mut c_char;
    let mut q: *mut c_char = std::ptr::null_mut();
    let path = c"/proc/meminfo".as_ptr();
    let tag = c"Hugepagesize:".as_ptr();
    let val: c_long;

    fd = open(path, O_RDONLY);
    if fd < 0 {
        /* Error opening the file */
        return -1;
    }

    len = read(fd, buf.as_mut_ptr() as *mut c_void, buf.len()) as c_int;
    close(fd);
    if len < 0 {
        /* Error in reading the file */
        return -1;
    }
    if len as usize == buf.len() {
        /* Error file is too large */
        return -1;
    }
    buf[len as usize] = b'\0' as c_char;

    /* Search for a tag if provided */
    if !tag.is_null() {
        p = strstr(buf.as_ptr(), tag);
        if p.is_null() {
            return -1; /* looks like the line we want isn't there */
        }
        p = p.add(strlen(tag));
    } else {
        p = buf.as_mut_ptr();
    }

    val = strtol(p, &mut q, 0);
    if *q != b' ' as c_char {
        /* Error parsing the file */
        return -1;
    }

    val
}

unsafe fn set_file(path: *const c_char, value: c_long) -> c_int {
    let file: *mut FILE;
    let ret: c_int;

    file = fopen(path, c"w".as_ptr());
    if file.is_null() {
        return -1;
    }
    ret = fprintf(file, c"%ld\n".as_ptr(), value);
    fclose(file);
    ret
}

unsafe fn set_nr_hugepages(value: c_long) -> c_int {
    set_file(c"/proc/sys/vm/nr_hugepages".as_ptr(), value)
}

unsafe fn check_first(addr: *mut c_char) -> c_uint {
    *(addr as *mut c_uint)
}

unsafe fn write_data(addr: *mut c_char) {
    let mut i: c_ulong;

    i = 0;
    while i < LENGTH {
        *addr.add(i as usize) = i as c_char;
        i += 1;
    }
}

unsafe extern "C" fn hugetlb_test_program(_cgroup: *const c_char, arg: *mut c_void) -> c_int {
    let test_group = arg as *mut c_char;
    let mut addr: *mut c_void;
    let mut old_current: c_long;
    let mut expected_current: c_long;
    let mut current: c_long;
    let mut ret: c_int = EXIT_FAILURE;

    old_current = cg_read_long(test_group, c"memory.current".as_ptr());
    set_nr_hugepages(20);
    current = cg_read_long(test_group, c"memory.current".as_ptr());
    if current - old_current >= MB(2) {
        ksft_print_msg(c"setting nr_hugepages should not increase hugepage usage.\n".as_ptr());
        ksft_print_msg(c"before: %ld, after: %ld\n".as_ptr(), old_current, current);
        return EXIT_FAILURE;
    }

    addr = mmap(ADDR, LENGTH as usize, PROTECTION, FLAGS, 0, 0);
    if addr == MAP_FAILED {
        ksft_print_msg(c"fail to mmap.\n".as_ptr());
        return EXIT_FAILURE;
    }
    current = cg_read_long(test_group, c"memory.current".as_ptr());
    if current - old_current >= MB(2) {
        ksft_print_msg(c"mmap should not increase hugepage usage.\n".as_ptr());
        ksft_print_msg(c"before: %ld, after: %ld\n".as_ptr(), old_current, current);
        munmap(addr, LENGTH as usize);
        return ret;
    }
    old_current = current;

    /* read the first page */
    check_first(addr as *mut c_char);
    expected_current = old_current + MB(2);
    current = cg_read_long(test_group, c"memory.current".as_ptr());
    if !values_close(expected_current, current, 5) {
        ksft_print_msg(c"memory usage should increase by around 2MB.\n".as_ptr());
        ksft_print_msg(
            c"expected memory: %ld, actual memory: %ld\n".as_ptr(),
            expected_current,
            current,
        );
        munmap(addr, LENGTH as usize);
        return ret;
    }

    /* write to the whole range */
    write_data(addr as *mut c_char);
    current = cg_read_long(test_group, c"memory.current".as_ptr());
    expected_current = old_current + MB(8);
    if !values_close(expected_current, current, 5) {
        ksft_print_msg(c"memory usage should increase by around 8MB.\n".as_ptr());
        ksft_print_msg(
            c"expected memory: %ld, actual memory: %ld\n".as_ptr(),
            expected_current,
            current,
        );
        munmap(addr, LENGTH as usize);
        return ret;
    }

    /* unmap the whole range */
    munmap(addr, LENGTH as usize);
    current = cg_read_long(test_group, c"memory.current".as_ptr());
    expected_current = old_current;
    if !values_close(expected_current, current, 5) {
        ksft_print_msg(c"memory usage should go back down.\n".as_ptr());
        ksft_print_msg(
            c"expected memory: %ld, actual memory: %ld\n".as_ptr(),
            expected_current,
            current,
        );
        return ret;
    }

    ret = EXIT_SUCCESS;
    ret
}

unsafe fn test_hugetlb_memcg(root: *mut c_char) -> c_int {
    let mut ret: c_int = KSFT_FAIL;
    let test_group: *mut c_char;

    test_group = cg_name(root, c"hugetlb_memcg_test".as_ptr());
    if test_group.is_null() || cg_create(test_group) != 0 {
        ksft_print_msg(c"fail to create cgroup.\n".as_ptr());
        cg_destroy(test_group);
        free(test_group as *mut c_void);
        return ret;
    }

    if cg_write(test_group, c"memory.max".as_ptr(), c"100M".as_ptr()) != 0 {
        ksft_print_msg(c"fail to set cgroup memory limit.\n".as_ptr());
        cg_destroy(test_group);
        free(test_group as *mut c_void);
        return ret;
    }

    /* disable swap */
    if cg_write(test_group, c"memory.swap.max".as_ptr(), c"0".as_ptr()) != 0 {
        ksft_print_msg(c"fail to disable swap.\n".as_ptr());
        cg_destroy(test_group);
        free(test_group as *mut c_void);
        return ret;
    }

    if cg_run(test_group, hugetlb_test_program, test_group as *mut c_void) == 0 {
        ret = KSFT_PASS;
    }
    cg_destroy(test_group);
    free(test_group as *mut c_void);
    ret
}

fn main() {
    unsafe {
        let mut root = [0 as c_char; PATH_MAX];
        let has_memory_hugetlb_acc: c_int;

        ksft_print_header();
        ksft_set_plan(1);

        has_memory_hugetlb_acc = proc_mount_contains(c"memory_hugetlb_accounting".as_ptr());
        if has_memory_hugetlb_acc < 0 {
            ksft_exit_skip(c"Failed to query cgroup mount option\n".as_ptr());
        } else if has_memory_hugetlb_acc == 0 {
            ksft_exit_skip(c"memory hugetlb accounting is disabled\n".as_ptr());
        }

        /* Unit is kB! */
        if get_hugepage_size() != 2048 {
            ksft_print_msg(c"test_hugetlb_memcg requires 2MB hugepages\n".as_ptr());
            ksft_test_result_skip(c"test_hugetlb_memcg\n".as_ptr());
            ksft_finished();
        }

        if cg_find_unified_root(root.as_mut_ptr(), root.len(), std::ptr::null_mut()) != 0 {
            ksft_exit_skip(c"cgroup v2 isn't mounted\n".as_ptr());
        }

        if cg_read_strstr(
            root.as_mut_ptr(),
            c"cgroup.controllers".as_ptr(),
            c"memory".as_ptr(),
        ) != 0
        {
            ksft_exit_skip(c"memory controller isn't available\n".as_ptr());
        }

        if cg_read_strstr(
            root.as_mut_ptr(),
            c"cgroup.subtree_control".as_ptr(),
            c"memory".as_ptr(),
        ) != 0
        {
            if cg_write(
                root.as_mut_ptr(),
                c"cgroup.subtree_control".as_ptr(),
                c"+memory".as_ptr(),
            ) != 0
            {
                ksft_exit_skip(c"Failed to set memory controller\n".as_ptr());
            }
        }

        match test_hugetlb_memcg(root.as_mut_ptr()) {
            KSFT_PASS => {
                ksft_test_result_pass(c"test_hugetlb_memcg\n".as_ptr());
            }
            KSFT_SKIP => {
                ksft_test_result_skip(c"test_hugetlb_memcg\n".as_ptr());
            }
            _ => {
                ksft_test_result_fail(c"test_hugetlb_memcg\n".as_ptr());
            }
        }

        ksft_finished();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
