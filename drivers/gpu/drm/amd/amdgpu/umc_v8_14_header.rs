/*
 * Copyright 2024 Advanced Micro Devices, Inc.
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
 *
 */

// Dependencies supplied by the surrounding translation unit:
// #include "soc15_common.h"
// #include "amdgpu.h"

/* number of umc channel instance with memory map register access */
pub const UMC_V8_14_CHANNEL_INSTANCE_NUM: u32 = 2;
/* number of umc instance with memory map register access */
#[macro_export]
macro_rules! UMC_V8_14_UMC_INSTANCE_NUM {
    ($adev:expr) => {
        (*$adev).umc.node_inst_num
    };
}

/* Total channel instances for all available umc nodes */
#[macro_export]
macro_rules! UMC_V8_14_TOTAL_CHANNEL_NUM {
    ($adev:expr) => {
        UMC_V8_14_CHANNEL_INSTANCE_NUM * (*$adev).gmc.num_umc
    };
}

/* UMC register per channel offset */
pub const UMC_V8_14_PER_CHANNEL_OFFSET: u32 = 0x400;

pub const UMC_V8_14_INST_DIST: u32 = 0x40000;

/* EccErrCnt max value */
pub const UMC_V8_14_CE_CNT_MAX: u32 = 0xffff;
/* umc ce interrupt threshold */
pub const UMC_V8_14_CE_INT_THRESHOLD: u32 = 0xffff;
/* umc ce count initial value */
pub const UMC_V8_14_CE_CNT_INIT: u32 =
    UMC_V8_14_CE_CNT_MAX - UMC_V8_14_CE_INT_THRESHOLD;

extern "C" {
    pub static mut umc_v8_14_ras: amdgpu_umc_ras;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
