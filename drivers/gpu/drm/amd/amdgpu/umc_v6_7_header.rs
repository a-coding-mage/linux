/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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
// soc15_common.h, amdgpu.h

/* EccErrCnt max value */
pub const UMC_V6_7_CE_CNT_MAX: u32 = 0xffff;
/* umc ce interrupt threshold */
pub const UMC_V6_7_CE_INT_THRESHOLD: u32 = 0xffff;
/* umc ce count initial value */
pub const UMC_V6_7_CE_CNT_INIT: u32 =
    UMC_V6_7_CE_CNT_MAX - UMC_V6_7_CE_INT_THRESHOLD;

pub const UMC_V6_7_INST_DIST: u32 = 0x40000;

/* number of umc channel instance with memory map register access */
pub const UMC_V6_7_UMC_INSTANCE_NUM: usize = 4;
/* number of umc instance with memory map register access */
pub const UMC_V6_7_CHANNEL_INSTANCE_NUM: usize = 8;
/* total channel instances in one umc block */
pub const UMC_V6_7_TOTAL_CHANNEL_NUM: usize =
    UMC_V6_7_CHANNEL_INSTANCE_NUM * UMC_V6_7_UMC_INSTANCE_NUM;
/* one piece of normalizing address is mapped to 8 pieces of physical address */
pub const UMC_V6_7_NA_MAP_PA_NUM: usize = 8;
/* R14 bit shift should be considered, double the number */
pub const UMC_V6_7_BAD_PAGE_NUM_PER_CHANNEL: usize = UMC_V6_7_NA_MAP_PA_NUM * 2;
/* The CH4 bit in SOC physical address */
pub const UMC_V6_7_PA_CH4_BIT: u32 = 12;
/* The C2 bit in SOC physical address */
pub const UMC_V6_7_PA_C2_BIT: u32 = 17;
/* The R14 bit in SOC physical address */
pub const UMC_V6_7_PA_R14_BIT: u32 = 34;
/* UMC regiser per channel offset */
pub const UMC_V6_7_PER_CHANNEL_OFFSET: u32 = 0x400;

/* XOR bit 20, 25, 34 of PA into CH4 bit (bit 12 of PA),
 * hash bit is only effective when related setting is enabled
 */
#[macro_export]
macro_rules! CHANNEL_HASH {
    ($channel_idx:expr, $pa:expr, $adev:expr) => {
        (($channel_idx >> 4)
            ^ (($pa >> 20) & 0x1u64 & $adev.df.hash_status.hash_64k)
            ^ (($pa >> 25) & 0x1u64 & $adev.df.hash_status.hash_2m)
            ^ (($pa >> 34) & 0x1u64 & $adev.df.hash_status.hash_1g))
    };
}

#[macro_export]
macro_rules! SET_CHANNEL_HASH {
    ($channel_idx:expr, $pa:expr, $adev:expr) => {{
        $pa &= !(0x1u64 << $crate::UMC_V6_7_PA_CH4_BIT);
        $pa |= CHANNEL_HASH!($channel_idx, $pa, $adev) << $crate::UMC_V6_7_PA_CH4_BIT;
    }};
}

extern "C" {
    pub static mut umc_v6_7_ras: amdgpu_umc_ras;
    pub static umc_v6_7_channel_idx_tbl_second:
        [[u32; UMC_V6_7_CHANNEL_INSTANCE_NUM]; UMC_V6_7_UMC_INSTANCE_NUM];
    pub static umc_v6_7_channel_idx_tbl_first:
        [[u32; UMC_V6_7_CHANNEL_INSTANCE_NUM]; UMC_V6_7_UMC_INSTANCE_NUM];

    pub fn umc_v6_7_convert_error_address(
        adev: *mut amdgpu_device,
        err_data: *mut ras_err_data,
        err_addr: u64,
        ch_inst: u32,
        umc_inst: u32,
    );
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
