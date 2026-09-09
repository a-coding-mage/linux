/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Following copyright information was take from the original file
 * <drivers/char/tpm/tpm_tis_core.h> where the definitions were moved
 * from:
 *
 * Copyright (C) 2005, 2006 IBM Corporation
 * Copyright (C) 2014, 2015 Intel Corporation
 *
 * Authors:
 * Leendert van Doorn <leendert@watson.ibm.com>
 * Kylene Hall <kjhall@us.ibm.com>
 *
 * Maintained by: <tpmdd-devel@lists.sourceforge.net>
 *
 * Device driver for TCG/TCPA TPM (trusted platform module).
 * Specifications at www.trustedcomputinggroup.org
 *
 * This device driver implements the TPM interface as defined in
 * the TCG TPM Interface Spec version 1.2, revision 1.0.
 */

/*
 * TCG PC Client Platform TPM Profile (PTP) Specification
 * https://trustedcomputinggroup.org/resource/pc-client-platform-tpm-profile-ptp-specification/
 */

/* TIS/FIFO macros and definitions */

#[repr(C)]
pub enum tis_access {
    TPM_ACCESS_VALID = 0x80,
    TPM_ACCESS_ACTIVE_LOCALITY = 0x20, /* (R) */
    TPM_ACCESS_RELINQUISH_LOCALITY = 0x20, /* (W) */
    TPM_ACCESS_REQUEST_PENDING = 0x04, /* (W) */
    TPM_ACCESS_REQUEST_USE = 0x02, /* (W) */
}

#[repr(C)]
pub enum tis_status {
    TPM_STS_VALID = 0x80, /* (R) */
    TPM_STS_COMMAND_READY = 0x40, /* (R) */
    TPM_STS_DATA_AVAIL = 0x10, /* (R) */
    TPM_STS_DATA_EXPECT = 0x08, /* (R) */
    TPM_STS_GO = 0x20, /* (W) */
    TPM_STS_RESPONSE_RETRY = 0x02, /* (R) */
    TPM_STS_READ_ZERO = 0x23, /* bits that must be zero on read */
}

#[repr(C)]
pub enum tis_int_flags {
    TPM_GLOBAL_INT_ENABLE = 0x80000000,
    TPM_INTF_BURST_COUNT_STATIC = 0x100,
    TPM_INTF_CMD_READY_INT = 0x080,
    TPM_INTF_INT_EDGE_FALLING = 0x040,
    TPM_INTF_INT_EDGE_RISING = 0x020,
    TPM_INTF_INT_LEVEL_LOW = 0x010,
    TPM_INTF_INT_LEVEL_HIGH = 0x008,
    TPM_INTF_LOCALITY_CHANGE_INT = 0x004,
    TPM_INTF_STS_VALID_INT = 0x002,
    TPM_INTF_DATA_AVAIL_INT = 0x001,
}

#[repr(C)]
pub enum tis_defaults {
    TIS_MEM_LEN = 0x5000,
    TIS_SHORT_TIMEOUT = 750, /* ms */
    TIS_LONG_TIMEOUT = 4000, /* 4 secs */
    TIS_TIMEOUT_MIN_ATML = 14700, /* usecs */
    TIS_TIMEOUT_MAX_ATML = 15000, /* usecs */
}

pub const TIS_MEM_X86_LPC_BASE: u32 = 0xFED40000;
pub const INTEL_LEGACY_BLK_BASE_ADDR: u32 = 0xFED08000;

#[repr(C)]
pub enum tis_x86_defaults {
    TIS_MEM_X86_LEN = 0x5000,
    ILB_REMAP_SIZE = 0x100,
    LPC_CNTRL_OFFSET = 0x84,
    LPC_CLKRUN_EN = 1 << 2,
}

/*
 * Some timeout values are needed before it is known whether the chip is
 * TPM 1.0 or TPM 2.0.
 */
#[macro_export]
macro_rules! TIS_TIMEOUT_A_MAX { () => { ::core::cmp::max(TIS_SHORT_TIMEOUT as i32, TPM2_TIMEOUT_A) }; }
#[macro_export]
macro_rules! TIS_TIMEOUT_B_MAX { () => { ::core::cmp::max(TIS_LONG_TIMEOUT as i32, TPM2_TIMEOUT_B) }; }
#[macro_export]
macro_rules! TIS_TIMEOUT_C_MAX { () => { ::core::cmp::max(TIS_SHORT_TIMEOUT as i32, TPM2_TIMEOUT_C) }; }
#[macro_export]
macro_rules! TIS_TIMEOUT_D_MAX { () => { ::core::cmp::max(TIS_SHORT_TIMEOUT as i32, TPM2_TIMEOUT_D) }; }

#[macro_export]
macro_rules! TPM_ACCESS { ($l:expr) => { 0x0000 | (($l) << 12) }; }
#[macro_export]
macro_rules! TPM_INT_ENABLE { ($l:expr) => { 0x0008 | (($l) << 12) }; }
#[macro_export]
macro_rules! TPM_INT_VECTOR { ($l:expr) => { 0x000C | (($l) << 12) }; }
#[macro_export]
macro_rules! TPM_INT_STATUS { ($l:expr) => { 0x0010 | (($l) << 12) }; }
#[macro_export]
macro_rules! TPM_INTF_CAPS { ($l:expr) => { 0x0014 | (($l) << 12) }; }
#[macro_export]
macro_rules! TPM_STS { ($l:expr) => { 0x0018 | (($l) << 12) }; }
#[macro_export]
macro_rules! TPM_STS3 { ($l:expr) => { 0x001b | (($l) << 12) }; }
#[macro_export]
macro_rules! TPM_DATA_FIFO { ($l:expr) => { 0x0024 | (($l) << 12) }; }
#[macro_export]
macro_rules! TPM_INTF_ID { ($l:expr) => { 0x0030 | (($l) << 12) }; }
#[macro_export]
macro_rules! TPM_DID_VID { ($l:expr) => { 0x0F00 | (($l) << 12) }; }
#[macro_export]
macro_rules! TPM_RID { ($l:expr) => { 0x0F04 | (($l) << 12) }; }

/* TPM HW Interface and Capabilities */
pub const TPM_TIS_INTF_ACTIVE: u32 = 0x00;
pub const TPM_CRB_INTF_ACTIVE: u32 = 0x01;

pub const TPM_INTID_INTERFACE_TYPE: u32 = GENMASK(3, 0);
pub const TPM_INTID_INTERFACE_VERSION: u32 = GENMASK(7, 4);
pub const TPM_INTID_CAP_LOCALITY: u32 = BIT(8);
pub const TPM_INTID_CAP_TIS: u32 = BIT(13);
pub const TPM_INTID_CAP_CRB: u32 = BIT(14);
pub const TPM_INTID_CAP_IF_RES: u32 = GENMASK(16, 15);
pub const TPM_INTID_INTERFACE_SELECTOR: u32 = GENMASK(18, 17);
pub const TPM_INTID_INTF_SEL_LOCK: u32 = BIT(19);

pub const TPM_TIS_INTF_12: u32 = 0x00;
pub const TPM_TIS_INTF_13: u32 = 0x02;
pub const TPM2_TIS_INTF_13: u32 = 0x03;

pub const TPM_INTF_DATA_AVAIL_INT_SUPPORT: u32 = BIT(0);
pub const TPM_INTF_STS_VALID_INT_SUPPORT: u32 = BIT(1);
pub const TPM_INTF_LOCALITY_CHANGE_INT_SUPPORT: u32 = BIT(2);
pub const TPM_INTF_INTERRUPT_LEVEL_HIGH: u32 = BIT(3);
pub const TPM_INTF_INTERRUPT_LEVEL_LOW: u32 = BIT(4);
pub const TPM_INTF_INTERRUPT_EDGE_RISING: u32 = BIT(5);
pub const TPM_INTF_INTERRUPT_EDGE_FALLING: u32 = BIT(6);
pub const TPM_INTF_COMMAND_READY_INT_SUPPORT: u32 = BIT(7);
pub const TPM_INTF_BURST_COUNT_STATIC: u32 = BIT(8);
pub const TPM_INTF_DATA_TRANSFER_SIZE_SUPPORT: u32 = GENMASK(10, 9);
pub const TPM_INTF_INTERFACE_VERSION: u32 = GENMASK(30, 28);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
