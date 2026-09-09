/* SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause) */
/*
 * Copyright 2014-2016 Freescale Semiconductor Inc.
 * Copyright 2016 NXP
 */

/* Dependencies corresponding to linux/types.h, linux/cpumask.h, and dpaa2-fd.h. */

#[repr(C)]
pub struct dpaa2_dq_common {
    pub verb: u8,
    pub reserved: [u8; 63],
}

#[repr(C)]
pub struct dpaa2_dq_dq {
    pub verb: u8,
    pub stat: u8,
    pub seqnum: __le16,
    pub oprid: __le16,
    pub reserved: u8,
    pub tok: u8,
    pub fqid: __le32,
    pub reserved2: u32,
    pub fq_byte_cnt: __le32,
    pub fq_frm_cnt: __le32,
    pub fqd_ctx: __le64,
    pub fd: [u8; 32],
}

#[repr(C)]
pub struct dpaa2_dq_scn {
    pub verb: u8,
    pub stat: u8,
    pub state: u8,
    pub reserved: u8,
    pub rid_tok: __le32,
    pub ctx: __le64,
}

#[repr(C)]
pub union dpaa2_dq_union {
    pub common: dpaa2_dq_common,
    pub dq: dpaa2_dq_dq,
    pub scn: dpaa2_dq_scn,
}

#[repr(C)]
pub struct dpaa2_dq {
    pub u: dpaa2_dq_union,
}

/* Parsing frame dequeue results */
/* FQ empty */
pub const DPAA2_DQ_STAT_FQEMPTY: u32 = 0x80;
/* FQ held active */
pub const DPAA2_DQ_STAT_HELDACTIVE: u32 = 0x40;
/* FQ force eligible */
pub const DPAA2_DQ_STAT_FORCEELIGIBLE: u32 = 0x20;
/* valid frame */
pub const DPAA2_DQ_STAT_VALIDFRAME: u32 = 0x10;
/* FQ ODP enable */
pub const DPAA2_DQ_STAT_ODPVALID: u32 = 0x04;
/* volatile dequeue */
pub const DPAA2_DQ_STAT_VOLATILE: u32 = 0x02;
/* volatile dequeue command is expired */
pub const DPAA2_DQ_STAT_EXPIRED: u32 = 0x01;

pub const DQ_FQID_MASK: u32 = 0x00FF_FFFF;
pub const DQ_FRAME_COUNT_MASK: u32 = 0x00FF_FFFF;

#[inline]
pub unsafe fn dpaa2_dq_flags(dq: *const dpaa2_dq) -> u8 {
    (*dq).u.dq.stat
}

#[inline]
pub unsafe fn dpaa2_dq_is_pull(dq: *const dpaa2_dq) -> i32 {
    (dpaa2_dq_flags(dq) as u32 & DPAA2_DQ_STAT_VOLATILE) as i32
}

#[inline]
pub unsafe fn dpaa2_dq_is_pull_complete(dq: *const dpaa2_dq) -> bool {
    (dpaa2_dq_flags(dq) as u32 & DPAA2_DQ_STAT_EXPIRED) != 0
}

#[inline]
pub unsafe fn dpaa2_dq_seqnum(dq: *const dpaa2_dq) -> u16 {
    le16_to_cpu((*dq).u.dq.seqnum)
}

#[inline]
pub unsafe fn dpaa2_dq_odpid(dq: *const dpaa2_dq) -> u16 {
    le16_to_cpu((*dq).u.dq.oprid)
}

#[inline]
pub unsafe fn dpaa2_dq_fqid(dq: *const dpaa2_dq) -> u32 {
    le32_to_cpu((*dq).u.dq.fqid) & DQ_FQID_MASK
}

#[inline]
pub unsafe fn dpaa2_dq_byte_count(dq: *const dpaa2_dq) -> u32 {
    le32_to_cpu((*dq).u.dq.fq_byte_cnt)
}

#[inline]
pub unsafe fn dpaa2_dq_frame_count(dq: *const dpaa2_dq) -> u32 {
    le32_to_cpu((*dq).u.dq.fq_frm_cnt) & DQ_FRAME_COUNT_MASK
}

#[inline]
pub unsafe fn dpaa2_dq_fqd_ctx(dq: *const dpaa2_dq) -> u64 {
    le64_to_cpu((*dq).u.dq.fqd_ctx)
}

#[inline]
pub unsafe fn dpaa2_dq_fd(dq: *const dpaa2_dq) -> *const dpaa2_fd {
    (*dq).u.dq.fd.as_ptr() as *const dpaa2_fd
}

pub const DPAA2_CSCN_SIZE: usize = core::mem::size_of::<dpaa2_dq>();
pub const DPAA2_CSCN_ALIGN: usize = 16;
pub const DPAA2_CSCN_STATE_CG: u32 = 1u32 << 0;

#[inline]
pub unsafe fn dpaa2_cscn_state_congested(cscn: *mut dpaa2_dq) -> bool {
    ((*cscn).u.scn.state as u32 & DPAA2_CSCN_STATE_CG) != 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
