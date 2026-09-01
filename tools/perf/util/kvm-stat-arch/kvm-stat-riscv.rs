// SPDX-License-Identifier: GPL-2.0
/*
 * Arch specific functions for perf kvm stat.
 *
 * Copyright 2024 Beijing ESWIN Computing Technology Co., Ltd.
 *
 */

use core::ffi::{c_char, c_int, c_ulonglong};

// C includes translated as external dependencies:
// <errno.h>, <memory.h>, "../evsel.h", "../kvm-stat.h",
// "riscv_trap_types.h", "debug.h"

pub const EM_RISCV: c_int = 243;

#[repr(C)]
pub struct evsel {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    pub evsel: *mut evsel,
}

#[repr(C)]
pub struct exit_reasons_table {
    pub exit_reasons: *const *const c_char,
}

#[repr(C)]
pub struct event_key {
    pub key: c_ulonglong,
    pub info: c_ulonglong,
    pub exit_reasons: *const exit_reasons_table,
}

#[repr(C)]
pub struct perf_kvm_stat {
    pub exit_reasons_isa: *const c_char,
}

#[repr(C)]
pub struct kvm_events_ops {
    pub is_begin_event:
        Option<unsafe extern "C" fn(sample: *mut perf_sample, key: *mut event_key) -> bool>,
    pub is_end_event: Option<unsafe extern "C" fn(sample: *mut perf_sample, key: *mut event_key) -> bool>,
    pub decode_key: Option<unsafe extern "C" fn(key: *mut event_key, decode: *mut c_char)>,
    pub name: *const c_char,
}

#[repr(C)]
pub struct kvm_reg_events_ops {
    pub name: *const c_char,
    pub ops: *const kvm_events_ops,
}

unsafe extern "C" {
    static riscv_exit_reasons: exit_reasons_table;

    fn perf_sample__intval(sample: *mut perf_sample, name: *const c_char) -> c_ulonglong;
    fn kvm_exit_reason(isa: c_int) -> *const c_char;
    fn kvm_entry_trace(isa: c_int) -> *const c_char;
    fn kvm_exit_trace(isa: c_int) -> *const c_char;
    fn evsel__name_is(evsel: *mut evsel, name: *const c_char) -> bool;
    fn exit_event_decode_key(key: *mut event_key, decode: *mut c_char);
}

// define_exit_reasons_table(riscv_exit_reasons, kvm_riscv_trap_class);

const KVM_KVM_ENTRY: &[u8] = b"kvm:kvm_entry\0";
const KVM_KVM_EXIT: &[u8] = b"kvm:kvm_exit\0";
const VM_EXIT: &[u8] = b"VM-EXIT\0";
const VMEXIT: &[u8] = b"vmexit\0";
const RISCV64: &[u8] = b"riscv64\0";

static __kvm_events_tp: [*const c_char; 3] = [
    KVM_KVM_ENTRY.as_ptr() as *const c_char,
    KVM_KVM_EXIT.as_ptr() as *const c_char,
    core::ptr::null(),
];

const fn CAUSE_IRQ_FLAG(xlen: c_int) -> c_ulonglong {
    1u64 << (xlen - 1)
}

unsafe extern "C" fn event_get_key(sample: *mut perf_sample, key: *mut event_key) {
    let xlen: c_int = 64; // TODO: 32-bit support.

    unsafe {
        (*key).info = 0;
        (*key).key = perf_sample__intval(sample, kvm_exit_reason(EM_RISCV)) & !CAUSE_IRQ_FLAG(xlen);
        (*key).exit_reasons = &riscv_exit_reasons;
    }
}

unsafe extern "C" fn event_begin(sample: *mut perf_sample, key: *mut event_key) -> bool {
    let _ = key;

    unsafe { evsel__name_is((*sample).evsel, kvm_entry_trace(EM_RISCV)) }
}

unsafe extern "C" fn event_end(sample: *mut perf_sample, key: *mut event_key) -> bool {
    unsafe {
        if evsel__name_is((*sample).evsel, kvm_exit_trace(EM_RISCV)) {
            event_get_key(sample, key);
            return true;
        }
    }
    false
}

static exit_events: kvm_events_ops = kvm_events_ops {
    is_begin_event: Some(event_begin),
    is_end_event: Some(event_end),
    decode_key: Some(exit_event_decode_key),
    name: VM_EXIT.as_ptr() as *const c_char,
};

static __kvm_reg_events_ops: [kvm_reg_events_ops; 2] = [
    kvm_reg_events_ops {
        name: VMEXIT.as_ptr() as *const c_char,
        ops: &exit_events,
    },
    kvm_reg_events_ops {
        name: core::ptr::null(),
        ops: core::ptr::null(),
    },
];

static __kvm_skip_events: [*const c_char; 1] = [core::ptr::null()];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __cpu_isa_init_riscv(kvm: *mut perf_kvm_stat) -> c_int {
    unsafe {
        (*kvm).exit_reasons_isa = RISCV64.as_ptr() as *const c_char;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __kvm_events_tp_riscv() -> *const *const c_char {
    __kvm_events_tp.as_ptr()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __kvm_reg_events_ops_riscv() -> *const kvm_reg_events_ops {
    __kvm_reg_events_ops.as_ptr()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __kvm_skip_events_riscv() -> *const *const c_char {
    __kvm_skip_events.as_ptr()
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
