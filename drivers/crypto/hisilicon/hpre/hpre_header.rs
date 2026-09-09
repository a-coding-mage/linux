/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2019 HiSilicon Limited. */

// Dependencies supplied by the surrounding kernel translation are intentionally
// not redefined here.

pub const HPRE_SQE_SIZE: usize = core::mem::size_of::<hpre_sqe>();
pub const HPRE_PF_DEF_Q_NUM: i32 = 64;
pub const HPRE_PF_DEF_Q_BASE: i32 = 0;

/*
 * type used in qm sqc DW6.
 * 0 - Algorithm which has been supported in V2, like RSA, DH and so on;
 * 1 - ECC algorithm in V3.
 */
pub const HPRE_V2_ALG_TYPE: i32 = 0;
pub const HPRE_V3_ECC_ALG_TYPE: i32 = 1;

pub const HPRE_CLUSTER0: i32 = 0;
pub const HPRE_CLUSTER1: i32 = 1;
pub const HPRE_CLUSTER2: i32 = 2;
pub const HPRE_CLUSTER3: i32 = 3;
pub const HPRE_CLUSTERS_NUM_MAX: i32 = 4;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum hpre_ctrl_dbgfs_file {
    HPRE_CLEAR_ENABLE = 0,
    HPRE_CLUSTER_CTRL,
    HPRE_DEBUG_FILE_NUM,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum hpre_dfx_dbgfs_file {
    HPRE_SEND_CNT = 0,
    HPRE_RECV_CNT,
    HPRE_SEND_FAIL_CNT,
    HPRE_SEND_BUSY_CNT,
    HPRE_OVER_THRHLD_CNT,
    HPRE_OVERTIME_THRHLD,
    HPRE_INVALID_REQ_CNT,
    HPRE_DFX_FILE_NUM,
}

pub const HPRE_DEBUGFS_FILE_NUM: i32 = HPRE_DEBUG_FILE_NUM as i32 + HPRE_CLUSTERS_NUM_MAX - 1;

#[repr(C)]
pub struct hpre_debugfs_file {
    pub index: i32,
    pub type_: hpre_ctrl_dbgfs_file,
    pub lock: spinlock_t,
    pub debug: *mut hpre_debug,
}

#[repr(C)]
pub struct hpre_dfx {
    pub value: atomic64_t,
    pub type_: hpre_dfx_dbgfs_file,
}

/*
 * One HPRE controller has one PF and multiple VFs, some global configurations
 * which PF has need this structure.
 * Just relevant for PF.
 */
#[repr(C)]
pub struct hpre_debug {
    pub dfx: [hpre_dfx; HPRE_DFX_FILE_NUM as usize],
    pub files: [hpre_debugfs_file; HPRE_DEBUGFS_FILE_NUM as usize],
}

#[repr(C)]
pub struct hpre {
    pub qm: hisi_qm,
    pub debug: hpre_debug,
    pub status: c_ulong,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum hpre_alg_type {
    HPRE_ALG_NC_NCRT = 0x0,
    HPRE_ALG_NC_CRT = 0x1,
    HPRE_ALG_KG_STD = 0x2,
    HPRE_ALG_KG_CRT = 0x3,
    HPRE_ALG_DH_G2 = 0x4,
    HPRE_ALG_DH = 0x5,
    HPRE_ALG_ECC_MUL = 0xD,
    /* shared by x25519 and x448, but x448 is not supported now */
    HPRE_ALG_CURVE25519_MUL = 0x10,
}

#[repr(C)]
pub struct hpre_sqe {
    pub dw0: __le32,
    pub task_len1: __u8,
    pub task_len2: __u8,
    pub mrttest_num: __u8,
    pub resv1: __u8,
    pub key: __le64,
    pub in_: __le64,
    pub out: __le64,
    pub tag: __le64,
    pub rsvd1: [__le32; 6],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum hpre_cap_table_type {
    QM_RAS_NFE_TYPE = 0x0,
    QM_RAS_NFE_RESET,
    QM_RAS_CE_TYPE,
    HPRE_RAS_NFE_TYPE,
    HPRE_RAS_NFE_RESET,
    HPRE_RAS_CE_TYPE,
    HPRE_CORE_INFO,
    HPRE_CORE_EN,
    HPRE_DRV_ALG_BITMAP,
    HPRE_ALG_BITMAP,
    HPRE_CORE1_BITMAP_CAP,
    HPRE_CORE2_BITMAP_CAP,
    HPRE_CORE3_BITMAP_CAP,
    HPRE_CORE4_BITMAP_CAP,
    HPRE_CORE5_BITMAP_CAP,
    HPRE_CORE6_BITMAP_CAP,
    HPRE_CORE7_BITMAP_CAP,
    HPRE_CORE8_BITMAP_CAP,
    HPRE_CORE9_BITMAP_CAP,
    HPRE_CORE10_BITMAP_CAP,
}

extern "C" {
    pub fn hpre_create_qp(type_: u8) -> *mut hisi_qp;
    pub fn hpre_algs_register(qm: *mut hisi_qm) -> i32;
    pub fn hpre_algs_unregister(qm: *mut hisi_qm);
    pub fn hpre_check_alg_support(qm: *mut hisi_qm, alg: u32) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
