/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * ICSWX api
 *
 * Copyright (C) 2015 IBM Corp.
 *
 * This provides the Initiate Coprocessor Store Word Indexed (ICSWX)
 * instruction.  This instruction is used to communicate with PowerPC
 * coprocessors.  This also provides definitions of the structures used
 * to communicate with the coprocessor.
 *
 * The RFC02130: Coprocessor Architecture document is the reference for
 * everything in this file unless otherwise noted.
 *
 * Dependency: PPC_ICSWX is supplied by asm/ppc-opcode.h.
 */

/* Chapter 6.5.8 Coprocessor-Completion Block (CCB) */
pub const CCB_VALUE: u64 = 0x3fffffffffffffff;
pub const CCB_ADDRESS: u64 = 0xfffffffffffffff8;
pub const CCB_CM: u64 = 0x0000000000000007;
pub const CCB_CM0: u64 = 0x0000000000000004;
pub const CCB_CM12: u64 = 0x0000000000000003;

pub const CCB_CM0_ALL_COMPLETIONS: u64 = 0x0;
pub const CCB_CM0_LAST_IN_CHAIN: u64 = 0x4;
pub const CCB_CM12_STORE: u64 = 0x0;
pub const CCB_CM12_INTERRUPT: u64 = 0x1;

pub const CCB_SIZE: usize = 0x10;
pub const CCB_ALIGN: usize = CCB_SIZE;

#[repr(C, packed(1), align(16))]
pub struct coprocessor_completion_block {
    pub value: __be64,
    pub address: __be64,
}

/* Chapter 6.5.7 Coprocessor-Status Block (CSB) */
pub const CSB_V: u8 = 0x80;
pub const CSB_F: u8 = 0x04;
pub const CSB_CH: u8 = 0x03;
pub const CSB_CE_INCOMPLETE: u8 = 0x80;
pub const CSB_CE_TERMINATION: u8 = 0x40;
pub const CSB_CE_TPBC: u8 = 0x20;

pub const CSB_CC_SUCCESS: u8 = 0;
pub const CSB_CC_INVALID_ALIGN: u8 = 1;
pub const CSB_CC_OPERAND_OVERLAP: u8 = 2;
pub const CSB_CC_DATA_LENGTH: u8 = 3;
pub const CSB_CC_TRANSLATION: u8 = 5;
pub const CSB_CC_PROTECTION: u8 = 6;
pub const CSB_CC_RD_EXTERNAL: u8 = 7;
pub const CSB_CC_INVALID_OPERAND: u8 = 8;
pub const CSB_CC_PRIVILEGE: u8 = 9;
pub const CSB_CC_INTERNAL: u8 = 10;
pub const CSB_CC_WR_EXTERNAL: u8 = 12;
pub const CSB_CC_NOSPC: u8 = 13;
pub const CSB_CC_EXCESSIVE_DDE: u8 = 14;
pub const CSB_CC_WR_TRANSLATION: u8 = 15;
pub const CSB_CC_WR_PROTECTION: u8 = 16;
pub const CSB_CC_UNKNOWN_CODE: u8 = 17;
pub const CSB_CC_ABORT: u8 = 18;
pub const CSB_CC_EXCEED_BYTE_COUNT: u8 = 19; /* P9 or later */
pub const CSB_CC_TRANSPORT: u8 = 20;
pub const CSB_CC_INVALID_CRB: u8 = 21; /* P9 or later */
pub const CSB_CC_INVALID_DDE: u8 = 30; /* P9 or later */
pub const CSB_CC_SEGMENTED_DDL: u8 = 31;
pub const CSB_CC_PROGRESS_POINT: u8 = 32;
pub const CSB_CC_DDE_OVERFLOW: u8 = 33;
pub const CSB_CC_SESSION: u8 = 34;
pub const CSB_CC_PROVISION: u8 = 36;
pub const CSB_CC_CHAIN: u8 = 37;
pub const CSB_CC_SEQUENCE: u8 = 38;
pub const CSB_CC_HW: u8 = 39;
/* P9 DD2 NX Workbook 3.2 (Table 4-36): Address translation fault */
pub const CSB_CC_FAULT_ADDRESS: u16 = 250;

pub const CSB_SIZE: usize = 0x10;
pub const CSB_ALIGN: usize = CSB_SIZE;

#[repr(C, packed(1), align(16))]
pub struct coprocessor_status_block {
    pub flags: u8,
    pub cs: u8,
    pub cc: u8,
    pub ce: u8,
    pub count: __be32,
    pub address: __be64,
}

/* Chapter 6.5.10 Data-Descriptor List (DDL)
 * each list contains one or more Data-Descriptor Entries (DDE)
 */
pub const DDE_P: u16 = 0x8000;
pub const DDE_SIZE: usize = 0x10;
pub const DDE_ALIGN: usize = DDE_SIZE;

#[repr(C, packed(1), align(16))]
pub struct data_descriptor_entry {
    pub flags: __be16,
    pub count: u8,
    pub index: u8,
    pub length: __be32,
    pub address: __be64,
}

/* 4.3.2 NX-stamped Fault CRB */
pub const NX_STAMP_ALIGN: usize = 0x10;

#[repr(C, packed(1), align(16))]
pub struct nx_fault_stamp {
    pub fault_storage_addr: __be64,
    pub reserved: __be16,
    pub flags: u8,
    pub fault_status: u8,
    pub pswid: __be32,
}

/* Chapter 6.5.2 Coprocessor-Request Block (CRB) */
pub const CRB_SIZE: usize = 0x80;
pub const CRB_ALIGN: usize = 0x100; /* Errata: requires 256 alignment */

pub const CRB_CSB_ADDRESS: u64 = 0xfffffffffffffff0;
pub const CRB_CSB_C: u64 = 0x0000000000000008;
pub const CRB_CSB_AT: u64 = 0x0000000000000002;
pub const CRB_CSB_M: u64 = 0x0000000000000001;

#[repr(C, align(128))]
pub union coprocessor_request_block_stamp {
    pub nx: nx_fault_stamp,
    pub reserved: [u8; 16],
}

#[repr(C, align(128))]
pub struct coprocessor_request_block {
    pub ccw: __be32,
    pub flags: __be32,
    pub csb_addr: __be64,
    pub source: data_descriptor_entry,
    pub target: data_descriptor_entry,
    pub ccb: coprocessor_completion_block,
    pub stamp: coprocessor_request_block_stamp,
    pub reserved: [u8; 32],
    pub csb: coprocessor_status_block,
}

/* The CCW must be converted to BE before passing to icswx(). */
pub const CCW_PS: u32 = 0xff000000;
pub const CCW_CT: u32 = 0x00ff0000;
pub const CCW_CD: u32 = 0x0000ffff;
pub const CCW_CL: u32 = 0x0000c000;

pub const ICSWX_INITIATED: u32 = 0x8;
pub const ICSWX_BUSY: u32 = 0x4;
pub const ICSWX_REJECTED: u32 = 0x2;
pub const ICSWX_XERS0: u32 = 0x1; /* undefined or set from XERSO. */

pub unsafe fn icswx(ccw: __be32, crb: *mut coprocessor_request_block) -> i32 {
    let ccw_reg: u64 = ccw as u64;
    let mut cr: u32;
    // PPC_ICSWX and the PowerPC inline assembly are supplied by dependencies.
    core::arch::asm!(
        "icswx {ccw_reg}, 0, {crb}",
        "mfcr {cr}",
        ccw_reg = in(reg) ccw_reg,
        crb = in(reg) crb,
        cr = out(reg) cr,
        lateout("cr0") _,
        options(nostack)
    );
    ((cr >> 28) & 0xf) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
