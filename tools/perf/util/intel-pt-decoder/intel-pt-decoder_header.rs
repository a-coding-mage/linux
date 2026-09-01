/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * intel_pt_decoder.h: Intel Processor Trace support
 * Copyright (c) 2013-2014, Intel Corporation.
 */

// C header dependencies: stdint.h, stddef.h, stdbool.h, linux/rbtree.h,
// and "intel-pt-insn-decoder.h".

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_void};

pub const INTEL_PT_IN_TX: u32 = 1 << 0;
pub const INTEL_PT_ABORT_TX: u32 = 1 << 1;
pub const INTEL_PT_IFLAG: u32 = 1 << 2;
pub const INTEL_PT_ASYNC: u32 = 1 << 2;
pub const INTEL_PT_FUP_IP: u32 = 1 << 3;
pub const INTEL_PT_SAMPLE_IPC: u32 = 1 << 4;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum intel_pt_sample_type {
    INTEL_PT_BRANCH = 1 << 0,
    INTEL_PT_INSTRUCTION = 1 << 1,
    INTEL_PT_TRANSACTION = 1 << 2,
    INTEL_PT_PTW = 1 << 3,
    INTEL_PT_MWAIT_OP = 1 << 4,
    INTEL_PT_PWR_ENTRY = 1 << 5,
    INTEL_PT_EX_STOP = 1 << 6,
    INTEL_PT_PWR_EXIT = 1 << 7,
    INTEL_PT_CBR_CHG = 1 << 8,
    INTEL_PT_TRACE_BEGIN = 1 << 9,
    INTEL_PT_TRACE_END = 1 << 10,
    INTEL_PT_BLK_ITEMS = 1 << 11,
    INTEL_PT_PSB_EVT = 1 << 12,
    INTEL_PT_EVT = 1 << 13,
    INTEL_PT_IFLAG_CHG = 1 << 14,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum intel_pt_period_type {
    INTEL_PT_PERIOD_NONE,
    INTEL_PT_PERIOD_INSTRUCTIONS,
    INTEL_PT_PERIOD_TICKS,
    INTEL_PT_PERIOD_MTC,
}

pub const INTEL_PT_ERR_NOMEM: c_int = 1;
pub const INTEL_PT_ERR_INTERN: c_int = 2;
pub const INTEL_PT_ERR_BADPKT: c_int = 3;
pub const INTEL_PT_ERR_NODATA: c_int = 4;
pub const INTEL_PT_ERR_NOINSN: c_int = 5;
pub const INTEL_PT_ERR_MISMAT: c_int = 6;
pub const INTEL_PT_ERR_OVR: c_int = 7;
pub const INTEL_PT_ERR_LOST: c_int = 8;
pub const INTEL_PT_ERR_UNK: c_int = 9;
pub const INTEL_PT_ERR_NELOOP: c_int = 10;
pub const INTEL_PT_ERR_EPTW: c_int = 11;
pub const INTEL_PT_ERR_MAX: c_int = 12;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum intel_pt_param_flags {
    /*
     * FUP packet can contain next linear instruction pointer instead of
     * current linear instruction pointer.
     */
    INTEL_PT_FUP_WITH_NLIP = 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum intel_pt_blk_type {
    INTEL_PT_GP_REGS = 1,
    INTEL_PT_PEBS_BASIC = 4,
    INTEL_PT_PEBS_MEM = 5,
    INTEL_PT_LBR_0 = 8,
    INTEL_PT_LBR_1 = 9,
    INTEL_PT_LBR_2 = 10,
    INTEL_PT_XMM = 16,
    INTEL_PT_BLK_TYPE_MAX,
}

/*
 * The block type numbers are not sequential but here they are given sequential
 * positions to avoid wasting space for array placement.
 */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum intel_pt_blk_type_pos {
    INTEL_PT_GP_REGS_POS,
    INTEL_PT_PEBS_BASIC_POS,
    INTEL_PT_PEBS_MEM_POS,
    INTEL_PT_LBR_0_POS,
    INTEL_PT_LBR_1_POS,
    INTEL_PT_LBR_2_POS,
    INTEL_PT_XMM_POS,
    INTEL_PT_BLK_TYPE_CNT,
}

pub const INTEL_PT_BLK_TYPE_MAX_VALUE: usize = 17;
pub const INTEL_PT_BLK_TYPE_CNT_VALUE: usize = 7;

/* Get the array position for a block type */
#[inline]
pub unsafe fn intel_pt_blk_type_pos(blk_type: intel_pt_blk_type) -> c_int {
    let mut map = [0 as c_int; INTEL_PT_BLK_TYPE_MAX_VALUE];
    map[intel_pt_blk_type::INTEL_PT_GP_REGS as usize] =
        intel_pt_blk_type_pos::INTEL_PT_GP_REGS_POS as c_int + 1;
    map[intel_pt_blk_type::INTEL_PT_PEBS_BASIC as usize] =
        intel_pt_blk_type_pos::INTEL_PT_PEBS_BASIC_POS as c_int + 1;
    map[intel_pt_blk_type::INTEL_PT_PEBS_MEM as usize] =
        intel_pt_blk_type_pos::INTEL_PT_PEBS_MEM_POS as c_int + 1;
    map[intel_pt_blk_type::INTEL_PT_LBR_0 as usize] =
        intel_pt_blk_type_pos::INTEL_PT_LBR_0_POS as c_int + 1;
    map[intel_pt_blk_type::INTEL_PT_LBR_1 as usize] =
        intel_pt_blk_type_pos::INTEL_PT_LBR_1_POS as c_int + 1;
    map[intel_pt_blk_type::INTEL_PT_LBR_2 as usize] =
        intel_pt_blk_type_pos::INTEL_PT_LBR_2_POS as c_int + 1;
    map[intel_pt_blk_type::INTEL_PT_XMM as usize] =
        intel_pt_blk_type_pos::INTEL_PT_XMM_POS as c_int + 1;

    if (blk_type as c_int) < intel_pt_blk_type::INTEL_PT_BLK_TYPE_MAX as c_int {
        map[blk_type as usize] - 1
    } else {
        -1
    }
}

pub const INTEL_PT_BLK_ITEM_ID_CNT: usize = 32;

/*
 * Use unions so that the block items can be accessed by name or by array index.
 * There is an array of 32-bit masks for each block type, which indicate which
 * values are present. Then arrays of 32 64-bit values for each block type.
 */
#[repr(C)]
pub struct intel_pt_blk_items {
    pub masks: intel_pt_blk_items_masks,
    pub values: intel_pt_blk_items_values,
    pub is_32_bit: bool,
}

#[repr(C)]
pub union intel_pt_blk_items_masks {
    pub mask: [u32; INTEL_PT_BLK_TYPE_CNT_VALUE],
    pub named: intel_pt_blk_items_masks_named,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pt_blk_items_masks_named {
    /*
     * C bitfields:
     * has_rflags:1 through has_r15:1, has_unused_0:14,
     * has_ip:1, has_applicable_counters:1, has_timestamp:1, has_unused_1:29,
     * has_mem_access_address:1, has_mem_aux_info:1,
     * has_mem_access_latency:1, has_tsx_aux_info:1, has_unused_2:28.
     */
    pub gp_regs: u32,
    pub pebs_basic: u32,
    pub pebs_mem: u32,
    pub has_lbr_0: u32,
    pub has_lbr_1: u32,
    pub has_lbr_2: u32,
    pub has_xmm: u32,
}

#[repr(C)]
pub union intel_pt_blk_items_values {
    pub val: [[u64; INTEL_PT_BLK_ITEM_ID_CNT]; INTEL_PT_BLK_TYPE_CNT_VALUE],
    pub named: intel_pt_blk_items_values_named,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pt_blk_items_values_named {
    pub gp_regs: intel_pt_blk_items_gp_regs,
    pub pebs_basic: intel_pt_blk_items_pebs_basic,
    pub pebs_mem: intel_pt_blk_items_pebs_mem,
    pub lbr_0: [u64; INTEL_PT_BLK_ITEM_ID_CNT],
    pub lbr_1: [u64; INTEL_PT_BLK_ITEM_ID_CNT],
    pub lbr_2: [u64; INTEL_PT_BLK_ITEM_ID_CNT],
    pub xmm: [u64; INTEL_PT_BLK_ITEM_ID_CNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pt_blk_items_gp_regs {
    pub rflags: u64,
    pub rip: u64,
    pub rax: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rbx: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub unused_0: [u64; INTEL_PT_BLK_ITEM_ID_CNT - 18],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pt_blk_items_pebs_basic {
    pub ip: u64,
    pub applicable_counters: u64,
    pub timestamp: u64,
    pub unused_1: [u64; INTEL_PT_BLK_ITEM_ID_CNT - 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pt_blk_items_pebs_mem {
    pub mem_access_address: u64,
    pub mem_aux_info: u64,
    pub mem_access_latency: u64,
    pub tsx_aux_info: u64,
    pub unused_2: [u64; INTEL_PT_BLK_ITEM_ID_CNT - 4],
}

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct intel_pt_vmcs_info {
    pub rb_node: rb_node,
    pub vmcs: u64,
    pub tsc_offset: u64,
    pub reliable: bool,
    pub error_printed: bool,
}

/*
 * Maximum number of event trace data in one go, assuming at most 1 per type
 * and 6-bits of type in the EVD packet.
 */
pub const INTEL_PT_MAX_EVDS: usize = 64;

/* Event trace data from EVD packet */
#[repr(C)]
pub struct intel_pt_evd {
    pub type_: c_int,
    pub payload: u64,
}

pub const INTEL_PT_INSN_BUF_SZ: usize = 16;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum intel_pt_insn_op {
    INTEL_PT_OP_OTHER = 0,
}

#[repr(C)]
pub struct intel_pt_state {
    pub type_: intel_pt_sample_type,
    pub from_nr: bool,
    pub to_nr: bool,
    pub from_iflag: bool,
    pub to_iflag: bool,
    pub err: c_int,
    pub from_ip: u64,
    pub to_ip: u64,
    pub tot_insn_cnt: u64,
    pub tot_cyc_cnt: u64,
    pub cycles: u64,
    pub timestamp: u64,
    pub est_timestamp: u64,
    pub trace_nr: u64,
    pub ptw_payload: u64,
    pub mwait_payload: u64,
    pub pwre_payload: u64,
    pub pwrx_payload: u64,
    pub cbr_payload: u64,
    pub psb_offset: u64,
    pub cbr: u32,
    pub flags: u32,
    pub insn_op: intel_pt_insn_op,
    pub insn_len: c_int,
    pub insn: [c_char; INTEL_PT_INSN_BUF_SZ],
    pub items: intel_pt_blk_items,
    pub cfe_type: c_int,
    pub cfe_vector: c_int,
    pub evd_cnt: c_int,
    pub evd: *mut intel_pt_evd,
}

#[repr(C)]
pub struct intel_pt_insn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct intel_pt_buffer {
    pub buf: *const c_uchar,
    pub len: usize,
    pub consecutive: bool,
    pub ref_timestamp: u64,
    pub trace_nr: u64,
}

pub type intel_pt_lookahead_cb_t = Option<
    unsafe extern "C" fn(*mut intel_pt_buffer, *mut c_void) -> c_int,
>;

#[repr(C)]
pub struct intel_pt_params {
    pub get_trace: Option<unsafe extern "C" fn(buffer: *mut intel_pt_buffer, data: *mut c_void) -> c_int>,
    pub walk_insn: Option<
        unsafe extern "C" fn(
            intel_pt_insn: *mut intel_pt_insn,
            insn_cnt_ptr: *mut u64,
            ip: *mut u64,
            to_ip: u64,
            max_insn_cnt: u64,
            data: *mut c_void,
        ) -> c_int,
    >,
    pub pgd_ip: Option<unsafe extern "C" fn(ip: u64, data: *mut c_void) -> bool>,
    pub lookahead: Option<
        unsafe extern "C" fn(
            data: *mut c_void,
            cb: intel_pt_lookahead_cb_t,
            cb_data: *mut c_void,
        ) -> c_int,
    >,
    pub findnew_vmcs_info: Option<
        unsafe extern "C" fn(data: *mut c_void, vmcs: u64) -> *mut intel_pt_vmcs_info,
    >,
    pub data: *mut c_void,
    pub return_compression: bool,
    pub branch_enable: bool,
    pub vm_time_correlation: bool,
    pub vm_tm_corr_dry_run: bool,
    pub first_timestamp: u64,
    pub ctl: u64,
    pub period: u64,
    pub period_type: intel_pt_period_type,
    pub max_non_turbo_ratio: c_uint,
    pub mtc_period: c_uint,
    pub tsc_ctc_ratio_n: u32,
    pub tsc_ctc_ratio_d: u32,
    pub flags: intel_pt_param_flags,
    pub quick: c_uint,
    pub max_loops: c_int,
}

#[repr(C)]
pub struct intel_pt_decoder {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn intel_pt_decoder_new(params: *mut intel_pt_params) -> *mut intel_pt_decoder;
    pub fn intel_pt_decoder_free(decoder: *mut intel_pt_decoder);

    pub fn intel_pt_decode(decoder: *mut intel_pt_decoder) -> *const intel_pt_state;

    pub fn intel_pt_fast_forward(decoder: *mut intel_pt_decoder, timestamp: u64) -> c_int;

    pub fn intel_pt_find_overlap(
        buf_a: *mut c_uchar,
        len_a: usize,
        buf_b: *mut c_uchar,
        len_b: usize,
        have_tsc: bool,
        consecutive: *mut bool,
        ooo_tsc: bool,
    ) -> *mut c_uchar;

    pub fn intel_pt__strerror(code: c_int, buf: *mut c_char, buflen: usize) -> c_int;

    pub fn intel_pt_set_first_timestamp(
        decoder: *mut intel_pt_decoder,
        first_timestamp: u64,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
