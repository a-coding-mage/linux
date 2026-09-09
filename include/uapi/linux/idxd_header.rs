/* SPDX-License-Identifier: LGPL-2.1 WITH Linux-syscall-note */
/* Copyright(c) 2019 Intel Corporation. All rights rsvd. */

/* Linux type aliases are represented by Rust's fixed-width integer types. */

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum idxd_scmd_stat {
    IDXD_SCMD_DEV_ENABLED = 0x80000010,
    IDXD_SCMD_DEV_NOT_ENABLED = 0x80000020,
    IDXD_SCMD_WQ_ENABLED = 0x80000021,
    IDXD_SCMD_DEV_DMA_ERR = 0x80020000,
    IDXD_SCMD_WQ_NO_GRP = 0x80030000,
    IDXD_SCMD_WQ_NO_NAME = 0x80040000,
    IDXD_SCMD_WQ_NO_SVM = 0x80050000,
    IDXD_SCMD_WQ_NO_THRESH = 0x80060000,
    IDXD_SCMD_WQ_PORTAL_ERR = 0x80070000,
    IDXD_SCMD_WQ_RES_ALLOC_ERR = 0x80080000,
    IDXD_SCMD_PERCPU_ERR = 0x80090000,
    IDXD_SCMD_DMA_CHAN_ERR = 0x800a0000,
    IDXD_SCMD_CDEV_ERR = 0x800b0000,
    IDXD_SCMD_WQ_NO_SWQ_SUPPORT = 0x800c0000,
    IDXD_SCMD_WQ_NONE_CONFIGURED = 0x800d0000,
    IDXD_SCMD_WQ_NO_SIZE = 0x800e0000,
    IDXD_SCMD_WQ_NO_PRIV = 0x800f0000,
    IDXD_SCMD_WQ_IRQ_ERR = 0x80100000,
    IDXD_SCMD_WQ_USER_NO_IOMMU = 0x80110000,
    IDXD_SCMD_DEV_EVL_ERR = 0x80120000,
    IDXD_SCMD_WQ_NO_DRV_NAME = 0x80200000,
}

pub const IDXD_SCMD_SOFTERR_MASK: u32 = 0x80000000;
pub const IDXD_SCMD_SOFTERR_SHIFT: u32 = 16;

pub const IDXD_OP_FLAG_FENCE: u32 = 0x0001;
pub const IDXD_OP_FLAG_BOF: u32 = 0x0002;
pub const IDXD_OP_FLAG_CRAV: u32 = 0x0004;
pub const IDXD_OP_FLAG_RCR: u32 = 0x0008;
pub const IDXD_OP_FLAG_RCI: u32 = 0x0010;
pub const IDXD_OP_FLAG_CRSTS: u32 = 0x0020;
pub const IDXD_OP_FLAG_CR: u32 = 0x0080;
pub const IDXD_OP_FLAG_CC: u32 = 0x0100;
pub const IDXD_OP_FLAG_ADDR1_TCS: u32 = 0x0200;
pub const IDXD_OP_FLAG_ADDR2_TCS: u32 = 0x0400;
pub const IDXD_OP_FLAG_ADDR3_TCS: u32 = 0x0800;
pub const IDXD_OP_FLAG_CR_TCS: u32 = 0x1000;
pub const IDXD_OP_FLAG_STORD: u32 = 0x2000;
pub const IDXD_OP_FLAG_DRDBK: u32 = 0x4000;
pub const IDXD_OP_FLAG_DSTS: u32 = 0x8000;
pub const IDXD_OP_FLAG_RD_SRC2_AECS: u32 = 0x010000;
pub const IDXD_OP_FLAG_RD_SRC2_2ND: u32 = 0x020000;
pub const IDXD_OP_FLAG_WR_SRC2_AECS_COMP: u32 = 0x040000;
pub const IDXD_OP_FLAG_WR_SRC2_AECS_OVFL: u32 = 0x080000;
pub const IDXD_OP_FLAG_SRC2_STS: u32 = 0x100000;
pub const IDXD_OP_FLAG_CRC_RFC3720: u32 = 0x200000;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dsa_opcode { DSA_OPCODE_NOOP = 0, DSA_OPCODE_BATCH, DSA_OPCODE_DRAIN, DSA_OPCODE_MEMMOVE, DSA_OPCODE_MEMFILL, DSA_OPCODE_COMPARE, DSA_OPCODE_COMPVAL, DSA_OPCODE_CR_DELTA, DSA_OPCODE_AP_DELTA, DSA_OPCODE_DUALCAST, DSA_OPCODE_TRANSL_FETCH, DSA_OPCODE_CRCGEN = 0x10, DSA_OPCODE_COPY_CRC, DSA_OPCODE_DIF_CHECK, DSA_OPCODE_DIF_INS, DSA_OPCODE_DIF_STRP, DSA_OPCODE_DIF_UPDT, DSA_OPCODE_DIX_GEN = 0x17, DSA_OPCODE_CFLUSH = 0x20 }
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum iax_opcode { IAX_OPCODE_NOOP = 0, IAX_OPCODE_DRAIN = 2, IAX_OPCODE_MEMMOVE, IAX_OPCODE_DECOMPRESS = 0x42, IAX_OPCODE_COMPRESS, IAX_OPCODE_CRC64, IAX_OPCODE_ZERO_DECOMP_32 = 0x48, IAX_OPCODE_ZERO_DECOMP_16, IAX_OPCODE_ZERO_COMP_32 = 0x4c, IAX_OPCODE_ZERO_COMP_16, IAX_OPCODE_SCAN = 0x50, IAX_OPCODE_SET_MEMBER, IAX_OPCODE_EXTRACT, IAX_OPCODE_SELECT, IAX_OPCODE_RLE_BURST, IAX_OPCODE_FIND_UNIQUE, IAX_OPCODE_EXPAND }

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dsa_completion_status { DSA_COMP_NONE=0, DSA_COMP_SUCCESS, DSA_COMP_SUCCESS_PRED, DSA_COMP_PAGE_FAULT_NOBOF, DSA_COMP_PAGE_FAULT_IR, DSA_COMP_BATCH_FAIL, DSA_COMP_BATCH_PAGE_FAULT, DSA_COMP_DR_OFFSET_NOINC, DSA_COMP_DR_OFFSET_ERANGE, DSA_COMP_DIF_ERR, DSA_COMP_BAD_OPCODE=0x10, DSA_COMP_INVALID_FLAGS, DSA_COMP_NOZERO_RESERVE, DSA_COMP_XFER_ERANGE, DSA_COMP_DESC_CNT_ERANGE, DSA_COMP_DR_ERANGE, DSA_COMP_OVERLAP_BUFFERS, DSA_COMP_DCAST_ERR, DSA_COMP_DESCLIST_ALIGN, DSA_COMP_INT_HANDLE_INVAL, DSA_COMP_CRA_XLAT, DSA_COMP_CRA_ALIGN, DSA_COMP_ADDR_ALIGN, DSA_COMP_PRIV_BAD, DSA_COMP_TRAFFIC_CLASS_CONF, DSA_COMP_PFAULT_RDBA, DSA_COMP_HW_ERR1, DSA_COMP_HW_ERR_DRB, DSA_COMP_TRANSLATION_FAIL, DSA_COMP_DRAIN_EVL=0x26, DSA_COMP_BATCH_EVL_ERR }
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum iax_completion_status { IAX_COMP_NONE=0, IAX_COMP_SUCCESS, IAX_COMP_PAGE_FAULT_IR=0x04, IAX_COMP_ANALYTICS_ERROR=0x0a, IAX_COMP_OUTBUF_OVERFLOW, IAX_COMP_BAD_OPCODE=0x10, IAX_COMP_INVALID_FLAGS, IAX_COMP_NOZERO_RESERVE, IAX_COMP_INVALID_SIZE, IAX_COMP_OVERLAP_BUFFERS=0x16, IAX_COMP_INT_HANDLE_INVAL=0x19, IAX_COMP_CRA_XLAT, IAX_COMP_CRA_ALIGN, IAX_COMP_ADDR_ALIGN, IAX_COMP_PRIV_BAD, IAX_COMP_TRAFFIC_CLASS_CONF, IAX_COMP_PFAULT_RDBA, IAX_COMP_HW_ERR1, IAX_COMP_HW_ERR_DRB, IAX_COMP_TRANSLATION_FAIL, IAX_COMP_PRS_TIMEOUT, IAX_COMP_WATCHDOG, IAX_COMP_INVALID_COMP_FLAG=0x30, IAX_COMP_INVALID_FILTER_FLAG, IAX_COMP_INVALID_INPUT_SIZE, IAX_COMP_INVALID_NUM_ELEMS, IAX_COMP_INVALID_SRC1_WIDTH, IAX_COMP_INVALID_INVERT_OUT }

pub const DSA_COMP_STATUS_MASK: u8 = 0x7f;
pub const DSA_COMP_STATUS_WRITE: u8 = 0x80;
#[inline] pub const fn DSA_COMP_STATUS(status: u8) -> u8 { status & DSA_COMP_STATUS_MASK }

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct dsa_hw_desc {
    pub pasid_rsvd_priv: u32, pub flags_opcode: u32, pub completion_addr: u64,
    pub src: dsa_src_union, pub dst: dsa_dst_union, pub size: dsa_size_union,
    pub int_handle: u16, pub rsvd1: u16, pub op_specific: [u8; 24],
}
#[repr(C)] #[derive(Copy, Clone)] pub union dsa_src_union { pub src_addr:u64, pub rdback_addr:u64, pub pattern:u64, pub desc_list_addr:u64, pub pattern_lower:u64, pub transl_fetch_addr:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union dsa_dst_union { pub dst_addr:u64, pub rdback_addr2:u64, pub src2_addr:u64, pub comp_pattern:u64 }
#[repr(C)] #[derive(Copy, Clone)] pub union dsa_size_union { pub xfer_size:u32, pub desc_count:u32, pub region_size:u32 }

#[repr(C, packed)] #[derive(Copy, Clone)] pub struct iax_hw_desc { pub pasid_rsvd_priv:u32, pub flags_opcode:u32, pub completion_addr:u64, pub src1_addr:u64, pub dst_addr:u64, pub src1_size:u32, pub int_handle:u16, pub compr_flags:u16, pub src2_addr:u64, pub max_dst_size:u32, pub src2_size:u32, pub filter_flags:u32, pub num_inputs:u32 }
#[repr(C, packed)] #[derive(Copy, Clone)] pub struct dsa_raw_desc { pub field:[u64;8] }
#[repr(C, packed)] #[derive(Copy, Clone)] pub struct dsa_completion_record { pub status:u8, pub result_dif_status:u8, pub fault_info:u8, pub rsvd:u8, pub bytes_descs_completed:u32, pub fault_addr:u64, pub op_specific:[u8;16] }
#[repr(C, packed)] #[derive(Copy, Clone)] pub struct dsa_raw_completion_record { pub field:[u64;4] }
#[repr(C, packed)] #[derive(Copy, Clone)] pub struct iax_completion_record { pub status:u8, pub error_code:u8, pub fault_info:u8, pub rsvd:u8, pub bytes_completed:u32, pub fault_addr:u64, pub invalid_flags:u32, pub rsvd2:u32, pub output_size:u32, pub output_bits:u8, pub rsvd3:u8, pub xor_csum:u16, pub crc:u32, pub min:u32, pub max:u32, pub sum:u32, pub rsvd4:[u64;2] }
#[repr(C, packed)] #[derive(Copy, Clone)] pub struct iax_raw_completion_record { pub field:[u64;8] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
