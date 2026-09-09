/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2014 - 2020 Intel Corporation */
// C dependency: icp_qat_fw_loader_handle.h

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hal_global_csr {
    MISC_CONTROL = 0xA04,
    ICP_RESET = 0xA0c,
    ICP_GLOBAL_CLK_ENABLE = 0xA50,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hal_global_csr_aux {
    MISC_CONTROL_C4XXX = 0xAA0,
    ICP_RESET_CPP0 = 0x938,
    ICP_RESET_CPP1 = 0x93c,
    ICP_GLOBAL_CLK_ENABLE_CPP0 = 0x964,
    ICP_GLOBAL_CLK_ENABLE_CPP1 = 0x968,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum hal_ae_csr {
    USTORE_ADDRESS = 0x000,
    USTORE_DATA_LOWER = 0x004,
    USTORE_DATA_UPPER = 0x008,
    ALU_OUT = 0x010,
    CTX_ARB_CNTL = 0x014,
    CTX_ENABLES = 0x018,
    CC_ENABLE = 0x01c,
    CSR_CTX_POINTER = 0x020,
    CTX_STS_INDIRECT = 0x040,
    ACTIVE_CTX_STATUS = 0x044,
    CTX_SIG_EVENTS_INDIRECT = 0x048,
    CTX_SIG_EVENTS_ACTIVE = 0x04c,
    CTX_WAKEUP_EVENTS_INDIRECT = 0x050,
    LM_ADDR_0_INDIRECT = 0x060,
    LM_ADDR_1_INDIRECT = 0x068,
    LM_ADDR_2_INDIRECT = 0x0cc,
    LM_ADDR_3_INDIRECT = 0x0d4,
    INDIRECT_LM_ADDR_0_BYTE_INDEX = 0x0e0,
    INDIRECT_LM_ADDR_1_BYTE_INDEX = 0x0e8,
    INDIRECT_LM_ADDR_2_BYTE_INDEX = 0x10c,
    INDIRECT_LM_ADDR_3_BYTE_INDEX = 0x114,
    INDIRECT_T_INDEX = 0x0f8,
    INDIRECT_T_INDEX_BYTE_INDEX = 0x0fc,
    FUTURE_COUNT_SIGNAL_INDIRECT = 0x078,
    TIMESTAMP_LOW = 0x0c0,
    TIMESTAMP_HIGH = 0x0c4,
    PROFILE_COUNT = 0x144,
    SIGNATURE_ENABLE = 0x150,
    AE_MISC_CONTROL = 0x160,
    LOCAL_CSR_STATUS = 0x180,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum fcu_csr {
    FCU_CONTROL = 0x8c0,
    FCU_STATUS = 0x8c4,
    FCU_STATUS1 = 0x8c8,
    FCU_DRAM_ADDR_LO = 0x8cc,
    FCU_DRAM_ADDR_HI = 0x8d0,
    FCU_RAMBASE_ADDR_HI = 0x8d4,
    FCU_RAMBASE_ADDR_LO = 0x8d8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum fcu_csr_4xxx {
    FCU_CONTROL_4XXX = 0x1000,
    FCU_STATUS_4XXX = 0x1004,
    FCU_ME_BROADCAST_MASK_TYPE = 0x1008,
    FCU_AE_LOADED_4XXX = 0x1010,
    FCU_DRAM_ADDR_LO_4XXX = 0x1014,
    FCU_DRAM_ADDR_HI_4XXX = 0x1018,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum fcu_cmd {
    FCU_CTRL_CMD_NOOP = 0,
    FCU_CTRL_CMD_AUTH = 1,
    FCU_CTRL_CMD_LOAD = 2,
    FCU_CTRL_CMD_START = 3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum fcu_sts {
    FCU_STS_NO_STS = 0,
    FCU_STS_VERI_DONE = 1,
    FCU_STS_LOAD_DONE = 2,
    FCU_STS_VERI_FAIL = 3,
    FCU_STS_LOAD_FAIL = 4,
    FCU_STS_BUSY = 5,
}

pub const ALL_AE_MASK: u32 = 0xFFFFFFFF;
pub const UA_ECS: u32 = 0x1 << 31;
pub const ACS_ABO_BITPOS: u32 = 31;
pub const ACS_ACNO: u32 = 0x7;
pub const CE_ENABLE_BITPOS: u32 = 0x8;
pub const CE_LMADDR_0_GLOBAL_BITPOS: u32 = 16;
pub const CE_LMADDR_1_GLOBAL_BITPOS: u32 = 17;
pub const CE_LMADDR_2_GLOBAL_BITPOS: u32 = 22;
pub const CE_LMADDR_3_GLOBAL_BITPOS: u32 = 23;
pub const CE_T_INDEX_GLOBAL_BITPOS: u32 = 21;
pub const CE_NN_MODE_BITPOS: u32 = 20;
pub const CE_REG_PAR_ERR_BITPOS: u32 = 25;
pub const CE_BREAKPOINT_BITPOS: u32 = 27;
pub const CE_CNTL_STORE_PARITY_ERROR_BITPOS: u32 = 29;
pub const CE_INUSE_CONTEXTS_BITPOS: u32 = 31;
pub const CE_NN_MODE: u32 = 0x1 << CE_NN_MODE_BITPOS;
pub const CE_INUSE_CONTEXTS: u32 = 0x1 << CE_INUSE_CONTEXTS_BITPOS;
pub const XCWE_VOLUNTARY: u32 = 0x1;
pub const LCS_STATUS: u32 = 0x1;
pub const MMC_SHARE_CS_BITPOS: u32 = 2;
pub const WAKEUP_EVENT: u32 = 0x10000;
pub const FCU_CTRL_BROADCAST_POS: u32 = 0x4;
pub const FCU_CTRL_AE_POS: u32 = 0x8;
pub const FCU_AUTH_STS_MASK: u32 = 0x7;
pub const FCU_STS_DONE_POS: u32 = 0x9;
pub const FCU_STS_AUTHFWLD_POS: u32 = 0x8;
pub const FCU_LOADED_AE_POS: u32 = 0x16;
pub const FW_AUTH_WAIT_PERIOD: u32 = 10;
pub const FW_AUTH_MAX_RETRY: u32 = 300;
pub const ICP_QAT_AE_OFFSET: u32 = 0x20000;
pub const ICP_QAT_CAP_OFFSET: u32 = ICP_QAT_AE_OFFSET + 0x10000;
pub const LOCAL_TO_XFER_REG_OFFSET: u32 = 0x800;
pub const ICP_QAT_EP_OFFSET: u32 = 0x3a000;
pub const ICP_QAT_EP_OFFSET_4XXX: u32 = 0x200000;
pub const ICP_QAT_AE_OFFSET_4XXX: u32 = 0x600000;
pub const ICP_QAT_CAP_OFFSET_4XXX: u32 = 0x640000;

#[macro_export]
macro_rules! SET_CAP_CSR {
    ($handle:expr, $csr:expr, $val:expr) => {
        ADF_CSR_WR(($handle).hal_cap_g_ctl_csr_addr_v, $csr, $val)
    };
}
#[macro_export]
macro_rules! GET_CAP_CSR {
    ($handle:expr, $csr:expr) => {
        ADF_CSR_RD(($handle).hal_cap_g_ctl_csr_addr_v, $csr)
    };
}
#[macro_export]
macro_rules! AE_CSR {
    ($handle:expr, $ae:expr) => {
        (($handle).hal_cap_ae_local_csr_addr_v as *mut u8).wrapping_add(($ae) << 12)
    };
}
#[macro_export]
macro_rules! AE_CSR_ADDR {
    ($handle:expr, $ae:expr, $csr:expr) => {
        AE_CSR!($handle, $ae).wrapping_add(0x3ff & ($csr))
    };
}
#[macro_export]
macro_rules! SET_AE_CSR {
    ($handle:expr, $ae:expr, $csr:expr, $val:expr) => {
        ADF_CSR_WR(AE_CSR_ADDR!($handle, $ae, $csr), 0, $val)
    };
}
#[macro_export]
macro_rules! GET_AE_CSR {
    ($handle:expr, $ae:expr, $csr:expr) => {
        ADF_CSR_RD(AE_CSR_ADDR!($handle, $ae, $csr), 0)
    };
}
#[macro_export]
macro_rules! AE_XFER {
    ($handle:expr, $ae:expr) => {
        (($handle).hal_cap_ae_xfer_csr_addr_v as *mut u8).wrapping_add(($ae) << 12)
    };
}
#[macro_export]
macro_rules! AE_XFER_ADDR {
    ($handle:expr, $ae:expr, $reg:expr) => {
        AE_XFER!($handle, $ae).wrapping_add((($reg) & 0xff) << 2)
    };
}
#[macro_export]
macro_rules! SET_AE_XFER {
    ($handle:expr, $ae:expr, $reg:expr, $val:expr) => {
        ADF_CSR_WR(AE_XFER_ADDR!($handle, $ae, $reg), 0, $val)
    };
}
#[macro_export]
macro_rules! SRAM_WRITE {
    ($handle:expr, $addr:expr, $val:expr) => {
        ADF_CSR_WR(($handle).hal_sram_addr_v, $addr, $val)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
