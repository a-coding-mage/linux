// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2024 Ampere Computing LLC

// C dependencies: errno.h, fcntl.h, signal.h, stdio.h, stdlib.h, string.h,
// ucontext.h, sys/mman.h, sys/stat.h, sys/types.h, sys/wait.h.
// Local dependencies: kselftest.h, mte_common_util.h, mte_def.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use libc::{
    c_char, c_int, c_long, c_uint, c_ulong, c_void, pid_t, size_t, FILE, MAP_ANONYMOUS, MAP_FAILED,
    MAP_HUGETLB, MAP_PRIVATE, PROT_READ, PROT_WRITE, SIGBUS, SIGSEGV,
};
use std::ptr;

const TAG_CHECK_ON: c_int = 0;
const TAG_CHECK_OFF: c_int = 1;

const PROT_MTE: c_int = 0x20;

#[repr(C)]
pub struct mte_context {
    pub fault_valid: bool,
}

unsafe extern "C" {
    static mut cur_mte_cxt: mte_context;

    static KSFT_PASS: c_int;
    static KSFT_FAIL: c_int;
    static MTE_ALLOW_NON_ZERO_TAG: c_int;
    static USE_MMAP: c_int;
    static USE_MPROTECT: c_int;
    static MTE_SYNC_ERR: c_int;
    static MTE_NONE_ERR: c_int;
    static MTE_ASYNC_ERR: c_int;
    static MT_GRANULE_SIZE: c_int;

    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> isize;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn free(ptr: *mut c_void);
    fn fork() -> pid_t;
    fn _exit(status: c_int) -> !;
    fn wait(wstatus: *mut c_int) -> pid_t;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: c_long,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn mprotect(addr: *mut c_void, len: size_t, prot: c_int) -> c_int;

    fn ksft_print_header();
    fn ksft_print_msg(msg: *const c_char, ...);
    fn ksft_exit_skip(msg: *const c_char, ...) -> !;
    fn ksft_set_plan(plan: c_uint);
    fn ksft_print_cnts();
    fn ksft_get_fail_cnt() -> c_int;

    fn mte_default_setup() -> c_int;
    fn mte_register_signal(signum: c_int, handler: unsafe extern "C" fn(c_int), is_sa_siginfo: bool);
    fn mte_default_handler(signum: c_int);
    fn mte_enable_pstate_tco();
    fn mte_disable_pstate_tco();
    fn mte_restore_setup();
    fn mte_switch_mode(mode: c_int, mask: c_int, include_all: bool);
    fn mte_allocate_memory(
        size: size_t,
        mem_type: c_int,
        mapping: c_int,
        zero: bool,
    ) -> *mut c_void;
    fn mte_allocate_memory_tag_range(
        size: size_t,
        mem_type: c_int,
        mapping: c_int,
        tag_start: c_int,
        tag_end: c_int,
    ) -> *mut c_void;
    fn mte_free_memory(ptr: *mut c_void, size: size_t, mem_type: c_int, zero: bool);
    fn mte_free_memory_tag_range(
        ptr: *mut c_void,
        size: size_t,
        mem_type: c_int,
        tag_start: c_int,
        tag_end: c_int,
    );
    fn check_allocated_memory(
        ptr: *mut c_char,
        size: size_t,
        mem_type: c_int,
        zero: bool,
    ) -> c_int;
    fn check_allocated_memory_range(
        ptr: *mut c_char,
        size: size_t,
        mem_type: c_int,
        tag_start: c_int,
        tag_end: c_int,
    ) -> c_int;
    fn mte_initialize_current_context(mode: c_int, ptr: usize, size: size_t);
    fn mte_wait_after_trig();
    fn mte_insert_tags(ptr: *mut c_void, size: size_t) -> *mut c_char;
    fn mte_clear_tags(ptr: *mut c_void, size: size_t);
    fn mte_get_tag_address(ptr: *mut c_char) -> *mut c_void;
    fn MT_FETCH_TAG(addr: usize) -> c_int;
    fn evaluate_test(result: c_int, msg: *const c_char);
}

unsafe fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn default_huge_page_size() -> c_ulong {
    let mut hps: c_ulong = 0;
    let mut line: *mut c_char = ptr::null_mut();
    let mut linelen: size_t = 0;
    let f = fopen(c"/proc/meminfo".as_ptr(), c"r".as_ptr());

    if f.is_null() {
        return 0;
    }
    while getline(&mut line, &mut linelen, f) > 0 {
        if sscanf(
            line,
            c"Hugepagesize:       %lu kB".as_ptr(),
            &mut hps as *mut c_ulong,
        ) == 1
        {
            hps <<= 10;
            break;
        }
    }

    free(line as *mut c_void);
    fclose(f);
    hps
}

unsafe fn is_hugetlb_allocated() -> bool {
    let mut hps: c_ulong = 0;
    let mut line: *mut c_char = ptr::null_mut();
    let mut linelen: size_t = 0;
    let f = fopen(c"/proc/meminfo".as_ptr(), c"r".as_ptr());

    if f.is_null() {
        return false;
    }
    while getline(&mut line, &mut linelen, f) > 0 {
        if sscanf(
            line,
            c"Hugetlb:       %lu kB".as_ptr(),
            &mut hps as *mut c_ulong,
        ) == 1
        {
            hps <<= 10;
            break;
        }
    }

    free(line as *mut c_void);
    fclose(f);

    if hps > 0 {
        return true;
    }

    false
}

unsafe fn write_sysfs(str_: *mut c_char, val: c_ulong) {
    let mut f: *mut FILE;

    f = fopen(str_, c"w".as_ptr());
    if f.is_null() {
        ksft_print_msg(c"ERR: missing %s\n".as_ptr(), str_);
        return;
    }
    fprintf(f, c"%lu".as_ptr(), val);
    fclose(f);
}

unsafe fn allocate_hugetlb() {
    write_sysfs(c"/proc/sys/vm/nr_hugepages".as_ptr() as *mut c_char, 2);
}

unsafe fn free_hugetlb() {
    write_sysfs(c"/proc/sys/vm/nr_hugepages".as_ptr() as *mut c_char, 0);
}

unsafe fn check_child_tag_inheritance(ptr: *mut c_char, size: c_int, mode: c_int) -> c_int {
    let mut i: c_int;
    let parent_tag: c_int;
    let mut child_tag: c_int;
    let mut fault: c_int;
    let mut child_status: c_int = 0;
    let child: pid_t;

    parent_tag = MT_FETCH_TAG(ptr as usize);
    fault = 0;

    child = fork();
    if child == -1 {
        ksft_print_msg(c"FAIL: child process creation\n".as_ptr());
        return KSFT_FAIL;
    } else if child == 0 {
        mte_initialize_current_context(mode, ptr as usize, size as size_t);
        /* Do copy on write */
        memset(ptr as *mut c_void, '1' as c_int, size as size_t);
        mte_wait_after_trig();
        if cur_mte_cxt.fault_valid == true {
            fault = 1;
        } else {
            i = 0;
            while i < size {
                child_tag = MT_FETCH_TAG(mte_get_tag_address(ptr.offset(i as isize)) as usize);
                if parent_tag != child_tag {
                    ksft_print_msg(c"FAIL: child mte tag (%d) mismatch\n".as_ptr(), i);
                    fault = 1;
                    break;
                }
                i += MT_GRANULE_SIZE;
            }
        }
        _exit(fault);
    }
    /* Wait for child process to terminate */
    wait(&mut child_status);
    if WIFEXITED(child_status) {
        fault = WEXITSTATUS(child_status);
    } else {
        fault = 1;
    }
    if fault != 0 {
        KSFT_FAIL
    } else {
        KSFT_PASS
    }
}

unsafe fn check_mte_memory(
    ptr: *mut c_char,
    size: c_int,
    mode: c_int,
    _tag_check: c_int,
) -> c_int {
    mte_initialize_current_context(mode, ptr as usize, size as size_t);
    memset(ptr as *mut c_void, '1' as c_int, size as size_t);
    mte_wait_after_trig();
    if cur_mte_cxt.fault_valid == true {
        return KSFT_FAIL;
    }

    KSFT_PASS
}

unsafe fn check_hugetlb_memory_mapping(
    mem_type: c_int,
    mode: c_int,
    mapping: c_int,
    tag_check: c_int,
) -> c_int {
    let mut ptr: *mut c_char;
    let map_ptr: *mut c_char;
    let result: c_int;
    let map_size: c_ulong;

    map_size = default_huge_page_size();

    mte_switch_mode(mode, MTE_ALLOW_NON_ZERO_TAG, false);
    map_ptr = mte_allocate_memory(map_size as size_t, mem_type, mapping, false) as *mut c_char;
    if check_allocated_memory(map_ptr, map_size as size_t, mem_type, false) != KSFT_PASS {
        return KSFT_FAIL;
    }

    mte_initialize_current_context(mode, map_ptr as usize, map_size as size_t);
    /* Only mte enabled memory will allow tag insertion */
    ptr = mte_insert_tags(map_ptr as *mut c_void, map_size as size_t);
    if ptr.is_null() || cur_mte_cxt.fault_valid == true {
        ksft_print_msg(c"FAIL: Insert tags on anonymous mmap memory\n".as_ptr());
        munmap(map_ptr as *mut c_void, map_size as size_t);
        return KSFT_FAIL;
    }
    result = check_mte_memory(ptr, map_size as c_int, mode, tag_check);
    mte_clear_tags(ptr as *mut c_void, map_size as size_t);
    mte_free_memory(map_ptr as *mut c_void, map_size as size_t, mem_type, false);
    if result == KSFT_FAIL {
        return KSFT_FAIL;
    }

    KSFT_PASS
}

unsafe fn check_clear_prot_mte_flag(mem_type: c_int, mode: c_int, mapping: c_int) -> c_int {
    let map_ptr: *mut c_char;
    let prot_flag: c_int;
    let result: c_int;
    let map_size: c_ulong;

    prot_flag = PROT_READ | PROT_WRITE;
    mte_switch_mode(mode, MTE_ALLOW_NON_ZERO_TAG, false);
    map_size = default_huge_page_size();
    map_ptr = mte_allocate_memory_tag_range(map_size as size_t, mem_type, mapping, 0, 0)
        as *mut c_char;
    if check_allocated_memory_range(map_ptr, map_size as size_t, mem_type, 0, 0) != KSFT_PASS {
        return KSFT_FAIL;
    }
    /* Try to clear PROT_MTE property and verify it by tag checking */
    if mprotect(map_ptr as *mut c_void, map_size as size_t, prot_flag) != 0 {
        mte_free_memory_tag_range(map_ptr as *mut c_void, map_size as size_t, mem_type, 0, 0);
        ksft_print_msg(c"FAIL: mprotect not ignoring clear PROT_MTE property\n".as_ptr());
        return KSFT_FAIL;
    }
    result = check_mte_memory(map_ptr, map_size as c_int, mode, TAG_CHECK_ON);
    mte_free_memory_tag_range(map_ptr as *mut c_void, map_size as size_t, mem_type, 0, 0);
    if result != KSFT_PASS {
        return KSFT_FAIL;
    }

    KSFT_PASS
}

unsafe fn check_child_hugetlb_memory_mapping(
    mem_type: c_int,
    mode: c_int,
    mapping: c_int,
) -> c_int {
    let ptr: *mut c_char;
    let result: c_int;
    let map_size: c_ulong;

    map_size = default_huge_page_size();

    mte_switch_mode(mode, MTE_ALLOW_NON_ZERO_TAG, false);
    ptr = mte_allocate_memory_tag_range(map_size as size_t, mem_type, mapping, 0, 0) as *mut c_char;
    if check_allocated_memory_range(ptr, map_size as size_t, mem_type, 0, 0) != KSFT_PASS {
        return KSFT_FAIL;
    }
    result = check_child_tag_inheritance(ptr, map_size as c_int, mode);
    mte_free_memory_tag_range(ptr as *mut c_void, map_size as size_t, mem_type, 0, 0);
    if result == KSFT_FAIL {
        return result;
    }

    KSFT_PASS
}

unsafe fn main_impl(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut err: c_int;
    let mut map_ptr: *mut c_void;
    let mut map_size: c_ulong;

    ksft_print_header();

    err = mte_default_setup();
    if err != 0 {
        return err;
    }

    /* Register signal handlers */
    mte_register_signal(SIGBUS, mte_default_handler, false);
    mte_register_signal(SIGSEGV, mte_default_handler, false);

    allocate_hugetlb();

    if !is_hugetlb_allocated() {
        ksft_print_msg(c"ERR: Unable allocate hugetlb pages\n".as_ptr());
        return KSFT_FAIL;
    }

    /* Check if MTE supports hugetlb mappings */
    map_size = default_huge_page_size();
    map_ptr = mmap(
        ptr::null_mut(),
        map_size as size_t,
        PROT_READ | PROT_MTE,
        MAP_PRIVATE | MAP_ANONYMOUS | MAP_HUGETLB,
        -1,
        0,
    );
    if map_ptr == MAP_FAILED {
        ksft_exit_skip(c"PROT_MTE not supported with MAP_HUGETLB mappings\n".as_ptr());
    } else {
        munmap(map_ptr, map_size as size_t);
    }

    /* Set test plan */
    ksft_set_plan(12);

    mte_enable_pstate_tco();

    evaluate_test(
        check_hugetlb_memory_mapping(
            USE_MMAP,
            MTE_SYNC_ERR,
            MAP_PRIVATE | MAP_HUGETLB,
            TAG_CHECK_OFF,
        ),
        c"Check hugetlb memory with private mapping, sync error mode, mmap memory and tag check off\n"
            .as_ptr(),
    );

    mte_disable_pstate_tco();
    evaluate_test(
        check_hugetlb_memory_mapping(
            USE_MMAP,
            MTE_NONE_ERR,
            MAP_PRIVATE | MAP_HUGETLB,
            TAG_CHECK_OFF,
        ),
        c"Check hugetlb memory with private mapping, no error mode, mmap memory and tag check off\n"
            .as_ptr(),
    );

    evaluate_test(
        check_hugetlb_memory_mapping(
            USE_MMAP,
            MTE_SYNC_ERR,
            MAP_PRIVATE | MAP_HUGETLB,
            TAG_CHECK_ON,
        ),
        c"Check hugetlb memory with private mapping, sync error mode, mmap memory and tag check on\n"
            .as_ptr(),
    );
    evaluate_test(
        check_hugetlb_memory_mapping(
            USE_MPROTECT,
            MTE_SYNC_ERR,
            MAP_PRIVATE | MAP_HUGETLB,
            TAG_CHECK_ON,
        ),
        c"Check hugetlb memory with private mapping, sync error mode, mmap/mprotect memory and tag check on\n"
            .as_ptr(),
    );
    evaluate_test(
        check_hugetlb_memory_mapping(
            USE_MMAP,
            MTE_ASYNC_ERR,
            MAP_PRIVATE | MAP_HUGETLB,
            TAG_CHECK_ON,
        ),
        c"Check hugetlb memory with private mapping, async error mode, mmap memory and tag check on\n"
            .as_ptr(),
    );
    evaluate_test(
        check_hugetlb_memory_mapping(
            USE_MPROTECT,
            MTE_ASYNC_ERR,
            MAP_PRIVATE | MAP_HUGETLB,
            TAG_CHECK_ON,
        ),
        c"Check hugetlb memory with private mapping, async error mode, mmap/mprotect memory and tag check on\n"
            .as_ptr(),
    );

    evaluate_test(
        check_clear_prot_mte_flag(USE_MMAP, MTE_SYNC_ERR, MAP_PRIVATE | MAP_HUGETLB),
        c"Check clear PROT_MTE flags with private mapping, sync error mode and mmap memory\n"
            .as_ptr(),
    );
    evaluate_test(
        check_clear_prot_mte_flag(USE_MPROTECT, MTE_SYNC_ERR, MAP_PRIVATE | MAP_HUGETLB),
        c"Check clear PROT_MTE flags with private mapping and sync error mode and mmap/mprotect memory\n"
            .as_ptr(),
    );

    evaluate_test(
        check_child_hugetlb_memory_mapping(USE_MMAP, MTE_SYNC_ERR, MAP_PRIVATE | MAP_HUGETLB),
        c"Check child hugetlb memory with private mapping, sync error mode and mmap memory\n".as_ptr(),
    );
    evaluate_test(
        check_child_hugetlb_memory_mapping(USE_MMAP, MTE_ASYNC_ERR, MAP_PRIVATE | MAP_HUGETLB),
        c"Check child hugetlb memory with private mapping, async error mode and mmap memory\n".as_ptr(),
    );
    evaluate_test(
        check_child_hugetlb_memory_mapping(USE_MPROTECT, MTE_SYNC_ERR, MAP_PRIVATE | MAP_HUGETLB),
        c"Check child hugetlb memory with private mapping, sync error mode and mmap/mprotect memory\n"
            .as_ptr(),
    );
    evaluate_test(
        check_child_hugetlb_memory_mapping(USE_MPROTECT, MTE_ASYNC_ERR, MAP_PRIVATE | MAP_HUGETLB),
        c"Check child hugetlb memory with private mapping, async error mode and mmap/mprotect memory\n"
            .as_ptr(),
    );

    mte_restore_setup();
    free_hugetlb();
    ksft_print_cnts();
    if ksft_get_fail_cnt() == 0 {
        KSFT_PASS
    } else {
        KSFT_FAIL
    }
}

fn main() {
    let code = unsafe { main_impl(0, ptr::null_mut()) };
    std::process::exit(code);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
