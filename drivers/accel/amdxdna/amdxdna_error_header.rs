/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2025, Advanced Micro Devices, Inc.
 */

// Translated from amdxdna_error.h.

pub const AMDXDNA_ERR_DRV_AIE: u64 = 4;
pub const AMDXDNA_ERR_SEV_CRITICAL: u64 = 3;
pub const AMDXDNA_ERR_CLASS_AIE: u64 = 2;

pub const AMDXDNA_ERR_NUM_MASK: u64 = 0x0000_0000_0000_ffff;
pub const AMDXDNA_ERR_DRV_MASK: u64 = 0x0000_0000_00ff_0000;
pub const AMDXDNA_ERR_SEV_MASK: u64 = 0x0000_0000_ff00_0000;
pub const AMDXDNA_ERR_MOD_MASK: u64 = 0x0000_00ff_0000_0000;
pub const AMDXDNA_ERR_CLASS_MASK: u64 = 0x0000_ff00_0000_0000;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum amdxdna_error_num {
    AMDXDNA_ERROR_NUM_AIE_SATURATION = 3,
    AMDXDNA_ERROR_NUM_AIE_FP,
    AMDXDNA_ERROR_NUM_AIE_STREAM,
    AMDXDNA_ERROR_NUM_AIE_ACCESS,
    AMDXDNA_ERROR_NUM_AIE_BUS,
    AMDXDNA_ERROR_NUM_AIE_INSTRUCTION,
    AMDXDNA_ERROR_NUM_AIE_ECC,
    AMDXDNA_ERROR_NUM_AIE_LOCK,
    AMDXDNA_ERROR_NUM_AIE_DMA,
    AMDXDNA_ERROR_NUM_AIE_MEM_PARITY,
    AMDXDNA_ERROR_NUM_UNKNOWN = 15,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum amdxdna_error_module {
    AMDXDNA_ERROR_MODULE_AIE_CORE = 3,
    AMDXDNA_ERROR_MODULE_AIE_MEMORY,
    AMDXDNA_ERROR_MODULE_AIE_SHIM,
    AMDXDNA_ERROR_MODULE_AIE_NOC,
    AMDXDNA_ERROR_MODULE_AIE_PL,
    AMDXDNA_ERROR_MODULE_UNKNOWN = 8,
}

#[inline]
const fn field_prep(mask: u64, value: u64) -> u64 {
    (value << mask.trailing_zeros()) & mask
}

#[macro_export]
macro_rules! AMDXDNA_ERROR_ENCODE {
    ($err_num:expr, $err_mod:expr) => {
        $crate::field_prep($crate::AMDXDNA_ERR_NUM_MASK, $err_num as u64)
            | $crate::field_prep($crate::AMDXDNA_ERR_DRV_MASK, $crate::AMDXDNA_ERR_DRV_AIE)
            | $crate::field_prep($crate::AMDXDNA_ERR_SEV_MASK, $crate::AMDXDNA_ERR_SEV_CRITICAL)
            | $crate::field_prep($crate::AMDXDNA_ERR_MOD_MASK, $err_mod as u64)
            | $crate::field_prep($crate::AMDXDNA_ERR_CLASS_MASK, $crate::AMDXDNA_ERR_CLASS_AIE)
    };
}

pub const AMDXDNA_EXTRA_ERR_COL_MASK: u64 = 0x0000_0000_0000_00ff;
pub const AMDXDNA_EXTRA_ERR_ROW_MASK: u64 = 0x0000_0000_0000_ff00;

#[macro_export]
macro_rules! AMDXDNA_EXTRA_ERR_ENCODE {
    ($row:expr, $col:expr) => {
        $crate::field_prep($crate::AMDXDNA_EXTRA_ERR_COL_MASK, $col as u64)
            | $crate::field_prep($crate::AMDXDNA_EXTRA_ERR_ROW_MASK, $row as u64)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
