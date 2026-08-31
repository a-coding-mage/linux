// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2020 ARM Limited

// C dependencies removed from executable Rust:
// errno.h, signal.h, stdio.h, stdlib.h, string.h, ucontext.h, sys/wait.h
// kselftest.h, mte_common_util.h, mte_def.h

use core::ffi::{c_char, c_int, c_ulong, c_void};

const BUFFER_SIZE: usize = 5 * MT_GRANULE_SIZE;
const RUNS: c_int = (MT_TAG_COUNT * 2) as c_int;
const MTE_LAST_TAG_MASK: c_int = 0x7FFF;

const SIGSEGV: c_int = 11;

#[repr(C)]
pub struct mte_context {
    pub fault_valid: bool,
}

extern "C" {
    static mut cur_mte_cxt: mte_context;

    static KSFT_PASS: c_int;
    static KSFT_FAIL: c_int;
    static MT_GRANULE_SIZE: usize;
    static MT_TAG_COUNT: usize;
    static MT_INCLUDE_TAG_MASK: c_ulong;
    static MT_EXCLUDE_TAG_MASK: c_ulong;
    static USE_MMAP: c_int;
    static MTE_SYNC_ERR: c_int;

    static mte_default_handler: extern "C" fn(c_int, *mut c_void, *mut c_void);

    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;

    fn ksft_print_header();
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_set_plan(plan: c_int);
    fn ksft_print_cnts();
    fn ksft_get_fail_cnt() -> c_int;

    fn mte_default_setup() -> c_int;
    fn mte_restore_setup();
    fn mte_register_signal(
        signal: c_int,
        handler: extern "C" fn(c_int, *mut c_void, *mut c_void),
        sa_flags: bool,
    );
    fn mte_initialize_current_context(mode: c_int, ptr: usize, size: usize);
    fn mte_wait_after_trig();
    fn mte_allocate_memory(size: usize, mem_type: c_int, mapping: c_int, tags: bool) -> *mut c_char;
    fn check_allocated_memory(ptr: *mut c_char, size: usize, mem_type: c_int, tags: bool) -> c_int;
    fn mte_insert_tags(ptr: *mut c_char, size: usize) -> *mut c_char;
    fn mte_free_memory_tag_range(
        ptr: *mut c_char,
        size: usize,
        mem_type: c_int,
        mapping: c_int,
        range: usize,
    );
    fn mte_free_memory(ptr: *mut c_void, size: usize, mem_type: c_int, tags: bool);
    fn mte_switch_mode(mode: c_int, mask: c_ulong, enable: bool) -> c_int;
    fn evaluate_test(result: c_int, name: *const c_char);

    fn MT_FETCH_TAG(ptr: usize) -> c_ulong;
    fn MT_INCLUDE_VALID_TAG(tag: c_int) -> c_ulong;
    fn MT_INCLUDE_VALID_TAGS(mask: c_ulong) -> c_ulong;
}

unsafe fn verify_mte_pointer_validity(ptr: *mut c_char, mode: c_int) -> c_int {
    mte_initialize_current_context(mode, ptr as usize, BUFFER_SIZE);
    /* Check the validity of the tagged pointer */
    memset(ptr as *mut c_void, b'1' as c_int, BUFFER_SIZE);
    mte_wait_after_trig();
    if cur_mte_cxt.fault_valid {
        ksft_print_msg(
            b"Unexpected fault recorded for %p-%p in mode %x\n\0".as_ptr() as *const c_char,
            ptr,
            ptr.add(BUFFER_SIZE),
            mode,
        );
        return KSFT_FAIL;
    }
    /* Proceed further for nonzero tags */
    if MT_FETCH_TAG(ptr as usize) == 0 {
        return KSFT_PASS;
    }
    mte_initialize_current_context(mode, ptr as usize, BUFFER_SIZE + 1);
    /* Check the validity outside the range */
    *ptr.add(BUFFER_SIZE) = b'2' as c_char;
    mte_wait_after_trig();
    if !cur_mte_cxt.fault_valid {
        ksft_print_msg(
            b"No valid fault recorded for %p in mode %x\n\0".as_ptr() as *const c_char,
            ptr,
            mode,
        );
        return KSFT_FAIL;
    } else {
        return KSFT_PASS;
    }
}

unsafe fn check_single_included_tags(mem_type: c_int, mode: c_int) -> c_int {
    let mut ptr: *mut c_char;
    let mut tag: c_int;
    let mut run: c_int;
    let mut ret: c_int;
    let mut result: c_int = KSFT_PASS;

    ptr = mte_allocate_memory(BUFFER_SIZE + MT_GRANULE_SIZE, mem_type, 0, false);
    if check_allocated_memory(ptr, BUFFER_SIZE + MT_GRANULE_SIZE, mem_type, false) != KSFT_PASS {
        return KSFT_FAIL;
    }

    tag = 0;
    while tag < MT_TAG_COUNT as c_int && result == KSFT_PASS {
        ret = mte_switch_mode(mode, MT_INCLUDE_VALID_TAG(tag), false);
        if ret != 0 {
            result = KSFT_FAIL;
        }
        /* Try to catch a excluded tag by a number of tries. */
        run = 0;
        while run < RUNS && result == KSFT_PASS {
            ptr = mte_insert_tags(ptr, BUFFER_SIZE);
            /* Check tag value */
            if MT_FETCH_TAG(ptr as usize) == tag as c_ulong {
                ksft_print_msg(
                    b"FAIL: wrong tag = 0x%lx with include mask=0x%x\n\0".as_ptr()
                        as *const c_char,
                    MT_FETCH_TAG(ptr as usize),
                    MT_INCLUDE_VALID_TAG(tag),
                );
                result = KSFT_FAIL;
                break;
            }
            result = verify_mte_pointer_validity(ptr, mode);
            run += 1;
        }
        tag += 1;
    }
    mte_free_memory_tag_range(ptr, BUFFER_SIZE, mem_type, 0, MT_GRANULE_SIZE);
    return result;
}

unsafe fn check_multiple_included_tags(mem_type: c_int, mode: c_int) -> c_int {
    let mut ptr: *mut c_char;
    let mut tag: c_int;
    let mut run: c_int;
    let mut result: c_int = KSFT_PASS;
    let mut excl_mask: c_ulong = 0;

    ptr = mte_allocate_memory(BUFFER_SIZE + MT_GRANULE_SIZE, mem_type, 0, false);
    if check_allocated_memory(ptr, BUFFER_SIZE + MT_GRANULE_SIZE, mem_type, false) != KSFT_PASS {
        return KSFT_FAIL;
    }

    tag = 0;
    while tag < (MT_TAG_COUNT - 1) as c_int && result == KSFT_PASS {
        excl_mask |= (1 as c_ulong) << tag;
        mte_switch_mode(mode, MT_INCLUDE_VALID_TAGS(excl_mask), false);
        /* Try to catch a excluded tag by a number of tries. */
        run = 0;
        while run < RUNS && result == KSFT_PASS {
            ptr = mte_insert_tags(ptr, BUFFER_SIZE);
            /* Check tag value */
            if MT_FETCH_TAG(ptr as usize) < tag as c_ulong {
                ksft_print_msg(
                    b"FAIL: wrong tag = 0x%lx with include mask=0x%lx\n\0".as_ptr()
                        as *const c_char,
                    MT_FETCH_TAG(ptr as usize),
                    MT_INCLUDE_VALID_TAGS(excl_mask),
                );
                result = KSFT_FAIL;
                break;
            }
            result = verify_mte_pointer_validity(ptr, mode);
            run += 1;
        }
        tag += 1;
    }
    mte_free_memory_tag_range(ptr, BUFFER_SIZE, mem_type, 0, MT_GRANULE_SIZE);
    return result;
}

unsafe fn check_all_included_tags(mem_type: c_int, mode: c_int) -> c_int {
    let mut ptr: *mut c_char;
    let mut run: c_int;
    let mut ret: c_int;
    let mut result: c_int = KSFT_PASS;

    ptr = mte_allocate_memory(BUFFER_SIZE + MT_GRANULE_SIZE, mem_type, 0, false);
    if check_allocated_memory(ptr, BUFFER_SIZE + MT_GRANULE_SIZE, mem_type, false) != KSFT_PASS {
        return KSFT_FAIL;
    }

    ret = mte_switch_mode(mode, MT_INCLUDE_TAG_MASK, false);
    if ret != 0 {
        return KSFT_FAIL;
    }
    /* Try to catch a excluded tag by a number of tries. */
    run = 0;
    while run < RUNS && result == KSFT_PASS {
        ptr = mte_insert_tags(ptr, BUFFER_SIZE) as *mut c_char;
        /*
         * Here tag byte can be between 0x0 to 0xF (full allowed range)
         * so no need to match so just verify if it is writable.
         */
        result = verify_mte_pointer_validity(ptr, mode);
        run += 1;
    }
    mte_free_memory_tag_range(ptr, BUFFER_SIZE, mem_type, 0, MT_GRANULE_SIZE);
    return result;
}

unsafe fn check_none_included_tags(mem_type: c_int, mode: c_int) -> c_int {
    let mut ptr: *mut c_char;
    let mut run: c_int;
    let mut ret: c_int;

    ptr = mte_allocate_memory(BUFFER_SIZE, mem_type, 0, false);
    if check_allocated_memory(ptr, BUFFER_SIZE, mem_type, false) != KSFT_PASS {
        return KSFT_FAIL;
    }

    ret = mte_switch_mode(mode, MT_EXCLUDE_TAG_MASK, false);
    if ret != 0 {
        return KSFT_FAIL;
    }
    /* Try to catch a excluded tag by a number of tries. */
    run = 0;
    while run < RUNS {
        ptr = mte_insert_tags(ptr, BUFFER_SIZE) as *mut c_char;
        /* Here all tags exluded so tag value generated should be 0 */
        if MT_FETCH_TAG(ptr as usize) != 0 {
            ksft_print_msg(b"FAIL: included tag value found\n\0".as_ptr() as *const c_char);
            mte_free_memory(ptr as *mut c_void, BUFFER_SIZE, mem_type, true);
            return KSFT_FAIL;
        }
        mte_initialize_current_context(mode, ptr as usize, BUFFER_SIZE);
        /* Check the write validity of the untagged pointer */
        memset(ptr as *mut c_void, b'1' as c_int, BUFFER_SIZE);
        mte_wait_after_trig();
        if cur_mte_cxt.fault_valid {
            break;
        }
        run += 1;
    }
    mte_free_memory(ptr as *mut c_void, BUFFER_SIZE, mem_type, false);
    if cur_mte_cxt.fault_valid {
        return KSFT_FAIL;
    } else {
        return KSFT_PASS;
    }
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut err: c_int;

    let _ = argc;
    let _ = argv;

    ksft_print_header();

    err = mte_default_setup();
    if err != 0 {
        return err;
    }

    /* Register SIGSEGV handler */
    mte_register_signal(SIGSEGV, mte_default_handler, false);

    /* Set test plan */
    ksft_set_plan(4);

    evaluate_test(
        check_single_included_tags(USE_MMAP, MTE_SYNC_ERR),
        b"Check an included tag value with sync mode\n\0".as_ptr() as *const c_char,
    );
    evaluate_test(
        check_multiple_included_tags(USE_MMAP, MTE_SYNC_ERR),
        b"Check different included tags value with sync mode\n\0".as_ptr() as *const c_char,
    );
    evaluate_test(
        check_none_included_tags(USE_MMAP, MTE_SYNC_ERR),
        b"Check none included tags value with sync mode\n\0".as_ptr() as *const c_char,
    );
    evaluate_test(
        check_all_included_tags(USE_MMAP, MTE_SYNC_ERR),
        b"Check all included tags value with sync mode\n\0".as_ptr() as *const c_char,
    );

    mte_restore_setup();
    ksft_print_cnts();
    return if ksft_get_fail_cnt() == 0 {
        KSFT_PASS
    } else {
        KSFT_FAIL
    };
}
