/*
 * Copyright 2019 Advanced Micro Devices, Inc.
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
// #include "soc15_common.h"
// #include "amdgpu.h"

/* HBM  Memory Channel Width */
pub const UMC_V6_1_HBM_MEMORY_CHANNEL_WIDTH: u32 = 128;
/* number of umc channel instance with memory map register access */
pub const UMC_V6_1_CHANNEL_INSTANCE_NUM: u32 = 4;
/* number of umc instance with memory map register access */
pub const UMC_V6_1_UMC_INSTANCE_NUM: u32 = 8;
/* total channel instances in one umc block */
pub const UMC_V6_1_TOTAL_CHANNEL_NUM: u32 =
    UMC_V6_1_CHANNEL_INSTANCE_NUM * UMC_V6_1_UMC_INSTANCE_NUM;
/* UMC regiser per channel offset */
pub const UMC_V6_1_PER_CHANNEL_OFFSET_VG20: u32 = 0x800;
pub const UMC_V6_1_PER_CHANNEL_OFFSET_ARCT: u32 = 0x400;

/* EccErrCnt max value */
pub const UMC_V6_1_CE_CNT_MAX: u32 = 0xffff;
/* umc ce interrupt threshold */
pub const UMC_V6_1_CE_INT_THRESHOLD: u32 = 0xffff;
/* umc ce count initial value */
pub const UMC_V6_1_CE_CNT_INIT: u32 =
    UMC_V6_1_CE_CNT_MAX - UMC_V6_1_CE_INT_THRESHOLD;

unsafe extern "C" {
    pub static mut umc_v6_1_ras: amdgpu_umc_ras;
    pub static umc_v6_1_channel_idx_tbl:
        [[u32; UMC_V6_1_CHANNEL_INSTANCE_NUM as usize]; UMC_V6_1_UMC_INSTANCE_NUM as usize];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
