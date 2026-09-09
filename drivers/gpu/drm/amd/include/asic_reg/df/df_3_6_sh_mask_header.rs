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
// #define _df_3_6_SH_MASK_HEADER

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

/* DF_CS_UMC_AON0_DfGlobalCtrl */
pub const DF_CS_UMC_AON0_DfGlobalCtrl__GlbHashIntlvCtl64K__SHIFT: u32 = 0x14;
pub const DF_CS_UMC_AON0_DfGlobalCtrl__GlbHashIntlvCtl2M__SHIFT: u32 = 0x15;
pub const DF_CS_UMC_AON0_DfGlobalCtrl__GlbHashIntlvCtl1G__SHIFT: u32 = 0x16;
pub const DF_CS_UMC_AON0_DfGlobalCtrl__GlbHashIntlvCtl64K_MASK: u32 = 0x00100000;
pub const DF_CS_UMC_AON0_DfGlobalCtrl__GlbHashIntlvCtl2M_MASK: u32 = 0x00200000;
pub const DF_CS_UMC_AON0_DfGlobalCtrl__GlbHashIntlvCtl1G_MASK: u32 = 0x00400000;

/* DF_CS_AON0_DramBaseAddress0 */
pub const DF_CS_UMC_AON0_DramBaseAddress0__AddrRngVal__SHIFT: u32 = 0x0;
pub const DF_CS_UMC_AON0_DramBaseAddress0__LgcyMmioHoleEn__SHIFT: u32 = 0x1;
pub const DF_CS_UMC_AON0_DramBaseAddress0__IntLvNumChan__SHIFT: u32 = 0x2;
pub const DF_CS_UMC_AON0_DramBaseAddress0__IntLvAddrSel__SHIFT: u32 = 0x9;
pub const DF_CS_UMC_AON0_DramBaseAddress0__DramBaseAddr__SHIFT: u32 = 0xc;
pub const DF_CS_UMC_AON0_DramBaseAddress0__AddrRngVal_MASK: u32 = 0x00000001;
pub const DF_CS_UMC_AON0_DramBaseAddress0__LgcyMmioHoleEn_MASK: u32 = 0x00000002;
pub const DF_CS_UMC_AON0_DramBaseAddress0__IntLvNumChan_MASK: u32 = 0x0000003C;
pub const ALDEBARAN_DF_CS_UMC_AON0_DramBaseAddress0__IntLvNumChan_MASK: u32 = 0x0000007C;
pub const DF_CS_UMC_AON0_DramBaseAddress0__IntLvAddrSel_MASK: u32 = 0x00000E00;
pub const DF_CS_UMC_AON0_DramBaseAddress0__DramBaseAddr_MASK: u32 = 0xFFFFF000;

//DF_CS_UMC_AON0_DramLimitAddress0
pub const DF_CS_UMC_AON0_DramLimitAddress0__DstFabricID__SHIFT: u32 = 0x0;
pub const DF_CS_UMC_AON0_DramLimitAddress0__AllowReqIO__SHIFT: u32 = 0xa;
pub const DF_CS_UMC_AON0_DramLimitAddress0__DramLimitAddr__SHIFT: u32 = 0xc;
pub const DF_CS_UMC_AON0_DramLimitAddress0__DstFabricID_MASK: u32 = 0x000003FF;
pub const DF_CS_UMC_AON0_DramLimitAddress0__AllowReqIO_MASK: u32 = 0x00000400;
pub const DF_CS_UMC_AON0_DramLimitAddress0__DramLimitAddr_MASK: u32 = 0xFFFFF000;

//DF_CS_UMC_AON0_HardwareAssertMaskLow
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk0__SHIFT: u32 = 0x0;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk1__SHIFT: u32 = 0x1;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk2__SHIFT: u32 = 0x2;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk3__SHIFT: u32 = 0x3;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk4__SHIFT: u32 = 0x4;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk5__SHIFT: u32 = 0x5;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk6__SHIFT: u32 = 0x6;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk7__SHIFT: u32 = 0x7;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk8__SHIFT: u32 = 0x8;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk9__SHIFT: u32 = 0x9;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk10__SHIFT: u32 = 0xa;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk11__SHIFT: u32 = 0xb;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk12__SHIFT: u32 = 0xc;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk13__SHIFT: u32 = 0xd;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk14__SHIFT: u32 = 0xe;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk15__SHIFT: u32 = 0xf;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk16__SHIFT: u32 = 0x10;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk17__SHIFT: u32 = 0x11;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk18__SHIFT: u32 = 0x12;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk19__SHIFT: u32 = 0x13;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk20__SHIFT: u32 = 0x14;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk21__SHIFT: u32 = 0x15;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk22__SHIFT: u32 = 0x16;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk23__SHIFT: u32 = 0x17;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk24__SHIFT: u32 = 0x18;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk25__SHIFT: u32 = 0x19;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk26__SHIFT: u32 = 0x1a;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk27__SHIFT: u32 = 0x1b;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk28__SHIFT: u32 = 0x1c;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk29__SHIFT: u32 = 0x1d;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk30__SHIFT: u32 = 0x1e;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk31__SHIFT: u32 = 0x1f;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk0_MASK: u32 = 0x00000001;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk1_MASK: u32 = 0x00000002;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk2_MASK: u32 = 0x00000004;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk3_MASK: u32 = 0x00000008;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk4_MASK: u32 = 0x00000010;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk5_MASK: u32 = 0x00000020;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk6_MASK: u32 = 0x00000040;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk7_MASK: u32 = 0x00000080;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk8_MASK: u32 = 0x00000100;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk9_MASK: u32 = 0x00000200;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk10_MASK: u32 = 0x00000400;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk11_MASK: u32 = 0x00000800;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk12_MASK: u32 = 0x00001000;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk13_MASK: u32 = 0x00002000;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk14_MASK: u32 = 0x00004000;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk15_MASK: u32 = 0x00008000;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk16_MASK: u32 = 0x00010000;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk17_MASK: u32 = 0x00020000;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk18_MASK: u32 = 0x00040000;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk19_MASK: u32 = 0x00080000;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk20_MASK: u32 = 0x00100000;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk21_MASK: u32 = 0x00200000;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk22_MASK: u32 = 0x00400000;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk23_MASK: u32 = 0x00800000;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk24_MASK: u32 = 0x01000000;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk25_MASK: u32 = 0x02000000;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk26_MASK: u32 = 0x04000000;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk27_MASK: u32 = 0x08000000;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk28_MASK: u32 = 0x10000000;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk29_MASK: u32 = 0x20000000;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk30_MASK: u32 = 0x40000000;
pub const DF_CS_UMC_AON0_HardwareAssertMaskLow__HWAssertMsk31_MASK: u32 = 0x80000000;

//DF_NCS_PG0_HardwareAssertMaskHigh
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk0__SHIFT: u32 = 0x0;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk1__SHIFT: u32 = 0x1;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk2__SHIFT: u32 = 0x2;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk3__SHIFT: u32 = 0x3;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk4__SHIFT: u32 = 0x4;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk5__SHIFT: u32 = 0x5;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk6__SHIFT: u32 = 0x6;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk7__SHIFT: u32 = 0x7;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk8__SHIFT: u32 = 0x8;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk9__SHIFT: u32 = 0x9;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk10__SHIFT: u32 = 0xa;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk11__SHIFT: u32 = 0xb;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk12__SHIFT: u32 = 0xc;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk13__SHIFT: u32 = 0xd;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk14__SHIFT: u32 = 0xe;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk15__SHIFT: u32 = 0xf;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk16__SHIFT: u32 = 0x10;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk17__SHIFT: u32 = 0x11;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk18__SHIFT: u32 = 0x12;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk19__SHIFT: u32 = 0x13;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk20__SHIFT: u32 = 0x14;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk21__SHIFT: u32 = 0x15;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk22__SHIFT: u32 = 0x16;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk23__SHIFT: u32 = 0x17;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk24__SHIFT: u32 = 0x18;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk25__SHIFT: u32 = 0x19;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk26__SHIFT: u32 = 0x1a;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk27__SHIFT: u32 = 0x1b;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk28__SHIFT: u32 = 0x1c;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk29__SHIFT: u32 = 0x1d;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk30__SHIFT: u32 = 0x1e;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk31__SHIFT: u32 = 0x1f;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk0_MASK: u32 = 0x00000001;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk1_MASK: u32 = 0x00000002;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk2_MASK: u32 = 0x00000004;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk3_MASK: u32 = 0x00000008;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk4_MASK: u32 = 0x00000010;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk5_MASK: u32 = 0x00000020;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk6_MASK: u32 = 0x00000040;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk7_MASK: u32 = 0x00000080;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk8_MASK: u32 = 0x00000100;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk9_MASK: u32 = 0x00000200;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk10_MASK: u32 = 0x00000400;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk11_MASK: u32 = 0x00000800;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk12_MASK: u32 = 0x00001000;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk13_MASK: u32 = 0x00002000;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk14_MASK: u32 = 0x00004000;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk15_MASK: u32 = 0x00008000;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk16_MASK: u32 = 0x00010000;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk17_MASK: u32 = 0x00020000;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk18_MASK: u32 = 0x00040000;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk19_MASK: u32 = 0x00080000;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk20_MASK: u32 = 0x00100000;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk21_MASK: u32 = 0x00200000;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk22_MASK: u32 = 0x00400000;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk23_MASK: u32 = 0x00800000;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk24_MASK: u32 = 0x01000000;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk25_MASK: u32 = 0x02000000;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk26_MASK: u32 = 0x04000000;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk27_MASK: u32 = 0x08000000;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk28_MASK: u32 = 0x10000000;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk29_MASK: u32 = 0x20000000;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk30_MASK: u32 = 0x40000000;
pub const DF_NCS_PG0_HardwareAssertMaskHigh__HWAssertMsk31_MASK: u32 = 0x80000000;



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
