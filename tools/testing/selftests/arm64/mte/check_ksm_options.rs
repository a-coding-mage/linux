// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2020 ARM Limited

// C source included errno/fcntl/signal/stdbool/stdio/stdlib/string/ucontext/sys/mman
// plus kselftest.h, mte_common_util.h, and mte_def.h.

use core::ffi::{c_char, c_int, c_ulong, c_void};

const TEST_UNIT: usize = 10;
const PATH_KSM: &[u8] = b"/sys/kernel/mm/ksm/\0";
const MAX_LOOP: c_int = 4;

const R_OK: c_int = 4;
const W_OK: c_int = 2;
const F_OK: c_int = 0;
const ENOENT: c_int = 2;
const SIGBUS: c_int = 7;
const SIGSEGV: c_int = 11;
const MAP_PRIVATE: c_int = 0x02;
const MAP_SHARED: c_int = 0x01;
const MADV_MERGEABLE: c_int = 12;

const KSFT_PASS: c_int = 0;
const KSFT_FAIL: c_int = 1;

const USE_MMAP: c_int = 0;
const MTE_SYNC_ERR: c_int = 1;
const MTE_ASYNC_ERR: c_int = 2;
const MTE_ALLOW_NON_ZERO_TAG: c_int = 0;

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

static mut page_sz: usize = 0;
static mut ksm_sysfs: [c_ulong; 5] = [0; 5];
static mut has_merge_across_nodes: bool = false;

unsafe extern "C" {
    static mut errno: c_int;

    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn madvise(addr: *mut c_void, length: usize, advice: c_int) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn geteuid() -> c_uint;
    fn getpagesize() -> c_int;

    fn ksft_exit_skip(msg: *const c_char, ...) -> !;
    fn ksft_print_msg(msg: *const c_char, ...);
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_print_cnts();
    fn ksft_get_fail_cnt() -> c_int;

    fn mte_default_setup() -> c_int;
    fn mte_restore_setup();
    fn mte_switch_mode(mode: c_int, mask: c_int, include_all: bool);
    fn mte_allocate_memory(
        size: usize,
        mem_type: c_int,
        mapping: c_int,
        tags: bool,
    ) -> *mut c_char;
    fn check_allocated_memory(
        ptr: *mut c_char,
        size: usize,
        mem_type: c_int,
        tags: bool,
    ) -> c_int;
    fn mte_free_memory(ptr: *mut c_char, size: usize, mem_type: c_int, tags: bool);
    fn mte_register_signal(signal: c_int, handler: extern "C" fn(c_int, *mut c_void, *mut c_void), sa_flags: bool);
    fn mte_default_handler(signal: c_int, si: *mut c_void, uc: *mut c_void);
    fn evaluate_test(result: c_int, name: *const c_char);
}

type c_uint = u32;

unsafe fn ksm_path(suffix: &[u8]) -> Vec<u8> {
    let mut path = Vec::with_capacity(PATH_KSM.len() + suffix.len());
    path.extend_from_slice(&PATH_KSM[..PATH_KSM.len() - 1]);
    path.extend_from_slice(suffix);
    path
}

unsafe fn merge_across_nodes_available() -> bool {
    let path = ksm_path(b"merge_across_nodes\0");

    if access(path.as_ptr() as *const c_char, R_OK | W_OK) == 0 {
        return true;
    }
    if errno == ENOENT {
        return false;
    }

    ksft_exit_skip(
        b"Unable to read and write %s: %s\n\0".as_ptr() as *const c_char,
        path.as_ptr() as *const c_char,
        strerror(errno),
    );
}

unsafe fn read_sysfs(str_: *mut c_char) -> c_ulong {
    let mut f: *mut FILE;
    let mut val: c_ulong = 0;

    f = fopen(str_, b"r\0".as_ptr() as *const c_char);
    if f.is_null() {
        ksft_print_msg(
            b"ERR: missing %s\n\0".as_ptr() as *const c_char,
            str_,
        );
        return 0;
    }
    if fscanf(
        f,
        b"%lu\0".as_ptr() as *const c_char,
        &mut val as *mut c_ulong,
    ) != 1
    {
        ksft_print_msg(
            b"ERR: parsing %s\n\0".as_ptr() as *const c_char,
            str_,
        );
        val = 0;
    }
    fclose(f);
    return val;
}

unsafe fn write_sysfs(str_: *mut c_char, val: c_ulong) {
    let mut f: *mut FILE;

    f = fopen(str_, b"w\0".as_ptr() as *const c_char);
    if f.is_null() {
        ksft_print_msg(
            b"ERR: missing %s\n\0".as_ptr() as *const c_char,
            str_,
        );
        return;
    }
    fprintf(f, b"%lu\0".as_ptr() as *const c_char, val);
    fclose(f);
}

unsafe fn mte_ksm_setup() {
    if has_merge_across_nodes {
        let path = ksm_path(b"merge_across_nodes\0");
        ksm_sysfs[0] = read_sysfs(path.as_ptr() as *mut c_char);
        write_sysfs(path.as_ptr() as *mut c_char, 1);
    }
    let path = ksm_path(b"sleep_millisecs\0");
    ksm_sysfs[1] = read_sysfs(path.as_ptr() as *mut c_char);
    write_sysfs(path.as_ptr() as *mut c_char, 0);
    let path = ksm_path(b"run\0");
    ksm_sysfs[2] = read_sysfs(path.as_ptr() as *mut c_char);
    write_sysfs(path.as_ptr() as *mut c_char, 1);
    let path = ksm_path(b"max_page_sharing\0");
    ksm_sysfs[3] = read_sysfs(path.as_ptr() as *mut c_char);
    write_sysfs(path.as_ptr() as *mut c_char, ksm_sysfs[3] + TEST_UNIT as c_ulong);
    let path = ksm_path(b"pages_to_scan\0");
    ksm_sysfs[4] = read_sysfs(path.as_ptr() as *mut c_char);
    write_sysfs(path.as_ptr() as *mut c_char, ksm_sysfs[4] + TEST_UNIT as c_ulong);
}

unsafe fn mte_ksm_restore() {
    if has_merge_across_nodes {
        let path = ksm_path(b"merge_across_nodes\0");
        write_sysfs(path.as_ptr() as *mut c_char, ksm_sysfs[0]);
    }
    let path = ksm_path(b"sleep_millisecs\0");
    write_sysfs(path.as_ptr() as *mut c_char, ksm_sysfs[1]);
    let path = ksm_path(b"run\0");
    write_sysfs(path.as_ptr() as *mut c_char, ksm_sysfs[2]);
    let path = ksm_path(b"max_page_sharing\0");
    write_sysfs(path.as_ptr() as *mut c_char, ksm_sysfs[3]);
    let path = ksm_path(b"pages_to_scan\0");
    write_sysfs(path.as_ptr() as *mut c_char, ksm_sysfs[4]);
}

unsafe fn mte_ksm_scan() {
    let path = ksm_path(b"full_scans\0");
    let mut cur_count: c_int = read_sysfs(path.as_ptr() as *mut c_char) as c_int;
    let scan_count: c_int = cur_count + 1;
    let mut max_loop_count: c_int = MAX_LOOP;

    while cur_count < scan_count && max_loop_count != 0 {
        sleep(1);
        cur_count = read_sysfs(path.as_ptr() as *mut c_char) as c_int;
        max_loop_count -= 1;
    }
    // Original C prints DEBUG-only pages_shared/pages_sharing diagnostics here.
}

unsafe fn check_madvise_options(mem_type: c_int, mode: c_int, mapping: c_int) -> c_int {
    let mut ptr: *mut c_char;
    let mut err: c_int;
    let ret: c_int;

    err = KSFT_FAIL;
    if access(PATH_KSM.as_ptr() as *const c_char, F_OK) == -1 {
        ksft_print_msg(b"ERR: Kernel KSM config not enabled\n\0".as_ptr() as *const c_char);
        return err;
    }

    mte_switch_mode(mode, MTE_ALLOW_NON_ZERO_TAG, false);
    ptr = mte_allocate_memory(TEST_UNIT * page_sz, mem_type, mapping, true);
    if check_allocated_memory(ptr, TEST_UNIT * page_sz, mem_type, false) != KSFT_PASS {
        return KSFT_FAIL;
    }

    /* Insert same data in all the pages */
    memset(ptr as *mut c_void, b'A' as c_int, TEST_UNIT * page_sz);
    ret = madvise(ptr as *mut c_void, TEST_UNIT * page_sz, MADV_MERGEABLE);
    if ret != 0 {
        ksft_print_msg(b"ERR: madvise failed to set MADV_UNMERGEABLE\n\0".as_ptr() as *const c_char);
    } else {
        mte_ksm_scan();
        /* Tagged pages should not merge */
        let pages_shared = ksm_path(b"pages_shared\0");
        let pages_sharing = ksm_path(b"pages_sharing\0");
        if read_sysfs(pages_shared.as_ptr() as *mut c_char) < 1
            || read_sysfs(pages_sharing.as_ptr() as *mut c_char) < (TEST_UNIT - 1) as c_ulong
        {
            err = KSFT_PASS;
        }
    }
    mte_free_memory(ptr, TEST_UNIT * page_sz, mem_type, true);
    return err;
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut err: c_int;

    ksft_print_header();

    err = mte_default_setup();
    if err != 0 {
        return err;
    }

    if geteuid() != 0 {
        ksft_exit_skip(b"Please run the test as root\n\0".as_ptr() as *const c_char);
    }

    has_merge_across_nodes = merge_across_nodes_available();
    page_sz = getpagesize() as usize;
    if page_sz == 0 {
        ksft_print_msg(b"ERR: Unable to get page size\n\0".as_ptr() as *const c_char);
        return KSFT_FAIL;
    }
    /* Register signal handlers */
    mte_register_signal(SIGBUS, mte_default_handler, false);
    mte_register_signal(SIGSEGV, mte_default_handler, false);

    /* Set test plan */
    ksft_set_plan(4);

    /* Enable KSM */
    mte_ksm_setup();

    evaluate_test(
        check_madvise_options(USE_MMAP, MTE_SYNC_ERR, MAP_PRIVATE),
        b"Check KSM mte page merge for private mapping, sync mode and mmap memory\n\0".as_ptr()
            as *const c_char,
    );
    evaluate_test(
        check_madvise_options(USE_MMAP, MTE_ASYNC_ERR, MAP_PRIVATE),
        b"Check KSM mte page merge for private mapping, async mode and mmap memory\n\0".as_ptr()
            as *const c_char,
    );
    evaluate_test(
        check_madvise_options(USE_MMAP, MTE_SYNC_ERR, MAP_SHARED),
        b"Check KSM mte page merge for shared mapping, sync mode and mmap memory\n\0".as_ptr()
            as *const c_char,
    );
    evaluate_test(
        check_madvise_options(USE_MMAP, MTE_ASYNC_ERR, MAP_SHARED),
        b"Check KSM mte page merge for shared mapping, async mode and mmap memory\n\0".as_ptr()
            as *const c_char,
    );

    mte_ksm_restore();
    mte_restore_setup();
    ksft_print_cnts();
    return if ksft_get_fail_cnt() == 0 {
        KSFT_PASS
    } else {
        KSFT_FAIL
    };
}
