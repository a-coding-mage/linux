// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2020 ARM Limited

// C source defined _GNU_SOURCE and included:
// <stddef.h>, <stdio.h>, <string.h>, "kselftest.h",
// "mte_common_util.h", and "mte_def.h".

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

const OVERFLOW_RANGE: usize = MT_GRANULE_SIZE;

static mut SIZES: [c_int; 8] = [
    1,
    555,
    1033,
    (MT_GRANULE_SIZE - 1) as c_int,
    MT_GRANULE_SIZE as c_int,
    /* page size - 1*/ 0,
    /* page_size */ 0,
    /* page size + 1 */ 0,
];

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum mte_block_test_alloc {
    UNTAGGED_TAGGED,
    TAGGED_UNTAGGED,
    TAGGED_TAGGED,
    BLOCK_ALLOC_MAX,
}

#[repr(C)]
pub struct mte_context {
    pub fault_valid: bool,
}

unsafe extern "C" {
    static mut cur_mte_cxt: mte_context;

    static MT_GRANULE_SIZE: usize;
    static KSFT_PASS: c_int;
    static KSFT_FAIL: c_int;
    static MTE_ALLOW_NON_ZERO_TAG: c_int;
    static MTE_NONE_ERR: c_int;
    static MTE_ASYNC_ERR: c_int;
    static MTE_SYNC_ERR: c_int;
    static USE_MMAP: c_int;
    static USE_MPROTECT: c_int;
    static MAP_PRIVATE: c_int;
    static MAP_SHARED: c_int;
    static SIGSEGV: c_int;

    fn mte_switch_mode(mode: c_int, tag: c_int, flag: bool);
    fn mte_allocate_memory(size: usize, mem_type: c_int, mapping: c_int, tagged: bool) -> *mut c_void;
    fn mte_allocate_memory_tag_range(
        size: usize,
        mem_type: c_int,
        mapping: c_int,
        underflow_range: c_int,
        overflow_range: c_int,
    ) -> *mut c_void;
    fn mte_allocate_file_memory(
        size: usize,
        mem_type: c_int,
        mapping: c_int,
        tagged: bool,
        fd: c_int,
    ) -> *mut c_void;
    fn check_allocated_memory(
        ptr: *mut c_char,
        size: usize,
        mem_type: c_int,
        tagged: bool,
    ) -> c_int;
    fn check_allocated_memory_range(
        ptr: *mut c_char,
        size: usize,
        mem_type: c_int,
        underflow_range: c_int,
        overflow_range: c_int,
    ) -> c_int;
    fn mte_initialize_current_context(mode: c_int, ptr: usize, size: isize);
    fn mte_wait_after_trig();
    fn mte_free_memory(ptr: *mut c_void, size: usize, mem_type: c_int, tagged: bool);
    fn mte_free_memory_tag_range(
        ptr: *mut c_void,
        size: usize,
        mem_type: c_int,
        underflow_range: c_int,
        overflow_range: c_int,
    );
    fn MT_CLEAR_TAG(ptr: usize) -> usize;
    fn MT_ALIGN_UP(size: c_int) -> usize;
    fn MT_FETCH_TAG(ptr: usize) -> c_int;
    fn mte_get_tag_address(ptr: *mut c_char) -> *mut c_char;
    fn create_temp_file() -> c_int;
    fn close(fd: c_int) -> c_int;
    fn getpagesize() -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_print_header();
    fn mte_default_setup() -> c_int;
    fn mte_register_signal(sig: c_int, handler: unsafe extern "C" fn(c_int), flag: bool);
    fn mte_default_handler(sig: c_int);
    fn ksft_set_plan(plan: c_int);
    fn evaluate_test(result: c_int, msg: *const c_char);
    fn mte_restore_setup();
    fn ksft_print_cnts();
    fn ksft_get_fail_cnt() -> c_int;
}

unsafe fn check_buffer_by_byte(mem_type: c_int, mode: c_int) -> c_int {
    let mut ptr: *mut c_char;
    let mut i: c_int;
    let mut j: c_int;
    let item: c_int;
    let mut err: bool = false;

    mte_switch_mode(mode, MTE_ALLOW_NON_ZERO_TAG, false);
    item = SIZES.len() as c_int;

    i = 0;
    while i < item {
        ptr = mte_allocate_memory(SIZES[i as usize] as usize, mem_type, 0, true) as *mut c_char;
        if check_allocated_memory(ptr, SIZES[i as usize] as usize, mem_type, true) != KSFT_PASS {
            return KSFT_FAIL;
        }
        mte_initialize_current_context(mode, ptr as usize, SIZES[i as usize] as isize);
        /* Set some value in tagged memory */
        j = 0;
        while j < SIZES[i as usize] {
            *ptr.offset(j as isize) = b'1' as c_char;
            j += 1;
        }
        mte_wait_after_trig();
        err = cur_mte_cxt.fault_valid;
        /* Check the buffer whether it is filled. */
        j = 0;
        while j < SIZES[i as usize] && !err {
            if *ptr.offset(j as isize) != b'1' as c_char {
                err = true;
            }
            j += 1;
        }
        mte_free_memory(ptr as *mut c_void, SIZES[i as usize] as usize, mem_type, true);

        if err {
            break;
        }
        i += 1;
    }
    if !err {
        KSFT_PASS
    } else {
        KSFT_FAIL
    }
}

unsafe fn check_buffer_underflow_by_byte(
    mem_type: c_int,
    mode: c_int,
    underflow_range: c_int,
) -> c_int {
    let mut ptr: *mut c_char;
    let mut i: c_int;
    let mut j: c_int;
    let item: c_int;
    let mut last_index: c_int;
    let mut err: bool = false;
    let mut und_ptr: *mut c_char = ptr::null_mut();

    mte_switch_mode(mode, MTE_ALLOW_NON_ZERO_TAG, false);
    item = SIZES.len() as c_int;
    i = 0;
    while i < item {
        ptr = mte_allocate_memory_tag_range(
            SIZES[i as usize] as usize,
            mem_type,
            0,
            underflow_range,
            0,
        ) as *mut c_char;
        if check_allocated_memory_range(
            ptr,
            SIZES[i as usize] as usize,
            mem_type,
            underflow_range,
            0,
        ) != KSFT_PASS
        {
            return KSFT_FAIL;
        }

        mte_initialize_current_context(mode, ptr as usize, -(underflow_range as isize));
        last_index = 0;
        /* Set some value in tagged memory and make the buffer underflow */
        j = SIZES[i as usize] - 1;
        while (j >= -underflow_range) && !cur_mte_cxt.fault_valid {
            *ptr.offset(j as isize) = b'1' as c_char;
            last_index = j;
            j -= 1;
        }
        mte_wait_after_trig();
        err = false;
        /* Check whether the buffer is filled */
        j = 0;
        while j < SIZES[i as usize] {
            if *ptr.offset(j as isize) != b'1' as c_char {
                err = true;
                ksft_print_msg(
                    b"Buffer is not filled at index:%d of ptr:0x%p\n\0".as_ptr() as *const c_char,
                    j,
                    ptr,
                );
                break;
            }
            j += 1;
        }
        if err {
            mte_free_memory_tag_range(
                ptr as *mut c_void,
                SIZES[i as usize] as usize,
                mem_type,
                underflow_range,
                0,
            );
            if err {
                break;
            }
            i += 1;
            continue;
        }

        if mode == MTE_NONE_ERR {
            if cur_mte_cxt.fault_valid == true || last_index != -underflow_range {
                err = true;
            } else {
                /* There were no fault so the underflow area should be filled */
                und_ptr = MT_CLEAR_TAG(ptr as usize - underflow_range as usize) as *mut c_char;
                j = 0;
                while j < underflow_range {
                    if *und_ptr.offset(j as isize) != b'1' as c_char {
                        err = true;
                        break;
                    }
                    j += 1;
                }
            }
        } else if mode == MTE_ASYNC_ERR {
            /* Imprecise fault should occur otherwise return error */
            if cur_mte_cxt.fault_valid == false {
                err = true;
            } else {
                /*
                 * The imprecise fault is checked after the write to the buffer,
                 * so the underflow area before the fault should be filled.
                 */
                und_ptr = MT_CLEAR_TAG(ptr as usize) as *mut c_char;
                j = last_index;
                while j < 0 {
                    if *und_ptr.offset(j as isize) != b'1' as c_char {
                        err = true;
                        break;
                    }
                    j += 1;
                }
            }
        } else if mode == MTE_SYNC_ERR {
            /* Precise fault should occur otherwise return error */
            if !cur_mte_cxt.fault_valid || last_index != -1 {
                err = true;
            } else {
                /* Underflow area should not be filled */
                und_ptr = MT_CLEAR_TAG(ptr as usize) as *mut c_char;
                if *und_ptr.offset(-1) == b'1' as c_char {
                    err = true;
                }
            }
        } else {
            err = true;
        }

        mte_free_memory_tag_range(
            ptr as *mut c_void,
            SIZES[i as usize] as usize,
            mem_type,
            underflow_range,
            0,
        );
        if err {
            break;
        }
        i += 1;
    }
    if err {
        KSFT_FAIL
    } else {
        KSFT_PASS
    }
}

unsafe fn check_buffer_overflow_by_byte(
    mem_type: c_int,
    mode: c_int,
    overflow_range: c_int,
) -> c_int {
    let mut ptr: *mut c_char;
    let mut i: c_int;
    let mut j: c_int;
    let item: c_int;
    let mut last_index: c_int;
    let mut err: bool = false;
    let mut tagged_size: usize;
    let mut overflow_size: usize;
    let mut over_ptr: *mut c_char = ptr::null_mut();

    mte_switch_mode(mode, MTE_ALLOW_NON_ZERO_TAG, false);
    item = SIZES.len() as c_int;
    i = 0;
    while i < item {
        ptr = mte_allocate_memory_tag_range(
            SIZES[i as usize] as usize,
            mem_type,
            0,
            0,
            overflow_range,
        ) as *mut c_char;
        if check_allocated_memory_range(
            ptr,
            SIZES[i as usize] as usize,
            mem_type,
            0,
            overflow_range,
        ) != KSFT_PASS
        {
            return KSFT_FAIL;
        }

        tagged_size = MT_ALIGN_UP(SIZES[i as usize]);

        mte_initialize_current_context(
            mode,
            ptr as usize,
            (SIZES[i as usize] + overflow_range) as isize,
        );

        /* Set some value in tagged memory and make the buffer underflow */
        j = 0;
        last_index = 0;
        while (j < SIZES[i as usize] + overflow_range) && (cur_mte_cxt.fault_valid == false) {
            *ptr.offset(j as isize) = b'1' as c_char;
            last_index = j;
            j += 1;
        }
        mte_wait_after_trig();
        err = false;
        /* Check whether the buffer is filled */
        j = 0;
        while j < SIZES[i as usize] {
            if *ptr.offset(j as isize) != b'1' as c_char {
                err = true;
                ksft_print_msg(
                    b"Buffer is not filled at index:%d of ptr:0x%p\n\0".as_ptr() as *const c_char,
                    j,
                    ptr,
                );
                break;
            }
            j += 1;
        }
        if err {
            mte_free_memory_tag_range(
                ptr as *mut c_void,
                SIZES[i as usize] as usize,
                mem_type,
                0,
                overflow_range,
            );
            if err {
                break;
            }
            i += 1;
            continue;
        }

        overflow_size = overflow_range as usize - (tagged_size - SIZES[i as usize] as usize);

        if mode == MTE_NONE_ERR {
            if (cur_mte_cxt.fault_valid == true)
                || (last_index != (SIZES[i as usize] + overflow_range - 1))
            {
                err = true;
            } else {
                /* There were no fault so the overflow area should be filled */
                over_ptr = MT_CLEAR_TAG(ptr as usize + tagged_size) as *mut c_char;
                j = 0;
                while (j as usize) < overflow_size {
                    if *over_ptr.offset(j as isize) != b'1' as c_char {
                        err = true;
                        break;
                    }
                    j += 1;
                }
            }
        } else if mode == MTE_ASYNC_ERR {
            /* Imprecise fault should occur otherwise return error */
            if cur_mte_cxt.fault_valid == false {
                err = true;
            } else {
                /*
                 * The imprecise fault is checked after the write to the buffer,
                 * so the overflow area should be filled before the fault.
                 */
                over_ptr = MT_CLEAR_TAG(ptr as usize) as *mut c_char;
                j = tagged_size as c_int;
                while j < last_index {
                    if *over_ptr.offset(j as isize) != b'1' as c_char {
                        err = true;
                        break;
                    }
                    j += 1;
                }
            }
        } else if mode == MTE_SYNC_ERR {
            /* Precise fault should occur otherwise return error */
            if !cur_mte_cxt.fault_valid || last_index != tagged_size as c_int {
                err = true;
            } else {
                /* Underflow area should not be filled */
                over_ptr = MT_CLEAR_TAG(ptr as usize + tagged_size) as *mut c_char;
                j = 0;
                while (j as usize) < overflow_size {
                    if *over_ptr.offset(j as isize) == b'1' as c_char {
                        err = true;
                    }
                    j += 1;
                }
            }
        } else {
            err = true;
        }

        mte_free_memory_tag_range(
            ptr as *mut c_void,
            SIZES[i as usize] as usize,
            mem_type,
            0,
            overflow_range,
        );
        if err {
            break;
        }
        i += 1;
    }
    if err {
        KSFT_FAIL
    } else {
        KSFT_PASS
    }
}

unsafe fn check_buffer_by_block_iterate(mem_type: c_int, mode: c_int, size: usize) -> c_int {
    let mut src: *mut c_char;
    let mut dst: *mut c_char;
    let mut j: usize;
    let mut result: c_int = KSFT_PASS;
    let mut alloc_type: c_int = mte_block_test_alloc::UNTAGGED_TAGGED as c_int;

    while alloc_type < mte_block_test_alloc::BLOCK_ALLOC_MAX as c_int {
        if alloc_type == mte_block_test_alloc::UNTAGGED_TAGGED as c_int {
            src = mte_allocate_memory(size, mem_type, 0, false) as *mut c_char;
            if check_allocated_memory(src, size, mem_type, false) != KSFT_PASS {
                return KSFT_FAIL;
            }

            dst = mte_allocate_memory(size, mem_type, 0, true) as *mut c_char;
            if check_allocated_memory(dst, size, mem_type, true) != KSFT_PASS {
                mte_free_memory(src as *mut c_void, size, mem_type, false);
                return KSFT_FAIL;
            }
        } else if alloc_type == mte_block_test_alloc::TAGGED_UNTAGGED as c_int {
            dst = mte_allocate_memory(size, mem_type, 0, false) as *mut c_char;
            if check_allocated_memory(dst, size, mem_type, false) != KSFT_PASS {
                return KSFT_FAIL;
            }

            src = mte_allocate_memory(size, mem_type, 0, true) as *mut c_char;
            if check_allocated_memory(src, size, mem_type, true) != KSFT_PASS {
                mte_free_memory(dst as *mut c_void, size, mem_type, false);
                return KSFT_FAIL;
            }
        } else if alloc_type == mte_block_test_alloc::TAGGED_TAGGED as c_int {
            src = mte_allocate_memory(size, mem_type, 0, true) as *mut c_char;
            if check_allocated_memory(src, size, mem_type, true) != KSFT_PASS {
                return KSFT_FAIL;
            }

            dst = mte_allocate_memory(size, mem_type, 0, true) as *mut c_char;
            if check_allocated_memory(dst, size, mem_type, true) != KSFT_PASS {
                mte_free_memory(src as *mut c_void, size, mem_type, true);
                return KSFT_FAIL;
            }
        } else {
            return KSFT_FAIL;
        }

        cur_mte_cxt.fault_valid = false;
        result = KSFT_PASS;
        mte_initialize_current_context(mode, dst as usize, size as isize);
        /* Set some value in memory and copy*/
        memset(src as *mut c_void, b'1' as c_int, size);
        memcpy(dst as *mut c_void, src as *mut c_void, size);
        mte_wait_after_trig();
        if cur_mte_cxt.fault_valid {
            result = KSFT_FAIL;
        } else {
            /* Check the buffer whether it is filled. */
            j = 0;
            while j < size {
                if *src.add(j) != *dst.add(j) || *src.add(j) != b'1' as c_char {
                    result = KSFT_FAIL;
                    break;
                }
                j += 1;
            }
        }
        mte_free_memory(
            src as *mut c_void,
            size,
            mem_type,
            if MT_FETCH_TAG(src as usize) != 0 { true } else { false },
        );
        mte_free_memory(
            dst as *mut c_void,
            size,
            mem_type,
            if MT_FETCH_TAG(dst as usize) != 0 { true } else { false },
        );
        if result != KSFT_PASS {
            return result;
        }
        alloc_type += 1;
    }
    result
}

unsafe fn check_buffer_by_block(mem_type: c_int, mode: c_int) -> c_int {
    let mut i: c_int;
    let item: c_int;
    let mut result: c_int = KSFT_PASS;

    mte_switch_mode(mode, MTE_ALLOW_NON_ZERO_TAG, false);
    item = SIZES.len() as c_int;
    cur_mte_cxt.fault_valid = false;
    i = 0;
    while i < item {
        result = check_buffer_by_block_iterate(mem_type, mode, SIZES[i as usize] as usize);
        if result != KSFT_PASS {
            break;
        }
        i += 1;
    }
    result
}

unsafe fn compare_memory_tags(ptr: *mut c_char, size: usize, tag: c_int) -> c_int {
    let mut i: usize;
    let mut new_tag: c_int;

    i = 0;
    while i < size {
        new_tag = MT_FETCH_TAG(mte_get_tag_address(ptr.add(i)) as usize);
        if tag != new_tag {
            ksft_print_msg(b"FAIL: child mte tag mismatch\n\0".as_ptr() as *const c_char);
            return KSFT_FAIL;
        }
        i += MT_GRANULE_SIZE;
    }
    KSFT_PASS
}

unsafe fn check_memory_initial_tags(mem_type: c_int, mode: c_int, mapping: c_int) -> c_int {
    let mut ptr: *mut c_char;
    let mut run: c_int;
    let mut fd: c_int;
    let total: c_int = SIZES.len() as c_int;

    mte_switch_mode(mode, MTE_ALLOW_NON_ZERO_TAG, false);
    run = 0;
    while run < total {
        /* check initial tags for anonymous mmap */
        ptr = mte_allocate_memory(SIZES[run as usize] as usize, mem_type, mapping, false)
            as *mut c_char;
        if check_allocated_memory(ptr, SIZES[run as usize] as usize, mem_type, false) != KSFT_PASS {
            return KSFT_FAIL;
        }
        if compare_memory_tags(ptr, SIZES[run as usize] as usize, 0) != KSFT_PASS {
            mte_free_memory(
                ptr as *mut c_void,
                SIZES[run as usize] as usize,
                mem_type,
                false,
            );
            return KSFT_FAIL;
        }
        mte_free_memory(
            ptr as *mut c_void,
            SIZES[run as usize] as usize,
            mem_type,
            false,
        );

        /* check initial tags for file mmap */
        fd = create_temp_file();
        if fd == -1 {
            return KSFT_FAIL;
        }
        ptr = mte_allocate_file_memory(
            SIZES[run as usize] as usize,
            mem_type,
            mapping,
            false,
            fd,
        ) as *mut c_char;
        if check_allocated_memory(ptr, SIZES[run as usize] as usize, mem_type, false) != KSFT_PASS {
            close(fd);
            return KSFT_FAIL;
        }
        if compare_memory_tags(ptr, SIZES[run as usize] as usize, 0) != KSFT_PASS {
            mte_free_memory(
                ptr as *mut c_void,
                SIZES[run as usize] as usize,
                mem_type,
                false,
            );
            close(fd);
            return KSFT_FAIL;
        }
        mte_free_memory(
            ptr as *mut c_void,
            SIZES[run as usize] as usize,
            mem_type,
            false,
        );
        close(fd);
        run += 1;
    }
    KSFT_PASS
}

#[no_mangle]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut err: c_int;
    let page_size: usize = getpagesize() as usize;
    let item: c_int = SIZES.len() as c_int;

    ksft_print_header();

    SIZES[(item - 3) as usize] = (page_size - 1) as c_int;
    SIZES[(item - 2) as usize] = page_size as c_int;
    SIZES[(item - 1) as usize] = (page_size + 1) as c_int;

    err = mte_default_setup();
    if err != 0 {
        return err;
    }

    /* Register SIGSEGV handler */
    mte_register_signal(SIGSEGV, mte_default_handler, false);

    /* Set test plan */
    ksft_set_plan(20);

    /* Buffer by byte tests */
    evaluate_test(
        check_buffer_by_byte(USE_MMAP, MTE_SYNC_ERR),
        b"Check buffer correctness by byte with sync err mode and mmap memory\n\0".as_ptr()
            as *const c_char,
    );
    evaluate_test(
        check_buffer_by_byte(USE_MMAP, MTE_ASYNC_ERR),
        b"Check buffer correctness by byte with async err mode and mmap memory\n\0".as_ptr()
            as *const c_char,
    );
    evaluate_test(
        check_buffer_by_byte(USE_MPROTECT, MTE_SYNC_ERR),
        b"Check buffer correctness by byte with sync err mode and mmap/mprotect memory\n\0".as_ptr()
            as *const c_char,
    );
    evaluate_test(
        check_buffer_by_byte(USE_MPROTECT, MTE_ASYNC_ERR),
        b"Check buffer correctness by byte with async err mode and mmap/mprotect memory\n\0".as_ptr()
            as *const c_char,
    );

    /* Check buffer underflow with underflow size as 16 */
    evaluate_test(
        check_buffer_underflow_by_byte(USE_MMAP, MTE_SYNC_ERR, MT_GRANULE_SIZE as c_int),
        b"Check buffer write underflow by byte with sync mode and mmap memory\n\0".as_ptr()
            as *const c_char,
    );
    evaluate_test(
        check_buffer_underflow_by_byte(USE_MMAP, MTE_ASYNC_ERR, MT_GRANULE_SIZE as c_int),
        b"Check buffer write underflow by byte with async mode and mmap memory\n\0".as_ptr()
            as *const c_char,
    );
    evaluate_test(
        check_buffer_underflow_by_byte(USE_MMAP, MTE_NONE_ERR, MT_GRANULE_SIZE as c_int),
        b"Check buffer write underflow by byte with tag check fault ignore and mmap memory\n\0"
            .as_ptr() as *const c_char,
    );

    /* Check buffer underflow with underflow size as page size */
    evaluate_test(
        check_buffer_underflow_by_byte(USE_MMAP, MTE_SYNC_ERR, page_size as c_int),
        b"Check buffer write underflow by byte with sync mode and mmap memory\n\0".as_ptr()
            as *const c_char,
    );
    evaluate_test(
        check_buffer_underflow_by_byte(USE_MMAP, MTE_ASYNC_ERR, page_size as c_int),
        b"Check buffer write underflow by byte with async mode and mmap memory\n\0".as_ptr()
            as *const c_char,
    );
    evaluate_test(
        check_buffer_underflow_by_byte(USE_MMAP, MTE_NONE_ERR, page_size as c_int),
        b"Check buffer write underflow by byte with tag check fault ignore and mmap memory\n\0"
            .as_ptr() as *const c_char,
    );

    /* Check buffer overflow with overflow size as 16 */
    evaluate_test(
        check_buffer_overflow_by_byte(USE_MMAP, MTE_SYNC_ERR, MT_GRANULE_SIZE as c_int),
        b"Check buffer write overflow by byte with sync mode and mmap memory\n\0".as_ptr()
            as *const c_char,
    );
    evaluate_test(
        check_buffer_overflow_by_byte(USE_MMAP, MTE_ASYNC_ERR, MT_GRANULE_SIZE as c_int),
        b"Check buffer write overflow by byte with async mode and mmap memory\n\0".as_ptr()
            as *const c_char,
    );
    evaluate_test(
        check_buffer_overflow_by_byte(USE_MMAP, MTE_NONE_ERR, MT_GRANULE_SIZE as c_int),
        b"Check buffer write overflow by byte with tag fault ignore mode and mmap memory\n\0"
            .as_ptr() as *const c_char,
    );

    /* Buffer by block tests */
    evaluate_test(
        check_buffer_by_block(USE_MMAP, MTE_SYNC_ERR),
        b"Check buffer write correctness by block with sync mode and mmap memory\n\0".as_ptr()
            as *const c_char,
    );
    evaluate_test(
        check_buffer_by_block(USE_MMAP, MTE_ASYNC_ERR),
        b"Check buffer write correctness by block with async mode and mmap memory\n\0".as_ptr()
            as *const c_char,
    );
    evaluate_test(
        check_buffer_by_block(USE_MMAP, MTE_NONE_ERR),
        b"Check buffer write correctness by block with tag fault ignore and mmap memory\n\0"
            .as_ptr() as *const c_char,
    );

    /* Initial tags are supposed to be 0 */
    evaluate_test(
        check_memory_initial_tags(USE_MMAP, MTE_SYNC_ERR, MAP_PRIVATE),
        b"Check initial tags with private mapping, sync error mode and mmap memory\n\0".as_ptr()
            as *const c_char,
    );
    evaluate_test(
        check_memory_initial_tags(USE_MPROTECT, MTE_SYNC_ERR, MAP_PRIVATE),
        b"Check initial tags with private mapping, sync error mode and mmap/mprotect memory\n\0"
            .as_ptr() as *const c_char,
    );
    evaluate_test(
        check_memory_initial_tags(USE_MMAP, MTE_SYNC_ERR, MAP_SHARED),
        b"Check initial tags with shared mapping, sync error mode and mmap memory\n\0".as_ptr()
            as *const c_char,
    );
    evaluate_test(
        check_memory_initial_tags(USE_MPROTECT, MTE_SYNC_ERR, MAP_SHARED),
        b"Check initial tags with shared mapping, sync error mode and mmap/mprotect memory\n\0"
            .as_ptr() as *const c_char,
    );

    mte_restore_setup();
    ksft_print_cnts();
    if ksft_get_fail_cnt() == 0 {
        KSFT_PASS
    } else {
        KSFT_FAIL
    }
}
