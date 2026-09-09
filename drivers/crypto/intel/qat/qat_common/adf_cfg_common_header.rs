/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2014 - 2020 Intel Corporation */

// Dependency: <linux/types.h>

pub const ADF_CFG_MAX_STR_LEN: usize = 64;
pub const ADF_CFG_MAX_KEY_LEN_IN_BYTES: usize = ADF_CFG_MAX_STR_LEN;
pub const ADF_CFG_MAX_VAL_LEN_IN_BYTES: usize = ADF_CFG_MAX_STR_LEN;
pub const ADF_CFG_MAX_SECTION_LEN_IN_BYTES: usize = ADF_CFG_MAX_STR_LEN;
pub const ADF_MAX_DEVICES: usize = 32 * 32;
// Equivalent to BITS_TO_LONGS(ADF_MAX_DEVICES), using the target's C long width.
pub const ADF_DEVS_ARRAY_SIZE: usize =
    (ADF_MAX_DEVICES + (usize::BITS as usize) - 1) / (usize::BITS as usize);

pub const ADF_CFG_SERV_RING_PAIR_0_SHIFT: u32 = 0;
pub const ADF_CFG_SERV_RING_PAIR_1_SHIFT: u32 = 3;
pub const ADF_CFG_SERV_RING_PAIR_2_SHIFT: u32 = 6;
pub const ADF_CFG_SERV_RING_PAIR_3_SHIFT: u32 = 9;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum adf_cfg_service_type {
    UNUSED = 0,
    CRYPTO,
    COMP,
    SYM,
    ASYM,
    DECOMP,
    USED,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum adf_cfg_val_type {
    ADF_DEC,
    ADF_STR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum adf_device_type {
    DEV_UNKNOWN = 0,
    DEV_DH895XCC,
    DEV_DH895XCCVF,
    DEV_C62X,
    DEV_C62XVF,
    DEV_C3XXX,
    DEV_C3XXXVF,
    DEV_4XXX,
    DEV_420XX,
    DEV_6XXX,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
