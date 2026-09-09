/*
 * Copyright (C) 2018  Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
 * AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

pub const mmFabricConfigAccessControl: u32 = 0x0410;
pub const mmFabricConfigAccessControl_BASE_IDX: u32 = 0;

pub const mmDF_PIE_AON0_DfGlobalClkGater: u32 = 0x00fc;
pub const mmDF_PIE_AON0_DfGlobalClkGater_BASE_IDX: u32 = 0;

pub const mmDF_CS_UMC_AON0_DfGlobalCtrl: u32 = 0x00fe;
pub const mmDF_CS_UMC_AON0_DfGlobalCtrl_BASE_IDX: u32 = 0;

pub const mmDF_CS_UMC_AON0_DramBaseAddress0: u32 = 0x0044;
pub const mmDF_CS_UMC_AON0_DramBaseAddress0_BASE_IDX: u32 = 0;

pub const mmDF_GCM_AON0_DramMegaBaseAddress0: u32 = 0x0064;
pub const mmDF_GCM_AON0_DramMegaBaseAddress0_BASE_IDX: u32 = 0;

pub const smnPerfMonCtlLo0: u64 = 0x01d440;
pub const smnPerfMonCtlHi0: u64 = 0x01d444;
pub const smnPerfMonCtlLo1: u64 = 0x01d450;
pub const smnPerfMonCtlHi1: u64 = 0x01d454;
pub const smnPerfMonCtlLo2: u64 = 0x01d460;
pub const smnPerfMonCtlHi2: u64 = 0x01d464;
pub const smnPerfMonCtlLo3: u64 = 0x01d470;
pub const smnPerfMonCtlHi3: u64 = 0x01d474;
pub const smnPerfMonCtlLo4: u64 = 0x01d880;
pub const smnPerfMonCtlHi4: u64 = 0x01d884;
pub const smnPerfMonCtlLo5: u64 = 0x01d888;
pub const smnPerfMonCtlHi5: u64 = 0x01d88c;
pub const smnPerfMonCtlLo6: u64 = 0x01d890;
pub const smnPerfMonCtlHi6: u64 = 0x01d894;
pub const smnPerfMonCtlLo7: u64 = 0x01d898;
pub const smnPerfMonCtlHi7: u64 = 0x01d89c;

pub const smnPerfMonCtrLo0: u64 = 0x01d448;
pub const smnPerfMonCtrHi0: u64 = 0x01d44c;
pub const smnPerfMonCtrLo1: u64 = 0x01d458;
pub const smnPerfMonCtrHi1: u64 = 0x01d45c;
pub const smnPerfMonCtrLo2: u64 = 0x01d468;
pub const smnPerfMonCtrHi2: u64 = 0x01d46c;
pub const smnPerfMonCtrLo3: u64 = 0x01d478;
pub const smnPerfMonCtrHi3: u64 = 0x01d47c;
pub const smnPerfMonCtrLo4: u64 = 0x01d790;
pub const smnPerfMonCtrHi4: u64 = 0x01d794;
pub const smnPerfMonCtrLo5: u64 = 0x01d798;
pub const smnPerfMonCtrHi5: u64 = 0x01d79c;
pub const smnPerfMonCtrLo6: u64 = 0x01d7a0;
pub const smnPerfMonCtrHi6: u64 = 0x01d7a4;
pub const smnPerfMonCtrLo7: u64 = 0x01d7a8;
pub const smnPerfMonCtrHi7: u64 = 0x01d7ac;

pub const smnDF_PIE_AON_FabricIndirectConfigAccessAddress3: u64 = 0x1d05c;
pub const smnDF_PIE_AON_FabricIndirectConfigAccessDataLo3: u64 = 0x1d098;
pub const smnDF_PIE_AON_FabricIndirectConfigAccessDataHi3: u64 = 0x1d09c;

pub const smnDF_CS_UMC_AON0_DramBaseAddress0: u64 = 0x1c110;
pub const smnDF_CS_UMC_AON0_DramLimitAddress0: u64 = 0x1c114;

pub const mmDF_CS_UMC_AON0_HardwareAssertMaskLow: u32 = 0x067e;
pub const mmDF_CS_UMC_AON0_HardwareAssertMaskLow_BASE_IDX: u32 = 0;
pub const mmDF_NCS_PG0_HardwareAssertMaskHigh: u32 = 0x067f;
pub const mmDF_NCS_PG0_HardwareAssertMaskHigh_BASE_IDX: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
