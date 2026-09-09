/*
 * Copyright 2023 Advanced Micro Devices, Inc.
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

/* one piece of normalized address is mapped to 8 pieces of physical address */
pub const UMC_V12_0_NA_MAP_PA_NUM: u32 = 8;
/* R13 bit shift should be considered, double the number */
pub const UMC_V12_0_BAD_PAGE_NUM_PER_CHANNEL: u32 = UMC_V12_0_NA_MAP_PA_NUM * 2;

/* column bits in SOC physical address */
pub const UMC_V12_0_PA_C2_BIT: u32 = 15;
pub const UMC_V12_0_PA_C3_BIT: u32 = 16;
pub const UMC_V12_0_PA_C4_BIT: u32 = 21;
/* row bits in SOC physical address */
pub const UMC_V12_0_PA_R0_BIT: u32 = 22;
pub const UMC_V12_0_PA_R10_BIT: u32 = 32;
pub const UMC_V12_0_PA_R11_BIT: u32 = 33;
pub const UMC_V12_0_PA_R12_BIT: u32 = 34;
pub const UMC_V12_0_PA_R13_BIT: u32 = 35;
/* channel bit in SOC physical address */
pub const UMC_V12_0_PA_CH4_BIT: u32 = 12;
pub const UMC_V12_0_PA_CH5_BIT: u32 = 13;
/* bank bit in SOC physical address */
pub const UMC_V12_0_PA_B0_BIT: u32 = 19;
pub const UMC_V12_0_PA_B1_BIT: u32 = 20;
/* row bits in MCA address */
pub const UMC_V12_0_MA_R0_BIT: u32 = 10;

#[macro_export]
macro_rules! MCA_IPID_LO_2_UMC_CH {
    ($ipid_lo:expr) => {
        (((($ipid_lo >> 20) & 0x1) * 4) + (($ipid_lo >> 12) & 0xF))
    };
}

#[macro_export]
macro_rules! MCA_IPID_LO_2_UMC_INST {
    ($ipid_lo:expr) => {
        (($ipid_lo >> 21) & 0x7)
    };
}

#[macro_export]
macro_rules! MCA_IPID_2_DIE_ID {
    ($ipid:expr) => {
        ((REG_GET_FIELD!($ipid, MCMP1_IPIDT0, InstanceIdHi) >> 2) & 0x03)
    };
}

#[macro_export]
macro_rules! MCA_IPID_2_UMC_CH {
    ($ipid:expr) => {
        MCA_IPID_LO_2_UMC_CH!(REG_GET_FIELD!($ipid, MCMP1_IPIDT0, InstanceIdLo))
    };
}

#[macro_export]
macro_rules! MCA_IPID_2_UMC_INST {
    ($ipid:expr) => {
        MCA_IPID_LO_2_UMC_INST!(REG_GET_FIELD!($ipid, MCMP1_IPIDT0, InstanceIdLo))
    };
}

#[macro_export]
macro_rules! MCA_IPID_2_SOCKET_ID {
    ($ipid:expr) => {
        (((REG_GET_FIELD!($ipid, MCMP1_IPIDT0, InstanceIdLo) & 0x1) << 2)
            | (REG_GET_FIELD!($ipid, MCMP1_IPIDT0, InstanceIdHi) & 0x03))
    };
}

extern "C" {
    pub fn umc_v12_0_is_uncorrectable_error(
        adev: *mut amdgpu_device,
        mc_umc_status: u64,
    ) -> bool;
    pub fn umc_v12_0_is_correctable_error(
        adev: *mut amdgpu_device,
        mc_umc_status: u64,
    ) -> bool;

    pub static mut umc_v12_0_ras: amdgpu_umc_ras;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
