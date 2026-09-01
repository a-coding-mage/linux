// SPDX-License-Identifier: GPL-2.0-only
/*
 * intel_pt_decoder.c: Intel Processor Trace support
 * Copyright (c) 2013-2014, Intel Corporation.
 *
 * Source-level Rust translation of intel-pt-decoder.c.
 *
 * This file intentionally keeps the original C-facing ABI, names, raw pointer
 * behavior, and external dependencies. Types, constants, and functions supplied
 * by the original headers are declared here only as external dependencies.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(clippy::missing_safety_doc)]

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type uint64_t = u64;
type uint32_t = u32;

const fn BITULL(x: u32) -> u64 {
    1u64 << x
}

/* IA32_RTIT_CTL MSR bits */
const INTEL_PT_CYC_ENABLE: u64 = BITULL(1);
const INTEL_PT_CYC_THRESHOLD: u64 = BITULL(22) | BITULL(21) | BITULL(20) | BITULL(19);
const INTEL_PT_CYC_THRESHOLD_SHIFT: u32 = 19;

const INTEL_PT_BLK_SIZE: usize = 1024;
const BIT63: u64 = 1u64 << 63;
const SEVEN_BYTES: u64 = 0x00ff_ffff_ffff_ffff;
const NO_VMCS: u64 = 0x0000_00ff_ffff_ffff;
const INTEL_PT_RETURN: c_int = 1;

/*
 * Default maximum number of loops with no packets consumed i.e. stuck in a
 * loop.
 */
const INTEL_PT_MAX_LOOPS: c_int = 100000;

const ENOMEM: c_int = 12;
const ENOSYS: c_int = 38;
const EBADMSG: c_int = 74;
const ENODATA: c_int = 61;
const EILSEQ: c_int = 84;
const ENOENT: c_int = 2;
const EOVERFLOW: c_int = 75;
const ENOSPC: c_int = 28;
const ELOOP: c_int = 40;
const ECONNRESET: c_int = 104;
const EINVAL: c_int = 22;
const ENOLINK: c_int = 67;
const EAGAIN: c_int = 11;

/* Header-supplied constants. Kept as extern-like Rust constants for linkage-time replacement. */
const INTEL_PT_PKT_MAX_SZ: usize = 16;
const INTEL_PT_MAX_EVDS: usize = 4;
const INTEL_PT_INSN_BUF_SZ: usize = 16;
const INTEL_PT_PSB_LEN: usize = 16;
const INTEL_PT_NEED_MORE_BYTES: c_int = -1;
const BITS_PER_LONG: c_int = 64;
const PERF_AUXTRACE_RECORD_ALIGNMENT: usize = 8;
const MAX_PADDING: usize = PERF_AUXTRACE_RECORD_ALIGNMENT - 1;

const INTEL_PT_ERR_NOMEM: c_int = 1;
const INTEL_PT_ERR_INTERN: c_int = 2;
const INTEL_PT_ERR_BADPKT: c_int = 3;
const INTEL_PT_ERR_NODATA: c_int = 4;
const INTEL_PT_ERR_NOINSN: c_int = 5;
const INTEL_PT_ERR_MISMAT: c_int = 6;
const INTEL_PT_ERR_OVR: c_int = 7;
const INTEL_PT_ERR_LOST: c_int = 8;
const INTEL_PT_ERR_UNK: c_int = 9;
const INTEL_PT_ERR_NELOOP: c_int = 10;
const INTEL_PT_ERR_EPTW: c_int = 11;
const INTEL_PT_ERR_MAX: c_int = 12;

const INTEL_PT_IN_TX: c_uint = 1 << 0;
const INTEL_PT_ABORT_TX: c_uint = 1 << 1;
const INTEL_PT_ASYNC: c_uint = 1 << 2;
const INTEL_PT_FUP_IP: c_uint = 1 << 3;
const INTEL_PT_SAMPLE_IPC: c_uint = 1 << 4;
const INTEL_PT_IFLAG: c_uint = 1 << 0;
const INTEL_PT_FUP_WITH_NLIP: c_uint = 1 << 0;

const INTEL_PT_BRANCH: c_uint = 1 << 0;
const INTEL_PT_INSTRUCTION: c_uint = 1 << 1;
const INTEL_PT_TRANSACTION: c_uint = 1 << 2;
const INTEL_PT_PTW: c_uint = 1 << 3;
const INTEL_PT_MWAIT_OP: c_uint = 1 << 4;
const INTEL_PT_PWR_ENTRY: c_uint = 1 << 5;
const INTEL_PT_EX_STOP: c_uint = 1 << 6;
const INTEL_PT_BLK_ITEMS: c_uint = 1 << 7;
const INTEL_PT_TRACE_BEGIN: c_uint = 1 << 8;
const INTEL_PT_TRACE_END: c_uint = 1 << 9;
const INTEL_PT_EVT: c_uint = 1 << 10;
const INTEL_PT_CBR_CHG: c_uint = 1 << 11;
const INTEL_PT_PSB_EVT: c_uint = 1 << 12;
const INTEL_PT_IFLAG_CHG: c_uint = 1 << 13;
const INTEL_PT_PWR_EXIT: c_uint = 1 << 14;

const INTEL_PT_BR_NO_BRANCH: c_int = 0;
const INTEL_PT_BR_UNCONDITIONAL: c_int = 1;
const INTEL_PT_BR_INDIRECT: c_int = 2;
const INTEL_PT_BR_CONDITIONAL: c_int = 3;

const INTEL_PT_OP_OTHER: c_int = 0;
const INTEL_PT_OP_CALL: c_int = 1;
const INTEL_PT_OP_RET: c_int = 2;

const INTEL_PT_PERIOD_NONE: c_int = 0;
const INTEL_PT_PERIOD_INSTRUCTIONS: c_int = 1;
const INTEL_PT_PERIOD_TICKS: c_int = 2;
const INTEL_PT_PERIOD_MTC: c_int = 3;

const INTEL_PT_PAD: c_int = 0;
const INTEL_PT_TNT: c_int = 1;
const INTEL_PT_TIP_PGE: c_int = 2;
const INTEL_PT_TIP: c_int = 3;
const INTEL_PT_FUP: c_int = 4;
const INTEL_PT_PSB: c_int = 5;
const INTEL_PT_PIP: c_int = 6;
const INTEL_PT_MODE_EXEC: c_int = 7;
const INTEL_PT_MODE_TSX: c_int = 8;
const INTEL_PT_PSBEND: c_int = 9;
const INTEL_PT_VMCS: c_int = 10;
const INTEL_PT_MNT: c_int = 11;
const INTEL_PT_PTWRITE: c_int = 12;
const INTEL_PT_PTWRITE_IP: c_int = 13;
const INTEL_PT_BBP: c_int = 14;
const INTEL_PT_BIP: c_int = 15;
const INTEL_PT_BEP: c_int = 16;
const INTEL_PT_BEP_IP: c_int = 17;
const INTEL_PT_CFE: c_int = 18;
const INTEL_PT_CFE_IP: c_int = 19;
const INTEL_PT_EVD: c_int = 20;
const INTEL_PT_MTC: c_int = 21;
const INTEL_PT_TSC: c_int = 22;
const INTEL_PT_TMA: c_int = 23;
const INTEL_PT_CYC: c_int = 24;
const INTEL_PT_CBR: c_int = 25;
const INTEL_PT_TIP_PGD: c_int = 26;
const INTEL_PT_TRACESTOP: c_int = 27;
const INTEL_PT_EXSTOP: c_int = 28;
const INTEL_PT_EXSTOP_IP: c_int = 29;
const INTEL_PT_MWAIT: c_int = 30;
const INTEL_PT_PWRE: c_int = 31;
const INTEL_PT_PWRX: c_int = 32;
const INTEL_PT_OVF: c_int = 33;
const INTEL_PT_BAD: c_int = 34;

const INTEL_PT_NO_CTX: c_int = 0;
const INTEL_PT_GP_REGS: c_int = 1;
const INTEL_PT_PEBS_BASIC: c_int = 2;
const INTEL_PT_BLK_ITEM_ID_CNT: usize = 8;

static INTEL_PT_PSB_STR: &[u8; INTEL_PT_PSB_LEN] =
    b"\x02\x82\x02\x82\x02\x82\x02\x82\x02\x82\x02\x82\x02\x82\x02\x82";

#[repr(C)]
pub struct intel_pt_buffer {
    pub buf: *const c_uchar,
    pub len: size_t,
    pub ref_timestamp: u64,
    pub consecutive: bool,
    pub trace_nr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pt_pkt {
    pub type_: c_int,
    pub count: u32,
    pub payload: u64,
}

impl Default for intel_pt_pkt {
    fn default() -> Self {
        Self { type_: 0, count: 0, payload: 0 }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pt_insn {
    pub branch: c_int,
    pub op: c_int,
    pub length: u64,
    pub rel: u64,
    pub buf: [u8; INTEL_PT_INSN_BUF_SZ],
    pub emulated_ptwrite: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pt_evd {
    pub type_: u32,
    pub payload: u64,
}

#[repr(C)]
pub struct intel_pt_items {
    pub mask: [u32; 8],
    pub val: [[u64; INTEL_PT_BLK_ITEM_ID_CNT]; 8],
    pub is_32_bit: bool,
}

#[repr(C)]
pub struct intel_pt_state {
    pub type_: c_uint,
    pub flags: c_uint,
    pub err: c_int,
    pub from_ip: u64,
    pub to_ip: u64,
    pub timestamp: u64,
    pub est_timestamp: u64,
    pub tot_insn_cnt: u64,
    pub tot_cyc_cnt: u64,
    pub cycles: u64,
    pub trace_nr: u64,
    pub psb_offset: u64,
    pub insn_op: c_int,
    pub insn_len: u64,
    pub insn: [u8; INTEL_PT_INSN_BUF_SZ],
    pub from_nr: bool,
    pub to_nr: bool,
    pub from_iflag: bool,
    pub to_iflag: bool,
    pub ptw_payload: u64,
    pub mwait_payload: u64,
    pub pwre_payload: u64,
    pub pwrx_payload: u64,
    pub cbr_payload: u64,
    pub cbr: c_uint,
    pub cfe_type: u32,
    pub cfe_vector: u64,
    pub evd_cnt: c_int,
    pub evd: *mut intel_pt_evd,
    pub items: intel_pt_items,
}

impl Default for intel_pt_state {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
pub struct intel_pt_vmcs_info {
    pub vmcs: u64,
    pub tsc_offset: u64,
    pub reliable: bool,
    pub error_printed: bool,
}

pub type intel_pt_lookahead_cb_t =
    Option<unsafe extern "C" fn(buffer: *mut intel_pt_buffer, data: *mut c_void) -> c_int>;
pub type intel_pt_param_flags = c_uint;
pub type intel_pt_pkt_ctx = c_int;
pub type intel_pt_blk_type = c_int;
pub type intel_pt_period_type = c_int;
pub type intel_pt_sample_type = c_uint;

#[repr(C)]
pub struct intel_pt_params {
    pub get_trace: Option<unsafe extern "C" fn(*mut intel_pt_buffer, *mut c_void) -> c_int>,
    pub walk_insn: Option<
        unsafe extern "C" fn(
            *mut intel_pt_insn,
            *mut u64,
            *mut u64,
            u64,
            u64,
            *mut c_void,
        ) -> c_int,
    >,
    pub pgd_ip: Option<unsafe extern "C" fn(u64, *mut c_void) -> bool>,
    pub lookahead: Option<unsafe extern "C" fn(*mut c_void, intel_pt_lookahead_cb_t, *mut c_void) -> c_int>,
    pub findnew_vmcs_info: Option<unsafe extern "C" fn(*mut c_void, u64) -> *mut intel_pt_vmcs_info>,
    pub data: *mut c_void,
    pub return_compression: bool,
    pub branch_enable: bool,
    pub quick: c_int,
    pub vm_time_correlation: bool,
    pub vm_tm_corr_dry_run: bool,
    pub first_timestamp: u64,
    pub max_loops: c_int,
    pub flags: intel_pt_param_flags,
    pub ctl: u64,
    pub period: u64,
    pub period_type: intel_pt_period_type,
    pub max_non_turbo_ratio: c_uint,
    pub mtc_period: c_int,
    pub tsc_ctc_ratio_n: u32,
    pub tsc_ctc_ratio_d: u32,
}

unsafe extern "C" {
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(dst: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, n: size_t) -> c_int;
    fn memmem(haystack: *const c_void, haystacklen: size_t, needle: *const c_void, needlelen: size_t) -> *mut c_void;
    fn memrchr(s: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strlcpy(dst: *mut c_char, src: *const c_char, size: size_t) -> size_t;

    fn intel_pt_get_packet(
        buf: *const c_uchar,
        len: size_t,
        packet: *mut intel_pt_pkt,
        ctx: *mut intel_pt_pkt_ctx,
    ) -> c_int;
    fn intel_pt_log_packet(packet: *const intel_pt_pkt, len: c_int, pos: u64, buf: *const c_uchar);
    fn intel_pt_blk_type_pos(blk_type: intel_pt_blk_type) -> c_int;
}

macro_rules! intel_pt_log {
    ($($arg:tt)*) => {{}};
}
macro_rules! intel_pt_log_at {
    ($($arg:tt)*) => {{}};
}
macro_rules! intel_pt_log_to {
    ($($arg:tt)*) => {{}};
}
macro_rules! intel_pt_log_x64 {
    ($($arg:tt)*) => {{}};
}
macro_rules! intel_pt_log_x32 {
    ($($arg:tt)*) => {{}};
}

#[repr(C)]
pub struct intel_pt_blk {
    prev: *mut intel_pt_blk,
    ip: [u64; INTEL_PT_BLK_SIZE],
}

#[repr(C)]
pub struct intel_pt_stack {
    blk: *mut intel_pt_blk,
    spare: *mut intel_pt_blk,
    pos: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_pt_p_once {
    INTEL_PT_PRT_ONCE_UNK_VMCS,
    INTEL_PT_PRT_ONCE_ERANGE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_pt_pkt_state {
    INTEL_PT_STATE_NO_PSB,
    INTEL_PT_STATE_NO_IP,
    INTEL_PT_STATE_ERR_RESYNC,
    INTEL_PT_STATE_IN_SYNC,
    INTEL_PT_STATE_TNT_CONT,
    INTEL_PT_STATE_TNT,
    INTEL_PT_STATE_TIP,
    INTEL_PT_STATE_TIP_PGD,
    INTEL_PT_STATE_FUP,
    INTEL_PT_STATE_FUP_NO_TIP,
    INTEL_PT_STATE_FUP_IN_PSB,
    INTEL_PT_STATE_RESAMPLE,
    INTEL_PT_STATE_VM_TIME_CORRELATION,
}

fn intel_pt_sample_time(pkt_state: intel_pt_pkt_state) -> bool {
    match pkt_state {
        intel_pt_pkt_state::INTEL_PT_STATE_NO_PSB
        | intel_pt_pkt_state::INTEL_PT_STATE_NO_IP
        | intel_pt_pkt_state::INTEL_PT_STATE_ERR_RESYNC
        | intel_pt_pkt_state::INTEL_PT_STATE_IN_SYNC
        | intel_pt_pkt_state::INTEL_PT_STATE_TNT_CONT
        | intel_pt_pkt_state::INTEL_PT_STATE_RESAMPLE
        | intel_pt_pkt_state::INTEL_PT_STATE_VM_TIME_CORRELATION => true,
        intel_pt_pkt_state::INTEL_PT_STATE_TNT
        | intel_pt_pkt_state::INTEL_PT_STATE_TIP
        | intel_pt_pkt_state::INTEL_PT_STATE_TIP_PGD
        | intel_pt_pkt_state::INTEL_PT_STATE_FUP
        | intel_pt_pkt_state::INTEL_PT_STATE_FUP_NO_TIP
        | intel_pt_pkt_state::INTEL_PT_STATE_FUP_IN_PSB => false,
    }
}

/* INTEL_PT_STRICT is a C build-time option; this translation follows the non-strict default. */

#[repr(C)]
pub struct intel_pt_decoder {
    get_trace: Option<unsafe extern "C" fn(*mut intel_pt_buffer, *mut c_void) -> c_int>,
    walk_insn: Option<unsafe extern "C" fn(*mut intel_pt_insn, *mut u64, *mut u64, u64, u64, *mut c_void) -> c_int>,
    pgd_ip: Option<unsafe extern "C" fn(u64, *mut c_void) -> bool>,
    lookahead: Option<unsafe extern "C" fn(*mut c_void, intel_pt_lookahead_cb_t, *mut c_void) -> c_int>,
    findnew_vmcs_info: Option<unsafe extern "C" fn(*mut c_void, u64) -> *mut intel_pt_vmcs_info>,
    data: *mut c_void,
    state: intel_pt_state,
    buf: *const c_uchar,
    len: size_t,
    return_compression: bool,
    branch_enable: bool,
    mtc_insn: bool,
    pge: bool,
    have_tma: bool,
    have_cyc: bool,
    fixup_last_mtc: bool,
    have_last_ip: bool,
    in_psb: bool,
    hop: bool,
    leap: bool,
    emulated_ptwrite: bool,
    vm_time_correlation: bool,
    vm_tm_corr_dry_run: bool,
    vm_tm_corr_reliable: bool,
    vm_tm_corr_same_buf: bool,
    vm_tm_corr_continuous: bool,
    nr: bool,
    next_nr: bool,
    iflag: bool,
    next_iflag: bool,
    flags: intel_pt_param_flags,
    pos: u64,
    last_ip: u64,
    ip: u64,
    pip_payload: u64,
    timestamp: u64,
    tsc_timestamp: u64,
    ref_timestamp: u64,
    buf_timestamp: u64,
    sample_timestamp: u64,
    ret_addr: u64,
    ctc_timestamp: u64,
    ctc_delta: u64,
    cycle_cnt: u64,
    cyc_ref_timestamp: u64,
    first_timestamp: u64,
    last_reliable_timestamp: u64,
    vmcs: u64,
    print_once: u64,
    last_ctc: u64,
    last_mtc: u32,
    tsc_ctc_ratio_n: u32,
    tsc_ctc_ratio_d: u32,
    tsc_ctc_mult: u32,
    tsc_slip: u32,
    ctc_rem_mask: u32,
    mtc_shift: c_int,
    stack: intel_pt_stack,
    pkt_state: intel_pt_pkt_state,
    pkt_ctx: intel_pt_pkt_ctx,
    prev_pkt_ctx: intel_pt_pkt_ctx,
    blk_type: intel_pt_blk_type,
    blk_type_pos: c_int,
    packet: intel_pt_pkt,
    tnt: intel_pt_pkt,
    pkt_step: c_int,
    pkt_len: c_int,
    last_packet_type: c_int,
    cbr: c_uint,
    cbr_seen: c_uint,
    max_non_turbo_ratio: c_uint,
    max_non_turbo_ratio_fp: f64,
    cbr_cyc_to_tsc: f64,
    calc_cyc_to_tsc: f64,
    have_calc_cyc_to_tsc: bool,
    exec_mode: c_int,
    insn_bytes: c_uint,
    period: u64,
    period_type: intel_pt_period_type,
    tot_insn_cnt: u64,
    period_insn_cnt: u64,
    period_mask: u64,
    period_ticks: u64,
    last_masked_timestamp: u64,
    tot_cyc_cnt: u64,
    sample_tot_cyc_cnt: u64,
    base_cyc_cnt: u64,
    cyc_cnt_timestamp: u64,
    ctl: u64,
    cyc_threshold: u64,
    tsc_to_cyc: f64,
    continuous_period: bool,
    overflow: bool,
    set_fup_tx_flags: bool,
    set_fup_ptw: bool,
    set_fup_mwait: bool,
    set_fup_pwre: bool,
    set_fup_exstop: bool,
    set_fup_bep: bool,
    set_fup_cfe_ip: bool,
    set_fup_cfe: bool,
    set_fup_mode_exec: bool,
    sample_cyc: bool,
    fup_tx_flags: c_uint,
    tx_flags: c_uint,
    fup_ptw_payload: u64,
    fup_mwait_payload: u64,
    fup_pwre_payload: u64,
    cbr_payload: u64,
    timestamp_insn_cnt: u64,
    sample_insn_cnt: u64,
    stuck_ip: u64,
    fup_cfe_pkt: intel_pt_pkt,
    max_loops: c_int,
    no_progress: c_int,
    stuck_ip_prd: c_int,
    stuck_ip_cnt: c_int,
    psb_ip: u64,
    next_buf: *const c_uchar,
    next_len: size_t,
    temp_buf: [c_uchar; INTEL_PT_PKT_MAX_SZ],
    evd_cnt: c_int,
    evd: [intel_pt_evd; INTEL_PT_MAX_EVDS],
}

fn intel_pt_lower_power_of_2(mut x: u64) -> u64 {
    let mut i = 0;
    while x != 1 {
        x >>= 1;
        i += 1;
    }
    x << i
}

fn p_log(_fmt: *const c_char) {
    /* Variadic C logging is represented by call-site logging macros above. */
}

unsafe fn intel_pt_print_once(decoder: *mut intel_pt_decoder, id: intel_pt_p_once) -> bool {
    let bit = 1u64 << (id as u32);
    if (*decoder).print_once & bit != 0 {
        return false;
    }
    (*decoder).print_once |= bit;
    true
}

fn intel_pt_cyc_threshold(ctl: u64) -> u64 {
    if ctl & INTEL_PT_CYC_ENABLE == 0 { 0 } else { (ctl & INTEL_PT_CYC_THRESHOLD) >> INTEL_PT_CYC_THRESHOLD_SHIFT }
}

unsafe fn intel_pt_setup_period(decoder: *mut intel_pt_decoder) {
    if (*decoder).period_type == INTEL_PT_PERIOD_TICKS {
        let period = intel_pt_lower_power_of_2((*decoder).period);
        (*decoder).period_mask = !(period - 1);
        (*decoder).period_ticks = period;
    }
}

fn multdiv(t: u64, n: u32, d: u32) -> u64 {
    if d == 0 { return 0; }
    (t / d as u64) * n as u64 + ((t % d as u64) * n as u64) / d as u64
}

#[no_mangle]
pub unsafe extern "C" fn intel_pt_decoder_new(params: *mut intel_pt_params) -> *mut intel_pt_decoder {
    if params.is_null() || (*params).get_trace.is_none() || (*params).walk_insn.is_none() {
        return ptr::null_mut();
    }

    let decoder = malloc(mem::size_of::<intel_pt_decoder>()) as *mut intel_pt_decoder;
    if decoder.is_null() {
        return ptr::null_mut();
    }
    memset(decoder as *mut c_void, 0, mem::size_of::<intel_pt_decoder>());

    (*decoder).get_trace = (*params).get_trace;
    (*decoder).walk_insn = (*params).walk_insn;
    (*decoder).pgd_ip = (*params).pgd_ip;
    (*decoder).lookahead = (*params).lookahead;
    (*decoder).findnew_vmcs_info = (*params).findnew_vmcs_info;
    (*decoder).data = (*params).data;
    (*decoder).return_compression = (*params).return_compression;
    (*decoder).branch_enable = (*params).branch_enable;
    (*decoder).hop = (*params).quick >= 1;
    (*decoder).leap = (*params).quick >= 2;
    (*decoder).vm_time_correlation = (*params).vm_time_correlation;
    (*decoder).vm_tm_corr_dry_run = (*params).vm_tm_corr_dry_run;
    (*decoder).first_timestamp = (*params).first_timestamp;
    (*decoder).last_reliable_timestamp = (*params).first_timestamp;
    (*decoder).max_loops = if (*params).max_loops != 0 { (*params).max_loops } else { INTEL_PT_MAX_LOOPS };
    (*decoder).flags = (*params).flags;
    (*decoder).ctl = (*params).ctl;
    (*decoder).period = (*params).period;
    (*decoder).period_type = (*params).period_type;
    (*decoder).max_non_turbo_ratio = (*params).max_non_turbo_ratio;
    (*decoder).max_non_turbo_ratio_fp = (*params).max_non_turbo_ratio as f64;
    (*decoder).cyc_threshold = intel_pt_cyc_threshold((*decoder).ctl);
    intel_pt_setup_period(decoder);
    (*decoder).mtc_shift = (*params).mtc_period;
    (*decoder).ctc_rem_mask = ((1u32) << (*decoder).mtc_shift) - 1;
    (*decoder).tsc_ctc_ratio_n = (*params).tsc_ctc_ratio_n;
    (*decoder).tsc_ctc_ratio_d = (*params).tsc_ctc_ratio_d;
    if (*decoder).tsc_ctc_ratio_n == 0 {
        (*decoder).tsc_ctc_ratio_d = 0;
    }
    if (*decoder).tsc_ctc_ratio_d != 0 && (*decoder).tsc_ctc_ratio_n % (*decoder).tsc_ctc_ratio_d == 0 {
        (*decoder).tsc_ctc_mult = (*decoder).tsc_ctc_ratio_n / (*decoder).tsc_ctc_ratio_d;
    }
    (*decoder).tsc_slip = 0x10000;
    decoder
}

#[no_mangle]
pub unsafe extern "C" fn intel_pt_set_first_timestamp(decoder: *mut intel_pt_decoder, first_timestamp: u64) {
    (*decoder).first_timestamp = first_timestamp;
}

unsafe fn intel_pt_pop_blk(stack: *mut intel_pt_stack) {
    let blk = (*stack).blk;
    (*stack).blk = (*blk).prev;
    if (*stack).spare.is_null() {
        (*stack).spare = blk;
    } else {
        free(blk as *mut c_void);
    }
}

unsafe fn intel_pt_pop(stack: *mut intel_pt_stack) -> u64 {
    if (*stack).pos == 0 {
        if (*stack).blk.is_null() {
            return 0;
        }
        intel_pt_pop_blk(stack);
        if (*stack).blk.is_null() {
            return 0;
        }
        (*stack).pos = INTEL_PT_BLK_SIZE as c_int;
    }
    (*stack).pos -= 1;
    (*(*stack).blk).ip[(*stack).pos as usize]
}

unsafe fn intel_pt_alloc_blk(stack: *mut intel_pt_stack) -> c_int {
    let blk = if !(*stack).spare.is_null() {
        let blk = (*stack).spare;
        (*stack).spare = ptr::null_mut();
        blk
    } else {
        let blk = malloc(mem::size_of::<intel_pt_blk>()) as *mut intel_pt_blk;
        if blk.is_null() {
            return -ENOMEM;
        }
        blk
    };
    (*blk).prev = (*stack).blk;
    (*stack).blk = blk;
    (*stack).pos = 0;
    0
}

unsafe fn intel_pt_push(stack: *mut intel_pt_stack, ip: u64) -> c_int {
    if (*stack).blk.is_null() || (*stack).pos == INTEL_PT_BLK_SIZE as c_int {
        let err = intel_pt_alloc_blk(stack);
        if err != 0 {
            return err;
        }
    }
    (*(*stack).blk).ip[(*stack).pos as usize] = ip;
    (*stack).pos += 1;
    0
}

unsafe fn intel_pt_clear_stack(stack: *mut intel_pt_stack) {
    while !(*stack).blk.is_null() {
        intel_pt_pop_blk(stack);
    }
    (*stack).pos = 0;
}

unsafe fn intel_pt_free_stack(stack: *mut intel_pt_stack) {
    intel_pt_clear_stack(stack);
    if !(*stack).blk.is_null() {
        free((*stack).blk as *mut c_void);
        (*stack).blk = ptr::null_mut();
    }
    if !(*stack).spare.is_null() {
        free((*stack).spare as *mut c_void);
        (*stack).spare = ptr::null_mut();
    }
}

#[no_mangle]
pub unsafe extern "C" fn intel_pt_decoder_free(decoder: *mut intel_pt_decoder) {
    if decoder.is_null() {
        return;
    }
    intel_pt_free_stack(&mut (*decoder).stack);
    free(decoder as *mut c_void);
}

fn intel_pt_ext_err(code: c_int) -> c_int {
    match code {
        x if x == -ENOMEM => INTEL_PT_ERR_NOMEM,
        x if x == -ENOSYS => INTEL_PT_ERR_INTERN,
        x if x == -EBADMSG => INTEL_PT_ERR_BADPKT,
        x if x == -ENODATA => INTEL_PT_ERR_NODATA,
        x if x == -EILSEQ => INTEL_PT_ERR_NOINSN,
        x if x == -ENOENT => INTEL_PT_ERR_MISMAT,
        x if x == -EOVERFLOW => INTEL_PT_ERR_OVR,
        x if x == -ENOSPC => INTEL_PT_ERR_LOST,
        x if x == -ELOOP => INTEL_PT_ERR_NELOOP,
        x if x == -ECONNRESET => INTEL_PT_ERR_EPTW,
        _ => INTEL_PT_ERR_UNK,
    }
}

static INTEL_PT_ERR_MSGS: [&[u8]; INTEL_PT_ERR_MAX as usize] = [
    b"\0",
    b"Memory allocation failed\0",
    b"Internal error\0",
    b"Bad packet\0",
    b"No more data\0",
    b"Failed to get instruction\0",
    b"Trace doesn't match instruction\0",
    b"Overflow packet\0",
    b"Lost trace data\0",
    b"Unknown error!\0",
    b"Never-ending loop (refer perf config intel-pt.max-loops)\0",
    b"Broken emulated ptwrite\0",
];

#[no_mangle]
pub unsafe extern "C" fn intel_pt__strerror(mut code: c_int, buf: *mut c_char, buflen: size_t) -> c_int {
    if code < 1 || code >= INTEL_PT_ERR_MAX {
        code = INTEL_PT_ERR_UNK;
    }
    strlcpy(buf, INTEL_PT_ERR_MSGS[code as usize].as_ptr() as *const c_char, buflen);
    0
}

unsafe fn intel_pt_calc_ip(packet: *const intel_pt_pkt, last_ip: u64) -> u64 {
    match (*packet).count {
        1 => (last_ip & 0xffff_ffff_ffff_0000) | (*packet).payload,
        2 => (last_ip & 0xffff_ffff_0000_0000) | (*packet).payload,
        3 => {
            let mut ip = (*packet).payload;
            /* Sign-extend 6-byte ip */
            if ip & 0x8000_0000_0000 != 0 {
                ip |= 0xffff_0000_0000_0000;
            }
            ip
        }
        4 => (last_ip & 0xffff_0000_0000_0000) | (*packet).payload,
        6 => (*packet).payload,
        _ => 0,
    }
}

unsafe fn intel_pt_set_last_ip(decoder: *mut intel_pt_decoder) {
    (*decoder).last_ip = intel_pt_calc_ip(&(*decoder).packet, (*decoder).last_ip);
    (*decoder).have_last_ip = true;
}

unsafe fn intel_pt_set_ip(decoder: *mut intel_pt_decoder) {
    intel_pt_set_last_ip(decoder);
    (*decoder).ip = (*decoder).last_ip;
}

unsafe fn intel_pt_decoder_log_packet(decoder: *mut intel_pt_decoder) {
    intel_pt_log_packet(&(*decoder).packet, (*decoder).pkt_len, (*decoder).pos, (*decoder).buf);
}

unsafe fn intel_pt_bug(decoder: *mut intel_pt_decoder) -> c_int {
    intel_pt_log!("ERROR: Internal error\n");
    (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_NO_PSB;
    -ENOSYS
}

unsafe fn intel_pt_clear_tx_flags(decoder: *mut intel_pt_decoder) {
    (*decoder).tx_flags = 0;
}

unsafe fn intel_pt_update_in_tx(decoder: *mut intel_pt_decoder) {
    (*decoder).tx_flags = ((*decoder).packet.payload as c_uint) & INTEL_PT_IN_TX;
}

unsafe fn intel_pt_update_pip(decoder: *mut intel_pt_decoder) {
    (*decoder).pip_payload = (*decoder).packet.payload;
}

unsafe fn intel_pt_update_nr(decoder: *mut intel_pt_decoder) {
    (*decoder).next_nr = ((*decoder).pip_payload & 1) != 0;
}

unsafe fn intel_pt_set_nr(decoder: *mut intel_pt_decoder) {
    (*decoder).nr = ((*decoder).pip_payload & 1) != 0;
    (*decoder).next_nr = (*decoder).nr;
}

unsafe fn intel_pt_set_pip(decoder: *mut intel_pt_decoder) {
    intel_pt_update_pip(decoder);
    intel_pt_set_nr(decoder);
}

unsafe fn intel_pt_bad_packet(decoder: *mut intel_pt_decoder) -> c_int {
    intel_pt_clear_tx_flags(decoder);
    (*decoder).have_tma = false;
    (*decoder).pkt_len = 1;
    (*decoder).pkt_step = 1;
    intel_pt_decoder_log_packet(decoder);
    if (*decoder).pkt_state != intel_pt_pkt_state::INTEL_PT_STATE_NO_PSB {
        (*decoder).pkt_state = (*decoder).pkt_state;
    }
    -EBADMSG
}

unsafe fn intel_pt_update_sample_time(decoder: *mut intel_pt_decoder) {
    (*decoder).sample_timestamp = (*decoder).timestamp;
    (*decoder).sample_insn_cnt = (*decoder).timestamp_insn_cnt;
    (*decoder).state.cycles = (*decoder).tot_cyc_cnt;
}

unsafe fn intel_pt_reposition(decoder: *mut intel_pt_decoder) {
    (*decoder).ip = 0;
    (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_NO_PSB;
    (*decoder).timestamp = 0;
    (*decoder).have_tma = false;
}

unsafe fn intel_pt_get_data(decoder: *mut intel_pt_decoder, reposition: bool) -> c_int {
    let mut buffer: intel_pt_buffer = mem::zeroed();
    (*decoder).pkt_step = 0;
    let ret = ((*decoder).get_trace.unwrap())(&mut buffer, (*decoder).data);
    if ret != 0 {
        return ret;
    }
    (*decoder).buf = buffer.buf;
    (*decoder).len = buffer.len;
    if (*decoder).len == 0 {
        return -ENODATA;
    }
    (*decoder).buf_timestamp = buffer.ref_timestamp;
    if !buffer.consecutive || reposition {
        intel_pt_reposition(decoder);
        (*decoder).ref_timestamp = buffer.ref_timestamp;
        (*decoder).state.trace_nr = buffer.trace_nr;
        (*decoder).vm_tm_corr_same_buf = false;
        return -ENOLINK;
    }
    0
}

unsafe fn intel_pt_get_next_data(decoder: *mut intel_pt_decoder, reposition: bool) -> c_int {
    if (*decoder).next_buf.is_null() {
        return intel_pt_get_data(decoder, reposition);
    }
    (*decoder).buf = (*decoder).next_buf;
    (*decoder).len = (*decoder).next_len;
    (*decoder).next_buf = ptr::null();
    (*decoder).next_len = 0;
    0
}

unsafe fn intel_pt_get_split_packet(decoder: *mut intel_pt_decoder) -> c_int {
    let buf = (*decoder).temp_buf.as_mut_ptr();
    let old_len = (*decoder).len;
    let mut len = (*decoder).len;
    memcpy(buf as *mut c_void, (*decoder).buf as *const c_void, len);
    let ret = intel_pt_get_data(decoder, false);
    if ret != 0 {
        (*decoder).pos += old_len as u64;
        return if ret < 0 { ret } else { -EINVAL };
    }
    let mut n = INTEL_PT_PKT_MAX_SZ - len;
    if n > (*decoder).len {
        n = (*decoder).len;
    }
    memcpy(buf.add(len) as *mut c_void, (*decoder).buf as *const c_void, n);
    len += n;
    (*decoder).prev_pkt_ctx = (*decoder).pkt_ctx;
    let ret2 = intel_pt_get_packet(buf, len, &mut (*decoder).packet, &mut (*decoder).pkt_ctx);
    if ret2 < old_len as c_int {
        (*decoder).next_buf = (*decoder).buf;
        (*decoder).next_len = (*decoder).len;
        (*decoder).buf = buf;
        (*decoder).len = old_len;
        return intel_pt_bad_packet(decoder);
    }
    (*decoder).next_buf = (*decoder).buf.add((ret2 as usize) - old_len);
    (*decoder).next_len = (*decoder).len - ((ret2 as usize) - old_len);
    (*decoder).buf = buf;
    (*decoder).len = ret2 as usize;
    ret2
}

#[repr(C)]
struct intel_pt_pkt_info {
    decoder: *mut intel_pt_decoder,
    packet: intel_pt_pkt,
    pos: u64,
    pkt_len: c_int,
    last_packet_type: c_int,
    data: *mut c_void,
}

type intel_pt_pkt_cb_t = unsafe fn(*mut intel_pt_pkt_info) -> c_int;

/* Lookahead packets in current buffer */
unsafe fn intel_pt_pkt_lookahead(decoder: *mut intel_pt_decoder, cb: intel_pt_pkt_cb_t, data: *mut c_void) -> c_int {
    let mut pkt_info = intel_pt_pkt_info {
        decoder,
        packet: intel_pt_pkt::default(),
        pos: (*decoder).pos,
        pkt_len: (*decoder).pkt_step,
        last_packet_type: (*decoder).last_packet_type,
        data,
    };
    let mut buf = (*decoder).buf;
    let mut pkt_ctx = (*decoder).pkt_ctx;
    let mut len = (*decoder).len;
    loop {
        loop {
            pkt_info.pos += pkt_info.pkt_len as u64;
            buf = buf.add(pkt_info.pkt_len as usize);
            len -= pkt_info.pkt_len as usize;
            if len == 0 {
                return INTEL_PT_NEED_MORE_BYTES;
            }
            let ret = intel_pt_get_packet(buf, len, &mut pkt_info.packet, &mut pkt_ctx);
            if ret == 0 {
                return INTEL_PT_NEED_MORE_BYTES;
            }
            if ret < 0 {
                return ret;
            }
            pkt_info.pkt_len = ret;
            if pkt_info.packet.type_ != INTEL_PT_PAD {
                break;
            }
        }
        let ret = cb(&mut pkt_info);
        if ret != 0 {
            return 0;
        }
        pkt_info.last_packet_type = pkt_info.packet.type_;
    }
}

#[repr(C)]
struct intel_pt_calc_cyc_to_tsc_info {
    cycle_cnt: u64,
    cbr: c_uint,
    last_mtc: u32,
    ctc_timestamp: u64,
    ctc_delta: u64,
    tsc_timestamp: u64,
    timestamp: u64,
    have_tma: bool,
    fixup_last_mtc: bool,
    from_mtc: bool,
    cbr_cyc_to_tsc: f64,
}

/*
 * MTC provides a 8-bit slice of CTC but the TMA packet only provides the lower
 * 16 bits of CTC. If mtc_shift > 8 then some of the MTC bits are not in the CTC
 * provided by the TMA packet. Fix-up the last_mtc calculated from the TMA
 * packet by copying the missing bits from the current MTC assuming the least
 * difference between the two, and that the current MTC comes after last_mtc.
 */
unsafe fn intel_pt_fixup_last_mtc(mtc: u32, mtc_shift: c_int, last_mtc: *mut u32) {
    let first_missing_bit = 1u32 << (16 - mtc_shift);
    let mask = !(first_missing_bit - 1);
    *last_mtc |= mtc & mask;
    if *last_mtc >= mtc {
        *last_mtc -= first_missing_bit;
        *last_mtc &= 0xff;
    }
}

unsafe fn intel_pt_calc_cyc_cb(pkt_info: *mut intel_pt_pkt_info) -> c_int {
    let decoder = (*pkt_info).decoder;
    let data = (*pkt_info).data as *mut intel_pt_calc_cyc_to_tsc_info;
    let mut timestamp: u64;
    match (*pkt_info).packet.type_ {
        INTEL_PT_TNT | INTEL_PT_TIP_PGE | INTEL_PT_TIP | INTEL_PT_FUP | INTEL_PT_PSB
        | INTEL_PT_PIP | INTEL_PT_MODE_EXEC | INTEL_PT_MODE_TSX | INTEL_PT_PSBEND
        | INTEL_PT_PAD | INTEL_PT_VMCS | INTEL_PT_MNT | INTEL_PT_PTWRITE
        | INTEL_PT_PTWRITE_IP | INTEL_PT_BBP | INTEL_PT_BIP | INTEL_PT_BEP
        | INTEL_PT_BEP_IP | INTEL_PT_CFE | INTEL_PT_CFE_IP | INTEL_PT_EVD => return 0,
        INTEL_PT_MTC => {
            if !(*data).have_tma { return 0; }
            let mtc = (*pkt_info).packet.payload as u32;
            if (*decoder).mtc_shift > 8 && (*data).fixup_last_mtc {
                (*data).fixup_last_mtc = false;
                intel_pt_fixup_last_mtc(mtc, (*decoder).mtc_shift, &mut (*data).last_mtc);
            }
            let mtc_delta = if mtc > (*data).last_mtc { mtc - (*data).last_mtc } else { mtc + 256 - (*data).last_mtc };
            (*data).ctc_delta += (mtc_delta as u64) << (*decoder).mtc_shift;
            (*data).last_mtc = mtc;
            timestamp = (*data).ctc_timestamp + if (*decoder).tsc_ctc_mult != 0 {
                (*data).ctc_delta * (*decoder).tsc_ctc_mult as u64
            } else {
                multdiv((*data).ctc_delta, (*decoder).tsc_ctc_ratio_n, (*decoder).tsc_ctc_ratio_d)
            };
            if timestamp < (*data).timestamp { return 1; }
            if (*pkt_info).last_packet_type != INTEL_PT_CYC {
                (*data).timestamp = timestamp;
                return 0;
            }
        }
        INTEL_PT_TSC => {
            if (*data).from_mtc { return 1; }
            timestamp = (*pkt_info).packet.payload | ((*data).timestamp & (0xffu64 << 56));
            if (*data).from_mtc && timestamp < (*data).timestamp && (*data).timestamp - timestamp < (*decoder).tsc_slip as u64 {
                return 1;
            }
            if timestamp < (*data).timestamp { timestamp += 1u64 << 56; }
            if (*pkt_info).last_packet_type != INTEL_PT_CYC {
                if (*data).from_mtc { return 1; }
                (*data).tsc_timestamp = timestamp;
                (*data).timestamp = timestamp;
                return 0;
            }
        }
        INTEL_PT_TMA => {
            if (*data).from_mtc { return 1; }
            if (*decoder).tsc_ctc_ratio_d == 0 { return 0; }
            let ctc = (*pkt_info).packet.payload as u32;
            let fc = (*pkt_info).packet.count;
            let ctc_rem = ctc & (*decoder).ctc_rem_mask;
            (*data).last_mtc = (ctc >> (*decoder).mtc_shift) & 0xff;
            (*data).ctc_timestamp = (*data).tsc_timestamp - fc as u64;
            (*data).ctc_timestamp -= if (*decoder).tsc_ctc_mult != 0 {
                ctc_rem as u64 * (*decoder).tsc_ctc_mult as u64
            } else {
                multdiv(ctc_rem as u64, (*decoder).tsc_ctc_ratio_n, (*decoder).tsc_ctc_ratio_d)
            };
            (*data).ctc_delta = 0;
            (*data).have_tma = true;
            (*data).fixup_last_mtc = true;
            return 0;
        }
        INTEL_PT_CYC => {
            (*data).cycle_cnt += (*pkt_info).packet.payload;
            return 0;
        }
        INTEL_PT_CBR => {
            let cbr = (*pkt_info).packet.payload as c_uint;
            if (*data).cbr != 0 && (*data).cbr != cbr { return 1; }
            (*data).cbr = cbr;
            (*data).cbr_cyc_to_tsc = (*decoder).max_non_turbo_ratio_fp / cbr as f64;
            return 0;
        }
        INTEL_PT_TIP_PGD | INTEL_PT_TRACESTOP | INTEL_PT_EXSTOP | INTEL_PT_EXSTOP_IP
        | INTEL_PT_MWAIT | INTEL_PT_PWRE | INTEL_PT_PWRX | INTEL_PT_OVF | INTEL_PT_BAD | _ => return 1,
    }
    if (*data).cbr == 0 && (*decoder).cbr != 0 {
        (*data).cbr = (*decoder).cbr;
        (*data).cbr_cyc_to_tsc = (*decoder).cbr_cyc_to_tsc;
    }
    if (*data).cycle_cnt == 0 { return 1; }
    let cyc_to_tsc = (timestamp - (*decoder).timestamp) as f64 / (*data).cycle_cnt as f64;
    if (*data).cbr != 0 && cyc_to_tsc > (*data).cbr_cyc_to_tsc && cyc_to_tsc / (*data).cbr_cyc_to_tsc > 1.25 {
        return 1;
    }
    (*decoder).calc_cyc_to_tsc = cyc_to_tsc;
    (*decoder).have_calc_cyc_to_tsc = true;
    1
}

unsafe fn intel_pt_calc_cyc_to_tsc(decoder: *mut intel_pt_decoder, from_mtc: bool) {
    let mut data = intel_pt_calc_cyc_to_tsc_info {
        cycle_cnt: 0,
        cbr: 0,
        last_mtc: (*decoder).last_mtc,
        ctc_timestamp: (*decoder).ctc_timestamp,
        ctc_delta: (*decoder).ctc_delta,
        tsc_timestamp: (*decoder).tsc_timestamp,
        timestamp: (*decoder).timestamp,
        have_tma: (*decoder).have_tma,
        fixup_last_mtc: (*decoder).fixup_last_mtc,
        from_mtc,
        cbr_cyc_to_tsc: 0.0,
    };
    /*
     * For now, do not support using TSC packets for at least the reasons:
     * 1) timing might have stopped
     * 2) TSC packets within PSB+ can slip against CYC packets
     */
    if !from_mtc {
        return;
    }
    intel_pt_pkt_lookahead(decoder, intel_pt_calc_cyc_cb, &mut data as *mut _ as *mut c_void);
}

unsafe fn intel_pt_get_next_packet(decoder: *mut intel_pt_decoder) -> c_int {
    (*decoder).last_packet_type = (*decoder).packet.type_;
    loop {
        (*decoder).pos += (*decoder).pkt_step as u64;
        (*decoder).buf = (*decoder).buf.add((*decoder).pkt_step as usize);
        (*decoder).len -= (*decoder).pkt_step as usize;
        if (*decoder).len == 0 {
            let ret = intel_pt_get_next_data(decoder, false);
            if ret != 0 { return ret; }
        }
        (*decoder).prev_pkt_ctx = (*decoder).pkt_ctx;
        let mut ret = intel_pt_get_packet((*decoder).buf, (*decoder).len, &mut (*decoder).packet, &mut (*decoder).pkt_ctx);
        if ret == INTEL_PT_NEED_MORE_BYTES && BITS_PER_LONG == 32 && (*decoder).len < INTEL_PT_PKT_MAX_SZ && (*decoder).next_buf.is_null() {
            ret = intel_pt_get_split_packet(decoder);
            if ret < 0 { return ret; }
        }
        if ret <= 0 { return intel_pt_bad_packet(decoder); }
        (*decoder).pkt_len = ret;
        (*decoder).pkt_step = ret;
        intel_pt_decoder_log_packet(decoder);
        if (*decoder).packet.type_ != INTEL_PT_PAD {
            return 0;
        }
    }
}

unsafe fn intel_pt_next_period(decoder: *mut intel_pt_decoder) -> u64 {
    let mut timestamp = (*decoder).timestamp + (*decoder).timestamp_insn_cnt;
    let mut masked_timestamp = timestamp & (*decoder).period_mask;
    if (*decoder).continuous_period {
        if masked_timestamp > (*decoder).last_masked_timestamp { return 1; }
    } else {
        timestamp += 1;
        masked_timestamp = timestamp & (*decoder).period_mask;
        if masked_timestamp > (*decoder).last_masked_timestamp {
            (*decoder).last_masked_timestamp = masked_timestamp;
            (*decoder).continuous_period = true;
        }
    }
    if masked_timestamp < (*decoder).last_masked_timestamp {
        return (*decoder).period_ticks;
    }
    (*decoder).period_ticks - (timestamp - masked_timestamp)
}

unsafe fn intel_pt_next_sample(decoder: *mut intel_pt_decoder) -> u64 {
    match (*decoder).period_type {
        INTEL_PT_PERIOD_INSTRUCTIONS => (*decoder).period - (*decoder).period_insn_cnt,
        INTEL_PT_PERIOD_TICKS => intel_pt_next_period(decoder),
        _ => 0,
    }
}

unsafe fn intel_pt_sample_insn(decoder: *mut intel_pt_decoder) {
    match (*decoder).period_type {
        INTEL_PT_PERIOD_INSTRUCTIONS => (*decoder).period_insn_cnt = 0,
        INTEL_PT_PERIOD_TICKS => {
            let timestamp = (*decoder).timestamp + (*decoder).timestamp_insn_cnt;
            let masked_timestamp = timestamp & (*decoder).period_mask;
            if masked_timestamp > (*decoder).last_masked_timestamp {
                (*decoder).last_masked_timestamp = masked_timestamp;
            } else {
                (*decoder).last_masked_timestamp += (*decoder).period_ticks;
            }
        }
        _ => {}
    }
    (*decoder).state.type_ |= INTEL_PT_INSTRUCTION;
}

unsafe fn intel_pt_sample_fup_insn(decoder: *mut intel_pt_decoder) {
    let mut insn: intel_pt_insn = mem::zeroed();
    let mut insn_cnt = 0u64;
    (*decoder).state.insn_op = INTEL_PT_OP_OTHER;
    (*decoder).state.insn_len = 0;
    if !(*decoder).branch_enable || !(*decoder).pge || (*decoder).hop || (*decoder).ip != (*decoder).last_ip { return; }
    if !(*decoder).mtc_insn { (*decoder).mtc_insn = true; }
    let max_insn_cnt = intel_pt_next_sample(decoder);
    if max_insn_cnt != 1 { return; }
    let err = ((*decoder).walk_insn.unwrap())(&mut insn, &mut insn_cnt, &mut (*decoder).ip, 0, max_insn_cnt, (*decoder).data);
    if err != 0 { return; }
    if insn.branch != INTEL_PT_BR_NO_BRANCH { return; }
    (*decoder).tot_insn_cnt += insn_cnt;
    (*decoder).timestamp_insn_cnt += insn_cnt;
    (*decoder).sample_insn_cnt += insn_cnt;
    (*decoder).period_insn_cnt += insn_cnt;
    intel_pt_sample_insn(decoder);
    (*decoder).state.type_ |= INTEL_PT_INSTRUCTION;
    (*decoder).ip += insn.length;
}

unsafe fn intel_pt_walk_insn(decoder: *mut intel_pt_decoder, insn: *mut intel_pt_insn, ip: u64) -> c_int {
    if !(*decoder).mtc_insn { (*decoder).mtc_insn = true; }
    let max_insn_cnt = intel_pt_next_sample(decoder);
    let mut insn_cnt = 0u64;
    let mut err = ((*decoder).walk_insn.unwrap())(insn, &mut insn_cnt, &mut (*decoder).ip, ip, max_insn_cnt, (*decoder).data);
    (*decoder).tot_insn_cnt += insn_cnt;
    (*decoder).timestamp_insn_cnt += insn_cnt;
    (*decoder).sample_insn_cnt += insn_cnt;
    (*decoder).period_insn_cnt += insn_cnt;
    if err != 0 {
        (*decoder).no_progress = 0;
        (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_NO_IP;
        if err == -ENOENT { return -ENOLINK; }
        return -EILSEQ;
    }
    if ip != 0 && (*decoder).ip == ip {
        err = -EAGAIN;
        (*decoder).no_progress = 0;
    } else if max_insn_cnt != 0 && insn_cnt >= max_insn_cnt {
        intel_pt_sample_insn(decoder);
    }
    if err == 0 && (*insn).branch == INTEL_PT_BR_NO_BRANCH {
        (*decoder).state.type_ = INTEL_PT_INSTRUCTION;
        (*decoder).state.from_ip = (*decoder).ip;
        (*decoder).state.to_ip = 0;
        (*decoder).ip += (*insn).length;
        err = INTEL_PT_RETURN;
    } else if err == 0 {
        if (*insn).op == INTEL_PT_OP_CALL {
            if (*insn).branch != INTEL_PT_BR_UNCONDITIONAL || (*insn).rel != 0 {
                err = intel_pt_push(&mut (*decoder).stack, (*decoder).ip + (*insn).length);
            }
        } else if (*insn).op == INTEL_PT_OP_RET {
            (*decoder).ret_addr = intel_pt_pop(&mut (*decoder).stack);
        }
        if err == 0 && (*insn).branch == INTEL_PT_BR_UNCONDITIONAL {
            let cnt = (*decoder).no_progress;
            (*decoder).no_progress += 1;
            (*decoder).state.from_ip = (*decoder).ip;
            (*decoder).ip += (*insn).length + (*insn).rel;
            (*decoder).state.to_ip = (*decoder).ip;
            err = INTEL_PT_RETURN;
            if cnt != 0 {
                if cnt == 1 {
                    (*decoder).stuck_ip = (*decoder).state.to_ip;
                    (*decoder).stuck_ip_prd = 1;
                    (*decoder).stuck_ip_cnt = 1;
                } else if cnt > (*decoder).max_loops || (*decoder).state.to_ip == (*decoder).stuck_ip {
                    (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_ERR_RESYNC;
                    err = -ELOOP;
                } else {
                    (*decoder).stuck_ip_cnt -= 1;
                    if (*decoder).stuck_ip_cnt == 0 {
                        (*decoder).stuck_ip_prd += 1;
                        (*decoder).stuck_ip_cnt = (*decoder).stuck_ip_prd;
                        (*decoder).stuck_ip = (*decoder).state.to_ip;
                    }
                }
            }
        } else {
            (*decoder).no_progress = 0;
        }
    }
    (*decoder).state.insn_op = (*insn).op;
    (*decoder).state.insn_len = (*insn).length;
    memcpy((*decoder).state.insn.as_mut_ptr() as *mut c_void, (*insn).buf.as_ptr() as *const c_void, INTEL_PT_INSN_BUF_SZ);
    if (*decoder).tx_flags & INTEL_PT_IN_TX != 0 {
        (*decoder).state.flags |= INTEL_PT_IN_TX;
    }
    err
}

unsafe fn intel_pt_mode_exec_status(decoder: *mut intel_pt_decoder) {
    let iflag = ((*decoder).packet.count & INTEL_PT_IFLAG) != 0;
    (*decoder).exec_mode = (*decoder).packet.payload as c_int;
    (*decoder).iflag = iflag;
    (*decoder).next_iflag = iflag;
    (*decoder).state.from_iflag = iflag;
    (*decoder).state.to_iflag = iflag;
}

unsafe fn intel_pt_mode_exec(decoder: *mut intel_pt_decoder) {
    let iflag = ((*decoder).packet.count & INTEL_PT_IFLAG) != 0;
    (*decoder).exec_mode = (*decoder).packet.payload as c_int;
    (*decoder).next_iflag = iflag;
}

unsafe fn intel_pt_sample_iflag(decoder: *mut intel_pt_decoder) {
    (*decoder).state.type_ |= INTEL_PT_IFLAG_CHG;
    (*decoder).state.from_iflag = (*decoder).iflag;
    (*decoder).state.to_iflag = (*decoder).next_iflag;
    (*decoder).iflag = (*decoder).next_iflag;
}

unsafe fn intel_pt_sample_iflag_chg(decoder: *mut intel_pt_decoder) {
    if (*decoder).iflag != (*decoder).next_iflag {
        intel_pt_sample_iflag(decoder);
    }
}

unsafe fn intel_pt_clear_fup_event(decoder: *mut intel_pt_decoder) {
    (*decoder).set_fup_tx_flags = false;
    (*decoder).set_fup_ptw = false;
    (*decoder).set_fup_mwait = false;
    (*decoder).set_fup_pwre = false;
    (*decoder).set_fup_exstop = false;
    (*decoder).set_fup_bep = false;
    (*decoder).set_fup_cfe_ip = false;
    (*decoder).set_fup_cfe = false;
    (*decoder).evd_cnt = 0;
    (*decoder).set_fup_mode_exec = false;
    (*decoder).iflag = (*decoder).next_iflag;
}

unsafe fn intel_pt_fup_event(decoder: *mut intel_pt_decoder, no_tip: bool) -> bool {
    let type_ = (*decoder).state.type_;
    let mut sample_fup_insn = false;
    let mut ret = false;
    (*decoder).state.type_ &= !INTEL_PT_BRANCH;
    (*decoder).state.insn_op = INTEL_PT_OP_OTHER;
    (*decoder).state.insn_len = 0;
    if (*decoder).set_fup_cfe_ip || (*decoder).set_fup_cfe {
        let ip = (*decoder).set_fup_cfe_ip;
        (*decoder).set_fup_cfe_ip = false;
        (*decoder).set_fup_cfe = false;
        (*decoder).state.type_ |= INTEL_PT_EVT;
        if !ip && (*decoder).pge { (*decoder).state.type_ |= INTEL_PT_BRANCH; }
        (*decoder).state.cfe_type = (*decoder).fup_cfe_pkt.count;
        (*decoder).state.cfe_vector = (*decoder).fup_cfe_pkt.payload;
        (*decoder).state.evd_cnt = (*decoder).evd_cnt;
        (*decoder).state.evd = (*decoder).evd.as_mut_ptr();
        (*decoder).evd_cnt = 0;
        if ip || (*decoder).pge { (*decoder).state.flags |= INTEL_PT_FUP_IP; }
        ret = true;
    }
    if (*decoder).set_fup_mode_exec {
        (*decoder).set_fup_mode_exec = false;
        intel_pt_sample_iflag(decoder);
        sample_fup_insn = no_tip;
        ret = true;
    }
    if (*decoder).set_fup_tx_flags {
        (*decoder).set_fup_tx_flags = false;
        (*decoder).tx_flags = (*decoder).fup_tx_flags;
        (*decoder).state.type_ |= INTEL_PT_TRANSACTION;
        if (*decoder).fup_tx_flags & INTEL_PT_ABORT_TX != 0 { (*decoder).state.type_ |= INTEL_PT_BRANCH; }
        (*decoder).state.flags = (*decoder).fup_tx_flags;
        ret = true;
    }
    if (*decoder).set_fup_ptw {
        (*decoder).set_fup_ptw = false;
        (*decoder).state.type_ |= INTEL_PT_PTW;
        (*decoder).state.flags |= INTEL_PT_FUP_IP;
        (*decoder).state.ptw_payload = (*decoder).fup_ptw_payload;
        ret = true;
    }
    if (*decoder).set_fup_mwait {
        (*decoder).set_fup_mwait = false;
        (*decoder).state.type_ |= INTEL_PT_MWAIT_OP;
        (*decoder).state.mwait_payload = (*decoder).fup_mwait_payload;
        ret = true;
    }
    if (*decoder).set_fup_pwre {
        (*decoder).set_fup_pwre = false;
        (*decoder).state.type_ |= INTEL_PT_PWR_ENTRY;
        (*decoder).state.pwre_payload = (*decoder).fup_pwre_payload;
        ret = true;
    }
    if (*decoder).set_fup_exstop {
        (*decoder).set_fup_exstop = false;
        (*decoder).state.type_ |= INTEL_PT_EX_STOP;
        (*decoder).state.flags |= INTEL_PT_FUP_IP;
        ret = true;
    }
    if (*decoder).set_fup_bep {
        (*decoder).set_fup_bep = false;
        (*decoder).state.type_ |= INTEL_PT_BLK_ITEMS;
        ret = true;
    }
    if (*decoder).overflow {
        (*decoder).overflow = false;
        if !ret && !(*decoder).pge {
            if (*decoder).hop {
                (*decoder).state.type_ = 0;
                (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_RESAMPLE;
            }
            (*decoder).pge = true;
            (*decoder).state.type_ |= INTEL_PT_BRANCH | INTEL_PT_TRACE_BEGIN;
            (*decoder).state.from_ip = 0;
            (*decoder).state.to_ip = (*decoder).ip;
            return true;
        }
    }
    if ret {
        (*decoder).state.from_ip = (*decoder).ip;
        (*decoder).state.to_ip = 0;
        if sample_fup_insn {
            intel_pt_sample_fup_insn(decoder);
        }
    } else {
        (*decoder).state.type_ = type_;
    }
    ret
}

unsafe fn intel_pt_fup_with_nlip(decoder: *mut intel_pt_decoder, insn: *mut intel_pt_insn, ip: u64, err: c_int) -> bool {
    ((*decoder).flags & INTEL_PT_FUP_WITH_NLIP) != 0
        && err == 0
        && (*insn).branch == INTEL_PT_BR_INDIRECT
        && ip == (*decoder).ip + (*insn).length
}

unsafe fn intel_pt_walk_fup(decoder: *mut intel_pt_decoder) -> c_int {
    let mut insn: intel_pt_insn = mem::zeroed();
    let ip = (*decoder).last_ip;
    loop {
        let err = intel_pt_walk_insn(decoder, &mut insn, ip);
        if err == INTEL_PT_RETURN { return 0; }
        if err == -EAGAIN || intel_pt_fup_with_nlip(decoder, &mut insn, ip, err) {
            let no_tip = (*decoder).pkt_state != intel_pt_pkt_state::INTEL_PT_STATE_FUP;
            (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_IN_SYNC;
            if intel_pt_fup_event(decoder, no_tip) && no_tip { return 0; }
            return -EAGAIN;
        }
        (*decoder).set_fup_tx_flags = false;
        if err != 0 { return err; }
        if insn.branch == INTEL_PT_BR_INDIRECT {
            (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_ERR_RESYNC;
            return -ENOENT;
        }
        if insn.branch == INTEL_PT_BR_CONDITIONAL {
            (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_ERR_RESYNC;
            return -ENOENT;
        }
        return intel_pt_bug(decoder);
    }
}

unsafe fn intel_pt_walk_tip(decoder: *mut intel_pt_decoder) -> c_int {
    let mut insn: intel_pt_insn = mem::zeroed();
    let err = intel_pt_walk_insn(decoder, &mut insn, 0);
    if err == INTEL_PT_RETURN
        && (*decoder).pgd_ip.is_some()
        && (*decoder).pkt_state == intel_pt_pkt_state::INTEL_PT_STATE_TIP_PGD
        && ((*decoder).state.type_ & INTEL_PT_BRANCH) != 0
        && ((*decoder).pgd_ip.unwrap())((*decoder).state.to_ip, (*decoder).data)
    {
        (*decoder).no_progress = 0;
        (*decoder).pge = false;
        (*decoder).continuous_period = false;
        (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_IN_SYNC;
        (*decoder).state.type_ |= INTEL_PT_TRACE_END;
        intel_pt_update_nr(decoder);
        return 0;
    }
    if err == INTEL_PT_RETURN { return 0; }
    if err != 0 { return err; }
    intel_pt_update_nr(decoder);
    intel_pt_sample_iflag_chg(decoder);
    if insn.branch == INTEL_PT_BR_INDIRECT {
        if (*decoder).pkt_state == intel_pt_pkt_state::INTEL_PT_STATE_TIP_PGD {
            (*decoder).pge = false;
            (*decoder).continuous_period = false;
            (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_IN_SYNC;
            (*decoder).state.type_ |= INTEL_PT_TRACE_END;
        } else {
            (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_IN_SYNC;
        }
        (*decoder).state.from_ip = (*decoder).ip;
        if (*decoder).packet.count == 0 {
            (*decoder).state.to_ip = 0;
        } else {
            (*decoder).state.to_ip = (*decoder).last_ip;
            (*decoder).ip = (*decoder).last_ip;
        }
        return 0;
    }
    if insn.branch == INTEL_PT_BR_CONDITIONAL {
        let to_ip = (*decoder).ip + insn.length + insn.rel;
        if (*decoder).pgd_ip.is_some()
            && (*decoder).pkt_state == intel_pt_pkt_state::INTEL_PT_STATE_TIP_PGD
            && ((*decoder).pgd_ip.unwrap())(to_ip, (*decoder).data)
        {
            (*decoder).pge = false;
            (*decoder).continuous_period = false;
            (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_IN_SYNC;
            (*decoder).ip = to_ip;
            (*decoder).state.from_ip = (*decoder).ip;
            (*decoder).state.to_ip = to_ip;
            (*decoder).state.type_ |= INTEL_PT_TRACE_END;
            return 0;
        }
        (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_ERR_RESYNC;
        return -ENOENT;
    }
    intel_pt_bug(decoder)
}

#[repr(C)]
struct eptw_data {
    bit_countdown: c_int,
    payload: u64,
}

unsafe fn intel_pt_eptw_lookahead_cb(pkt_info: *mut intel_pt_pkt_info) -> c_int {
    let data = (*pkt_info).data as *mut eptw_data;
    match (*pkt_info).packet.type_ {
        INTEL_PT_TNT => {
            let mut nr_bits = (*data).bit_countdown;
            if nr_bits > (*pkt_info).packet.count as c_int { nr_bits = (*pkt_info).packet.count as c_int; }
            (*data).payload <<= nr_bits;
            (*data).payload |= (*pkt_info).packet.payload >> (64 - nr_bits);
            (*data).bit_countdown -= nr_bits;
            ((*data).bit_countdown == 0) as c_int
        }
        INTEL_PT_TIP_PGE | INTEL_PT_TIP_PGD | INTEL_PT_TIP | INTEL_PT_BAD | INTEL_PT_OVF | INTEL_PT_TRACESTOP => 1,
        _ => 0,
    }
}

unsafe fn intel_pt_emulated_ptwrite(decoder: *mut intel_pt_decoder) -> c_int {
    let n = 64 - (*decoder).tnt.count as c_int;
    let mut data = eptw_data { bit_countdown: n, payload: (*decoder).tnt.payload >> n };
    (*decoder).emulated_ptwrite = false;
    intel_pt_pkt_lookahead(decoder, intel_pt_eptw_lookahead_cb, &mut data as *mut _ as *mut c_void);
    if data.bit_countdown != 0 { return -ECONNRESET; }
    (*decoder).state.type_ = INTEL_PT_PTW;
    (*decoder).state.from_ip = (*decoder).ip;
    (*decoder).state.to_ip = 0;
    (*decoder).state.ptw_payload = data.payload;
    0
}

unsafe fn intel_pt_walk_tnt(decoder: *mut intel_pt_decoder) -> c_int {
    let mut insn: intel_pt_insn = mem::zeroed();
    loop {
        if (*decoder).emulated_ptwrite { return intel_pt_emulated_ptwrite(decoder); }
        let err = intel_pt_walk_insn(decoder, &mut insn, 0);
        if err == INTEL_PT_RETURN {
            (*decoder).emulated_ptwrite = insn.emulated_ptwrite;
            return 0;
        }
        if err != 0 {
            (*decoder).emulated_ptwrite = false;
            return err;
        }
        if insn.op == INTEL_PT_OP_RET {
            if !(*decoder).return_compression || (*decoder).ret_addr == 0 || ((*decoder).tnt.payload & BIT63) == 0 {
                (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_ERR_RESYNC;
                return -ENOENT;
            }
            (*decoder).tnt.count -= 1;
            (*decoder).pkt_state = if (*decoder).tnt.count != 0 { intel_pt_pkt_state::INTEL_PT_STATE_TNT_CONT } else { intel_pt_pkt_state::INTEL_PT_STATE_IN_SYNC };
            (*decoder).tnt.payload <<= 1;
            (*decoder).state.from_ip = (*decoder).ip;
            (*decoder).ip = (*decoder).ret_addr;
            (*decoder).state.to_ip = (*decoder).ip;
            return 0;
        }
        if insn.branch == INTEL_PT_BR_INDIRECT {
            let err = intel_pt_get_next_packet(decoder);
            if err != 0 { return err; }
            if (*decoder).packet.type_ != INTEL_PT_TIP || (*decoder).packet.count == 0 {
                (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_ERR_RESYNC;
                (*decoder).pkt_step = 0;
                return -ENOENT;
            }
            intel_pt_set_last_ip(decoder);
            (*decoder).state.from_ip = (*decoder).ip;
            (*decoder).state.to_ip = (*decoder).last_ip;
            (*decoder).ip = (*decoder).last_ip;
            intel_pt_update_nr(decoder);
            intel_pt_sample_iflag_chg(decoder);
            return 0;
        }
        if insn.branch == INTEL_PT_BR_CONDITIONAL {
            (*decoder).tnt.count -= 1;
            (*decoder).pkt_state = if (*decoder).tnt.count != 0 { intel_pt_pkt_state::INTEL_PT_STATE_TNT_CONT } else { intel_pt_pkt_state::INTEL_PT_STATE_IN_SYNC };
            if (*decoder).tnt.payload & BIT63 != 0 {
                (*decoder).tnt.payload <<= 1;
                (*decoder).state.from_ip = (*decoder).ip;
                (*decoder).ip += insn.length + insn.rel;
                (*decoder).state.to_ip = (*decoder).ip;
                return 0;
            }
            if (*decoder).state.type_ & INTEL_PT_INSTRUCTION != 0 {
                (*decoder).tnt.payload <<= 1;
                (*decoder).state.type_ = INTEL_PT_INSTRUCTION;
                (*decoder).state.from_ip = (*decoder).ip;
                (*decoder).state.to_ip = 0;
                (*decoder).ip += insn.length;
                return 0;
            }
            (*decoder).sample_cyc = false;
            (*decoder).ip += insn.length;
            if (*decoder).tnt.count == 0 {
                intel_pt_update_sample_time(decoder);
                return -EAGAIN;
            }
            (*decoder).tnt.payload <<= 1;
            continue;
        }
        return intel_pt_bug(decoder);
    }
}

unsafe fn intel_pt_mode_tsx(decoder: *mut intel_pt_decoder, no_tip: *mut bool) -> c_int {
    let fup_tx_flags = ((*decoder).packet.payload as c_uint) & (INTEL_PT_IN_TX | INTEL_PT_ABORT_TX);
    let err = intel_pt_get_next_packet(decoder);
    if err != 0 { return err; }
    if (*decoder).packet.type_ == INTEL_PT_FUP {
        (*decoder).fup_tx_flags = fup_tx_flags;
        (*decoder).set_fup_tx_flags = true;
        if (*decoder).fup_tx_flags & INTEL_PT_ABORT_TX == 0 { *no_tip = true; }
    } else {
        intel_pt_update_in_tx(decoder);
    }
    0
}

unsafe fn intel_pt_evd(decoder: *mut intel_pt_decoder) -> c_int {
    if (*decoder).evd_cnt >= INTEL_PT_MAX_EVDS as c_int { return -ENOSYS; }
    (*decoder).evd[(*decoder).evd_cnt as usize] = intel_pt_evd { type_: (*decoder).packet.count, payload: (*decoder).packet.payload };
    (*decoder).evd_cnt += 1;
    0
}

fn intel_pt_8b_tsc(mut timestamp: u64, ref_timestamp: u64) -> u64 {
    timestamp |= ref_timestamp & (0xffu64 << 56);
    if timestamp < ref_timestamp {
        if ref_timestamp - timestamp > (1u64 << 55) { timestamp += 1u64 << 56; }
    } else if timestamp - ref_timestamp > (1u64 << 55) {
        timestamp -= 1u64 << 56;
    }
    timestamp
}

unsafe fn intel_pt_time_in_range(decoder: *mut intel_pt_decoder, timestamp: u64) -> bool {
    let mut max_timestamp = (*decoder).buf_timestamp;
    if max_timestamp == 0 {
        max_timestamp = (*decoder).last_reliable_timestamp + 0x400000000;
    }
    timestamp >= (*decoder).last_reliable_timestamp && timestamp < max_timestamp
}

unsafe fn intel_pt_calc_tsc_timestamp(decoder: *mut intel_pt_decoder) {
    let mut bad = false;
    (*decoder).have_tma = false;
    if (*decoder).ref_timestamp != 0 {
        let timestamp = intel_pt_8b_tsc((*decoder).packet.payload, (*decoder).ref_timestamp);
        (*decoder).tsc_timestamp = timestamp;
        (*decoder).timestamp = timestamp;
        (*decoder).ref_timestamp = 0;
        (*decoder).timestamp_insn_cnt = 0;
    } else if (*decoder).timestamp != 0 {
        let mut timestamp = (*decoder).packet.payload | ((*decoder).timestamp & (0xffu64 << 56));
        (*decoder).tsc_timestamp = timestamp;
        if timestamp < (*decoder).timestamp && (*decoder).timestamp - timestamp < (*decoder).tsc_slip as u64 {
            timestamp = (*decoder).timestamp;
        }
        if timestamp < (*decoder).timestamp {
            if (*decoder).buf_timestamp == 0 || timestamp + (1u64 << 56) < (*decoder).buf_timestamp {
                timestamp += 1u64 << 56;
                (*decoder).tsc_timestamp = timestamp;
            } else {
                timestamp = (*decoder).timestamp;
                bad = true;
            }
        }
        if (*decoder).vm_time_correlation && (bad || !intel_pt_time_in_range(decoder, timestamp)) {
            let _ = intel_pt_print_once(decoder, intel_pt_p_once::INTEL_PT_PRT_ONCE_ERANGE);
        }
        (*decoder).timestamp = timestamp;
        (*decoder).timestamp_insn_cnt = 0;
    }
    if (*decoder).last_packet_type == INTEL_PT_CYC {
        (*decoder).cyc_ref_timestamp = (*decoder).timestamp;
        (*decoder).cycle_cnt = 0;
        (*decoder).have_calc_cyc_to_tsc = false;
        intel_pt_calc_cyc_to_tsc(decoder, false);
    }
}

unsafe fn intel_pt_overflow(decoder: *mut intel_pt_decoder) -> c_int {
    intel_pt_clear_tx_flags(decoder);
    intel_pt_set_nr(decoder);
    (*decoder).timestamp_insn_cnt = 0;
    (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_IN_SYNC;
    (*decoder).state.from_ip = (*decoder).ip;
    (*decoder).ip = 0;
    (*decoder).pge = false;
    intel_pt_clear_fup_event(decoder);
    (*decoder).overflow = true;
    -EOVERFLOW
}

unsafe fn intel_pt_mtc_cyc_cnt_pge(decoder: *mut intel_pt_decoder) {
    if (*decoder).have_cyc { return; }
    (*decoder).cyc_cnt_timestamp = (*decoder).timestamp;
    (*decoder).base_cyc_cnt = (*decoder).tot_cyc_cnt;
}

unsafe fn intel_pt_mtc_cyc_cnt_cbr(decoder: *mut intel_pt_decoder) {
    (*decoder).tsc_to_cyc = (*decoder).cbr as f64 / (*decoder).max_non_turbo_ratio_fp;
    if (*decoder).pge { intel_pt_mtc_cyc_cnt_pge(decoder); }
}

unsafe fn intel_pt_mtc_cyc_cnt_upd(decoder: *mut intel_pt_decoder) {
    if (*decoder).have_cyc { return; }
    (*decoder).sample_cyc = true;
    if !(*decoder).pge || (*decoder).timestamp <= (*decoder).cyc_cnt_timestamp { return; }
    let tsc_delta = (*decoder).timestamp - (*decoder).cyc_cnt_timestamp;
    let tot_cyc_cnt = (tsc_delta as f64 * (*decoder).tsc_to_cyc) as u64 + (*decoder).base_cyc_cnt;
    if tot_cyc_cnt > (*decoder).tot_cyc_cnt { (*decoder).tot_cyc_cnt = tot_cyc_cnt; }
}

unsafe fn intel_pt_calc_tma(decoder: *mut intel_pt_decoder) {
    let ctc = (*decoder).packet.payload as u32;
    let fc = (*decoder).packet.count;
    let ctc_rem = ctc & (*decoder).ctc_rem_mask;
    if (*decoder).tsc_ctc_ratio_d == 0 { return; }
    if (*decoder).pge && !(*decoder).in_psb { intel_pt_mtc_cyc_cnt_pge(decoder); } else { intel_pt_mtc_cyc_cnt_upd(decoder); }
    (*decoder).last_mtc = (ctc >> (*decoder).mtc_shift) & 0xff;
    (*decoder).last_ctc = (ctc - ctc_rem) as u64;
    (*decoder).ctc_timestamp = (*decoder).tsc_timestamp - fc as u64;
    (*decoder).ctc_timestamp -= if (*decoder).tsc_ctc_mult != 0 {
        ctc_rem as u64 * (*decoder).tsc_ctc_mult as u64
    } else {
        multdiv(ctc_rem as u64, (*decoder).tsc_ctc_ratio_n, (*decoder).tsc_ctc_ratio_d)
    };
    (*decoder).ctc_delta = 0;
    (*decoder).have_tma = true;
    (*decoder).fixup_last_mtc = true;
}

unsafe fn intel_pt_calc_mtc_timestamp(decoder: *mut intel_pt_decoder) {
    if !(*decoder).have_tma { return; }
    let mtc = (*decoder).packet.payload as u32;
    if (*decoder).mtc_shift > 8 && (*decoder).fixup_last_mtc {
        (*decoder).fixup_last_mtc = false;
        intel_pt_fixup_last_mtc(mtc, (*decoder).mtc_shift, &mut (*decoder).last_mtc);
    }
    let mtc_delta = if mtc > (*decoder).last_mtc { mtc - (*decoder).last_mtc } else { mtc + 256 - (*decoder).last_mtc };
    (*decoder).ctc_delta += (mtc_delta as u64) << (*decoder).mtc_shift;
    let timestamp = (*decoder).ctc_timestamp + if (*decoder).tsc_ctc_mult != 0 {
        (*decoder).ctc_delta * (*decoder).tsc_ctc_mult as u64
    } else {
        multdiv((*decoder).ctc_delta, (*decoder).tsc_ctc_ratio_n, (*decoder).tsc_ctc_ratio_d)
    };
    if timestamp >= (*decoder).timestamp { (*decoder).timestamp = timestamp; }
    intel_pt_mtc_cyc_cnt_upd(decoder);
    (*decoder).timestamp_insn_cnt = 0;
    (*decoder).last_mtc = mtc;
    if (*decoder).last_packet_type == INTEL_PT_CYC {
        (*decoder).cyc_ref_timestamp = (*decoder).timestamp;
        (*decoder).cycle_cnt = 0;
        (*decoder).have_calc_cyc_to_tsc = false;
        intel_pt_calc_cyc_to_tsc(decoder, true);
    }
}

unsafe fn intel_pt_calc_cbr(decoder: *mut intel_pt_decoder) {
    let cbr = ((*decoder).packet.payload & 0xff) as c_uint;
    (*decoder).cbr_payload = (*decoder).packet.payload;
    if (*decoder).cbr == cbr { return; }
    (*decoder).cbr = cbr;
    (*decoder).cbr_cyc_to_tsc = (*decoder).max_non_turbo_ratio_fp / cbr as f64;
    (*decoder).cyc_ref_timestamp = (*decoder).timestamp;
    (*decoder).cycle_cnt = 0;
    intel_pt_mtc_cyc_cnt_cbr(decoder);
}

unsafe fn intel_pt_calc_cyc_timestamp(decoder: *mut intel_pt_decoder) {
    let mut timestamp = (*decoder).cyc_ref_timestamp;
    (*decoder).have_cyc = true;
    (*decoder).cycle_cnt += (*decoder).packet.payload;
    if (*decoder).pge { (*decoder).tot_cyc_cnt += (*decoder).packet.payload; }
    (*decoder).sample_cyc = true;
    if (*decoder).cyc_ref_timestamp == 0 { return; }
    if (*decoder).have_calc_cyc_to_tsc {
        timestamp += ((*decoder).cycle_cnt as f64 * (*decoder).calc_cyc_to_tsc) as u64;
    } else if (*decoder).cbr != 0 {
        timestamp += ((*decoder).cycle_cnt as f64 * (*decoder).cbr_cyc_to_tsc) as u64;
    } else {
        return;
    }
    if timestamp >= (*decoder).timestamp { (*decoder).timestamp = timestamp; }
    (*decoder).timestamp_insn_cnt = 0;
}

unsafe fn intel_pt_bbp(decoder: *mut intel_pt_decoder) {
    if (*decoder).prev_pkt_ctx == INTEL_PT_NO_CTX {
        memset((*decoder).state.items.mask.as_mut_ptr() as *mut c_void, 0, mem::size_of_val(&(*decoder).state.items.mask));
        (*decoder).state.items.is_32_bit = false;
    }
    (*decoder).blk_type = (*decoder).packet.payload as intel_pt_blk_type;
    (*decoder).blk_type_pos = intel_pt_blk_type_pos((*decoder).blk_type);
    if (*decoder).blk_type == INTEL_PT_GP_REGS {
        (*decoder).state.items.is_32_bit = (*decoder).packet.count != 0;
    }
}

unsafe fn intel_pt_bip(decoder: *mut intel_pt_decoder) {
    let id = (*decoder).packet.count;
    let bit = 1u32 << id;
    let pos = (*decoder).blk_type_pos;
    if pos < 0 || id as usize >= INTEL_PT_BLK_ITEM_ID_CNT { return; }
    (*decoder).state.items.mask[pos as usize] |= bit;
    (*decoder).state.items.val[pos as usize][id as usize] = (*decoder).packet.payload;
}

unsafe fn intel_pt_walk_psbend(decoder: *mut intel_pt_decoder) -> c_int {
    (*decoder).in_psb = true;
    loop {
        let mut err = intel_pt_get_next_packet(decoder);
        if err != 0 { (*decoder).in_psb = false; return err; }
        match (*decoder).packet.type_ {
            INTEL_PT_PSBEND => { (*decoder).in_psb = false; return 0; }
            INTEL_PT_TIP_PGD | INTEL_PT_TIP_PGE | INTEL_PT_TIP | INTEL_PT_TNT | INTEL_PT_TRACESTOP
            | INTEL_PT_BAD | INTEL_PT_PSB | INTEL_PT_PTWRITE | INTEL_PT_PTWRITE_IP | INTEL_PT_EXSTOP
            | INTEL_PT_EXSTOP_IP | INTEL_PT_MWAIT | INTEL_PT_PWRE | INTEL_PT_PWRX | INTEL_PT_BBP
            | INTEL_PT_BIP | INTEL_PT_BEP | INTEL_PT_BEP_IP | INTEL_PT_CFE | INTEL_PT_CFE_IP | INTEL_PT_EVD => {
                (*decoder).have_tma = false;
                err = -EAGAIN;
                (*decoder).in_psb = false;
                return err;
            }
            INTEL_PT_OVF => { err = intel_pt_overflow(decoder); (*decoder).in_psb = false; return err; }
            INTEL_PT_TSC => intel_pt_calc_tsc_timestamp(decoder),
            INTEL_PT_TMA => intel_pt_calc_tma(decoder),
            INTEL_PT_CBR => intel_pt_calc_cbr(decoder),
            INTEL_PT_MODE_EXEC => intel_pt_mode_exec_status(decoder),
            INTEL_PT_PIP => intel_pt_set_pip(decoder),
            INTEL_PT_FUP => {
                (*decoder).pge = true;
                if (*decoder).packet.count != 0 {
                    intel_pt_set_last_ip(decoder);
                    (*decoder).psb_ip = (*decoder).last_ip;
                }
            }
            INTEL_PT_MODE_TSX => intel_pt_update_in_tx(decoder),
            INTEL_PT_MTC => {
                intel_pt_calc_mtc_timestamp(decoder);
                if (*decoder).period_type == INTEL_PT_PERIOD_MTC { (*decoder).state.type_ |= INTEL_PT_INSTRUCTION; }
            }
            INTEL_PT_CYC => intel_pt_calc_cyc_timestamp(decoder),
            _ => {}
        }
    }
}

unsafe fn intel_pt_walk_fup_tip(decoder: *mut intel_pt_decoder) -> c_int {
    if (*decoder).tx_flags & INTEL_PT_ABORT_TX != 0 {
        (*decoder).tx_flags = 0;
        (*decoder).state.flags &= !INTEL_PT_IN_TX;
        (*decoder).state.flags |= INTEL_PT_ABORT_TX;
    } else {
        (*decoder).state.flags |= INTEL_PT_ASYNC;
    }
    loop {
        let err = intel_pt_get_next_packet(decoder);
        if err != 0 { return err; }
        match (*decoder).packet.type_ {
            INTEL_PT_CBR => intel_pt_calc_cbr(decoder),
            INTEL_PT_OVF => return intel_pt_overflow(decoder),
            INTEL_PT_TIP_PGD => {
                (*decoder).state.from_ip = (*decoder).ip;
                if (*decoder).packet.count == 0 { (*decoder).state.to_ip = 0; } else { intel_pt_set_ip(decoder); (*decoder).state.to_ip = (*decoder).ip; }
                (*decoder).pge = false;
                (*decoder).continuous_period = false;
                (*decoder).state.type_ |= INTEL_PT_TRACE_END;
                intel_pt_update_nr(decoder);
                return 0;
            }
            INTEL_PT_TIP_PGE => {
                (*decoder).pge = true;
                (*decoder).state.from_ip = 0;
                if (*decoder).packet.count == 0 { (*decoder).state.to_ip = 0; } else { intel_pt_set_ip(decoder); (*decoder).state.to_ip = (*decoder).ip; }
                (*decoder).state.type_ |= INTEL_PT_TRACE_BEGIN;
                intel_pt_mtc_cyc_cnt_pge(decoder);
                intel_pt_set_nr(decoder);
                return 0;
            }
            INTEL_PT_TIP => {
                (*decoder).state.from_ip = (*decoder).ip;
                if (*decoder).packet.count == 0 { (*decoder).state.to_ip = 0; } else { intel_pt_set_ip(decoder); (*decoder).state.to_ip = (*decoder).ip; }
                intel_pt_update_nr(decoder);
                intel_pt_sample_iflag_chg(decoder);
                return 0;
            }
            INTEL_PT_PIP => intel_pt_update_pip(decoder),
            INTEL_PT_MTC => { intel_pt_calc_mtc_timestamp(decoder); if (*decoder).period_type == INTEL_PT_PERIOD_MTC { (*decoder).state.type_ |= INTEL_PT_INSTRUCTION; } }
            INTEL_PT_CYC => intel_pt_calc_cyc_timestamp(decoder),
            INTEL_PT_MODE_EXEC => intel_pt_mode_exec(decoder),
            INTEL_PT_VMCS | INTEL_PT_MNT | INTEL_PT_PAD => {}
            _ => {
                (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_ERR_RESYNC;
                (*decoder).pkt_step = 0;
                return -ENOENT;
            }
        }
    }
}

unsafe fn intel_pt_resample(decoder: *mut intel_pt_decoder) -> c_int {
    (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_IN_SYNC;
    (*decoder).state.type_ = INTEL_PT_INSTRUCTION;
    (*decoder).state.from_ip = (*decoder).ip;
    (*decoder).state.to_ip = 0;
    0
}

#[repr(C)]
struct intel_pt_vm_tsc_info {
    pip_packet: intel_pt_pkt,
    vmcs_packet: intel_pt_pkt,
    tma_packet: intel_pt_pkt,
    tsc: bool,
    pip: bool,
    vmcs: bool,
    tma: bool,
    psbend: bool,
    ctc_delta: u64,
    last_ctc: u64,
    max_lookahead: c_int,
}

unsafe fn intel_pt_vm_psb_lookahead_cb(pkt_info: *mut intel_pt_pkt_info) -> c_int {
    let data = (*pkt_info).data as *mut intel_pt_vm_tsc_info;
    match (*pkt_info).packet.type_ {
        INTEL_PT_TSC => (*data).tsc = true,
        INTEL_PT_TMA => { (*data).tma_packet = (*pkt_info).packet; (*data).tma = true; }
        INTEL_PT_PIP => { (*data).pip_packet = (*pkt_info).packet; (*data).pip = true; }
        INTEL_PT_VMCS => { (*data).vmcs_packet = (*pkt_info).packet; (*data).vmcs = true; }
        INTEL_PT_PSBEND => { (*data).psbend = true; return 1; }
        INTEL_PT_PAD | INTEL_PT_MNT | INTEL_PT_MODE_EXEC | INTEL_PT_MODE_TSX | INTEL_PT_MTC | INTEL_PT_FUP | INTEL_PT_CYC | INTEL_PT_CBR => {}
        _ => return 1,
    }
    0
}

#[repr(C)]
struct intel_pt_ovf_fup_info {
    max_lookahead: c_int,
    found: bool,
}

unsafe fn intel_pt_ovf_fup_lookahead_cb(pkt_info: *mut intel_pt_pkt_info) -> c_int {
    let data = (*pkt_info).data as *mut intel_pt_ovf_fup_info;
    if (*pkt_info).packet.type_ == INTEL_PT_CYC || (*pkt_info).packet.type_ == INTEL_PT_MTC || (*pkt_info).packet.type_ == INTEL_PT_TSC {
        (*data).max_lookahead -= 1;
        return ((*data).max_lookahead == 0) as c_int;
    }
    (*data).found = (*pkt_info).packet.type_ == INTEL_PT_FUP;
    1
}

unsafe fn intel_pt_ovf_fup_lookahead(decoder: *mut intel_pt_decoder) -> bool {
    let mut data = intel_pt_ovf_fup_info { max_lookahead: 16, found: false };
    intel_pt_pkt_lookahead(decoder, intel_pt_ovf_fup_lookahead_cb, &mut data as *mut _ as *mut c_void);
    data.found
}

unsafe fn intel_pt_tma_lookahead_cb(pkt_info: *mut intel_pt_pkt_info) -> c_int {
    let data = (*pkt_info).data as *mut intel_pt_vm_tsc_info;
    if (*pkt_info).packet.type_ == INTEL_PT_CYC || (*pkt_info).packet.type_ == INTEL_PT_MTC {
        (*data).max_lookahead -= 1;
        return ((*data).max_lookahead == 0) as c_int;
    }
    if (*pkt_info).packet.type_ == INTEL_PT_TMA {
        (*data).tma_packet = (*pkt_info).packet;
        (*data).tma = true;
    }
    1
}

unsafe fn intel_pt_ctc_to_tsc(decoder: *mut intel_pt_decoder, ctc: u64) -> u64 {
    if (*decoder).tsc_ctc_mult != 0 { ctc * (*decoder).tsc_ctc_mult as u64 } else { multdiv(ctc, (*decoder).tsc_ctc_ratio_n, (*decoder).tsc_ctc_ratio_d) }
}

unsafe fn intel_pt_calc_expected_tsc(decoder: *mut intel_pt_decoder, ctc: u32, fc: u32, last_ctc_timestamp: u64, ctc_delta: u64, last_ctc: u32) -> u64 {
    let last_mtc_ctc = last_ctc as u64 + ctc_delta;
    let delta = ((ctc as u16).wrapping_sub(last_mtc_ctc as u16) as i16) as i64 as u64;
    let new_ctc_delta = ctc_delta.wrapping_add(delta);
    last_ctc_timestamp + intel_pt_ctc_to_tsc(decoder, new_ctc_delta) + fc as u64
}

unsafe fn intel_pt_expected_tsc(decoder: *mut intel_pt_decoder, data: *mut intel_pt_vm_tsc_info) -> u64 {
    let ctc = (*data).tma_packet.payload as u32;
    let fc = (*data).tma_packet.count;
    intel_pt_calc_expected_tsc(decoder, ctc, fc, (*decoder).ctc_timestamp, (*data).ctc_delta, (*data).last_ctc as u32)
}

unsafe fn intel_pt_translate_vm_tsc(decoder: *mut intel_pt_decoder, vmcs_info: *mut intel_pt_vmcs_info) {
    (*decoder).packet.payload = (*decoder).packet.payload.wrapping_sub((*vmcs_info).tsc_offset);
    (*decoder).packet.payload &= SEVEN_BYTES;
    if !(*decoder).vm_tm_corr_dry_run {
        memcpy((*decoder).buf.add(1) as *mut c_void, &(*decoder).packet.payload as *const _ as *const c_void, 7);
    }
}

unsafe fn intel_pt_translate_vm_tsc_offset(decoder: *mut intel_pt_decoder, tsc_offset: u64) {
    let mut vmcs_info = intel_pt_vmcs_info { vmcs: NO_VMCS, tsc_offset, reliable: false, error_printed: false };
    intel_pt_translate_vm_tsc(decoder, &mut vmcs_info);
}

fn in_vm(pip_payload: u64) -> bool { pip_payload & 1 != 0 }
unsafe fn pip_in_vm(pip_packet: *mut intel_pt_pkt) -> bool { (*pip_packet).payload & 1 != 0 }
unsafe fn intel_pt_print_vmcs_info(_vmcs_info: *mut intel_pt_vmcs_info) {}

unsafe fn intel_pt_vm_tm_corr_psb(decoder: *mut intel_pt_decoder, data: *mut intel_pt_vm_tsc_info) {
    memset(data as *mut c_void, 0, mem::size_of::<intel_pt_vm_tsc_info>());
    (*data).ctc_delta = (*decoder).ctc_delta;
    (*data).last_ctc = (*decoder).last_ctc;
    intel_pt_pkt_lookahead(decoder, intel_pt_vm_psb_lookahead_cb, data as *mut c_void);
    (*decoder).in_psb = (*data).psbend;
}

unsafe fn intel_pt_vm_tm_corr_first_tsc(decoder: *mut intel_pt_decoder, data: *mut intel_pt_vm_tsc_info, vmcs_info: *mut intel_pt_vmcs_info, host_tsc: u64) {
    if (*data).pip {
        if pip_in_vm(&mut (*data).pip_packet) {
            if !vmcs_info.is_null() && (*vmcs_info).tsc_offset != 0 {
                intel_pt_translate_vm_tsc(decoder, vmcs_info);
                (*decoder).vm_tm_corr_reliable = true;
            }
        } else {
            (*decoder).vm_tm_corr_reliable = true;
        }
    } else {
        (*decoder).vm_tm_corr_reliable = false;
        if !intel_pt_time_in_range(decoder, host_tsc) && !vmcs_info.is_null() && (*vmcs_info).tsc_offset != 0 {
            intel_pt_translate_vm_tsc(decoder, vmcs_info);
        }
    }
}

unsafe fn intel_pt_vm_tm_corr_tsc(decoder: *mut intel_pt_decoder, data: *mut intel_pt_vm_tsc_info) {
    if !(*decoder).in_psb {
        memset(data as *mut c_void, 0, mem::size_of::<intel_pt_vm_tsc_info>());
        (*data).ctc_delta = (*decoder).ctc_delta;
        (*data).last_ctc = (*decoder).last_ctc;
        (*data).max_lookahead = 16;
        intel_pt_pkt_lookahead(decoder, intel_pt_tma_lookahead_cb, data as *mut c_void);
        if (*decoder).pge {
            (*data).pip = true;
            (*data).pip_packet.payload = (*decoder).pip_payload;
        }
    }
    if !(*data).tma { return; }
    let mut vmcs = if (*data).vmcs { (*data).vmcs_packet.payload } else { (*decoder).vmcs };
    if vmcs == NO_VMCS { vmcs = 0; }
    let vmcs_info = ((*decoder).findnew_vmcs_info.unwrap())((*decoder).data, vmcs);
    let ref_timestamp = if (*decoder).timestamp != 0 { (*decoder).timestamp } else { (*decoder).buf_timestamp };
    let host_tsc = intel_pt_8b_tsc((*decoder).packet.payload, ref_timestamp);
    if (*decoder).ctc_timestamp == 0 {
        intel_pt_vm_tm_corr_first_tsc(decoder, data, vmcs_info, host_tsc);
        return;
    }
    let expected_tsc = intel_pt_expected_tsc(decoder, data);
    let tsc_offset = host_tsc.wrapping_sub(expected_tsc);
    let mut reliable = true;
    if (*data).pip {
        if pip_in_vm(&mut (*data).pip_packet) {
            if vmcs_info.is_null() {
                intel_pt_translate_vm_tsc_offset(decoder, tsc_offset);
                (*decoder).vm_tm_corr_reliable = false;
                return;
            }
        } else {
            (*decoder).last_reliable_timestamp = host_tsc;
            (*decoder).vm_tm_corr_reliable = true;
            return;
        }
    } else {
        reliable = false;
        if (*decoder).in_psb {
            if tsc_offset == 0 { return; }
            if !vmcs_info.is_null() && (tsc_offset & SEVEN_BYTES) == ((*vmcs_info).tsc_offset & SEVEN_BYTES) {
                /* Assume Guest */
            }
        }
        if host_tsc >= expected_tsc && intel_pt_time_in_range(decoder, host_tsc) {
            (*decoder).vm_tm_corr_reliable = false;
            return;
        }
    }
    let mut assign = false;
    let mut assign_reliable = false;
    if !vmcs_info.is_null() && (*vmcs_info).vmcs != 0 {
        if (*vmcs_info).tsc_offset != 0 && (*vmcs_info).reliable {
            assign = false;
        } else if (*decoder).in_psb && (*data).pip && (*decoder).vm_tm_corr_reliable && (*decoder).vm_tm_corr_continuous && (*decoder).vm_tm_corr_same_buf {
            assign = true;
            assign_reliable = true;
        } else if (*decoder).in_psb && (*data).pip && (*decoder).vm_tm_corr_same_buf {
            assign = true;
            assign_reliable = false;
        }
    }
    if assign && ((*vmcs_info).tsc_offset != tsc_offset || (*vmcs_info).reliable != assign_reliable) {
        let print = (*vmcs_info).tsc_offset != tsc_offset;
        (*vmcs_info).tsc_offset = tsc_offset;
        (*vmcs_info).reliable = assign_reliable;
        if print { intel_pt_print_vmcs_info(vmcs_info); }
    }
    if !vmcs_info.is_null() && (*vmcs_info).tsc_offset != 0 {
        if !(*vmcs_info).reliable { reliable = false; }
        intel_pt_translate_vm_tsc(decoder, vmcs_info);
    } else {
        reliable = false;
        intel_pt_translate_vm_tsc_offset(decoder, tsc_offset);
    }
    (*decoder).vm_tm_corr_reliable = reliable;
}

unsafe fn intel_pt_vm_tm_corr_pebs_tsc(decoder: *mut intel_pt_decoder) {
    let guest_tsc = (*decoder).packet.payload;
    let mut vmcs = (*decoder).vmcs;
    if vmcs == NO_VMCS { vmcs = 0; }
    let vmcs_info = ((*decoder).findnew_vmcs_info.unwrap())((*decoder).data, vmcs);
    if (*decoder).pge {
        if !in_vm((*decoder).pip_payload) { return; }
    } else if intel_pt_time_in_range(decoder, guest_tsc) {
        return;
    }
    let host_tsc = if !vmcs_info.is_null() {
        let host = ((guest_tsc & SEVEN_BYTES).wrapping_sub((*vmcs_info).tsc_offset)) & SEVEN_BYTES;
        intel_pt_8b_tsc(host, (*decoder).timestamp)
    } else {
        (*decoder).timestamp
    };
    (*decoder).packet.payload = host_tsc;
    if !(*decoder).vm_tm_corr_dry_run {
        memcpy((*decoder).buf.add(1) as *mut c_void, &host_tsc as *const _ as *const c_void, 8);
    }
}

unsafe fn intel_pt_vm_time_correlation(decoder: *mut intel_pt_decoder) -> c_int {
    let mut data: intel_pt_vm_tsc_info = mem::zeroed();
    if (*decoder).in_psb { intel_pt_vm_tm_corr_psb(decoder, &mut data); }
    loop {
        let err = intel_pt_get_next_packet(decoder);
        if err == -ENOLINK { continue; }
        if err != 0 { return err; }
        match (*decoder).packet.type_ {
            INTEL_PT_TIP_PGD => { (*decoder).pge = false; (*decoder).vm_tm_corr_continuous = false; }
            INTEL_PT_TNT | INTEL_PT_TIP | INTEL_PT_TIP_PGE => (*decoder).pge = true,
            INTEL_PT_OVF => {
                (*decoder).in_psb = false;
                let pge = (*decoder).pge;
                (*decoder).pge = intel_pt_ovf_fup_lookahead(decoder);
                if pge != (*decoder).pge {}
                if !(*decoder).pge { (*decoder).vm_tm_corr_continuous = false; }
            }
            INTEL_PT_FUP => if (*decoder).in_psb { (*decoder).pge = true; }
            INTEL_PT_TRACESTOP => { (*decoder).pge = false; (*decoder).vm_tm_corr_continuous = false; (*decoder).have_tma = false; }
            INTEL_PT_PSB => intel_pt_vm_tm_corr_psb(decoder, &mut data),
            INTEL_PT_PIP => (*decoder).pip_payload = (*decoder).packet.payload,
            INTEL_PT_MTC => intel_pt_calc_mtc_timestamp(decoder),
            INTEL_PT_TSC => {
                intel_pt_vm_tm_corr_tsc(decoder, &mut data);
                intel_pt_calc_tsc_timestamp(decoder);
                (*decoder).vm_tm_corr_same_buf = true;
                (*decoder).vm_tm_corr_continuous = (*decoder).pge;
            }
            INTEL_PT_TMA => intel_pt_calc_tma(decoder),
            INTEL_PT_CYC => intel_pt_calc_cyc_timestamp(decoder),
            INTEL_PT_CBR => intel_pt_calc_cbr(decoder),
            INTEL_PT_PSBEND => { (*decoder).in_psb = false; data.psbend = false; }
            INTEL_PT_VMCS => if (*decoder).packet.payload != NO_VMCS { (*decoder).vmcs = (*decoder).packet.payload; }
            INTEL_PT_BBP => (*decoder).blk_type = (*decoder).packet.payload as intel_pt_blk_type,
            INTEL_PT_BIP => if (*decoder).blk_type == INTEL_PT_PEBS_BASIC && (*decoder).packet.count == 2 { intel_pt_vm_tm_corr_pebs_tsc(decoder); }
            INTEL_PT_BEP | INTEL_PT_BEP_IP => (*decoder).blk_type = 0,
            _ => {}
        }
    }
}

const HOP_PROCESS: c_int = 0;
const HOP_IGNORE: c_int = 1;
const HOP_RETURN: c_int = 2;
const HOP_AGAIN: c_int = 3;

unsafe fn intel_pt_hop_trace(decoder: *mut intel_pt_decoder, no_tip: *mut bool, err: *mut c_int) -> c_int {
    *err = 0;
    if (*decoder).leap && !(*decoder).in_psb && (*decoder).packet.type_ != INTEL_PT_PSB {
        *err = intel_pt_scan_for_psb(decoder);
        if *err != 0 { return HOP_RETURN; }
    }
    match (*decoder).packet.type_ {
        INTEL_PT_TNT => HOP_IGNORE,
        INTEL_PT_TIP_PGD => {
            (*decoder).pge = false;
            if (*decoder).packet.count == 0 { intel_pt_set_nr(decoder); return HOP_IGNORE; }
            intel_pt_set_ip(decoder);
            (*decoder).state.type_ |= INTEL_PT_TRACE_END;
            (*decoder).state.from_ip = 0;
            (*decoder).state.to_ip = (*decoder).ip;
            intel_pt_update_nr(decoder);
            HOP_RETURN
        }
        INTEL_PT_TIP => {
            if (*decoder).packet.count == 0 { intel_pt_set_nr(decoder); return HOP_IGNORE; }
            intel_pt_set_ip(decoder);
            (*decoder).state.type_ = INTEL_PT_INSTRUCTION;
            (*decoder).state.from_ip = (*decoder).ip;
            (*decoder).state.to_ip = 0;
            intel_pt_update_nr(decoder);
            intel_pt_sample_iflag_chg(decoder);
            HOP_RETURN
        }
        INTEL_PT_FUP => {
            if (*decoder).packet.count == 0 { return HOP_IGNORE; }
            intel_pt_set_ip(decoder);
            if (*decoder).set_fup_mwait || (*decoder).set_fup_pwre { *no_tip = true; }
            if !(*decoder).branch_enable || !(*decoder).pge { *no_tip = true; }
            if *no_tip {
                (*decoder).state.type_ = INTEL_PT_INSTRUCTION;
                (*decoder).state.from_ip = (*decoder).ip;
                (*decoder).state.to_ip = 0;
                intel_pt_fup_event(decoder, *no_tip);
                return HOP_RETURN;
            }
            intel_pt_fup_event(decoder, *no_tip);
            (*decoder).state.type_ |= INTEL_PT_INSTRUCTION | INTEL_PT_BRANCH;
            *err = intel_pt_walk_fup_tip(decoder);
            if *err == 0 && (*decoder).state.to_ip != 0 { (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_RESAMPLE; }
            HOP_RETURN
        }
        INTEL_PT_PSB => {
            (*decoder).state.psb_offset = (*decoder).pos;
            (*decoder).psb_ip = 0;
            (*decoder).last_ip = 0;
            (*decoder).have_last_ip = true;
            *err = intel_pt_walk_psbend(decoder);
            if *err == -EAGAIN { return HOP_AGAIN; }
            if *err != 0 { return HOP_RETURN; }
            (*decoder).state.type_ = INTEL_PT_PSB_EVT;
            if (*decoder).psb_ip != 0 {
                (*decoder).state.type_ |= INTEL_PT_INSTRUCTION;
                (*decoder).ip = (*decoder).psb_ip;
            }
            (*decoder).state.from_ip = (*decoder).psb_ip;
            (*decoder).state.to_ip = 0;
            HOP_RETURN
        }
        _ => HOP_PROCESS,
    }
}

#[repr(C)]
struct intel_pt_psb_info {
    fup_packet: intel_pt_pkt,
    fup: bool,
    after_psbend: c_int,
}

unsafe fn intel_pt_psb_lookahead_cb(pkt_info: *mut intel_pt_pkt_info) -> c_int {
    let data = (*pkt_info).data as *mut intel_pt_psb_info;
    match (*pkt_info).packet.type_ {
        INTEL_PT_FUP => {
            if (*data).after_psbend != 0 { return 1; }
            if (*data).fup || (*pkt_info).packet.count == 0 { return 1; }
            (*data).fup_packet = (*pkt_info).packet;
            (*data).fup = true;
        }
        INTEL_PT_PSBEND => {
            if !(*data).fup { return 1; }
            (*data).after_psbend = 6;
        }
        INTEL_PT_TIP_PGE => {
            if (*data).after_psbend != 0 { (*data).fup = false; }
            return 1;
        }
        INTEL_PT_OVF | INTEL_PT_BAD | INTEL_PT_TNT | INTEL_PT_TIP_PGD | INTEL_PT_TIP | INTEL_PT_PSB | INTEL_PT_TRACESTOP => return 1,
        _ => {
            if (*data).after_psbend != 0 {
                (*data).after_psbend -= 1;
                if (*data).after_psbend == 0 { return 1; }
            }
        }
    }
    0
}

unsafe fn intel_pt_psb(decoder: *mut intel_pt_decoder) -> c_int {
    (*decoder).last_ip = 0;
    (*decoder).psb_ip = 0;
    (*decoder).have_last_ip = true;
    intel_pt_clear_stack(&mut (*decoder).stack);
    let err = intel_pt_walk_psbend(decoder);
    if err != 0 { return err; }
    (*decoder).state.type_ = INTEL_PT_PSB_EVT;
    (*decoder).state.from_ip = (*decoder).psb_ip;
    (*decoder).state.to_ip = 0;
    0
}

unsafe fn intel_pt_fup_in_psb(decoder: *mut intel_pt_decoder) -> c_int {
    if (*decoder).ip != (*decoder).last_ip {
        let err = intel_pt_walk_fup(decoder);
        if err == 0 || err != -EAGAIN { return err; }
    }
    (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_IN_SYNC;
    let err = intel_pt_psb(decoder);
    if err != 0 {
        (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_ERR_RESYNC;
        return -ENOENT;
    }
    0
}

unsafe fn intel_pt_psb_with_fup(decoder: *mut intel_pt_decoder, err: *mut c_int) -> bool {
    let mut data: intel_pt_psb_info = mem::zeroed();
    if !(*decoder).branch_enable { return false; }
    intel_pt_pkt_lookahead(decoder, intel_pt_psb_lookahead_cb, &mut data as *mut _ as *mut c_void);
    if !data.fup { return false; }
    (*decoder).packet = data.fup_packet;
    intel_pt_set_last_ip(decoder);
    (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_FUP_IN_PSB;
    *err = intel_pt_fup_in_psb(decoder);
    true
}

unsafe fn intel_pt_have_ip(decoder: *mut intel_pt_decoder) -> bool {
    (*decoder).packet.count != 0 && ((*decoder).have_last_ip || (*decoder).packet.count == 3 || (*decoder).packet.count == 6)
}

unsafe fn intel_pt_walk_trace(decoder: *mut intel_pt_decoder) -> c_int {
    let mut last_packet_type = INTEL_PT_PAD;
    let mut no_tip = false;
    loop {
        let mut err = intel_pt_get_next_packet(decoder);
        if err != 0 { return err; }
        'next: loop {
            err = 0;
            if (*decoder).cyc_threshold != 0 {
                if (*decoder).sample_cyc && last_packet_type != INTEL_PT_CYC { (*decoder).sample_cyc = false; }
                last_packet_type = (*decoder).packet.type_;
            }
            if (*decoder).hop {
                match intel_pt_hop_trace(decoder, &mut no_tip, &mut err) {
                    HOP_IGNORE => break 'next,
                    HOP_RETURN => return err,
                    HOP_AGAIN => continue 'next,
                    _ => {}
                }
            }
            match (*decoder).packet.type_ {
                INTEL_PT_TNT => {
                    if (*decoder).packet.count == 0 { break 'next; }
                    (*decoder).tnt = (*decoder).packet;
                    (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_TNT;
                    err = intel_pt_walk_tnt(decoder);
                    if err == -EAGAIN { break 'next; }
                    return err;
                }
                INTEL_PT_TIP_PGD => { if (*decoder).packet.count != 0 { intel_pt_set_last_ip(decoder); } (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_TIP_PGD; return intel_pt_walk_tip(decoder); }
                INTEL_PT_TIP_PGE => {
                    (*decoder).pge = true;
                    (*decoder).overflow = false;
                    intel_pt_mtc_cyc_cnt_pge(decoder);
                    intel_pt_set_nr(decoder);
                    if (*decoder).packet.count == 0 { break 'next; }
                    intel_pt_sample_iflag_chg(decoder);
                    intel_pt_set_ip(decoder);
                    (*decoder).state.from_ip = 0;
                    (*decoder).state.to_ip = (*decoder).ip;
                    (*decoder).state.type_ |= INTEL_PT_TRACE_BEGIN;
                    if (*decoder).hop { (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_RESAMPLE; }
                    return 0;
                }
                INTEL_PT_OVF => return intel_pt_overflow(decoder),
                INTEL_PT_TIP => { if (*decoder).packet.count != 0 { intel_pt_set_last_ip(decoder); } (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_TIP; return intel_pt_walk_tip(decoder); }
                INTEL_PT_FUP => {
                    if (*decoder).packet.count == 0 { no_tip = false; break 'next; }
                    intel_pt_set_last_ip(decoder);
                    if !(*decoder).branch_enable || !(*decoder).pge {
                        (*decoder).ip = (*decoder).last_ip;
                        if intel_pt_fup_event(decoder, no_tip) { return 0; }
                        no_tip = false;
                        break 'next;
                    }
                    if (*decoder).set_fup_mwait { no_tip = true; }
                    (*decoder).pkt_state = if no_tip { intel_pt_pkt_state::INTEL_PT_STATE_FUP_NO_TIP } else { intel_pt_pkt_state::INTEL_PT_STATE_FUP };
                    err = intel_pt_walk_fup(decoder);
                    if err != -EAGAIN { return err; }
                    if no_tip { no_tip = false; break 'next; }
                    return intel_pt_walk_fup_tip(decoder);
                }
                INTEL_PT_TRACESTOP => { (*decoder).pge = false; (*decoder).continuous_period = false; intel_pt_clear_tx_flags(decoder); (*decoder).have_tma = false; }
                INTEL_PT_PSB => {
                    (*decoder).state.psb_offset = (*decoder).pos;
                    (*decoder).psb_ip = 0;
                    if intel_pt_psb_with_fup(decoder, &mut err) { return err; }
                    err = intel_pt_psb(decoder);
                    if err == -EAGAIN { continue 'next; }
                    return err;
                }
                INTEL_PT_PIP => intel_pt_update_pip(decoder),
                INTEL_PT_MTC => {
                    intel_pt_calc_mtc_timestamp(decoder);
                    if (*decoder).period_type == INTEL_PT_PERIOD_MTC && (*decoder).mtc_insn && (*decoder).timestamp != 0 {
                        (*decoder).state.type_ = INTEL_PT_INSTRUCTION;
                        (*decoder).state.from_ip = (*decoder).ip;
                        (*decoder).state.to_ip = 0;
                        (*decoder).mtc_insn = false;
                        return 0;
                    }
                }
                INTEL_PT_TSC => intel_pt_calc_tsc_timestamp(decoder),
                INTEL_PT_TMA => intel_pt_calc_tma(decoder),
                INTEL_PT_CYC => intel_pt_calc_cyc_timestamp(decoder),
                INTEL_PT_CBR => { intel_pt_calc_cbr(decoder); if (*decoder).cbr != (*decoder).cbr_seen { (*decoder).state.type_ = 0; return 0; } }
                INTEL_PT_MODE_EXEC => {
                    intel_pt_mode_exec(decoder);
                    err = intel_pt_get_next_packet(decoder);
                    if err != 0 { return err; }
                    if (*decoder).packet.type_ == INTEL_PT_FUP { (*decoder).set_fup_mode_exec = true; no_tip = true; }
                    continue 'next;
                }
                INTEL_PT_MODE_TSX => {
                    if !(*decoder).pge || (*decoder).in_psb { intel_pt_update_in_tx(decoder); }
                    else { err = intel_pt_mode_tsx(decoder, &mut no_tip); if err != 0 { return err; } continue 'next; }
                }
                INTEL_PT_PTWRITE_IP => {
                    (*decoder).fup_ptw_payload = (*decoder).packet.payload;
                    err = intel_pt_get_next_packet(decoder);
                    if err != 0 { return err; }
                    if (*decoder).packet.type_ == INTEL_PT_FUP { (*decoder).set_fup_ptw = true; no_tip = true; }
                    continue 'next;
                }
                INTEL_PT_PTWRITE => { (*decoder).state.type_ = INTEL_PT_PTW; (*decoder).state.from_ip = (*decoder).ip; (*decoder).state.to_ip = 0; (*decoder).state.ptw_payload = (*decoder).packet.payload; return 0; }
                INTEL_PT_MWAIT => { (*decoder).fup_mwait_payload = (*decoder).packet.payload; (*decoder).set_fup_mwait = true; }
                INTEL_PT_PWRE => {
                    if (*decoder).set_fup_mwait { (*decoder).fup_pwre_payload = (*decoder).packet.payload; (*decoder).set_fup_pwre = true; }
                    else { (*decoder).state.type_ = INTEL_PT_PWR_ENTRY; (*decoder).state.from_ip = (*decoder).ip; (*decoder).state.to_ip = 0; (*decoder).state.pwrx_payload = (*decoder).packet.payload; return 0; }
                }
                INTEL_PT_EXSTOP_IP => { err = intel_pt_get_next_packet(decoder); if err != 0 { return err; } if (*decoder).packet.type_ == INTEL_PT_FUP { (*decoder).set_fup_exstop = true; no_tip = true; } continue 'next; }
                INTEL_PT_EXSTOP => { (*decoder).state.type_ = INTEL_PT_EX_STOP; (*decoder).state.from_ip = (*decoder).ip; (*decoder).state.to_ip = 0; return 0; }
                INTEL_PT_PWRX => { (*decoder).state.type_ = INTEL_PT_PWR_EXIT; (*decoder).state.from_ip = (*decoder).ip; (*decoder).state.to_ip = 0; (*decoder).state.pwrx_payload = (*decoder).packet.payload; return 0; }
                INTEL_PT_BBP => intel_pt_bbp(decoder),
                INTEL_PT_BIP => intel_pt_bip(decoder),
                INTEL_PT_BEP => { (*decoder).state.type_ = INTEL_PT_BLK_ITEMS; (*decoder).state.from_ip = (*decoder).ip; (*decoder).state.to_ip = 0; return 0; }
                INTEL_PT_BEP_IP => { err = intel_pt_get_next_packet(decoder); if err != 0 { return err; } if (*decoder).packet.type_ == INTEL_PT_FUP { (*decoder).set_fup_bep = true; no_tip = true; } continue 'next; }
                INTEL_PT_CFE => { (*decoder).fup_cfe_pkt = (*decoder).packet; (*decoder).set_fup_cfe = true; if !(*decoder).pge { intel_pt_fup_event(decoder, true); return 0; } }
                INTEL_PT_CFE_IP => { (*decoder).fup_cfe_pkt = (*decoder).packet; err = intel_pt_get_next_packet(decoder); if err != 0 { return err; } if (*decoder).packet.type_ == INTEL_PT_FUP { (*decoder).set_fup_cfe_ip = true; no_tip = true; } continue 'next; }
                INTEL_PT_EVD => { err = intel_pt_evd(decoder); if err != 0 { return err; } }
                INTEL_PT_BAD => return intel_pt_bug(decoder),
                _ => {}
            }
            break 'next;
        }
    }
}

unsafe fn intel_pt_walk_psb(decoder: *mut intel_pt_decoder) -> c_int {
    (*decoder).in_psb = true;
    loop {
        let err = intel_pt_get_next_packet(decoder);
        if err != 0 { (*decoder).in_psb = false; return err; }
        match (*decoder).packet.type_ {
            INTEL_PT_TIP_PGD => { (*decoder).continuous_period = false; (*decoder).in_psb = false; return -ENOENT; }
            INTEL_PT_TIP_PGE | INTEL_PT_TIP | INTEL_PT_PTWRITE | INTEL_PT_PTWRITE_IP | INTEL_PT_EXSTOP | INTEL_PT_EXSTOP_IP | INTEL_PT_MWAIT | INTEL_PT_PWRE | INTEL_PT_PWRX | INTEL_PT_BBP | INTEL_PT_BIP | INTEL_PT_BEP | INTEL_PT_BEP_IP | INTEL_PT_CFE | INTEL_PT_CFE_IP | INTEL_PT_EVD => { (*decoder).in_psb = false; return -ENOENT; }
            INTEL_PT_FUP => { (*decoder).pge = true; if intel_pt_have_ip(decoder) { intel_pt_set_ip(decoder); (*decoder).psb_ip = (*decoder).ip; } }
            INTEL_PT_MTC => intel_pt_calc_mtc_timestamp(decoder),
            INTEL_PT_TSC => intel_pt_calc_tsc_timestamp(decoder),
            INTEL_PT_TMA => intel_pt_calc_tma(decoder),
            INTEL_PT_CYC => intel_pt_calc_cyc_timestamp(decoder),
            INTEL_PT_CBR => intel_pt_calc_cbr(decoder),
            INTEL_PT_PIP => intel_pt_set_pip(decoder),
            INTEL_PT_MODE_EXEC => intel_pt_mode_exec_status(decoder),
            INTEL_PT_MODE_TSX => intel_pt_update_in_tx(decoder),
            INTEL_PT_TRACESTOP | INTEL_PT_TNT => { (*decoder).have_tma = false; (*decoder).pkt_state = if (*decoder).ip != 0 { intel_pt_pkt_state::INTEL_PT_STATE_IN_SYNC } else { intel_pt_pkt_state::INTEL_PT_STATE_ERR_RESYNC }; (*decoder).in_psb = false; return -ENOENT; }
            INTEL_PT_BAD => { let e = intel_pt_bug(decoder); (*decoder).in_psb = false; return e; }
            INTEL_PT_OVF => { let e = intel_pt_overflow(decoder); (*decoder).in_psb = false; return e; }
            INTEL_PT_PSBEND => { (*decoder).in_psb = false; return 0; }
            _ => {}
        }
    }
}

unsafe fn intel_pt_walk_to_ip(decoder: *mut intel_pt_decoder) -> c_int {
    loop {
        let mut err = intel_pt_get_next_packet(decoder);
        if err != 0 { return err; }
        match (*decoder).packet.type_ {
            INTEL_PT_TIP_PGD => { (*decoder).continuous_period = false; (*decoder).pge = false; if intel_pt_have_ip(decoder) { intel_pt_set_ip(decoder); } if (*decoder).ip != 0 { (*decoder).state.type_ |= INTEL_PT_TRACE_END; return 0; } }
            INTEL_PT_TIP_PGE => { (*decoder).pge = true; intel_pt_mtc_cyc_cnt_pge(decoder); if intel_pt_have_ip(decoder) { intel_pt_set_ip(decoder); } if (*decoder).ip != 0 { (*decoder).state.type_ |= INTEL_PT_TRACE_BEGIN; return 0; } }
            INTEL_PT_TIP => { (*decoder).pge = true; if intel_pt_have_ip(decoder) { intel_pt_set_ip(decoder); } if (*decoder).ip != 0 { return 0; } }
            INTEL_PT_FUP => { if intel_pt_have_ip(decoder) { intel_pt_set_ip(decoder); } if (*decoder).ip != 0 { return 0; } }
            INTEL_PT_MTC => intel_pt_calc_mtc_timestamp(decoder),
            INTEL_PT_TSC => intel_pt_calc_tsc_timestamp(decoder),
            INTEL_PT_TMA => intel_pt_calc_tma(decoder),
            INTEL_PT_CYC => intel_pt_calc_cyc_timestamp(decoder),
            INTEL_PT_CBR => intel_pt_calc_cbr(decoder),
            INTEL_PT_PIP => intel_pt_set_pip(decoder),
            INTEL_PT_MODE_EXEC => intel_pt_mode_exec_status(decoder),
            INTEL_PT_MODE_TSX => intel_pt_update_in_tx(decoder),
            INTEL_PT_OVF => return intel_pt_overflow(decoder),
            INTEL_PT_BAD => return intel_pt_bug(decoder),
            INTEL_PT_TRACESTOP => { (*decoder).pge = false; (*decoder).continuous_period = false; intel_pt_clear_tx_flags(decoder); (*decoder).have_tma = false; }
            INTEL_PT_PSB => {
                (*decoder).state.psb_offset = (*decoder).pos;
                (*decoder).psb_ip = 0;
                (*decoder).last_ip = 0;
                (*decoder).have_last_ip = true;
                intel_pt_clear_stack(&mut (*decoder).stack);
                err = intel_pt_walk_psb(decoder);
                if err != 0 { return err; }
                (*decoder).state.type_ = INTEL_PT_PSB_EVT;
                (*decoder).state.from_ip = (*decoder).psb_ip;
                (*decoder).state.to_ip = 0;
                return 0;
            }
            _ => {}
        }
    }
}

unsafe fn intel_pt_sync_ip(decoder: *mut intel_pt_decoder) -> c_int {
    intel_pt_clear_fup_event(decoder);
    (*decoder).overflow = false;
    if !(*decoder).branch_enable {
        (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_IN_SYNC;
        (*decoder).state.type_ = 0;
        return 0;
    }
    let err = intel_pt_walk_to_ip(decoder);
    if err != 0 || (((*decoder).state.type_ & INTEL_PT_PSB_EVT) != 0 && (*decoder).ip == 0) { return err; }
    (*decoder).pkt_state = if (*decoder).hop { intel_pt_pkt_state::INTEL_PT_STATE_RESAMPLE } else { intel_pt_pkt_state::INTEL_PT_STATE_IN_SYNC };
    (*decoder).state.from_ip = 0;
    (*decoder).state.to_ip = (*decoder).ip;
    0
}

unsafe fn intel_pt_part_psb(decoder: *mut intel_pt_decoder) -> c_int {
    let end = (*decoder).buf.add((*decoder).len);
    let mut i = INTEL_PT_PSB_LEN - 1;
    while i != 0 {
        if i <= (*decoder).len && memcmp(end.sub(i) as *const c_void, INTEL_PT_PSB_STR.as_ptr() as *const c_void, i) == 0 { return i as c_int; }
        i -= 1;
    }
    0
}

unsafe fn intel_pt_rest_psb(decoder: *mut intel_pt_decoder, part_psb: c_int) -> c_int {
    let rest_psb = INTEL_PT_PSB_LEN - part_psb as usize;
    if rest_psb > (*decoder).len || memcmp((*decoder).buf as *const c_void, INTEL_PT_PSB_STR.as_ptr().add(part_psb as usize) as *const c_void, rest_psb) != 0 { return 0; }
    rest_psb as c_int
}

unsafe fn intel_pt_get_split_psb(decoder: *mut intel_pt_decoder, part_psb: c_int) -> c_int {
    (*decoder).pos += (*decoder).len as u64;
    (*decoder).len = 0;
    let ret = intel_pt_get_next_data(decoder, false);
    if ret != 0 { return ret; }
    let rest_psb = intel_pt_rest_psb(decoder, part_psb);
    if rest_psb == 0 { return 0; }
    (*decoder).pos -= part_psb as u64;
    (*decoder).next_buf = (*decoder).buf.add(rest_psb as usize);
    (*decoder).next_len = (*decoder).len - rest_psb as usize;
    memcpy((*decoder).temp_buf.as_mut_ptr() as *mut c_void, INTEL_PT_PSB_STR.as_ptr() as *const c_void, INTEL_PT_PSB_LEN);
    (*decoder).buf = (*decoder).temp_buf.as_ptr();
    (*decoder).len = INTEL_PT_PSB_LEN;
    0
}

unsafe fn intel_pt_scan_for_psb(decoder: *mut intel_pt_decoder) -> c_int {
    loop {
        if (*decoder).len == 0 {
            let ret = intel_pt_get_next_data(decoder, false);
            if ret != 0 { return ret; }
        }
        let next = memmem((*decoder).buf as *const c_void, (*decoder).len, INTEL_PT_PSB_STR.as_ptr() as *const c_void, INTEL_PT_PSB_LEN) as *mut c_uchar;
        if next.is_null() {
            let part_psb = intel_pt_part_psb(decoder);
            if part_psb != 0 {
                let ret = intel_pt_get_split_psb(decoder, part_psb);
                if ret != 0 { return ret; }
            } else {
                (*decoder).pos += (*decoder).len as u64;
                (*decoder).len = 0;
            }
            continue;
        }
        (*decoder).pkt_step = next.offset_from((*decoder).buf) as c_int;
        return intel_pt_get_next_packet(decoder);
    }
}

unsafe fn intel_pt_sync(decoder: *mut intel_pt_decoder) -> c_int {
    (*decoder).pge = false;
    (*decoder).continuous_period = false;
    (*decoder).have_last_ip = false;
    (*decoder).last_ip = 0;
    (*decoder).psb_ip = 0;
    (*decoder).ip = 0;
    intel_pt_clear_stack(&mut (*decoder).stack);
    let err = intel_pt_scan_for_psb(decoder);
    if err != 0 { return err; }
    if (*decoder).vm_time_correlation {
        (*decoder).in_psb = true;
        if (*decoder).timestamp == 0 { (*decoder).timestamp = 1; }
        (*decoder).state.type_ = 0;
        (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_VM_TIME_CORRELATION;
        return 0;
    }
    (*decoder).have_last_ip = true;
    (*decoder).pkt_state = intel_pt_pkt_state::INTEL_PT_STATE_IN_SYNC;
    let err = intel_pt_walk_psb(decoder);
    if err != 0 { return err; }
    (*decoder).state.type_ = INTEL_PT_PSB_EVT;
    (*decoder).state.from_ip = (*decoder).psb_ip;
    (*decoder).state.to_ip = 0;
    if (*decoder).ip != 0 {
        (*decoder).pkt_state = if (*decoder).hop { intel_pt_pkt_state::INTEL_PT_STATE_RESAMPLE } else { intel_pt_pkt_state::INTEL_PT_STATE_IN_SYNC };
    }
    0
}

unsafe fn intel_pt_est_timestamp(decoder: *mut intel_pt_decoder) -> u64 {
    let mut est = (*decoder).sample_insn_cnt << 1;
    if (*decoder).cbr != 0 && (*decoder).max_non_turbo_ratio != 0 {
        est *= (*decoder).max_non_turbo_ratio as u64;
        est /= (*decoder).cbr as u64;
    }
    (*decoder).sample_timestamp + est
}

#[no_mangle]
pub unsafe extern "C" fn intel_pt_decode(decoder: *mut intel_pt_decoder) -> *const intel_pt_state {
    let mut err;
    loop {
        (*decoder).state.type_ = INTEL_PT_BRANCH;
        (*decoder).state.flags = 0;
        err = match (*decoder).pkt_state {
            intel_pt_pkt_state::INTEL_PT_STATE_NO_PSB => intel_pt_sync(decoder),
            intel_pt_pkt_state::INTEL_PT_STATE_NO_IP => { (*decoder).have_last_ip = false; (*decoder).last_ip = 0; (*decoder).ip = 0; intel_pt_sync_ip(decoder) }
            intel_pt_pkt_state::INTEL_PT_STATE_ERR_RESYNC => intel_pt_sync_ip(decoder),
            intel_pt_pkt_state::INTEL_PT_STATE_IN_SYNC => intel_pt_walk_trace(decoder),
            intel_pt_pkt_state::INTEL_PT_STATE_TNT | intel_pt_pkt_state::INTEL_PT_STATE_TNT_CONT => {
                let e = intel_pt_walk_tnt(decoder);
                if e == -EAGAIN { intel_pt_walk_trace(decoder) } else { e }
            }
            intel_pt_pkt_state::INTEL_PT_STATE_TIP | intel_pt_pkt_state::INTEL_PT_STATE_TIP_PGD => intel_pt_walk_tip(decoder),
            intel_pt_pkt_state::INTEL_PT_STATE_FUP => {
                let e = intel_pt_walk_fup(decoder);
                if e == -EAGAIN { intel_pt_walk_fup_tip(decoder) } else { e }
            }
            intel_pt_pkt_state::INTEL_PT_STATE_FUP_NO_TIP => {
                let e = intel_pt_walk_fup(decoder);
                if e == -EAGAIN { intel_pt_walk_trace(decoder) } else { e }
            }
            intel_pt_pkt_state::INTEL_PT_STATE_FUP_IN_PSB => intel_pt_fup_in_psb(decoder),
            intel_pt_pkt_state::INTEL_PT_STATE_RESAMPLE => intel_pt_resample(decoder),
            intel_pt_pkt_state::INTEL_PT_STATE_VM_TIME_CORRELATION => intel_pt_vm_time_correlation(decoder),
        };
        if err != -ENOLINK { break; }
    }
    if err != 0 {
        (*decoder).state.err = intel_pt_ext_err(err);
        if err != -EOVERFLOW { (*decoder).state.from_ip = (*decoder).ip; }
        intel_pt_update_sample_time(decoder);
        (*decoder).sample_tot_cyc_cnt = (*decoder).tot_cyc_cnt;
        intel_pt_set_nr(decoder);
    } else {
        (*decoder).state.err = 0;
        if (*decoder).cbr != (*decoder).cbr_seen {
            (*decoder).cbr_seen = (*decoder).cbr;
            if (*decoder).state.type_ == 0 {
                (*decoder).state.from_ip = (*decoder).ip;
                (*decoder).state.to_ip = 0;
            }
            (*decoder).state.type_ |= INTEL_PT_CBR_CHG;
            (*decoder).state.cbr_payload = (*decoder).cbr_payload;
            (*decoder).state.cbr = (*decoder).cbr;
        }
        if intel_pt_sample_time((*decoder).pkt_state) {
            intel_pt_update_sample_time(decoder);
            if (*decoder).sample_cyc {
                (*decoder).sample_tot_cyc_cnt = (*decoder).tot_cyc_cnt;
                (*decoder).state.flags |= INTEL_PT_SAMPLE_IPC;
                (*decoder).sample_cyc = false;
            }
        }
        if !(*decoder).have_cyc {
            (*decoder).state.flags |= INTEL_PT_SAMPLE_IPC;
        }
    }
    if ((*decoder).state.type_ & INTEL_PT_PSB_EVT) != 0 && (*decoder).tsc_timestamp != 0 {
        (*decoder).sample_timestamp = (*decoder).tsc_timestamp;
    }
    (*decoder).state.from_nr = (*decoder).nr;
    (*decoder).state.to_nr = (*decoder).next_nr;
    (*decoder).nr = (*decoder).next_nr;
    (*decoder).state.timestamp = (*decoder).sample_timestamp;
    (*decoder).state.est_timestamp = intel_pt_est_timestamp(decoder);
    (*decoder).state.tot_insn_cnt = (*decoder).tot_insn_cnt;
    (*decoder).state.tot_cyc_cnt = (*decoder).sample_tot_cyc_cnt;
    &(*decoder).state
}

unsafe fn intel_pt_next_psb(buf: *mut *mut c_uchar, len: *mut size_t) -> bool {
    let next = memmem(*buf as *const c_void, *len, INTEL_PT_PSB_STR.as_ptr() as *const c_void, INTEL_PT_PSB_LEN) as *mut c_uchar;
    if !next.is_null() {
        *len -= next.offset_from(*buf) as usize;
        *buf = next;
        true
    } else {
        false
    }
}

unsafe fn intel_pt_step_psb(buf: *mut *mut c_uchar, len: *mut size_t) -> bool {
    if *len == 0 { return false; }
    let next = memmem((*buf).add(1) as *const c_void, *len - 1, INTEL_PT_PSB_STR.as_ptr() as *const c_void, INTEL_PT_PSB_LEN) as *mut c_uchar;
    if !next.is_null() {
        *len -= next.offset_from(*buf) as usize;
        *buf = next;
        true
    } else {
        false
    }
}

unsafe fn intel_pt_last_psb(buf: *mut c_uchar, mut len: size_t) -> *mut c_uchar {
    if len < INTEL_PT_PSB_LEN { return ptr::null_mut(); }
    let mut k = len - INTEL_PT_PSB_LEN + 1;
    loop {
        let p = memrchr(buf as *const c_void, INTEL_PT_PSB_STR[0] as c_int, k) as *mut c_uchar;
        if p.is_null() { return ptr::null_mut(); }
        if memcmp(p.add(1) as *const c_void, INTEL_PT_PSB_STR.as_ptr().add(1) as *const c_void, INTEL_PT_PSB_LEN - 1) == 0 {
            return p;
        }
        k = p.offset_from(buf) as usize;
        if k == 0 { return ptr::null_mut(); }
        len = k;
        let _ = len;
    }
}

unsafe fn intel_pt_next_tsc(mut buf: *mut c_uchar, mut len: size_t, tsc: *mut u64, rem: *mut size_t) -> bool {
    let mut ctx: intel_pt_pkt_ctx = INTEL_PT_NO_CTX;
    let mut packet: intel_pt_pkt = mem::zeroed();
    while len != 0 {
        let ret = intel_pt_get_packet(buf, len, &mut packet, &mut ctx);
        if ret <= 0 { return false; }
        if packet.type_ == INTEL_PT_TSC {
            *tsc = packet.payload;
            *rem = len;
            return true;
        }
        if packet.type_ == INTEL_PT_PSBEND { return false; }
        buf = buf.add(ret as usize);
        len -= ret as usize;
    }
    false
}

fn intel_pt_tsc_cmp(tsc1: u64, tsc2: u64) -> c_int {
    let halfway = 1u64 << 55;
    if tsc1 == tsc2 { return 0; }
    if tsc1 < tsc2 {
        if tsc2 - tsc1 < halfway { -1 } else { 1 }
    } else if tsc1 - tsc2 < halfway {
        1
    } else {
        -1
    }
}

unsafe fn adj_for_padding(buf_b: *mut c_uchar, buf_a: *mut c_uchar, len_a: size_t) -> *mut c_uchar {
    let mut p = buf_b.sub(MAX_PADDING);
    let mut q = buf_a.add(len_a - MAX_PADDING);
    let mut i = MAX_PADDING;
    while i != 0 {
        if *p != *q { break; }
        p = p.add(1);
        q = q.add(1);
        i -= 1;
    }
    p
}

unsafe fn intel_pt_find_overlap_tsc(
    buf_a: *mut c_uchar,
    mut len_a: size_t,
    mut buf_b: *mut c_uchar,
    mut len_b: size_t,
    consecutive: *mut bool,
    ooo_tsc: bool,
) -> *mut c_uchar {
    let mut p = intel_pt_last_psb(buf_a, len_a);
    if p.is_null() { return buf_b; }
    let mut len = len_a - p.offset_from(buf_a) as usize;
    let mut tsc_a = 0;
    let mut rem_a = 0;
    if !intel_pt_next_tsc(p, len, &mut tsc_a, &mut rem_a) {
        len_a -= len;
        p = intel_pt_last_psb(buf_a, len_a);
        if p.is_null() { return buf_b; }
        len = len_a - p.offset_from(buf_a) as usize;
        if !intel_pt_next_tsc(p, len, &mut tsc_a, &mut rem_a) { return buf_b; }
    }
    loop {
        let mut tsc_b = 0;
        let mut rem_b = 0;
        if intel_pt_next_tsc(buf_b, len_b, &mut tsc_b, &mut rem_b) {
            let cmp = intel_pt_tsc_cmp(tsc_a, tsc_b);
            if cmp == 0 && rem_b >= rem_a {
                *consecutive = true;
                let start = buf_b.add(len_b - (rem_b - rem_a));
                return adj_for_padding(start, buf_a, len_a);
            }
            if cmp < 0 && !ooo_tsc { return buf_b; }
        }
        if !intel_pt_step_psb(&mut buf_b, &mut len_b) {
            return buf_b.add(len_b);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn intel_pt_find_overlap(
    mut buf_a: *mut c_uchar,
    mut len_a: size_t,
    mut buf_b: *mut c_uchar,
    mut len_b: size_t,
    have_tsc: bool,
    consecutive: *mut bool,
    ooo_tsc: bool,
) -> *mut c_uchar {
    let mut found: *mut c_uchar;
    if !intel_pt_next_psb(&mut buf_b, &mut len_b) {
        return buf_b.add(len_b);
    }
    if !intel_pt_next_psb(&mut buf_a, &mut len_a) {
        return buf_b;
    }
    if have_tsc {
        found = intel_pt_find_overlap_tsc(buf_a, len_a, buf_b, len_b, consecutive, ooo_tsc);
        if !found.is_null() { return found; }
    }
    while len_b < len_a {
        if !intel_pt_step_psb(&mut buf_a, &mut len_a) { return buf_b; }
    }
    loop {
        found = memmem(buf_a as *const c_void, len_a, buf_b as *const c_void, len_a) as *mut c_uchar;
        if !found.is_null() {
            *consecutive = true;
            return adj_for_padding(buf_b.add(len_a), buf_a, len_a);
        }
        if !intel_pt_step_psb(&mut buf_a, &mut len_a) { return buf_b; }
    }
}

#[repr(C)]
struct fast_forward_data {
    timestamp: u64,
    buf_timestamp: u64,
}

unsafe extern "C" fn intel_pt_ff_cb(buffer: *mut intel_pt_buffer, data: *mut c_void) -> c_int {
    let d = data as *mut fast_forward_data;
    let mut buf = (*buffer).buf as *mut c_uchar;
    let mut len = (*buffer).len;
    let mut tsc = 0;
    let mut rem = 0;
    if !intel_pt_next_psb(&mut buf, &mut len) || !intel_pt_next_tsc(buf, len, &mut tsc, &mut rem) {
        return 0;
    }
    tsc = intel_pt_8b_tsc(tsc, (*buffer).ref_timestamp);
    if tsc < (*d).timestamp {
        (*d).buf_timestamp = (*buffer).ref_timestamp;
    } else {
        return 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn intel_pt_fast_forward(decoder: *mut intel_pt_decoder, timestamp: u64) -> c_int {
    let mut d = fast_forward_data { timestamp, buf_timestamp: 0 };
    let mut err = ((*decoder).lookahead.unwrap())((*decoder).data, Some(intel_pt_ff_cb), &mut d as *mut _ as *mut c_void);
    if err < 0 { return err; }
    if d.buf_timestamp != 0 {
        loop {
            (*decoder).pos += (*decoder).len as u64;
            (*decoder).len = 0;
            err = intel_pt_get_next_data(decoder, true);
            if err != 0 && err != -ENOLINK { return err; }
            if (*decoder).buf_timestamp == d.buf_timestamp { break; }
        }
    }
    if (*decoder).buf.is_null() { return 0; }
    let mut buf = (*decoder).buf as *mut c_uchar;
    let mut len = (*decoder).len;
    if !intel_pt_next_psb(&mut buf, &mut len) { return 0; }
    loop {
        let mut tsc = 0;
        let mut rem = 0;
        if !intel_pt_next_tsc(buf, len, &mut tsc, &mut rem) { break; }
        tsc = intel_pt_8b_tsc(tsc, (*decoder).buf_timestamp);
        if tsc < timestamp {
            (*decoder).pos += ((*decoder).len - len) as u64;
            (*decoder).buf = buf;
            (*decoder).len = len;
            intel_pt_reposition(decoder);
        } else {
            break;
        }
        if !intel_pt_step_psb(&mut buf, &mut len) { break; }
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
