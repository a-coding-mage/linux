/*
 * Copyright 2022 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding translation unit:
// `amdgpu_umc_ras` and the `hweight32` operation.

pub const UMC_V8_10_CHANNEL_INSTANCE_NUM: u32 = 2;
pub const UMC_V8_10_UMC_INSTANCE_NUM: u32 = 2;

#[macro_export]
macro_rules! UMC_V8_10_TOTAL_CHANNEL_NUM {
    ($adev:expr) => {
        (UMC_V8_10_CHANNEL_INSTANCE_NUM * UMC_V8_10_UMC_INSTANCE_NUM
            * ($adev).gmc.num_umc
            - hweight32(($adev).gmc.m_half_use) * 2)
    };
}

pub const UMC_V8_10_PER_CHANNEL_OFFSET: u32 = 0x400;
pub const UMC_V8_10_CE_CNT_MAX: u32 = 0xffff;
pub const UUMC_V8_10_CE_INT_THRESHOLD: u32 = 0xffff;
pub const UMC_V8_10_CE_CNT_INIT: u32 =
    UMC_V8_10_CE_CNT_MAX - UUMC_V8_10_CE_INT_THRESHOLD;

pub const UMC_V8_10_NA_COL_2BITS_POWER_OF_2_NUM: u32 = 4;
pub const UMC_V8_10_NA_C5_BIT: u32 = 14;

#[macro_export]
macro_rules! SWIZZLE_MODE_TMP_ADDR {
    ($na:expr, $ch_num:expr, $ch_idx:expr) => {
        ((($na >> 10) * $ch_num + $ch_idx) << 10)
    };
}

#[macro_export]
macro_rules! SWIZZLE_MODE_ADDR_HI {
    ($addr:expr, $col_bit:expr) => {
        (($addr >> ($col_bit + 2)) << ($col_bit + 2))
    };
}

#[macro_export]
macro_rules! SWIZZLE_MODE_ADDR_MID {
    ($na:expr, $col_bit:expr) => {
        (((($na >> 8) & 0x3) << $col_bit))
    };
}

#[macro_export]
macro_rules! SWIZZLE_MODE_ADDR_LOW {
    ($addr:expr, $col_bit:expr) => {
        (((($addr >> 10) & ((0x1u64 << ($col_bit - 8)) - 1)) << 8))
    };
}

#[macro_export]
macro_rules! SWIZZLE_MODE_ADDR_LSB {
    ($na:expr) => {
        ($na & 0xFF)
    };
}

extern "C" {
    pub static mut umc_v8_10_ras: crate::amdgpu_umc_ras;
    pub static umc_v8_10_channel_idx_tbl:
        [[u32; UMC_V8_10_CHANNEL_INSTANCE_NUM as usize]; UMC_V8_10_UMC_INSTANCE_NUM as usize];
    pub static umc_v8_10_channel_idx_tbl_ext0:
        [[u32; UMC_V8_10_CHANNEL_INSTANCE_NUM as usize]; UMC_V8_10_UMC_INSTANCE_NUM as usize];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
