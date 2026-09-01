// SPDX-License-Identifier: GPL-2.0
//
// Translated from C source:
// perf/util/kvm-stat-arch/kvm-stat-arm64.c
//
// Dependencies originally provided by:
// <errno.h>, <memory.h>, "../debug.h", "../evsel.h", "../kvm-stat.h",
// and "arm64_exception_types.h".

use core::ffi::{c_char, c_int};
use core::ptr;

// C macro invocations preserved as external generated tables:
// define_exit_reasons_table(arm64_exit_reasons, kvm_arm_exception_type);
// define_exit_reasons_table(arm64_trap_exit_reasons, kvm_arm_exception_class);
unsafe extern "C" {
    static arm64_exit_reasons: exit_reasons_table;
    static arm64_trap_exit_reasons: exit_reasons_table;

    fn perf_sample__intval(sample: *mut perf_sample, name: *const c_char) -> u64;
    fn kvm_exit_reason(isa: c_int) -> *const c_char;
    fn kvm_entry_trace(isa: c_int) -> *const c_char;
    fn kvm_exit_trace(isa: c_int) -> *const c_char;
    fn evsel__name_is(evsel: *mut evsel, name: *const c_char) -> bool;
    fn exit_event_decode_key(key: *mut event_key, decode: *mut c_char) -> c_int;
}

const EM_AARCH64: c_int = 183;

const ARM_EXCEPTION_TRAP: u64 = 0;

static KVM_TRAP_EXIT_REASON: &[u8] = b"esr_ec\0";

static KVM_KVM_ENTRY: &[u8] = b"kvm:kvm_entry\0";
static KVM_KVM_EXIT: &[u8] = b"kvm:kvm_exit\0";
static VM_EXIT: &[u8] = b"VM-EXIT\0";
static VMEXIT: &[u8] = b"vmexit\0";
static ARM64: &[u8] = b"arm64\0";

static __kvm_events_tp: [*const c_char; 3] = [
    KVM_KVM_ENTRY.as_ptr() as *const c_char,
    KVM_KVM_EXIT.as_ptr() as *const c_char,
    ptr::null(),
];

unsafe extern "C" fn event_get_key(sample: *mut perf_sample, key: *mut event_key) {
    unsafe {
        (*key).info = 0;
        (*key).key = perf_sample__intval(sample, kvm_exit_reason(EM_AARCH64));
        (*key).exit_reasons = &raw const arm64_exit_reasons;

        /*
         * TRAP exceptions carry exception class info in esr_ec field
         * and, hence, we need to use a different exit_reasons table to
         * properly decode event's est_ec.
         */
        if (*key).key == ARM_EXCEPTION_TRAP {
            (*key).key = perf_sample__intval(sample, KVM_TRAP_EXIT_REASON.as_ptr() as *const c_char);
            (*key).exit_reasons = &raw const arm64_trap_exit_reasons;
        }
    }
}

unsafe extern "C" fn event_begin(sample: *mut perf_sample, _key: *mut event_key) -> bool {
    unsafe { evsel__name_is((*sample).evsel, kvm_entry_trace(EM_AARCH64)) }
}

unsafe extern "C" fn event_end(sample: *mut perf_sample, key: *mut event_key) -> bool {
    unsafe {
        if evsel__name_is((*sample).evsel, kvm_exit_trace(EM_AARCH64)) {
            event_get_key(sample, key);
            return true;
        }
        false
    }
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
        ops: &raw const exit_events,
    },
    kvm_reg_events_ops {
        name: ptr::null(),
        ops: ptr::null(),
    },
];

static __kvm_skip_events: [*const c_char; 1] = [ptr::null()];

#[no_mangle]
pub unsafe extern "C" fn __cpu_isa_init_arm64(kvm: *mut perf_kvm_stat) -> c_int {
    unsafe {
        (*kvm).exit_reasons_isa = ARM64.as_ptr() as *const c_char;
    }
    0
}

#[no_mangle]
pub extern "C" fn __kvm_events_tp_arm64() -> *const *const c_char {
    __kvm_events_tp.as_ptr()
}

#[no_mangle]
pub extern "C" fn __kvm_reg_events_ops_arm64() -> *const kvm_reg_events_ops {
    __kvm_reg_events_ops.as_ptr()
}

#[no_mangle]
pub extern "C" fn __kvm_skip_events_arm64() -> *const *const c_char {
    __kvm_skip_events.as_ptr()
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
