// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright(c) 2018 Intel Corporation. All rights reserved.
 * Intel specific definitions for NVDIMM Firmware Interface Table - NFIT
 */

pub const ND_INTEL_SMART: u32 = 1;

pub const ND_INTEL_SMART_SHUTDOWN_COUNT_VALID: u32 = 1 << 5;
pub const ND_INTEL_SMART_SHUTDOWN_VALID: u32 = 1 << 10;

#[repr(C, packed)]
pub struct NdIntelSmartVendor {
    pub flags: u32,
    pub reserved0: [u8; 4],
    pub health: u8,
    pub spares: u8,
    pub life_used: u8,
    pub alarm_flags: u8,
    pub media_temperature: u16,
    pub ctrl_temperature: u16,
    pub shutdown_count: u32,
    pub ait_status: u8,
    pub pmic_temperature: u16,
    pub reserved1: [u8; 8],
    pub shutdown_state: u8,
    pub vendor_size: u32,
    pub vendor_data: [u8; 92],
}

#[repr(C)]
pub union NdIntelSmartData {
    pub vendor: NdIntelSmartVendor,
    pub data: [u8; 128],
}

#[repr(C, packed)]
pub struct NdIntelSmart {
    pub status: u32,
    pub payload: NdIntelSmartData,
}

extern "C" {
    pub static intel_security_ops: *const NvdimmSecurityOps;
}

pub const ND_INTEL_STATUS_SIZE: usize = 4;
pub const ND_INTEL_PASSPHRASE_SIZE: usize = 32;

pub const ND_INTEL_STATUS_NOT_SUPPORTED: u32 = 1;
pub const ND_INTEL_STATUS_RETRY: u32 = 5;
pub const ND_INTEL_STATUS_NOT_READY: u32 = 9;
pub const ND_INTEL_STATUS_INVALID_STATE: u32 = 10;
pub const ND_INTEL_STATUS_INVALID_PASS: u32 = 11;
pub const ND_INTEL_STATUS_OVERWRITE_UNSUPPORTED: u32 = 0x10007;
pub const ND_INTEL_STATUS_OQUERY_INPROGRESS: u32 = 0x10007;
pub const ND_INTEL_STATUS_OQUERY_SEQUENCE_ERR: u32 = 0x20007;

pub const ND_INTEL_SEC_STATE_ENABLED: u8 = 0x02;
pub const ND_INTEL_SEC_STATE_LOCKED: u8 = 0x04;
pub const ND_INTEL_SEC_STATE_FROZEN: u8 = 0x08;
pub const ND_INTEL_SEC_STATE_PLIMIT: u8 = 0x10;
pub const ND_INTEL_SEC_STATE_UNSUPPORTED: u8 = 0x20;
pub const ND_INTEL_SEC_STATE_OVERWRITE: u8 = 0x40;

pub const ND_INTEL_SEC_ESTATE_ENABLED: u8 = 0x01;
pub const ND_INTEL_SEC_ESTATE_PLIMIT: u8 = 0x02;

#[repr(C, packed)]
pub struct NdIntelGetSecurityState {
    pub status: u32,
    pub extended_state: u8,
    pub reserved: [u8; 3],
    pub state: u8,
    pub reserved1: [u8; 3],
}

#[repr(C, packed)]
pub struct NdIntelSetPassphrase {
    pub old_pass: [u8; ND_INTEL_PASSPHRASE_SIZE],
    pub new_pass: [u8; ND_INTEL_PASSPHRASE_SIZE],
    pub status: u32,
}

#[repr(C, packed)]
pub struct NdIntelUnlockUnit {
    pub passphrase: [u8; ND_INTEL_PASSPHRASE_SIZE],
    pub status: u32,
}

#[repr(C, packed)]
pub struct NdIntelDisablePassphrase {
    pub passphrase: [u8; ND_INTEL_PASSPHRASE_SIZE],
    pub status: u32,
}

#[repr(C, packed)]
pub struct NdIntelFreezeLock {
    pub status: u32,
}

#[repr(C, packed)]
pub struct NdIntelSecureErase {
    pub passphrase: [u8; ND_INTEL_PASSPHRASE_SIZE],
    pub status: u32,
}

#[repr(C, packed)]
pub struct NdIntelOverwrite {
    pub passphrase: [u8; ND_INTEL_PASSPHRASE_SIZE],
    pub status: u32,
}

#[repr(C, packed)]
pub struct NdIntelQueryOverwrite {
    pub status: u32,
}

#[repr(C, packed)]
pub struct NdIntelSetMasterPassphrase {
    pub old_pass: [u8; ND_INTEL_PASSPHRASE_SIZE],
    pub new_pass: [u8; ND_INTEL_PASSPHRASE_SIZE],
    pub status: u32,
}

#[repr(C, packed)]
pub struct NdIntelMasterSecureErase {
    pub passphrase: [u8; ND_INTEL_PASSPHRASE_SIZE],
    pub status: u32,
}

pub const ND_INTEL_FWA_IDLE: u8 = 0;
pub const ND_INTEL_FWA_ARMED: u8 = 1;
pub const ND_INTEL_FWA_BUSY: u8 = 2;

pub const ND_INTEL_DIMM_FWA_NONE: u8 = 0;
pub const ND_INTEL_DIMM_FWA_NOTSTAGED: u8 = 1;
pub const ND_INTEL_DIMM_FWA_SUCCESS: u8 = 2;
pub const ND_INTEL_DIMM_FWA_NEEDRESET: u8 = 3;
pub const ND_INTEL_DIMM_FWA_MEDIAFAILED: u8 = 4;
pub const ND_INTEL_DIMM_FWA_ABORT: u8 = 5;
pub const ND_INTEL_DIMM_FWA_NOTSUPP: u8 = 6;
pub const ND_INTEL_DIMM_FWA_ERROR: u8 = 7;

#[repr(C, packed)]
pub struct NdIntelFwActivateDimminfo {
    pub status: u32,
    pub result: u16,
    pub state: u8,
    pub reserved: [u8; 7],
}

pub const ND_INTEL_DIMM_FWA_ARM: u8 = 1;
pub const ND_INTEL_DIMM_FWA_DISARM: u8 = 0;

#[repr(C, packed)]
pub struct NdIntelFwActivateArm {
    pub activate_arm: u8,
    pub status: u32,
}

/* Root device command payloads */
pub const ND_INTEL_BUS_FWA_CAP_FWQUIESCE: u8 = 1 << 0;
pub const ND_INTEL_BUS_FWA_CAP_OSQUIESCE: u8 = 1 << 1;
pub const ND_INTEL_BUS_FWA_CAP_RESET: u8 = 1 << 2;

#[repr(C, packed)]
pub struct NdIntelBusFwActivateBusinfo {
    pub status: u32,
    pub reserved: u16,
    pub state: u8,
    pub capability: u8,
    pub activate_tmo: u64,
    pub cpu_quiesce_tmo: u64,
    pub io_quiesce_tmo: u64,
    pub max_quiesce_tmo: u64,
}

pub const ND_INTEL_BUS_FWA_STATUS_NOARM: u32 = 6 | 1 << 16;
pub const ND_INTEL_BUS_FWA_STATUS_BUSY: u32 = 6 | 2 << 16;
pub const ND_INTEL_BUS_FWA_STATUS_NOFW: u32 = 6 | 3 << 16;
pub const ND_INTEL_BUS_FWA_STATUS_TMO: u32 = 6 | 4 << 16;
pub const ND_INTEL_BUS_FWA_STATUS_NOIDLE: u32 = 6 | 5 << 16;
pub const ND_INTEL_BUS_FWA_STATUS_ABORT: u32 = 6 | 6 << 16;

pub const ND_INTEL_BUS_FWA_IODEV_FORCE_IDLE: u8 = 0;
pub const ND_INTEL_BUS_FWA_IODEV_OS_IDLE: u8 = 1;

#[repr(C, packed)]
pub struct NdIntelBusFwActivate {
    pub iodev_state: u8,
    pub status: u32,
}

extern "C" {
    pub static intel_fw_ops: *const NvdimmFwOps;
    pub static intel_bus_fw_ops: *const NvdimmBusFwOps;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
