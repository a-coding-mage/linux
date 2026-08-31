// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2020 ARM Limited

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type size_t = usize;
type uintptr_t = usize;
type pid_t = c_int;

// Dependencies from kselftest.h, mte_common_util.h, mte_def.h, and system headers.
const KSFT_PASS: c_int = 0;
const KSFT_FAIL: c_int = 1;
const MT_GRANULE_SIZE: c_int = 16;
const MT_TAG_COUNT: c_int = 16;
const MTE_ALLOW_NON_ZERO_TAG: c_int = 1;
const USE_MMAP: c_int = 0;
const USE_MPROTECT: c_int = 1;
const MTE_SYNC_ERR: c_int = 1;
const MTE_ASYNC_ERR: c_int = 2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_SHARED: c_int = 0x01;
const SIGSEGV: c_int = 11;
const SIGBUS: c_int = 7;

const BUFFER_SIZE: c_int = 5 * MT_GRANULE_SIZE;
const RUNS: c_int = MT_TAG_COUNT;
const UNDERFLOW: c_int = MT_GRANULE_SIZE;
const OVERFLOW: c_int = MT_GRANULE_SIZE;

#[repr(C)]
pub struct mte_context {
    pub fault_valid: bool,
}

unsafe extern "C" {
    static mut cur_mte_cxt: mte_context;

    fn fork() -> pid_t;
    fn _exit(status: c_int) -> !;
    fn wait(status: *mut c_int) -> pid_t;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn close(fd: c_int) -> c_int;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn getpagesize() -> c_int;

    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_print_cnts();
    fn ksft_get_fail_cnt() -> c_int;

    fn mte_default_setup() -> c_int;
    fn mte_restore_setup();
    fn mte_default_handler(signo: c_int, si: *mut c_void, uc: *mut c_void);
    fn mte_register_signal(signal: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void, *mut c_void), sa_flags: bool);
    fn mte_switch_mode(mode: c_int, mask: c_int, incl: bool);
    fn mte_initialize_current_context(mode: c_int, ptr: uintptr_t, size: c_int);
    fn mte_wait_after_trig();
    fn mte_get_tag_address(ptr: *mut c_char) -> *mut c_char;
    fn mte_allocate_memory_tag_range(
        size: c_int,
        mem_type: c_int,
        mapping: c_int,
        underflow: c_int,
        overflow: c_int,
    ) -> *mut c_void;
    fn check_allocated_memory_range(
        ptr: *mut c_char,
        size: c_int,
        mem_type: c_int,
        underflow: c_int,
        overflow: c_int,
    ) -> c_int;
    fn mte_free_memory_tag_range(ptr: *mut c_void, size: c_int, mem_type: c_int, underflow: c_int, overflow: c_int);
    fn create_temp_file() -> c_int;
    fn mte_allocate_file_memory(size: c_int, mem_type: c_int, mapping: c_int, tags: bool, fd: c_int) -> *mut c_void;
    fn check_allocated_memory(ptr: *mut c_char, size: c_int, mem_type: c_int, tags: bool) -> c_int;
    fn mte_insert_tags(ptr: *mut c_void, size: c_int) -> *mut c_char;
    fn mte_clear_tags(ptr: *mut c_void, size: c_int);
    fn evaluate_test(result: c_int, name: *const c_char);
}

static mut page_size: size_t = 0;
static mut sizes: [c_int; 9] = [
    1,
    537,
    989,
    1269,
    MT_GRANULE_SIZE - 1,
    MT_GRANULE_SIZE,
    /* page size - 1*/ 0,
    /* page_size */ 0,
    /* page size + 1 */ 0,
];

unsafe fn MT_FETCH_TAG(ptr: uintptr_t) -> c_int {
    ((ptr >> 56) & 0xf) as c_int
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status >> 8) & 0xff
}

unsafe fn check_child_tag_inheritance(ptr: *mut c_char, size: c_int, mode: c_int) -> c_int {
    let mut i: c_int;
    let parent_tag: c_int;
    let mut child_tag: c_int;
    let mut fault: c_int;
    let mut child_status: c_int = 0;
    let child: pid_t;

    parent_tag = MT_FETCH_TAG(ptr as uintptr_t);
    fault = 0;

    child = fork();
    if child == -1 {
        ksft_print_msg(c"FAIL: child process creation\n".as_ptr());
        return KSFT_FAIL;
    } else if child == 0 {
        mte_initialize_current_context(mode, ptr as uintptr_t, size);
        /* Do copy on write */
        memset(ptr as *mut c_void, '1' as c_int, size as size_t);
        mte_wait_after_trig();
        if cur_mte_cxt.fault_valid == true {
            fault = 1;
        } else {
            i = 0;
            while i < size {
                child_tag = MT_FETCH_TAG(mte_get_tag_address(ptr.offset(i as isize)) as uintptr_t);
                if parent_tag != child_tag {
                    ksft_print_msg(c"FAIL: child mte tag mismatch\n".as_ptr());
                    fault = 1;
                    break;
                }
                i += MT_GRANULE_SIZE;
            }
            if fault == 0 {
                mte_initialize_current_context(mode, ptr as uintptr_t, -UNDERFLOW);
                memset(ptr.offset(-(UNDERFLOW as isize)) as *mut c_void, '2' as c_int, UNDERFLOW as size_t);
                mte_wait_after_trig();
                if cur_mte_cxt.fault_valid == false {
                    fault = 1;
                } else {
                    mte_initialize_current_context(mode, ptr as uintptr_t, size + OVERFLOW);
                    memset(ptr.offset(size as isize) as *mut c_void, '3' as c_int, OVERFLOW as size_t);
                    mte_wait_after_trig();
                    if cur_mte_cxt.fault_valid == false {
                        fault = 1;
                    }
                }
            }
        }
        _exit(fault);
    }
    /* Wait for child process to terminate */
    wait(&mut child_status as *mut c_int);
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

unsafe fn check_child_memory_mapping(mem_type: c_int, mode: c_int, mapping: c_int) -> c_int {
    let mut ptr: *mut c_char;
    let mut run: c_int;
    let mut result: c_int;
    let mut item: c_int = sizes.len() as c_int;

    item = sizes.len() as c_int;
    mte_switch_mode(mode, MTE_ALLOW_NON_ZERO_TAG, false);
    run = 0;
    while run < item {
        ptr = mte_allocate_memory_tag_range(
            sizes[run as usize],
            mem_type,
            mapping,
            UNDERFLOW,
            OVERFLOW,
        ) as *mut c_char;
        if check_allocated_memory_range(ptr, sizes[run as usize], mem_type, UNDERFLOW, OVERFLOW) != KSFT_PASS {
            return KSFT_FAIL;
        }
        result = check_child_tag_inheritance(ptr, sizes[run as usize], mode);
        mte_free_memory_tag_range(ptr as *mut c_void, sizes[run as usize], mem_type, UNDERFLOW, OVERFLOW);
        if result == KSFT_FAIL {
            return result;
        }
        run += 1;
    }
    KSFT_PASS
}

unsafe fn check_child_file_mapping(mem_type: c_int, mode: c_int, mapping: c_int) -> c_int {
    let mut ptr: *mut c_char;
    let mut map_ptr: *mut c_char;
    let mut run: c_int;
    let mut fd: c_int;
    let mut map_size: c_int;
    let mut result: c_int = KSFT_PASS;
    let total: c_int = sizes.len() as c_int;

    mte_switch_mode(mode, MTE_ALLOW_NON_ZERO_TAG, false);
    run = 0;
    while run < total {
        fd = create_temp_file();
        if fd == -1 {
            return KSFT_FAIL;
        }

        map_size = sizes[run as usize] + OVERFLOW + UNDERFLOW;
        map_ptr = mte_allocate_file_memory(map_size, mem_type, mapping, false, fd) as *mut c_char;
        if check_allocated_memory(map_ptr, map_size, mem_type, false) != KSFT_PASS {
            close(fd);
            return KSFT_FAIL;
        }
        ptr = map_ptr.offset(UNDERFLOW as isize);
        mte_initialize_current_context(mode, ptr as uintptr_t, sizes[run as usize]);
        /* Only mte enabled memory will allow tag insertion */
        ptr = mte_insert_tags(ptr as *mut c_void, sizes[run as usize]);
        if ptr.is_null() || cur_mte_cxt.fault_valid == true {
            ksft_print_msg(c"FAIL: Insert tags on file based memory\n".as_ptr());
            munmap(map_ptr as *mut c_void, map_size as size_t);
            close(fd);
            return KSFT_FAIL;
        }
        result = check_child_tag_inheritance(ptr, sizes[run as usize], mode);
        mte_clear_tags(ptr as *mut c_void, sizes[run as usize]);
        munmap(map_ptr as *mut c_void, map_size as size_t);
        close(fd);
        if result != KSFT_PASS {
            return KSFT_FAIL;
        }
        run += 1;
    }
    KSFT_PASS
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut err: c_int;
    let item: c_int = sizes.len() as c_int;

    ksft_print_header();

    page_size = getpagesize() as size_t;
    if page_size == 0 {
        ksft_print_msg(c"ERR: Unable to get page size\n".as_ptr());
        return KSFT_FAIL;
    }
    sizes[(item - 3) as usize] = (page_size - 1) as c_int;
    sizes[(item - 2) as usize] = page_size as c_int;
    sizes[(item - 1) as usize] = (page_size + 1) as c_int;

    err = mte_default_setup();
    if err != 0 {
        return err;
    }

    /* Register SIGSEGV handler */
    mte_register_signal(SIGSEGV, mte_default_handler, false);
    mte_register_signal(SIGBUS, mte_default_handler, false);

    /* Set test plan */
    ksft_set_plan(12);

    evaluate_test(
        check_child_memory_mapping(USE_MMAP, MTE_SYNC_ERR, MAP_PRIVATE),
        c"Check child anonymous memory with private mapping, precise mode and mmap memory\n".as_ptr(),
    );
    evaluate_test(
        check_child_memory_mapping(USE_MMAP, MTE_SYNC_ERR, MAP_SHARED),
        c"Check child anonymous memory with shared mapping, precise mode and mmap memory\n".as_ptr(),
    );
    evaluate_test(
        check_child_memory_mapping(USE_MMAP, MTE_ASYNC_ERR, MAP_PRIVATE),
        c"Check child anonymous memory with private mapping, imprecise mode and mmap memory\n".as_ptr(),
    );
    evaluate_test(
        check_child_memory_mapping(USE_MMAP, MTE_ASYNC_ERR, MAP_SHARED),
        c"Check child anonymous memory with shared mapping, imprecise mode and mmap memory\n".as_ptr(),
    );
    evaluate_test(
        check_child_memory_mapping(USE_MPROTECT, MTE_SYNC_ERR, MAP_PRIVATE),
        c"Check child anonymous memory with private mapping, precise mode and mmap/mprotect memory\n".as_ptr(),
    );
    evaluate_test(
        check_child_memory_mapping(USE_MPROTECT, MTE_SYNC_ERR, MAP_SHARED),
        c"Check child anonymous memory with shared mapping, precise mode and mmap/mprotect memory\n".as_ptr(),
    );

    evaluate_test(
        check_child_file_mapping(USE_MMAP, MTE_SYNC_ERR, MAP_PRIVATE),
        c"Check child file memory with private mapping, precise mode and mmap memory\n".as_ptr(),
    );
    evaluate_test(
        check_child_file_mapping(USE_MMAP, MTE_SYNC_ERR, MAP_SHARED),
        c"Check child file memory with shared mapping, precise mode and mmap memory\n".as_ptr(),
    );
    evaluate_test(
        check_child_memory_mapping(USE_MMAP, MTE_ASYNC_ERR, MAP_PRIVATE),
        c"Check child file memory with private mapping, imprecise mode and mmap memory\n".as_ptr(),
    );
    evaluate_test(
        check_child_memory_mapping(USE_MMAP, MTE_ASYNC_ERR, MAP_SHARED),
        c"Check child file memory with shared mapping, imprecise mode and mmap memory\n".as_ptr(),
    );
    evaluate_test(
        check_child_memory_mapping(USE_MPROTECT, MTE_SYNC_ERR, MAP_PRIVATE),
        c"Check child file memory with private mapping, precise mode and mmap/mprotect memory\n".as_ptr(),
    );
    evaluate_test(
        check_child_memory_mapping(USE_MPROTECT, MTE_SYNC_ERR, MAP_SHARED),
        c"Check child file memory with shared mapping, precise mode and mmap/mprotect memory\n".as_ptr(),
    );

    mte_restore_setup();
    ksft_print_cnts();
    if ksft_get_fail_cnt() == 0 {
        KSFT_PASS
    } else {
        KSFT_FAIL
    }
}
