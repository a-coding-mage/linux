/* SPDX-License-Identifier: GPL-2.0 */
/*
 * perf_dlfilter.h: API for perf --dlfilter shared object
 * Copyright (c) 2021, Intel Corporation.
 */

use core::ffi::{c_char, c_int, c_void};

/*
 * Dependencies from the original C header:
 * #include <linux/perf_event.h>
 * #include <linux/types.h>
 */

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __s32 = i32;

#[repr(C)]
pub struct perf_branch_entry {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct perf_event_attr {
    _unused: [u8; 0],
}

/*
 * The following macro can be used to determine if this header defines
 * perf_dlfilter_sample machine_pid and vcpu.
 */
pub const PERF_DLFILTER_HAS_MACHINE_PID: bool = true;

/* Definitions for perf_dlfilter_sample flags */
pub const PERF_DLFILTER_FLAG_BRANCH: __u64 = 1_u64 << 0;
pub const PERF_DLFILTER_FLAG_CALL: __u64 = 1_u64 << 1;
pub const PERF_DLFILTER_FLAG_RETURN: __u64 = 1_u64 << 2;
pub const PERF_DLFILTER_FLAG_CONDITIONAL: __u64 = 1_u64 << 3;
pub const PERF_DLFILTER_FLAG_SYSCALLRET: __u64 = 1_u64 << 4;
pub const PERF_DLFILTER_FLAG_ASYNC: __u64 = 1_u64 << 5;
pub const PERF_DLFILTER_FLAG_INTERRUPT: __u64 = 1_u64 << 6;
pub const PERF_DLFILTER_FLAG_TX_ABORT: __u64 = 1_u64 << 7;
pub const PERF_DLFILTER_FLAG_TRACE_BEGIN: __u64 = 1_u64 << 8;
pub const PERF_DLFILTER_FLAG_TRACE_END: __u64 = 1_u64 << 9;
pub const PERF_DLFILTER_FLAG_IN_TX: __u64 = 1_u64 << 10;
pub const PERF_DLFILTER_FLAG_VMENTRY: __u64 = 1_u64 << 11;
pub const PERF_DLFILTER_FLAG_VMEXIT: __u64 = 1_u64 << 12;

/*
 * perf sample event information (as per perf script and <linux/perf_event.h>)
 */
#[repr(C)]
pub struct perf_dlfilter_sample {
    pub size: __u32, /* Size of this structure (for compatibility checking) */
    pub ins_lat: __u16, /* Refer PERF_SAMPLE_WEIGHT_TYPE in <linux/perf_event.h> */
    pub p_stage_cyc: __u16, /* Refer PERF_SAMPLE_WEIGHT_TYPE in <linux/perf_event.h> */
    pub ip: __u64,
    pub pid: __s32,
    pub tid: __s32,
    pub time: __u64,
    pub addr: __u64,
    pub id: __u64,
    pub stream_id: __u64,
    pub period: __u64,
    pub weight: __u64, /* Refer PERF_SAMPLE_WEIGHT_TYPE in <linux/perf_event.h> */
    pub transaction: __u64, /* Refer PERF_SAMPLE_TRANSACTION in <linux/perf_event.h> */
    pub insn_cnt: __u64, /* For instructions-per-cycle (IPC) */
    pub cyc_cnt: __u64, /* For instructions-per-cycle (IPC) */
    pub cpu: __s32,
    pub flags: __u32, /* Refer PERF_DLFILTER_FLAG_* above */
    pub data_src: __u64, /* Refer PERF_SAMPLE_DATA_SRC in <linux/perf_event.h> */
    pub phys_addr: __u64, /* Refer PERF_SAMPLE_PHYS_ADDR in <linux/perf_event.h> */
    pub data_page_size: __u64, /* Refer PERF_SAMPLE_DATA_PAGE_SIZE in <linux/perf_event.h> */
    pub code_page_size: __u64, /* Refer PERF_SAMPLE_CODE_PAGE_SIZE in <linux/perf_event.h> */
    pub cgroup: __u64, /* Refer PERF_SAMPLE_CGROUP in <linux/perf_event.h> */
    pub cpumode: __u8, /* Refer CPUMODE_MASK etc in <linux/perf_event.h> */
    pub addr_correlates_sym: __u8, /* True => resolve_addr() can be called */
    pub misc: __u16, /* Refer perf_event_header in <linux/perf_event.h> */
    pub raw_size: __u32, /* Refer PERF_SAMPLE_RAW in <linux/perf_event.h> */
    pub raw_data: *const c_void, /* Refer PERF_SAMPLE_RAW in <linux/perf_event.h> */
    pub brstack_nr: __u64, /* Number of brstack entries */
    pub brstack: *const perf_branch_entry, /* Refer <linux/perf_event.h> */
    pub raw_callchain_nr: __u64, /* Number of raw_callchain entries */
    pub raw_callchain: *const __u64, /* Refer <linux/perf_event.h> */
    pub event: *const c_char,
    pub machine_pid: __s32,
    pub vcpu: __s32,
}

/*
 * Address location (as per perf script)
 */
#[repr(C)]
pub struct perf_dlfilter_al {
    pub size: __u32, /* Size of this structure (for compatibility checking) */
    pub symoff: __u32,
    pub sym: *const c_char,
    pub addr: __u64, /* Mapped address (from dso) */
    pub sym_start: __u64,
    pub sym_end: __u64,
    pub dso: *const c_char,
    pub sym_binding: __u8, /* STB_LOCAL, STB_GLOBAL or STB_WEAK, refer <elf.h> */
    pub is_64_bit: __u8, /* Only valid if dso is not NULL */
    pub is_kernel_ip: __u8, /* True if in kernel space */
    pub buildid_size: __u32,
    pub buildid: *const __u8,
    /* Below members are only populated by resolve_ip() */
    pub filtered: __u8, /* True if this sample event will be filtered out */
    pub comm: *const c_char,
    pub priv_: *mut c_void, /* Private data. Do not change */
}

#[repr(C)]
pub struct perf_dlfilter_fns {
    /* Return information about ip */
    pub resolve_ip: Option<unsafe extern "C" fn(ctx: *mut c_void) -> *const perf_dlfilter_al>,
    /* Return information about addr (if addr_correlates_sym) */
    pub resolve_addr: Option<unsafe extern "C" fn(ctx: *mut c_void) -> *const perf_dlfilter_al>,
    /* Return arguments from --dlarg option */
    pub args: Option<unsafe extern "C" fn(ctx: *mut c_void, dlargc: *mut c_int) -> *mut *mut c_char>,
    /*
     * Return information about address (al->size must be set before
     * calling). Returns 0 on success, -1 otherwise. Call al_cleanup()
     * when 'al' data is no longer needed.
     */
    pub resolve_address: Option<
        unsafe extern "C" fn(
            ctx: *mut c_void,
            address: __u64,
            al: *mut perf_dlfilter_al,
        ) -> __s32,
    >,
    /* Return instruction bytes and length */
    pub insn: Option<unsafe extern "C" fn(ctx: *mut c_void, length: *mut __u32) -> *const __u8>,
    /* Return source file name and line number */
    pub srcline:
        Option<unsafe extern "C" fn(ctx: *mut c_void, line_number: *mut __u32) -> *const c_char>,
    /* Return perf_event_attr, refer <linux/perf_event.h> */
    pub attr: Option<unsafe extern "C" fn(ctx: *mut c_void) -> *mut perf_event_attr>,
    /* Read object code, return numbers of bytes read */
    pub object_code:
        Option<unsafe extern "C" fn(ctx: *mut c_void, ip: __u64, buf: *mut c_void, len: __u32) -> __s32>,
    /*
     * If present (i.e. must check al_cleanup != NULL), call after
     * resolve_address() to free any associated resources.
     */
    pub al_cleanup: Option<unsafe extern "C" fn(ctx: *mut c_void, al: *mut perf_dlfilter_al)>,
    /* Reserved */
    pub reserved: [Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>; 119],
}

unsafe extern "C" {
    /*
     * If implemented, 'start' will be called at the beginning,
     * before any calls to 'filter_event'. Return 0 to indicate success,
     * or return a negative error code. '*data' can be assigned for use
     * by other functions. 'ctx' is needed for calls to perf_dlfilter_fns,
     * but most perf_dlfilter_fns are not valid when called from 'start'.
     */
    pub fn start(data: *mut *mut c_void, ctx: *mut c_void) -> c_int;

    /*
     * If implemented, 'stop' will be called at the end,
     * after any calls to 'filter_event'. Return 0 to indicate success, or
     * return a negative error code. 'data' is set by start(). 'ctx' is
     * needed for calls to perf_dlfilter_fns, but most perf_dlfilter_fns
     * are not valid when called from 'stop'.
     */
    pub fn stop(data: *mut c_void, ctx: *mut c_void) -> c_int;

    /*
     * If implemented, 'filter_event' will be called for each sample
     * event. Return 0 to keep the sample event, 1 to filter it out, or
     * return a negative error code. 'data' is set by start(). 'ctx' is
     * needed for calls to perf_dlfilter_fns.
     */
    pub fn filter_event(
        data: *mut c_void,
        sample: *const perf_dlfilter_sample,
        ctx: *mut c_void,
    ) -> c_int;

    /*
     * The same as 'filter_event' except it is called before internal
     * filtering.
     */
    pub fn filter_event_early(
        data: *mut c_void,
        sample: *const perf_dlfilter_sample,
        ctx: *mut c_void,
    ) -> c_int;

    /*
     * If implemented, return a one-line description of the filter, and optionally
     * a longer description.
     */
    pub fn filter_description(long_description: *mut *const c_char) -> *const c_char;
}
