/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by asm/msr-index.h in the original header.

/*
 * From PPR Vol 1 for AMD Family 19h Model 01h B1
 * 55898 Rev 0.35 - Feb 5, 2021
 */

/* IBS_OP_DATA2 DataSrc */
pub const IBS_DATA_SRC_LOC_CACHE: u64 = 2;
pub const IBS_DATA_SRC_DRAM: u64 = 3;
pub const IBS_DATA_SRC_REM_CACHE: u64 = 4;
pub const IBS_DATA_SRC_IO: u64 = 7;

/* IBS_OP_DATA2 DataSrc Extension */
pub const IBS_DATA_SRC_EXT_LOC_CACHE: u64 = 1;
pub const IBS_DATA_SRC_EXT_NEAR_CCX_CACHE: u64 = 2;
pub const IBS_DATA_SRC_EXT_DRAM: u64 = 3;
pub const IBS_DATA_SRC_EXT_FAR_CCX_CACHE: u64 = 5;
pub const IBS_DATA_SRC_EXT_PMEM: u64 = 6;
pub const IBS_DATA_SRC_EXT_IO: u64 = 7;
pub const IBS_DATA_SRC_EXT_EXT_MEM: u64 = 8;
pub const IBS_DATA_SRC_EXT_PEER_AGENT_MEM: u64 = 12;

/* IBS Hardware MSRs */

/* MSR 0xc0011030: IBS Fetch Control */
#[repr(C)]
pub union ibs_fetch_ctl {
    pub val: u64,
    pub bits: u64,
}

/* MSR 0xc0011033: IBS Execution Control */
#[repr(C)]
pub union ibs_op_ctl {
    pub val: u64,
    pub bits: u64,
}

/* MSR 0xc0011035: IBS Op Data 1 */
#[repr(C)]
pub union ibs_op_data {
    pub val: u64,
    pub bits: u64,
}

/* MSR 0xc0011036: IBS Op Data 2 */
#[repr(C)]
pub union ibs_op_data2 {
    pub val: u64,
    pub bits: u64,
}

/* MSR 0xc0011037: IBS Op Data 3 */
#[repr(C)]
pub union ibs_op_data3 {
    pub val: u64,
    pub bits: u64,
}

/* MSR 0xc001103c: IBS Fetch Control Extended */
#[repr(C)]
pub union ic_ibs_extd_ctl {
    pub val: u64,
    pub bits: u64,
}

/* Bitfield masks and shifts for the C anonymous bitfield members. */
pub const IBS_FETCH_CTL_FETCH_MAXCNT_SHIFT: u32 = 0;
pub const IBS_FETCH_CTL_FETCH_CNT_SHIFT: u32 = 16;
pub const IBS_FETCH_CTL_FETCH_LAT_SHIFT: u32 = 32;
pub const IBS_FETCH_CTL_FETCH_EN_SHIFT: u32 = 48;
pub const IBS_FETCH_CTL_FETCH_VAL_SHIFT: u32 = 49;
pub const IBS_FETCH_CTL_FETCH_COMP_SHIFT: u32 = 50;
pub const IBS_FETCH_CTL_IC_MISS_SHIFT: u32 = 51;
pub const IBS_FETCH_CTL_PHY_ADDR_VALID_SHIFT: u32 = 52;
pub const IBS_FETCH_CTL_L1TLB_PGSZ_SHIFT: u32 = 53;
pub const IBS_FETCH_CTL_L1TLB_MISS_SHIFT: u32 = 55;
pub const IBS_FETCH_CTL_L2TLB_MISS_SHIFT: u32 = 56;
pub const IBS_FETCH_CTL_RAND_EN_SHIFT: u32 = 57;
pub const IBS_FETCH_CTL_FETCH_L2_MISS_SHIFT: u32 = 58;
pub const IBS_FETCH_CTL_L3_MISS_ONLY_SHIFT: u32 = 59;
pub const IBS_FETCH_CTL_FETCH_OC_MISS_SHIFT: u32 = 60;
pub const IBS_FETCH_CTL_FETCH_L3_MISS_SHIFT: u32 = 61;

pub const IBS_OP_CTL_OPMAXCNT_SHIFT: u32 = 0;
pub const IBS_OP_CTL_L3_MISS_ONLY_SHIFT: u32 = 16;
pub const IBS_OP_CTL_OP_EN_SHIFT: u32 = 17;
pub const IBS_OP_CTL_OP_VAL_SHIFT: u32 = 18;
pub const IBS_OP_CTL_CNT_CTL_SHIFT: u32 = 19;
pub const IBS_OP_CTL_OPMAXCNT_EXT_SHIFT: u32 = 20;
pub const IBS_OP_CTL_OPCURCNT_SHIFT: u32 = 32;
pub const IBS_OP_CTL_LDLAT_THRSH_SHIFT: u32 = 59;
pub const IBS_OP_CTL_LDLAT_EN_SHIFT: u32 = 63;

pub const IBS_OP_DATA_COMP_TO_RET_CTR_SHIFT: u32 = 0;
pub const IBS_OP_DATA_TAG_TO_RET_CTR_SHIFT: u32 = 16;
pub const IBS_OP_DATA_OP_RETURN_SHIFT: u32 = 34;
pub const IBS_OP_DATA_OP_BRN_TAKEN_SHIFT: u32 = 35;
pub const IBS_OP_DATA_OP_BRN_MISP_SHIFT: u32 = 36;
pub const IBS_OP_DATA_OP_BRN_RET_SHIFT: u32 = 37;
pub const IBS_OP_DATA_OP_RIP_INVALID_SHIFT: u32 = 38;
pub const IBS_OP_DATA_OP_BRN_FUSE_SHIFT: u32 = 39;
pub const IBS_OP_DATA_OP_MICROCODE_SHIFT: u32 = 40;

pub const IBS_OP_DATA2_DATA_SRC_LO_SHIFT: u32 = 0;
pub const IBS_OP_DATA2_RMT_NODE_SHIFT: u32 = 4;
pub const IBS_OP_DATA2_CACHE_HIT_ST_SHIFT: u32 = 5;
pub const IBS_OP_DATA2_DATA_SRC_HI_SHIFT: u32 = 6;
pub const IBS_OP_DATA2_STRM_ST_SHIFT: u32 = 8;
pub const IBS_OP_DATA2_RMT_SOCKET_SHIFT: u32 = 9;

pub const IBS_OP_DATA3_LD_OP_SHIFT: u32 = 0;
pub const IBS_OP_DATA3_ST_OP_SHIFT: u32 = 1;
pub const IBS_OP_DATA3_DC_L1TLB_MISS_SHIFT: u32 = 2;
pub const IBS_OP_DATA3_DC_L2TLB_MISS_SHIFT: u32 = 3;
pub const IBS_OP_DATA3_DC_L1TLB_HIT_2M_SHIFT: u32 = 4;
pub const IBS_OP_DATA3_DC_L1TLB_HIT_1G_SHIFT: u32 = 5;
pub const IBS_OP_DATA3_DC_L2TLB_HIT_2M_SHIFT: u32 = 6;
pub const IBS_OP_DATA3_DC_MISS_SHIFT: u32 = 7;
pub const IBS_OP_DATA3_DC_MIS_ACC_SHIFT: u32 = 8;
pub const IBS_OP_DATA3_DC_WC_MEM_ACC_SHIFT: u32 = 13;
pub const IBS_OP_DATA3_DC_UC_MEM_ACC_SHIFT: u32 = 14;
pub const IBS_OP_DATA3_DC_LOCKED_OP_SHIFT: u32 = 15;
pub const IBS_OP_DATA3_DC_MISS_NO_MAB_ALLOC_SHIFT: u32 = 16;
pub const IBS_OP_DATA3_DC_LIN_ADDR_VALID_SHIFT: u32 = 17;
pub const IBS_OP_DATA3_DC_PHY_ADDR_VALID_SHIFT: u32 = 18;
pub const IBS_OP_DATA3_DC_L2_TLB_HIT_1G_SHIFT: u32 = 19;
pub const IBS_OP_DATA3_L2_MISS_SHIFT: u32 = 20;
pub const IBS_OP_DATA3_SW_PF_SHIFT: u32 = 21;
pub const IBS_OP_DATA3_OP_MEM_WIDTH_SHIFT: u32 = 22;
pub const IBS_OP_DATA3_OP_DC_MISS_OPEN_MEM_REQS_SHIFT: u32 = 26;
pub const IBS_OP_DATA3_DC_MISS_LAT_SHIFT: u32 = 32;
pub const IBS_OP_DATA3_TLB_REFILL_LAT_SHIFT: u32 = 48;

pub const IC_IBS_EXTD_CTL_ITLB_REFILL_LAT_SHIFT: u32 = 0;

#[repr(C)]
pub struct perf_ibs_data {
    pub size: u32,
    pub data: [u32; 0],
    pub regs: [u64; MSR_AMD64_IBS_REG_COUNT_MAX as usize],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
