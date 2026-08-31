/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2020 ARM Limited */

/* Translated from mte_common_util.h. C include/header-guard syntax omitted. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type siginfo_t = c_void;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mte_mem_type {
    USE_MALLOC,
    USE_MMAP,
    USE_MPROTECT,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum mte_mode {
    MTE_NONE_ERR,
    MTE_SYNC_ERR,
    MTE_ASYNC_ERR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mte_fault_cxt {
    /* Address start which triggers mte tag fault */
    pub trig_addr: c_ulong,
    /* Address range for mte tag fault and negative value means underflow */
    pub trig_range: ssize_t,
    /* siginfo si code */
    pub trig_si_code: c_ulong,
    /* Flag to denote if correct fault caught */
    pub fault_valid: bool,
}

unsafe extern "C" {
    pub static mut cur_mte_cxt: mte_fault_cxt;
    pub static mut mtefar_support: bool;
    pub static mut mtestonly_support: bool;

    /* MTE utility functions */
    pub fn mte_default_handler(signum: c_int, si: *mut siginfo_t, uc: *mut c_void);
    pub fn mte_register_signal(
        signal: c_int,
        handler: Option<unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void)>,
        export_tags: bool,
    );
    pub fn mte_wait_after_trig();
    pub fn mte_allocate_memory(
        size: size_t,
        mem_type: c_int,
        mapping: c_int,
        tags: bool,
    ) -> *mut c_void;
    pub fn mte_allocate_memory_tag_range(
        size: size_t,
        mem_type: c_int,
        mapping: c_int,
        range_before: size_t,
        range_after: size_t,
    ) -> *mut c_void;
    pub fn mte_allocate_file_memory(
        size: size_t,
        mem_type: c_int,
        mapping: c_int,
        tags: bool,
        fd: c_int,
    ) -> *mut c_void;
    pub fn mte_allocate_file_memory_tag_range(
        size: size_t,
        mem_type: c_int,
        mapping: c_int,
        range_before: size_t,
        range_after: size_t,
        fd: c_int,
    ) -> *mut c_void;
    pub fn mte_free_memory(ptr: *mut c_void, size: size_t, mem_type: c_int, tags: bool);
    pub fn mte_free_memory_tag_range(
        ptr: *mut c_void,
        size: size_t,
        mem_type: c_int,
        range_before: size_t,
        range_after: size_t,
    );
    pub fn mte_insert_tags(ptr: *mut c_void, size: size_t) -> *mut c_void;
    pub fn mte_clear_tags(ptr: *mut c_void, size: size_t);
    pub fn mte_insert_atag(ptr: *mut c_void) -> *mut c_void;
    pub fn mte_clear_atag(ptr: *mut c_void) -> *mut c_void;
    pub fn mte_default_setup() -> c_int;
    pub fn mte_restore_setup();
    pub fn mte_switch_mode(mte_option: c_int, incl_mask: c_ulong, stonly: bool) -> c_int;
    pub fn mte_initialize_current_context(mode: c_int, ptr: uintptr_t, range: ssize_t);

    /* Common utility functions */
    pub fn create_temp_file() -> c_int;

    /* Assembly MTE utility functions */
    pub fn mte_insert_random_tag(ptr: *mut c_void) -> *mut c_void;
    pub fn mte_insert_new_tag(ptr: *mut c_void) -> *mut c_void;
    pub fn mte_get_tag_address(ptr: *mut c_void) -> *mut c_void;
    pub fn mte_set_tag_address_range(ptr: *mut c_void, range: c_int);
    pub fn mte_clear_tag_address_range(ptr: *mut c_void, range: c_int);
    pub fn mte_disable_pstate_tco();
    pub fn mte_enable_pstate_tco();
    pub fn mte_get_pstate_tco() -> c_uint;

    /* Symbols supplied by kselftest.h and mte_def.h in the original header. */
    pub static KSFT_PASS: c_int;
    pub static KSFT_FAIL: c_int;
    pub static KSFT_SKIP: c_int;
    pub fn ksft_test_result_pass(fmt: *const c_char, ...);
    pub fn ksft_test_result_fail(fmt: *const c_char, ...);
    pub fn ksft_test_result_skip(fmt: *const c_char, ...);
    pub fn ksft_test_result_error(fmt: *const c_char, ...);
    pub fn ksft_print_msg(fmt: *const c_char, ...);
    pub fn MT_FETCH_TAG(addr: uintptr_t) -> uintptr_t;
}

/* Test framework static inline functions/macros */
pub unsafe fn evaluate_test(err: c_int, msg: *const c_char) {
    if err == unsafe { KSFT_PASS } {
        unsafe {
            ksft_test_result_pass(c"%s".as_ptr(), msg);
        }
    } else if err == unsafe { KSFT_FAIL } {
        unsafe {
            ksft_test_result_fail(c"%s".as_ptr(), msg);
        }
    } else if err == unsafe { KSFT_SKIP } {
        unsafe {
            ksft_test_result_skip(c"%s".as_ptr(), msg);
        }
    } else {
        unsafe {
            ksft_test_result_error(c"Unknown return code %d from %s".as_ptr(), err, msg);
        }
    }
}

pub unsafe fn check_allocated_memory(
    ptr: *mut c_void,
    size: size_t,
    mem_type: c_int,
    tags: bool,
) -> c_int {
    if ptr.is_null() {
        unsafe {
            ksft_print_msg(c"FAIL: memory allocation\n".as_ptr());
        }
        return unsafe { KSFT_FAIL };
    }

    if tags && unsafe { MT_FETCH_TAG(ptr as uintptr_t) == 0 } {
        unsafe {
            ksft_print_msg(c"FAIL: tag not found at addr(%p)\n".as_ptr(), ptr);
            mte_free_memory(ptr, size, mem_type, false);
        }
        return unsafe { KSFT_FAIL };
    }

    unsafe { KSFT_PASS }
}

pub unsafe fn check_allocated_memory_range(
    ptr: *mut c_void,
    size: size_t,
    mem_type: c_int,
    range_before: size_t,
    range_after: size_t,
) -> c_int {
    if ptr.is_null() {
        unsafe {
            ksft_print_msg(c"FAIL: memory allocation\n".as_ptr());
        }
        return unsafe { KSFT_FAIL };
    }

    if unsafe { MT_FETCH_TAG(ptr as uintptr_t) == 0 } {
        unsafe {
            ksft_print_msg(c"FAIL: tag not found at addr(%p)\n".as_ptr(), ptr);
            mte_free_memory_tag_range(ptr, size, mem_type, range_before, range_after);
        }
        return unsafe { KSFT_FAIL };
    }
    unsafe { KSFT_PASS }
}
