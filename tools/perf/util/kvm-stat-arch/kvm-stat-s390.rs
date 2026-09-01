// SPDX-License-Identifier: GPL-2.0-only
/*
 * Arch specific functions for perf kvm stat.
 *
 * Copyright 2014 IBM Corp.
 * Author(s): Alexander Yarygin <yarygin@linux.vnet.ibm.com>
 */

// C dependencies:
// errno.h
// string.h
// ../kvm-stat.h
// ../evsel.h
// ../../../arch/s390/include/uapi/asm/sie.h

use core::ffi::{c_char, c_int, c_void};

const ENOTSUP: c_int = 524;

#[repr(C)]
pub struct perf_sample {
    _private: [u8; 0],
}

#[repr(C)]
pub struct exit_reasons_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct event_key {
    pub key: u64,
    pub exit_reasons: *const exit_reasons_table,
}

#[repr(C)]
pub struct child_event_ops {
    pub name: *const c_char,
    pub get_key: Option<unsafe extern "C" fn(*mut perf_sample, *mut event_key)>,
}

#[repr(C)]
pub struct kvm_events_ops {
    pub is_begin_event: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> bool>,
    pub is_end_event: Option<unsafe extern "C" fn(*mut c_void, *mut c_void) -> bool>,
    pub child_ops: *const child_event_ops,
    pub decode_key: Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>,
    pub name: *const c_char,
}

#[repr(C)]
pub struct kvm_reg_events_ops {
    pub name: *const c_char,
    pub ops: *const kvm_events_ops,
}

#[repr(C)]
pub struct perf_kvm_stat {
    pub exit_reasons: *const exit_reasons_table,
    pub exit_reasons_isa: *const c_char,
}

unsafe extern "C" {
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;

    fn perf_sample__intval(sample: *mut perf_sample, name: *const c_char) -> u64;
    fn icpt_insn_decoder(insn: u64) -> u64;

    fn exit_event_begin(arg1: *mut c_void, arg2: *mut c_void) -> bool;
    fn exit_event_end(arg1: *mut c_void, arg2: *mut c_void) -> bool;
    fn exit_event_decode_key(arg1: *mut c_void, arg2: *mut c_void);

    // Generated in C by define_exit_reasons_table(...).
    static sie_exit_reasons: exit_reasons_table;
    static sie_icpt_insn_codes: exit_reasons_table;
    static sie_sigp_order_codes: exit_reasons_table;
    static sie_diagnose_codes: exit_reasons_table;
    static sie_icpt_prog_codes: exit_reasons_table;
}

unsafe extern "C" fn event_icpt_insn_get_key(sample: *mut perf_sample, key: *mut event_key) {
    let insn: u64;

    unsafe {
        insn = perf_sample__intval(sample, c"instruction".as_ptr());
        (*key).key = icpt_insn_decoder(insn);
        (*key).exit_reasons = &raw const sie_icpt_insn_codes;
    }
}

unsafe extern "C" fn event_sigp_get_key(sample: *mut perf_sample, key: *mut event_key) {
    unsafe {
        (*key).key = perf_sample__intval(sample, c"order_code".as_ptr());
        (*key).exit_reasons = &raw const sie_sigp_order_codes;
    }
}

unsafe extern "C" fn event_diag_get_key(sample: *mut perf_sample, key: *mut event_key) {
    unsafe {
        (*key).key = perf_sample__intval(sample, c"code".as_ptr());
        (*key).exit_reasons = &raw const sie_diagnose_codes;
    }
}

unsafe extern "C" fn event_icpt_prog_get_key(sample: *mut perf_sample, key: *mut event_key) {
    unsafe {
        (*key).key = perf_sample__intval(sample, c"code".as_ptr());
        (*key).exit_reasons = &raw const sie_icpt_prog_codes;
    }
}

static child_events: [child_event_ops; 5] = [
    child_event_ops {
        name: c"kvm:kvm_s390_intercept_instruction".as_ptr(),
        get_key: Some(event_icpt_insn_get_key),
    },
    child_event_ops {
        name: c"kvm:kvm_s390_handle_sigp".as_ptr(),
        get_key: Some(event_sigp_get_key),
    },
    child_event_ops {
        name: c"kvm:kvm_s390_handle_diag".as_ptr(),
        get_key: Some(event_diag_get_key),
    },
    child_event_ops {
        name: c"kvm:kvm_s390_intercept_prog".as_ptr(),
        get_key: Some(event_icpt_prog_get_key),
    },
    child_event_ops {
        name: core::ptr::null(),
        get_key: None,
    },
];

static exit_events: kvm_events_ops = kvm_events_ops {
    is_begin_event: Some(exit_event_begin),
    is_end_event: Some(exit_event_end),
    child_ops: child_events.as_ptr(),
    decode_key: Some(exit_event_decode_key),
    name: c"VM-EXIT".as_ptr(),
};

static __kvm_events_tp: [*const c_char; 7] = [
    c"kvm:kvm_s390_sie_enter".as_ptr(),
    c"kvm:kvm_s390_sie_exit".as_ptr(),
    c"kvm:kvm_s390_intercept_instruction".as_ptr(),
    c"kvm:kvm_s390_handle_sigp".as_ptr(),
    c"kvm:kvm_s390_handle_diag".as_ptr(),
    c"kvm:kvm_s390_intercept_prog".as_ptr(),
    core::ptr::null(),
];

static __kvm_reg_events_ops: [kvm_reg_events_ops; 2] = [
    kvm_reg_events_ops {
        name: c"vmexit".as_ptr(),
        ops: &raw const exit_events,
    },
    kvm_reg_events_ops {
        name: core::ptr::null(),
        ops: core::ptr::null(),
    },
];

static __kvm_skip_events: [*const c_char; 2] = [
    c"Wait state".as_ptr(),
    core::ptr::null(),
];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __cpu_isa_init_s390(
    kvm: *mut perf_kvm_stat,
    cpuid: *const c_char,
) -> c_int {
    unsafe {
        if !strstr(cpuid, c"IBM".as_ptr()).is_null() {
            (*kvm).exit_reasons = &raw const sie_exit_reasons;
            (*kvm).exit_reasons_isa = c"SIE".as_ptr();
        } else {
            return -ENOTSUP;
        }
    }

    0
}

#[unsafe(no_mangle)]
pub extern "C" fn __kvm_events_tp_s390() -> *const *const c_char {
    __kvm_events_tp.as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn __kvm_reg_events_ops_s390() -> *const kvm_reg_events_ops {
    __kvm_reg_events_ops.as_ptr()
}

#[unsafe(no_mangle)]
pub extern "C" fn __kvm_skip_events_s390() -> *const *const c_char {
    __kvm_skip_events.as_ptr()
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
