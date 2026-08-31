/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * intel_pt_pkt_decoder.h: Intel Processor Trace support
 * Copyright (c) 2013-2014, Intel Corporation.
 */

use core::ffi::{c_char, c_int};

pub const INTEL_PT_PKT_DESC_MAX: usize = 256;

pub const INTEL_PT_NEED_MORE_BYTES: c_int = -1;
pub const INTEL_PT_BAD_PACKET: c_int = -2;

pub const INTEL_PT_PSB_STR: &[u8; INTEL_PT_PSB_LEN] =
    b"\x02\x82\x02\x82\x02\x82\x02\x82\x02\x82\x02\x82\x02\x82\x02\x82";
pub const INTEL_PT_PSB_LEN: usize = 16;

pub const INTEL_PT_PKT_MAX_SZ: usize = 16;

pub const INTEL_PT_VMX_NR_FLAG: usize = 1;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum intel_pt_pkt_type {
    INTEL_PT_BAD,
    INTEL_PT_PAD,
    INTEL_PT_TNT,
    INTEL_PT_TIP_PGD,
    INTEL_PT_TIP_PGE,
    INTEL_PT_TSC,
    INTEL_PT_TMA,
    INTEL_PT_MODE_EXEC,
    INTEL_PT_MODE_TSX,
    INTEL_PT_MTC,
    INTEL_PT_TIP,
    INTEL_PT_FUP,
    INTEL_PT_CYC,
    INTEL_PT_VMCS,
    INTEL_PT_PSB,
    INTEL_PT_PSBEND,
    INTEL_PT_CBR,
    INTEL_PT_TRACESTOP,
    INTEL_PT_PIP,
    INTEL_PT_OVF,
    INTEL_PT_MNT,
    INTEL_PT_PTWRITE,
    INTEL_PT_PTWRITE_IP,
    INTEL_PT_EXSTOP,
    INTEL_PT_EXSTOP_IP,
    INTEL_PT_MWAIT,
    INTEL_PT_PWRE,
    INTEL_PT_PWRX,
    INTEL_PT_BBP,
    INTEL_PT_BIP,
    INTEL_PT_BEP,
    INTEL_PT_BEP_IP,
    INTEL_PT_CFE,
    INTEL_PT_CFE_IP,
    INTEL_PT_EVD,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct intel_pt_pkt {
    pub type_: intel_pt_pkt_type,
    pub count: c_int,
    pub payload: u64,
}

/*
 * Decoding of BIP packets conflicts with single-byte TNT packets. Since BIP
 * packets only occur in the context of a block (i.e. between BBP and BEP), that
 * context must be recorded and passed to the packet decoder.
 */
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum intel_pt_pkt_ctx {
    INTEL_PT_NO_CTX, /* BIP packets are invalid */
    INTEL_PT_BLK_4_CTX, /* 4-byte BIP packets */
    INTEL_PT_BLK_8_CTX, /* 8-byte BIP packets */
}

unsafe extern "C" {
    pub fn intel_pt_pkt_name(type_: intel_pt_pkt_type) -> *const c_char;

    pub fn intel_pt_get_packet(
        buf: *const u8,
        len: usize,
        packet: *mut intel_pt_pkt,
        ctx: *mut intel_pt_pkt_ctx,
    ) -> c_int;

    pub fn intel_pt_upd_pkt_ctx(packet: *const intel_pt_pkt, ctx: *mut intel_pt_pkt_ctx);

    pub fn intel_pt_pkt_desc(packet: *const intel_pt_pkt, buf: *mut c_char, len: usize) -> c_int;
}
