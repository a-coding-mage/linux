/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from perf/util/kvm-stat.h. */

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type u64 = u64;
pub type uint16_t = u16;
pub type bool_ = bool;

pub const KVM_EVENT_NAME_LEN: usize = 40;
pub const INVALID_KEY: u64 = !0u64;
pub const DEFAULT_VCPU_NUM: c_int = 8;

/* Types supplied by included perf/linux headers. */
#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_session {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_tool {
    _private: [u8; 0],
}

#[repr(C)]
pub struct record_opts {
    _private: [u8; 0],
}

#[repr(C)]
pub struct addr_location {
    _private: [u8; 0],
}

#[repr(C)]
pub struct intlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stats {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hist_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct refcount_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct event_key {
    pub key: u64,
    pub info: c_int,
    pub exit_reasons: *mut exit_reasons_table,
}

#[repr(C)]
pub struct kvm_info {
    pub name: [c_char; KVM_EVENT_NAME_LEN],
    pub refcnt: refcount_t,
}

#[repr(C)]
pub struct kvm_event_stats {
    pub time: u64,
    pub stats: stats,
}

#[repr(C)]
pub struct kvm_event {
    pub hash_entry: list_head,
    pub perf_kvm: *mut perf_kvm_stat,
    pub key: event_key,
    pub total: kvm_event_stats,
    pub max_vcpu: c_int,
    pub vcpu: *mut kvm_event_stats,
    pub he: hist_entry,
}

#[repr(C)]
pub struct child_event_ops {
    pub get_key: Option<unsafe extern "C" fn(sample: *mut perf_sample, key: *mut event_key)>,
    pub name: *const c_char,
}

#[repr(C)]
pub struct kvm_events_ops {
    pub is_begin_event:
        Option<unsafe extern "C" fn(sample: *mut perf_sample, key: *mut event_key) -> bool>,
    pub is_end_event:
        Option<unsafe extern "C" fn(sample: *mut perf_sample, key: *mut event_key) -> bool>,
    pub child_ops: *const child_event_ops,
    pub decode_key:
        Option<unsafe extern "C" fn(kvm: *mut perf_kvm_stat, key: *mut event_key, decode: *mut c_char)>,
    pub name: *const c_char,
}

#[repr(C)]
pub struct exit_reasons_table {
    pub exit_code: u64,
    pub reason: *const c_char,
}

#[repr(C)]
pub struct perf_kvm_stat {
    pub tool: perf_tool,
    pub opts: record_opts,
    pub evlist: *mut evlist,
    pub session: *mut perf_session,
    pub file_name: *const c_char,
    pub report_event: *const c_char,
    pub sort_key: *const c_char,
    pub trace_vcpu: c_int,

    /* Used when process events */
    pub al: addr_location,

    pub exit_reasons: *mut exit_reasons_table,
    pub exit_reasons_isa: *const c_char,
    pub events_ops: *const kvm_events_ops,
    pub total_time: u64,
    pub total_count: u64,
    pub lost_events: u64,
    pub duration: u64,
    pub pid_list: *mut intlist,
    pub timerfd: c_int,
    pub display_time: c_uint,
    pub live: bool,
    pub force: bool,
    pub use_stdio: bool,
}

#[repr(C)]
pub struct kvm_reg_events_ops {
    pub name: *const c_char,
    pub ops: *const kvm_events_ops,
}

unsafe extern "C" {
    pub fn refcount_inc(r: *mut refcount_t);
    pub fn refcount_dec_and_test(r: *mut refcount_t) -> bool;
    pub fn refcount_set(r: *mut refcount_t, n: c_int);
    pub fn free(ptr: *mut c_void);
    pub fn zalloc(size: usize) -> *mut c_void;
}

/* HAVE_LIBTRACEEVENT declarations. */
unsafe extern "C" {
    pub fn exit_event_get_key(sample: *mut perf_sample, key: *mut event_key);
    pub fn exit_event_begin(sample: *mut perf_sample, key: *mut event_key) -> bool;
    pub fn exit_event_end(sample: *mut perf_sample, key: *mut event_key) -> bool;
    pub fn exit_event_decode_key(kvm: *mut perf_kvm_stat, key: *mut event_key, decode: *mut c_char);

    pub fn kvm_exit_event(evsel: *mut evsel) -> bool;
    pub fn kvm_entry_event(evsel: *mut evsel) -> bool;

    /*
     * arch specific callbacks and data structures
     */
    pub fn setup_kvm_events_tp(kvm: *mut perf_kvm_stat, e_machine: uint16_t) -> c_int;
    pub fn __setup_kvm_events_tp_powerpc(kvm: *mut perf_kvm_stat) -> c_int;

    pub fn cpu_isa_init(
        kvm: *mut perf_kvm_stat,
        e_machine: uint16_t,
        cpuid: *const c_char,
    ) -> c_int;
    pub fn __cpu_isa_init_arm64(kvm: *mut perf_kvm_stat) -> c_int;
    pub fn __cpu_isa_init_loongarch(kvm: *mut perf_kvm_stat) -> c_int;
    pub fn __cpu_isa_init_powerpc(kvm: *mut perf_kvm_stat) -> c_int;
    pub fn __cpu_isa_init_riscv(kvm: *mut perf_kvm_stat) -> c_int;
    pub fn __cpu_isa_init_s390(kvm: *mut perf_kvm_stat, cpuid: *const c_char) -> c_int;
    pub fn __cpu_isa_init_x86(kvm: *mut perf_kvm_stat, cpuid: *const c_char) -> c_int;

    pub fn vcpu_id_str(e_machine: uint16_t) -> *const c_char;
    pub fn kvm_exit_reason(e_machine: uint16_t) -> *const c_char;
    pub fn kvm_entry_trace(e_machine: uint16_t) -> *const c_char;
    pub fn kvm_exit_trace(e_machine: uint16_t) -> *const c_char;

    pub fn kvm_events_tp(e_machine: uint16_t) -> *const *const c_char;
    pub fn __kvm_events_tp_arm64() -> *const *const c_char;
    pub fn __kvm_events_tp_loongarch() -> *const *const c_char;
    pub fn __kvm_events_tp_powerpc() -> *const *const c_char;
    pub fn __kvm_events_tp_riscv() -> *const *const c_char;
    pub fn __kvm_events_tp_s390() -> *const *const c_char;
    pub fn __kvm_events_tp_x86() -> *const *const c_char;

    pub fn kvm_reg_events_ops(e_machine: uint16_t) -> *const kvm_reg_events_ops;
    pub fn __kvm_reg_events_ops_arm64() -> *const kvm_reg_events_ops;
    pub fn __kvm_reg_events_ops_loongarch() -> *const kvm_reg_events_ops;
    pub fn __kvm_reg_events_ops_powerpc() -> *const kvm_reg_events_ops;
    pub fn __kvm_reg_events_ops_riscv() -> *const kvm_reg_events_ops;
    pub fn __kvm_reg_events_ops_s390() -> *const kvm_reg_events_ops;
    pub fn __kvm_reg_events_ops_x86() -> *const kvm_reg_events_ops;

    pub fn kvm_skip_events(e_machine: uint16_t) -> *const *const c_char;
    pub fn __kvm_skip_events_arm64() -> *const *const c_char;
    pub fn __kvm_skip_events_loongarch() -> *const *const c_char;
    pub fn __kvm_skip_events_powerpc() -> *const *const c_char;
    pub fn __kvm_skip_events_riscv() -> *const *const c_char;
    pub fn __kvm_skip_events_s390() -> *const *const c_char;
    pub fn __kvm_skip_events_x86() -> *const *const c_char;

    pub fn kvm_need_default_arch_event(
        e_machine: uint16_t,
        argc: c_int,
        argv: *const *const c_char,
    ) -> bool;
    pub fn kvm_add_default_arch_event(
        e_machine: uint16_t,
        argc: *mut c_int,
        argv: *const *const c_char,
    ) -> c_int;
    pub fn __kvm_add_default_arch_event_powerpc(
        argc: *mut c_int,
        argv: *const *const c_char,
    ) -> c_int;
    pub fn __kvm_add_default_arch_event_x86(
        argc: *mut c_int,
        argv: *const *const c_char,
    ) -> c_int;
}

/*
 * Macro equivalent:
 * define_exit_reasons_table(name, symbols) defines a static exit_reasons_table
 * array ending with { -1, NULL }. The -1 initializer maps to u64::MAX.
 */

/* !HAVE_LIBTRACEEVENT inline fallback:
 * kvm_need_default_arch_event(...) returns false;
 * kvm_add_default_arch_event(...) returns 0.
 */

#[inline]
pub unsafe fn kvm_info__get(ki: *mut kvm_info) -> *mut kvm_info {
    if !ki.is_null() {
        unsafe {
            refcount_inc(core::ptr::addr_of_mut!((*ki).refcnt));
        }
    }
    ki
}

#[inline]
pub unsafe fn kvm_info__put(ki: *mut kvm_info) {
    if !ki.is_null()
        && unsafe { refcount_dec_and_test(core::ptr::addr_of_mut!((*ki).refcnt)) }
    {
        unsafe {
            free(ki.cast::<c_void>());
        }
    }
}

#[inline]
pub unsafe fn __kvm_info__zput(ki: *mut *mut kvm_info) {
    unsafe {
        kvm_info__put(*ki);
        *ki = core::ptr::null_mut();
    }
}

/* Macro equivalent: kvm_info__zput(ki) expands to __kvm_info__zput(&ki). */

#[inline]
pub unsafe fn kvm_info__new() -> *mut kvm_info {
    let ki = unsafe { zalloc(core::mem::size_of::<kvm_info>()) as *mut kvm_info };
    if !ki.is_null() {
        unsafe {
            refcount_set(core::ptr::addr_of_mut!((*ki).refcnt), 1);
        }
    }

    ki
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
