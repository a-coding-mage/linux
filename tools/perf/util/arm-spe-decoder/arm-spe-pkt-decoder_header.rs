/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Arm Statistical Profiling Extensions (SPE) support
 * Copyright (c) 2017-2018, Arm Ltd.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::ffi::c_char;

/* Original C header included <linux/bitfield.h>, <stddef.h>, and <stdint.h>. */

pub type u64 = u64;

pub const fn BIT(nr: u32) -> u64 {
    1u64 << nr
}

pub const fn BIT_ULL(nr: u32) -> u64 {
    1u64 << nr
}

pub const fn GENMASK_ULL(h: u32, l: u32) -> u64 {
    if h >= 63 {
        u64::MAX << l
    } else {
        ((1u64 << (h + 1)) - 1) & !((1u64 << l) - 1)
    }
}

pub const fn FIELD_GET(mask: u64, reg: u64) -> u64 {
    (reg & mask) >> mask.trailing_zeros()
}

pub const ARM_SPE_PKT_DESC_MAX: usize = 512;

pub const ARM_SPE_NEED_MORE_BYTES: i32 = -1;
pub const ARM_SPE_BAD_PACKET: i32 = -2;

pub const ARM_SPE_PKT_MAX_SZ: usize = 16;

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum arm_spe_pkt_type {
    ARM_SPE_BAD,
    ARM_SPE_PAD,
    ARM_SPE_END,
    ARM_SPE_TIMESTAMP,
    ARM_SPE_ADDRESS,
    ARM_SPE_COUNTER,
    ARM_SPE_CONTEXT,
    ARM_SPE_OP_TYPE,
    ARM_SPE_EVENTS,
    ARM_SPE_DATA_SOURCE,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct arm_spe_pkt {
    pub type_: arm_spe_pkt_type,
    pub index: u8,
    pub payload: u64,
    pub midr: u64,
}

/* Short header (HEADER0) and extended header (HEADER1) */
pub const SPE_HEADER0_PAD: u64 = 0x0;
pub const SPE_HEADER0_END: u64 = 0x1;
pub const SPE_HEADER0_TIMESTAMP: u64 = 0x71;
/* Mask for event & data source */
pub const SPE_HEADER0_MASK1: u64 = GENMASK_ULL(7, 6) | GENMASK_ULL(3, 0);
pub const SPE_HEADER0_EVENTS: u64 = 0x42;
pub const SPE_HEADER0_SOURCE: u64 = 0x43;
/* Mask for context & operation */
pub const SPE_HEADER0_MASK2: u64 = GENMASK_ULL(7, 2);
pub const SPE_HEADER0_CONTEXT: u64 = 0x64;
pub const SPE_HEADER0_OP_TYPE: u64 = 0x48;
/* Mask for extended format */
pub const SPE_HEADER0_EXTENDED: u64 = 0x20;
/* Mask for address & counter */
pub const SPE_HEADER0_MASK3: u64 = GENMASK_ULL(7, 3);
pub const SPE_HEADER0_ADDRESS: u64 = 0xb0;
pub const SPE_HEADER0_COUNTER: u64 = 0x98;
pub const SPE_HEADER1_ALIGNMENT: u64 = 0x0;

pub const fn SPE_HDR_SHORT_INDEX(h: u64) -> u64 {
    h & GENMASK_ULL(2, 0)
}

pub const fn SPE_HDR_EXTENDED_INDEX(h0: u64, h1: u64) -> u64 {
    ((h0 & GENMASK_ULL(1, 0)) << 3) | SPE_HDR_SHORT_INDEX(h1)
}

/* Address packet header */
pub const SPE_ADDR_PKT_HDR_INDEX_INS: u64 = 0x0;
pub const SPE_ADDR_PKT_HDR_INDEX_BRANCH: u64 = 0x1;
pub const SPE_ADDR_PKT_HDR_INDEX_DATA_VIRT: u64 = 0x2;
pub const SPE_ADDR_PKT_HDR_INDEX_DATA_PHYS: u64 = 0x3;
pub const SPE_ADDR_PKT_HDR_INDEX_PREV_BRANCH: u64 = 0x4;

/* Address packet payload */
pub const SPE_ADDR_PKT_ADDR_BYTE7_SHIFT: u32 = 56;

pub const fn SPE_ADDR_PKT_ADDR_GET_BYTES_0_6(v: u64) -> u64 {
    v & GENMASK_ULL(55, 0)
}

pub const fn SPE_ADDR_PKT_ADDR_GET_BYTE_6(v: u64) -> u64 {
    (v & GENMASK_ULL(55, 48)) >> 48
}

pub const fn SPE_ADDR_PKT_GET_NS(v: u64) -> u64 {
    (v & BIT_ULL(63)) >> 63
}

pub const fn SPE_ADDR_PKT_GET_EL(v: u64) -> u64 {
    (v & GENMASK_ULL(62, 61)) >> 61
}

pub const fn SPE_ADDR_PKT_GET_CH(v: u64) -> u64 {
    (v & BIT_ULL(62)) >> 62
}

pub const fn SPE_ADDR_PKT_GET_PAT(v: u64) -> u64 {
    (v & GENMASK_ULL(59, 56)) >> 56
}

pub const SPE_ADDR_PKT_EL0: u64 = 0;
pub const SPE_ADDR_PKT_EL1: u64 = 1;
pub const SPE_ADDR_PKT_EL2: u64 = 2;
pub const SPE_ADDR_PKT_EL3: u64 = 3;

/* Context packet header */
pub const fn SPE_CTX_PKT_HDR_INDEX(h: u64) -> u64 {
    h & GENMASK_ULL(1, 0)
}

/* Counter packet header */
pub const SPE_CNT_PKT_HDR_INDEX_TOTAL_LAT: u64 = 0x0;
pub const SPE_CNT_PKT_HDR_INDEX_ISSUE_LAT: u64 = 0x1;
pub const SPE_CNT_PKT_HDR_INDEX_TRANS_LAT: u64 = 0x2;

/* Event packet payload */
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum arm_spe_events {
    EV_EXCEPTION_GEN = 0,
    EV_RETIRED = 1,
    EV_L1D_ACCESS = 2,
    EV_L1D_REFILL = 3,
    EV_TLB_ACCESS = 4,
    EV_TLB_WALK = 5,
    EV_NOT_TAKEN = 6,
    EV_MISPRED = 7,
    EV_LLC_ACCESS = 8,
    EV_LLC_MISS = 9,
    EV_REMOTE_ACCESS = 10,
    EV_ALIGNMENT = 11,
    EV_TRANSACTIONAL = 16,
    EV_PARTIAL_PREDICATE = 17,
    EV_EMPTY_PREDICATE = 18,
    EV_L2D_ACCESS = 19,
    EV_L2D_MISS = 20,
    EV_CACHE_DATA_MODIFIED = 21,
    EV_RECENTLY_FETCHED = 22,
    EV_DATA_SNOOPED = 23,
    EV_STREAMING_SVE_MODE = 24,
    EV_SMCU = 25,
}

/* Operation packet header */
pub const fn SPE_OP_PKT_HDR_CLASS(h: u64) -> u64 {
    h & GENMASK_ULL(1, 0)
}

pub const SPE_OP_PKT_HDR_CLASS_OTHER: u64 = 0x0;
pub const SPE_OP_PKT_HDR_CLASS_LD_ST_ATOMIC: u64 = 0x1;
pub const SPE_OP_PKT_HDR_CLASS_BR_ERET: u64 = 0x2;

pub const fn SPE_OP_PKT_OTHER_SUBCLASS_OTHER(v: u64) -> bool {
    (v & GENMASK_ULL(7, 3)) == 0x0
}

pub const fn SPE_OP_PKT_OTHER_SUBCLASS_SVE(v: u64) -> bool {
    (v & (BIT(7) | BIT(3) | BIT(0))) == 0x8
}

pub const fn SPE_OP_PKT_OTHER_SUBCLASS_SME(v: u64) -> bool {
    (v & (BIT(7) | BIT(3) | BIT(0))) == 0x88
}

pub const SPE_OP_PKT_OTHER_ASE: u64 = BIT(2);
pub const SPE_OP_PKT_OTHER_FP: u64 = BIT(1);

/*
 * SME effective vector length or tile size (ETS) is stored in byte 0
 * bits [6:4,2]; the length is rounded up to a power of two and use 128
 * as one step, so ETS calculation is:
 *
 *   128 * (2 ^ bits [6:4,2]) = 32 << (bits [6:4,2])
 */
pub const fn SPE_OP_PKG_SME_ETS(v: u64) -> u64 {
    128u64 << ((FIELD_GET(GENMASK_ULL(6, 4), v) << 1) | FIELD_GET(BIT(2), v))
}

pub const fn SPE_OP_PKT_LDST_SUBCLASS_GP_REG(v: u64) -> bool {
    (v & GENMASK_ULL(7, 1)) == 0x0
}

pub const fn SPE_OP_PKT_LDST_SUBCLASS_SIMD_FP(v: u64) -> bool {
    (v & GENMASK_ULL(7, 1)) == 0x4
}

pub const fn SPE_OP_PKT_LDST_SUBCLASS_UNSPEC_REG(v: u64) -> bool {
    (v & GENMASK_ULL(7, 1)) == 0x10
}

pub const fn SPE_OP_PKT_LDST_SUBCLASS_NV_SYSREG(v: u64) -> bool {
    (v & GENMASK_ULL(7, 1)) == 0x30
}

pub const fn SPE_OP_PKT_LDST_SUBCLASS_MTE_TAG(v: u64) -> bool {
    (v & GENMASK_ULL(7, 1)) == 0x14
}

pub const fn SPE_OP_PKT_LDST_SUBCLASS_MEMCPY(v: u64) -> bool {
    (v & GENMASK_ULL(7, 1)) == 0x20
}

pub const fn SPE_OP_PKT_LDST_SUBCLASS_MEMSET(v: u64) -> bool {
    (v & GENMASK_ULL(7, 0)) == 0x25
}

pub const fn SPE_OP_PKT_LDST_SUBCLASS_EXTENDED(v: u64) -> bool {
    (v & (GENMASK_ULL(7, 5) | BIT(1))) == 0x2
}

pub const SPE_OP_PKT_AR: u64 = BIT(4);
pub const SPE_OP_PKT_EXCL: u64 = BIT(3);
pub const SPE_OP_PKT_AT: u64 = BIT(2);
pub const SPE_OP_PKT_ST: u64 = BIT(0);

pub const fn SPE_OP_PKT_LDST_SUBCLASS_SVE_SME_REG(v: u64) -> bool {
    (v & (BIT(3) | BIT(1))) == 0x8
}

pub const SPE_OP_PKT_SVE_SG: u64 = BIT(7);
/*
 * SVE effective vector length (EVL) is stored in byte 0 bits [6:4];
 * the length is rounded up to a power of two and use 32 as one step,
 * so EVL calculation is:
 *
 *   32 * (2 ^ bits [6:4]) = 32 << (bits [6:4])
 */
pub const fn SPE_OP_PKG_SVE_EVL(v: u64) -> u64 {
    32u64 << ((v & GENMASK_ULL(6, 4)) >> 4)
}

pub const SPE_OP_PKT_SVE_PRED: u64 = BIT(2);
pub const SPE_OP_PKT_SVE_FP: u64 = BIT(1);

pub const fn SPE_OP_PKT_LDST_SUBCLASS_GCS(v: u64) -> bool {
    (v & (GENMASK_ULL(7, 3) | BIT(1))) == 0x40
}

pub const SPE_OP_PKT_GCS_COMM: u64 = BIT(2);

pub const SPE_OP_PKT_CR_MASK: u64 = GENMASK_ULL(4, 3);

pub const fn SPE_OP_PKT_CR_BL(v: u64) -> bool {
    FIELD_GET(SPE_OP_PKT_CR_MASK, v) == 1
}

pub const fn SPE_OP_PKT_CR_RET(v: u64) -> bool {
    FIELD_GET(SPE_OP_PKT_CR_MASK, v) == 2
}

pub const fn SPE_OP_PKT_CR_NON_BL_RET(v: u64) -> bool {
    FIELD_GET(SPE_OP_PKT_CR_MASK, v) == 3
}

pub const SPE_OP_PKT_GCS: u64 = BIT(2);
pub const SPE_OP_PKT_INDIRECT_BRANCH: u64 = BIT(1);
pub const SPE_OP_PKT_COND: u64 = BIT(0);

unsafe extern "C" {
    pub fn arm_spe_pkt_name(type_: arm_spe_pkt_type) -> *const c_char;

    pub fn arm_spe_get_packet(
        buf: *const u8,
        len: usize,
        packet: *mut arm_spe_pkt,
        midr: u64,
    ) -> i32;

    pub fn arm_spe_pkt_desc(packet: *const arm_spe_pkt, buf: *mut c_char, len: usize) -> i32;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
