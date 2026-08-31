/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Translated from perf/util/event.h.
 *
 * Original include dependencies:
 * - stdio.h
 * - linux/stddef.h
 * - perf/event.h
 * - linux/types.h
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type u8 = ::core::primitive::u8;
pub type u16 = ::core::primitive::u16;
pub type u32 = ::core::primitive::u32;
pub type u64 = ::core::primitive::u64;
pub type size_t = usize;
pub type FILE = c_void;

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    pub raw_size: u32,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_record_stat_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_stat_config {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_tool {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_header {
    pub type_: u32,
    pub misc: u16,
    pub size: u16,
}

#[repr(C)]
pub union perf_event {
    pub header: perf_event_header,
}

/*
 * On LP64 C builds these macros prepend "l" to the PRI*64 macros to match the
 * local __u64 typedef. Rust formatting does not use PRI macros; keep the
 * exported string intent.
 */
#[cfg(target_pointer_width = "64")]
pub const PRI_lu64: &str = "l";
#[cfg(target_pointer_width = "64")]
pub const PRI_lx64: &str = "l";
#[cfg(target_pointer_width = "64")]
pub const PRI_ld64: &str = "l";
#[cfg(not(target_pointer_width = "64"))]
pub const PRI_lu64: &str = "";
#[cfg(not(target_pointer_width = "64"))]
pub const PRI_lx64: &str = "";
#[cfg(not(target_pointer_width = "64"))]
pub const PRI_ld64: &str = "";

/* Values supplied by perf/event.h or linux/perf_event.h. */
extern "C" {
    pub static PERF_SAMPLE_IP: u64;
    pub static PERF_SAMPLE_TID: u64;
    pub static PERF_SAMPLE_TIME: u64;
    pub static PERF_SAMPLE_ADDR: u64;
    pub static PERF_SAMPLE_ID: u64;
    pub static PERF_SAMPLE_STREAM_ID: u64;
    pub static PERF_SAMPLE_CPU: u64;
    pub static PERF_SAMPLE_PERIOD: u64;
    pub static PERF_SAMPLE_IDENTIFIER: u64;
    pub static PERF_RECORD_MISC_GUEST_KERNEL: u32;
    pub static PERF_RECORD_MISC_GUEST_USER: u32;
    pub static PERF_RECORD_MISC_CPUMODE_MASK: u32;
}

pub unsafe fn PERF_SAMPLE_MASK() -> u64 {
    unsafe {
        PERF_SAMPLE_IP
            | PERF_SAMPLE_TID
            | PERF_SAMPLE_TIME
            | PERF_SAMPLE_ADDR
            | PERF_SAMPLE_ID
            | PERF_SAMPLE_STREAM_ID
            | PERF_SAMPLE_CPU
            | PERF_SAMPLE_PERIOD
            | PERF_SAMPLE_IDENTIFIER
    }
}

/* perf sample has 16 bits size limit */
pub const PERF_SAMPLE_MAX_SIZE: u32 = 1 << 16;

#[repr(C)]
pub struct ip_callchain {
    pub nr: u64,
    pub ips: [u64; 0],
}

#[repr(C)]
pub struct branch_stack {
    _private: [u8; 0],
}

pub const PERF_IP_FLAG_BRANCH: u64 = 1u64 << 0;
pub const PERF_IP_FLAG_CALL: u64 = 1u64 << 1;
pub const PERF_IP_FLAG_RETURN: u64 = 1u64 << 2;
pub const PERF_IP_FLAG_CONDITIONAL: u64 = 1u64 << 3;
pub const PERF_IP_FLAG_SYSCALLRET: u64 = 1u64 << 4;
pub const PERF_IP_FLAG_ASYNC: u64 = 1u64 << 5;
pub const PERF_IP_FLAG_INTERRUPT: u64 = 1u64 << 6;
pub const PERF_IP_FLAG_TX_ABORT: u64 = 1u64 << 7;
pub const PERF_IP_FLAG_TRACE_BEGIN: u64 = 1u64 << 8;
pub const PERF_IP_FLAG_TRACE_END: u64 = 1u64 << 9;
pub const PERF_IP_FLAG_IN_TX: u64 = 1u64 << 10;
pub const PERF_IP_FLAG_VMENTRY: u64 = 1u64 << 11;
pub const PERF_IP_FLAG_VMEXIT: u64 = 1u64 << 12;
pub const PERF_IP_FLAG_INTR_DISABLE: u64 = 1u64 << 13;
pub const PERF_IP_FLAG_INTR_TOGGLE: u64 = 1u64 << 14;
pub const PERF_IP_FLAG_BRANCH_MISS: u64 = 1u64 << 15;
pub const PERF_IP_FLAG_NOT_TAKEN: u64 = 1u64 << 16;

pub const PERF_IP_FLAG_CHARS: &str = "bcrosyiABExghDtmn";

pub const PERF_ADDITIONAL_STATE_MASK: u64 =
    PERF_IP_FLAG_IN_TX | PERF_IP_FLAG_INTR_DISABLE | PERF_IP_FLAG_INTR_TOGGLE;

pub const PERF_BRANCH_MASK: u64 = PERF_IP_FLAG_BRANCH
    | PERF_IP_FLAG_CALL
    | PERF_IP_FLAG_RETURN
    | PERF_IP_FLAG_CONDITIONAL
    | PERF_IP_FLAG_SYSCALLRET
    | PERF_IP_FLAG_ASYNC
    | PERF_IP_FLAG_INTERRUPT
    | PERF_IP_FLAG_TX_ABORT
    | PERF_IP_FLAG_TRACE_BEGIN
    | PERF_IP_FLAG_TRACE_END
    | PERF_IP_FLAG_VMENTRY
    | PERF_IP_FLAG_VMEXIT;

pub const PERF_IP_FLAG_BRANCH_EVENT_MASK: u64 =
    PERF_IP_FLAG_BRANCH_MISS | PERF_IP_FLAG_NOT_TAKEN;

/*
 * PERF_MEM_DATA_SRC_NONE expands through PERF_MEM_S() and PERF_MEM_* symbols
 * supplied by linux/perf_event.h.
 */

/* Attribute type for custom synthesized events */
pub const PERF_TYPE_SYNTH: c_uint = c_int::MAX as c_uint + 1;

/* Attribute config for custom synthesized events */
pub const PERF_SYNTH_INTEL_PTWRITE: c_uint = 0;
pub const PERF_SYNTH_INTEL_MWAIT: c_uint = 1;
pub const PERF_SYNTH_INTEL_PWRE: c_uint = 2;
pub const PERF_SYNTH_INTEL_EXSTOP: c_uint = 3;
pub const PERF_SYNTH_INTEL_PWRX: c_uint = 4;
pub const PERF_SYNTH_INTEL_CBR: c_uint = 5;
pub const PERF_SYNTH_INTEL_PSB: c_uint = 6;
pub const PERF_SYNTH_INTEL_EVT: c_uint = 7;
pub const PERF_SYNTH_INTEL_IFLAG_CHG: c_uint = 8;
pub const PERF_SYNTH_POWERPC_VPA_DTL: c_uint = 9;

/*
 * Raw data formats for synthesized events. Note that 4 bytes of padding are
 * present to match the 'size' member of PERF_SAMPLE_RAW data which is always
 * 8-byte aligned. That means we must dereference raw_data with an offset of 4.
 * Refer perf_sample__synth_ptr() and perf_synth__raw_data().  It also means the
 * structure sizes are 4 bytes bigger than the raw_size, refer
 * perf_synth__raw_size().
 */

#[repr(C)]
pub union perf_synth_intel_ptwrite_u {
    /* Bitfields in C: ip:1, reserved:31. */
    pub flags: u32,
}

#[repr(C)]
pub struct perf_synth_intel_ptwrite {
    pub padding: u32,
    pub u: perf_synth_intel_ptwrite_u,
    pub payload: u64,
}

#[repr(C)]
pub union perf_synth_intel_mwait_u {
    /* Bitfields in C: hints:8, reserved1:24, extensions:2, reserved2:30. */
    pub payload: u64,
}

#[repr(C)]
pub struct perf_synth_intel_mwait {
    pub padding: u32,
    pub reserved: u32,
    pub u: perf_synth_intel_mwait_u,
}

#[repr(C)]
pub union perf_synth_intel_pwre_u {
    /* Bitfields in C: reserved1:7, hw:1, subcstate:4, cstate:4, reserved2:48. */
    pub payload: u64,
}

#[repr(C)]
pub struct perf_synth_intel_pwre {
    pub padding: u32,
    pub reserved: u32,
    pub u: perf_synth_intel_pwre_u,
}

#[repr(C)]
pub union perf_synth_intel_exstop_u {
    /* Bitfields in C: ip:1, reserved:31. */
    pub flags: u32,
}

#[repr(C)]
pub struct perf_synth_intel_exstop {
    pub padding: u32,
    pub u: perf_synth_intel_exstop_u,
}

#[repr(C)]
pub union perf_synth_intel_pwrx_u {
    /* Bitfields in C: deepest_cstate:4, last_cstate:4, wake_reason:4, reserved1:52. */
    pub payload: u64,
}

#[repr(C)]
pub struct perf_synth_intel_pwrx {
    pub padding: u32,
    pub reserved: u32,
    pub u: perf_synth_intel_pwrx_u,
}

#[repr(C)]
pub union perf_synth_intel_cbr_u {
    /* Bitfields in C: cbr:8, reserved1:8, max_nonturbo:8, reserved2:8. */
    pub flags: u32,
}

#[repr(C)]
pub struct perf_synth_intel_cbr {
    pub padding: u32,
    pub u: perf_synth_intel_cbr_u,
    pub freq: u32,
    pub reserved3: u32,
}

#[repr(C)]
pub struct perf_synth_intel_psb {
    pub padding: u32,
    pub reserved: u32,
    pub offset: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_synth_intel_evd_parts {
    pub evd_type: u8,
    pub reserved: [u8; 7],
}

#[repr(C)]
pub union perf_synth_intel_evd {
    pub s: perf_synth_intel_evd_parts,
    pub et: u64,
    pub payload: u64,
}

#[repr(C)]
pub union perf_synth_intel_evt_u {
    /* Bitfields in C: type:5, reserved:2, ip:1, vector:8, evd_cnt:16. */
    pub cfe: u32,
}

/* Intel PT Event Trace */
#[repr(C)]
pub struct perf_synth_intel_evt {
    pub padding: u32,
    pub u: perf_synth_intel_evt_u,
    pub evd: [perf_synth_intel_evd; 0],
}

#[repr(C)]
pub union perf_synth_intel_iflag_chg_u {
    /* Bitfields in C: iflag:1, via_branch:1. */
    pub flags: u32,
}

#[repr(C)]
pub struct perf_synth_intel_iflag_chg {
    pub padding: u32,
    pub u: perf_synth_intel_iflag_chg_u,
    pub branch_ip: u64, /* If via_branch */
}

/*
 * The powerpc VPA DTL entries are of below format
 */
#[repr(C)]
pub struct powerpc_vpadtl_entry {
    pub dispatch_reason: u8,
    pub preempt_reason: u8,
    pub processor_id: u16,
    pub enqueue_to_dispatch_time: u32,
    pub ready_to_enqueue_time: u32,
    pub waiting_to_ready_time: u32,
    pub timebase: u64,
    pub fault_addr: u64,
    pub srr0: u64,
    pub srr1: u64,
}

extern "C" {
    pub static dispatch_reasons: [*const c_char; 11];
    pub static preempt_reasons: [*const c_char; 10];
}

#[inline]
pub unsafe fn perf_synth__raw_data(p: *mut c_void) -> *mut c_void {
    unsafe { (p as *mut u8).add(4) as *mut c_void }
}

pub const fn perf_synth__raw_size<T>() -> usize {
    core::mem::size_of::<T>() - 4
}

pub unsafe fn perf_sample__bad_synth_size<T>(s: *const perf_sample) -> bool {
    unsafe { (*s).raw_size < (core::mem::size_of::<T>() - 4) as u32 }
}

pub const PERF_STAT_ROUND_TYPE__INTERVAL: c_uint = 0;
pub const PERF_STAT_ROUND_TYPE__FINAL: c_uint = 1;

extern "C" {
    pub fn perf_event__print_totals();

    pub fn perf_event__read_stat_config(
        config: *mut perf_stat_config,
        event: *mut perf_record_stat_config,
    );

    pub fn perf_event__process_comm(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__process_lost(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__process_lost_samples(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__process_aux(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__process_itrace_start(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__process_aux_output_hw_id(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__process_switch(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__process_namespaces(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__process_cgroup(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__process_mmap(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__process_mmap2(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__process_fork(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__process_exit(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__exit_del_thread(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__process_ksymbol(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__process_bpf(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__process_text_poke(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
    pub fn perf_event__process(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;

    pub fn is_bts_event(attr: *mut perf_event_attr) -> bool;
    pub fn sample_addr_correlates_sym(attr: *mut perf_event_attr) -> bool;

    pub fn perf_event__name(id: c_uint) -> *const c_char;

    pub fn perf_event__fprintf_comm(event: *mut perf_event, fp: *mut FILE) -> size_t;
    pub fn perf_event__fprintf_mmap(event: *mut perf_event, fp: *mut FILE) -> size_t;
    pub fn perf_event__fprintf_mmap2(event: *mut perf_event, fp: *mut FILE) -> size_t;
    pub fn perf_event__fprintf_task(event: *mut perf_event, fp: *mut FILE) -> size_t;
    pub fn perf_event__fprintf_aux(event: *mut perf_event, fp: *mut FILE) -> size_t;
    pub fn perf_event__fprintf_itrace_start(event: *mut perf_event, fp: *mut FILE) -> size_t;
    pub fn perf_event__fprintf_aux_output_hw_id(event: *mut perf_event, fp: *mut FILE) -> size_t;
    pub fn perf_event__fprintf_switch(event: *mut perf_event, fp: *mut FILE) -> size_t;
    pub fn perf_event__fprintf_thread_map(event: *mut perf_event, fp: *mut FILE) -> size_t;
    pub fn perf_event__fprintf_cpu_map(event: *mut perf_event, fp: *mut FILE) -> size_t;
    pub fn perf_event__fprintf_namespaces(event: *mut perf_event, fp: *mut FILE) -> size_t;
    pub fn perf_event__fprintf_cgroup(event: *mut perf_event, fp: *mut FILE) -> size_t;
    pub fn perf_event__fprintf_ksymbol(event: *mut perf_event, fp: *mut FILE) -> size_t;
    pub fn perf_event__fprintf_bpf(event: *mut perf_event, fp: *mut FILE) -> size_t;
    pub fn perf_event__fprintf_bpf_metadata(event: *mut perf_event, fp: *mut FILE) -> size_t;
    pub fn perf_event__fprintf_text_poke(
        event: *mut perf_event,
        machine: *mut machine,
        fp: *mut FILE,
    ) -> size_t;
    pub fn perf_event__fprintf_schedstat_cpu(event: *mut perf_event, fp: *mut FILE) -> size_t;
    pub fn perf_event__fprintf_schedstat_domain(event: *mut perf_event, fp: *mut FILE) -> size_t;
    pub fn perf_event__fprintf(
        event: *mut perf_event,
        machine: *mut machine,
        fp: *mut FILE,
    ) -> size_t;

    pub fn kallsyms__get_function_start(
        kallsyms_filename: *const c_char,
        symbol_name: *const c_char,
        addr: *mut u64,
    ) -> c_int;
    pub fn kallsyms__get_symbol_start(
        kallsyms_filename: *const c_char,
        symbol_name: *const c_char,
        addr: *mut u64,
    ) -> c_int;

    pub fn event_attr_init(attr: *mut perf_event_attr);

    pub fn perf_event_paranoid() -> c_int;
    pub fn perf_event_paranoid_check(max_level: c_int) -> bool;

    pub static mut sysctl_perf_event_max_stack: c_int;
    pub static mut sysctl_perf_event_max_contexts_per_stack: c_int;
    pub static mut proc_map_timeout: c_uint;
}

pub const PAGE_SIZE_NAME_LEN: usize = 32;

extern "C" {
    pub fn get_page_size_name(size: u64, str_: *mut c_char) -> *mut c_char;
}

#[inline]
pub unsafe fn perf_event_header__cpumode_is_guest(cpumode: u8) -> bool {
    unsafe {
        cpumode as u32 == PERF_RECORD_MISC_GUEST_KERNEL
            || cpumode as u32 == PERF_RECORD_MISC_GUEST_USER
    }
}

#[inline]
pub unsafe fn perf_event_header__misc_is_guest(misc: u16) -> bool {
    unsafe { perf_event_header__cpumode_is_guest((misc as u32 & PERF_RECORD_MISC_CPUMODE_MASK) as u8) }
}

#[inline]
pub unsafe fn perf_event_header__is_guest(header: *const perf_event_header) -> bool {
    unsafe { perf_event_header__misc_is_guest((*header).misc) }
}

#[inline]
pub unsafe fn perf_event__is_guest(event: *const perf_event) -> bool {
    unsafe { perf_event_header__is_guest(&(*event).header as *const perf_event_header) }
}
