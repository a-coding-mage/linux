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

/* FabricConfigAccessControl */
pub const FabricConfigAccessControl__CfgRegInstAccEn__SHIFT: u32 = 0x0;
pub const FabricConfigAccessControl__CfgRegInstAccRegLock__SHIFT: u32 = 0x1;
pub const FabricConfigAccessControl__CfgRegInstID__SHIFT: u32 = 0x10;
pub const FabricConfigAccessControl__CfgRegInstAccEn_MASK: u32 = 0x00000001;
pub const FabricConfigAccessControl__CfgRegInstAccRegLock_MASK: u32 = 0x00000002;
pub const FabricConfigAccessControl__CfgRegInstID_MASK: u32 = 0x00FF0000;

/* DF_PIE_AON0_DfGlobalClkGater */
pub const DF_PIE_AON0_DfGlobalClkGater__MGCGMode__SHIFT: u32 = 0x0;
pub const DF_PIE_AON0_DfGlobalClkGater__MGCGMode_MASK: u32 = 0x0000000F;

/* DF_CS_AON0_DramBaseAddress0 */
pub const DF_CS_AON0_DramBaseAddress0__AddrRngVal__SHIFT: u32 = 0x0;
pub const DF_CS_AON0_DramBaseAddress0__LgcyMmioHoleEn__SHIFT: u32 = 0x1;
pub const DF_CS_AON0_DramBaseAddress0__IntLvNumChan__SHIFT: u32 = 0x4;
pub const DF_CS_AON0_DramBaseAddress0__IntLvAddrSel__SHIFT: u32 = 0x8;
pub const DF_CS_AON0_DramBaseAddress0__DramBaseAddr__SHIFT: u32 = 0xc;
pub const DF_CS_AON0_DramBaseAddress0__AddrRngVal_MASK: u32 = 0x00000001;
pub const DF_CS_AON0_DramBaseAddress0__LgcyMmioHoleEn_MASK: u32 = 0x00000002;
pub const DF_CS_AON0_DramBaseAddress0__IntLvNumChan_MASK: u32 = 0x000000F0;
pub const DF_CS_AON0_DramBaseAddress0__IntLvAddrSel_MASK: u32 = 0x00000700;
pub const DF_CS_AON0_DramBaseAddress0__DramBaseAddr_MASK: u32 = 0xFFFFF000;

//DF_CS_AON0_CoherentSlaveModeCtrlA0
pub const DF_CS_AON0_CoherentSlaveModeCtrlA0__ForceParWrRMW__SHIFT: u32 = 0x3;
pub const DF_CS_AON0_CoherentSlaveModeCtrlA0__ForceParWrRMW_MASK: u32 = 0x00000008;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
