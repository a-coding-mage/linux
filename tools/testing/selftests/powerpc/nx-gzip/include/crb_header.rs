/* SPDX-License-Identifier: GPL-2.0-or-later */

// Translated from C header dependencies:
// #include <linux/types.h>
// #include "nx.h"

#[allow(non_camel_case_types)]
pub type __u8 = u8;
#[allow(non_camel_case_types)]
pub type __be16 = u16;
#[allow(non_camel_case_types)]
pub type __be32 = u32;
#[allow(non_camel_case_types)]
pub type __be64 = u64;

/* CCW 842 CI/FC masks
 * NX P8 workbook, section 4.3.1, figure 4-6
 * "CI/FC Boundary by NX CT type"
 */
pub const CCW_CI_842: u32 = 0x00003ff8;
pub const CCW_FC_842: u32 = 0x00000007;

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

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Default)]
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
pub const CSB_CC_TRANSPORT: u8 = 20;
pub const CSB_CC_SEGMENTED_DDL: u8 = 31;
pub const CSB_CC_PROGRESS_POINT: u8 = 32;
pub const CSB_CC_DDE_OVERFLOW: u8 = 33;
pub const CSB_CC_SESSION: u8 = 34;
pub const CSB_CC_PROVISION: u8 = 36;
pub const CSB_CC_CHAIN: u8 = 37;
pub const CSB_CC_SEQUENCE: u8 = 38;
pub const CSB_CC_HW: u8 = 39;

pub const CSB_SIZE: usize = 0x10;
pub const CSB_ALIGN: usize = CSB_SIZE;

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Default)]
pub struct coprocessor_status_block {
    pub flags: __u8,
    pub cs: __u8,
    pub cc: __u8,
    pub ce: __u8,
    pub count: __be32,
    pub address: __be64,
}

/* Chapter 6.5.10 Data-Descriptor List (DDL)
 * each list contains one or more Data-Descriptor Entries (DDE)
 */

pub const DDE_P: u16 = 0x8000;

pub const DDE_SIZE: usize = 0x10;
pub const DDE_ALIGN: usize = DDE_SIZE;

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Default)]
pub struct data_descriptor_entry {
    pub flags: __be16,
    pub count: __u8,
    pub index: __u8,
    pub length: __be32,
    pub address: __be64,
}

/* Chapter 6.5.2 Coprocessor-Request Block (CRB) */

pub const CRB_SIZE: usize = 0x80;
pub const CRB_ALIGN: usize = 0x100; /* Errata: requires 256 alignment */

/* Coprocessor Status Block field
 *   ADDRESS	address of CSB
 *   C		CCB is valid
 *   AT		0 = addrs are virtual, 1 = addrs are phys
 *   M		enable perf monitor
 */
pub const CRB_CSB_ADDRESS: u64 = 0xfffffffffffffff0;
pub const CRB_CSB_C: u64 = 0x0000000000000008;
pub const CRB_CSB_AT: u64 = 0x0000000000000002;
pub const CRB_CSB_M: u64 = 0x0000000000000001;

#[repr(C, align(256))]
#[derive(Copy, Clone, Debug)]
pub struct coprocessor_request_block {
    pub ccw: __be32,
    pub flags: __be32,
    pub csb_addr: __be64,

    pub source: data_descriptor_entry,
    pub target: data_descriptor_entry,

    pub ccb: coprocessor_completion_block,

    pub reserved: [__u8; 48],

    pub csb: coprocessor_status_block,
}

impl Default for coprocessor_request_block {
    fn default() -> Self {
        Self {
            ccw: 0,
            flags: 0,
            csb_addr: 0,
            source: data_descriptor_entry::default(),
            target: data_descriptor_entry::default(),
            ccb: coprocessor_completion_block::default(),
            reserved: [0; 48],
            csb: coprocessor_status_block::default(),
        }
    }
}

#[inline]
pub fn crb_csb_addr(c: *const coprocessor_request_block) -> u64 {
    unsafe { u64::from_be((*c).csb_addr) }
}

// The following C macros refer to c->stamp.nx fields whose containing type is
// supplied outside this isolated header:
// #define crb_nx_fault_addr(c)    __be64_to_cpu(c->stamp.nx.fault_storage_addr)
// #define crb_nx_flags(c)         c->stamp.nx.flags
// #define crb_nx_fault_status(c)  c->stamp.nx.fault_status
// #define crb_nx_pswid(c)         c->stamp.nx.pswid

/* RFC02167 Initiate Coprocessor Instructions document
 * Chapter 8.2.1.1.1 RS
 * Chapter 8.2.3 Coprocessor Directive
 * Chapter 8.2.4 Execution
 *
 * The CCW must be converted to BE before passing to icswx()
 */

pub const CCW_PS: u32 = 0xff000000;
pub const CCW_CT: u32 = 0x00ff0000;
pub const CCW_CD: u32 = 0x0000ffff;
pub const CCW_CL: u32 = 0x0000c000;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
