/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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


// addressBlock: umc_w_phy_umc0_mca_ip_umc0_mca_map
//MCA_UMC_UMC0_MCUMC_STATUST0
pub const MCA_UMC_UMC0_MCUMC_STATUST0__ErrorCode__SHIFT: u64 = 0x0;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__ErrorCodeExt__SHIFT: u64 = 0x10;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__RESERV22__SHIFT: u64 = 0x16;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__AddrLsb__SHIFT: u64 = 0x18;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__RESERV30__SHIFT: u64 = 0x1e;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__ErrCoreId__SHIFT: u64 = 0x20;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__RESERV38__SHIFT: u64 = 0x26;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__Scrub__SHIFT: u64 = 0x28;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__RESERV41__SHIFT: u64 = 0x29;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__Poison__SHIFT: u64 = 0x2b;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__Deferred__SHIFT: u64 = 0x2c;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__UECC__SHIFT: u64 = 0x2d;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__CECC__SHIFT: u64 = 0x2e;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__RESERV47__SHIFT: u64 = 0x2f;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__Transparent__SHIFT: u64 = 0x34;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__SyndV__SHIFT: u64 = 0x35;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__RESERV54__SHIFT: u64 = 0x36;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__TCC__SHIFT: u64 = 0x37;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__ErrCoreIdVal__SHIFT: u64 = 0x38;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__PCC__SHIFT: u64 = 0x39;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__AddrV__SHIFT: u64 = 0x3a;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__MiscV__SHIFT: u64 = 0x3b;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__En__SHIFT: u64 = 0x3c;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__UC__SHIFT: u64 = 0x3d;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__Overflow__SHIFT: u64 = 0x3e;
pub const MCA_UMC_UMC0_MCUMC_STATUST0__Val__SHIFT: u64 = 0x3f;
pub const : u64 = 0x000000000000FFFF;
pub const : u64 = 0x00000000003F0000;
pub const : u64 = 0x0000000000C00000;
pub const : u64 = 0x000000003F000000;
pub const : u64 = 0x00000000C0000000;
pub const : u64 = 0x0000003F00000000;
pub const : u64 = 0x000000C000000000;
pub const : u64 = 0x0000010000000000;
pub const : u64 = 0x0000060000000000;
pub const : u64 = 0x0000080000000000;
pub const : u64 = 0x0000100000000000;
pub const : u64 = 0x0000200000000000;
pub const : u64 = 0x0000400000000000;
pub const : u64 = 0x000F800000000000;
pub const : u64 = 0x0010000000000000;
pub const : u64 = 0x0020000000000000;
pub const : u64 = 0x0040000000000000;
pub const : u64 = 0x0080000000000000;
pub const : u64 = 0x0100000000000000;
pub const : u64 = 0x0200000000000000;
pub const : u64 = 0x0400000000000000;
pub const : u64 = 0x0800000000000000;
pub const : u64 = 0x1000000000000000;
pub const : u64 = 0x2000000000000000;
pub const : u64 = 0x4000000000000000;
pub const : u64 = 0x8000000000000000;
//MCA_UMC_UMC0_MCUMC_ADDRT0
pub const MCA_UMC_UMC0_MCUMC_ADDRT0__ErrorAddr__SHIFT: u64 = 0x0;
pub const MCA_UMC_UMC0_MCUMC_ADDRT0__Reserved__SHIFT: u64 = 0x38;
pub const : u64 = 0x00FFFFFFFFFFFFFF;
pub const : u64 = 0xFF00000000000000;


// addressBlock: umc_w_phy_umc0_umcch0_umcchdec
//UMCCH0_0_BaseAddrCS0
pub const UMCCH0_0_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_0_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH0_0_AddrMaskCS01
pub const UMCCH0_0_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH0_0_AddrSelCS01
pub const UMCCH0_0_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH0_0_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH0_0_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH0_0_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH0_0_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH0_0_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH0_0_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH0_0_AddrHashBank0
pub const UMCCH0_0_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_0_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH0_0_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH0_0_AddrHashBank1
pub const UMCCH0_0_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_0_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH0_0_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH0_0_AddrHashBank2
pub const UMCCH0_0_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_0_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH0_0_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH0_0_AddrHashBank3
pub const UMCCH0_0_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_0_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH0_0_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH0_0_AddrHashBank4
pub const UMCCH0_0_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_0_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH0_0_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH0_0_AddrHashBank5
pub const UMCCH0_0_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_0_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH0_0_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH0_0_UMC_CONFIG
pub const UMCCH0_0_UMC_CONFIG__DDR_TYPE__SHIFT: u64 = 0x0;
pub const UMCCH0_0_UMC_CONFIG__BurstLength__SHIFT: u64 = 0x8;
pub const UMCCH0_0_UMC_CONFIG__BurstCtrl__SHIFT: u64 = 0xa;
pub const UMCCH0_0_UMC_CONFIG__DramReady__SHIFT: u64 = 0x1f;
pub const : u64 = 0x00000007;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00000C00;
pub const : u64 = 0x80000000;
//UMCCH0_0_EccCtrl
pub const UMCCH0_0_EccCtrl__WrEccEn__SHIFT: u64 = 0x0;
pub const UMCCH0_0_EccCtrl__EccReplayEn__SHIFT: u64 = 0x1;
pub const UMCCH0_0_EccCtrl__UCFatalEn__SHIFT: u64 = 0x8;
pub const UMCCH0_0_EccCtrl__RdEccEn__SHIFT: u64 = 0xa;
pub const UMCCH0_0_EccCtrl__PoisonFatalDis__SHIFT: u64 = 0xc;
pub const UMCCH0_0_EccCtrl__PoisonInhibit__SHIFT: u64 = 0xd;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00000002;
pub const : u64 = 0x00000100;
pub const : u64 = 0x00000400;
pub const : u64 = 0x00001000;
pub const : u64 = 0x00002000;
//UMCCH0_0_UmcLocalCap
pub const UMCCH0_0_UmcLocalCap__EccDis__SHIFT: u64 = 0x0;
pub const UMCCH0_0_UmcLocalCap__Spare__SHIFT: u64 = 0x1;
pub const UMCCH0_0_UmcLocalCap__WrDis__SHIFT: u64 = 0x1f;
pub const : u64 = 0x00000001;
pub const : u64 = 0x0000003E;
pub const : u64 = 0x80000000;
//UMCCH0_0_EccErrCntSel
pub const UMCCH0_0_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH0_0_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH0_0_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH0_0_EccErrCnt
pub const UMCCH0_0_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH0_0_PerfMonCtlClk
pub const UMCCH0_0_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH0_0_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH0_0_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH0_0_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH0_0_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH0_0_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH0_0_PerfMonCtrClk_Lo
pub const UMCCH0_0_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_0_PerfMonCtrClk_Hi
pub const UMCCH0_0_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_0_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH0_0_PerfMonCtl1
pub const UMCCH0_0_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_0_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_0_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_0_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_0_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_0_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_0_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_0_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_0_PerfMonCtr1_Lo
pub const UMCCH0_0_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_0_PerfMonCtr1_Hi
pub const UMCCH0_0_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_0_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_0_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_0_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_0_PerfMonCtl2
pub const UMCCH0_0_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_0_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_0_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_0_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_0_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_0_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_0_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_0_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_0_PerfMonCtr2_Lo
pub const UMCCH0_0_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_0_PerfMonCtr2_Hi
pub const UMCCH0_0_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_0_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_0_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_0_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_0_PerfMonCtl3
pub const UMCCH0_0_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_0_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_0_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_0_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_0_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_0_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_0_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_0_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_0_PerfMonCtr3_Lo
pub const UMCCH0_0_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_0_PerfMonCtr3_Hi
pub const UMCCH0_0_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_0_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_0_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_0_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_0_PerfMonCtl4
pub const UMCCH0_0_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_0_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_0_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_0_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_0_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_0_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_0_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_0_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_0_PerfMonCtr4_Lo
pub const UMCCH0_0_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_0_PerfMonCtr4_Hi
pub const UMCCH0_0_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_0_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_0_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_0_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_0_PerfMonCtl5
pub const UMCCH0_0_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_0_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_0_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_0_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_0_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_0_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_0_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_0_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_0_PerfMonCtr5_Lo
pub const UMCCH0_0_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_0_PerfMonCtr5_Hi
pub const UMCCH0_0_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_0_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_0_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_0_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_0_PerfMonCtl6
pub const UMCCH0_0_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_0_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_0_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_0_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_0_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_0_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_0_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_0_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_0_PerfMonCtr6_Lo
pub const UMCCH0_0_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_0_PerfMonCtr6_Hi
pub const UMCCH0_0_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_0_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_0_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_0_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_0_PerfMonCtl7
pub const UMCCH0_0_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_0_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_0_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_0_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_0_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_0_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_0_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_0_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_0_PerfMonCtr7_Lo
pub const UMCCH0_0_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_0_PerfMonCtr7_Hi
pub const UMCCH0_0_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_0_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_0_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_0_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_0_PerfMonCtl8
pub const UMCCH0_0_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_0_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_0_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_0_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_0_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_0_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_0_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_0_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_0_PerfMonCtr8_Lo
pub const UMCCH0_0_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_0_PerfMonCtr8_Hi
pub const UMCCH0_0_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_0_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_0_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_0_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc0_umcch1_umcchdec
//UMCCH1_0_BaseAddrCS0
pub const UMCCH1_0_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_0_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH1_0_AddrMaskCS01
pub const UMCCH1_0_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH1_0_AddrSelCS01
pub const UMCCH1_0_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH1_0_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH1_0_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH1_0_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH1_0_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH1_0_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH1_0_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH1_0_AddrHashBank0
pub const UMCCH1_0_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_0_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH1_0_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH1_0_AddrHashBank1
pub const UMCCH1_0_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_0_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH1_0_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH1_0_AddrHashBank2
pub const UMCCH1_0_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_0_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH1_0_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH1_0_AddrHashBank3
pub const UMCCH1_0_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_0_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH1_0_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH1_0_AddrHashBank4
pub const UMCCH1_0_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_0_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH1_0_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH1_0_AddrHashBank5
pub const UMCCH1_0_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_0_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH1_0_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH1_0_UMC_CONFIG
pub const UMCCH1_0_UMC_CONFIG__DDR_TYPE__SHIFT: u64 = 0x0;
pub const UMCCH1_0_UMC_CONFIG__BurstLength__SHIFT: u64 = 0x8;
pub const UMCCH1_0_UMC_CONFIG__BurstCtrl__SHIFT: u64 = 0xa;
pub const UMCCH1_0_UMC_CONFIG__DramReady__SHIFT: u64 = 0x1f;
pub const : u64 = 0x00000007;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00000C00;
pub const : u64 = 0x80000000;
//UMCCH1_0_EccCtrl
pub const UMCCH1_0_EccCtrl__WrEccEn__SHIFT: u64 = 0x0;
pub const UMCCH1_0_EccCtrl__EccReplayEn__SHIFT: u64 = 0x1;
pub const UMCCH1_0_EccCtrl__UCFatalEn__SHIFT: u64 = 0x8;
pub const UMCCH1_0_EccCtrl__RdEccEn__SHIFT: u64 = 0xa;
pub const UMCCH1_0_EccCtrl__PoisonFatalDis__SHIFT: u64 = 0xc;
pub const UMCCH1_0_EccCtrl__PoisonInhibit__SHIFT: u64 = 0xd;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00000002;
pub const : u64 = 0x00000100;
pub const : u64 = 0x00000400;
pub const : u64 = 0x00001000;
pub const : u64 = 0x00002000;
//UMCCH1_0_UmcLocalCap
pub const UMCCH1_0_UmcLocalCap__EccDis__SHIFT: u64 = 0x0;
pub const UMCCH1_0_UmcLocalCap__Spare__SHIFT: u64 = 0x1;
pub const UMCCH1_0_UmcLocalCap__WrDis__SHIFT: u64 = 0x1f;
pub const : u64 = 0x00000001;
pub const : u64 = 0x0000003E;
pub const : u64 = 0x80000000;
//UMCCH1_0_EccErrCntSel
pub const UMCCH1_0_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH1_0_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH1_0_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH1_0_EccErrCnt
pub const UMCCH1_0_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH1_0_PerfMonCtlClk
pub const UMCCH1_0_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH1_0_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH1_0_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH1_0_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH1_0_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH1_0_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH1_0_PerfMonCtrClk_Lo
pub const UMCCH1_0_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_0_PerfMonCtrClk_Hi
pub const UMCCH1_0_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_0_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH1_0_PerfMonCtl1
pub const UMCCH1_0_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_0_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_0_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_0_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_0_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_0_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_0_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_0_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_0_PerfMonCtr1_Lo
pub const UMCCH1_0_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_0_PerfMonCtr1_Hi
pub const UMCCH1_0_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_0_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_0_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_0_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_0_PerfMonCtl2
pub const UMCCH1_0_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_0_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_0_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_0_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_0_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_0_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_0_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_0_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_0_PerfMonCtr2_Lo
pub const UMCCH1_0_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_0_PerfMonCtr2_Hi
pub const UMCCH1_0_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_0_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_0_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_0_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_0_PerfMonCtl3
pub const UMCCH1_0_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_0_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_0_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_0_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_0_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_0_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_0_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_0_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_0_PerfMonCtr3_Lo
pub const UMCCH1_0_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_0_PerfMonCtr3_Hi
pub const UMCCH1_0_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_0_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_0_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_0_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_0_PerfMonCtl4
pub const UMCCH1_0_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_0_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_0_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_0_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_0_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_0_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_0_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_0_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_0_PerfMonCtr4_Lo
pub const UMCCH1_0_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_0_PerfMonCtr4_Hi
pub const UMCCH1_0_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_0_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_0_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_0_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_0_PerfMonCtl5
pub const UMCCH1_0_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_0_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_0_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_0_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_0_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_0_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_0_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_0_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_0_PerfMonCtr5_Lo
pub const UMCCH1_0_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_0_PerfMonCtr5_Hi
pub const UMCCH1_0_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_0_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_0_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_0_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_0_PerfMonCtl6
pub const UMCCH1_0_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_0_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_0_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_0_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_0_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_0_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_0_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_0_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_0_PerfMonCtr6_Lo
pub const UMCCH1_0_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_0_PerfMonCtr6_Hi
pub const UMCCH1_0_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_0_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_0_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_0_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_0_PerfMonCtl7
pub const UMCCH1_0_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_0_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_0_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_0_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_0_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_0_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_0_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_0_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_0_PerfMonCtr7_Lo
pub const UMCCH1_0_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_0_PerfMonCtr7_Hi
pub const UMCCH1_0_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_0_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_0_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_0_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_0_PerfMonCtl8
pub const UMCCH1_0_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_0_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_0_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_0_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_0_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_0_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_0_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_0_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_0_PerfMonCtr8_Lo
pub const UMCCH1_0_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_0_PerfMonCtr8_Hi
pub const UMCCH1_0_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_0_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_0_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_0_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc0_umcch2_umcchdec
//UMCCH2_0_BaseAddrCS0
pub const UMCCH2_0_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_0_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH2_0_AddrMaskCS01
pub const UMCCH2_0_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH2_0_AddrSelCS01
pub const UMCCH2_0_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH2_0_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH2_0_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH2_0_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH2_0_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH2_0_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH2_0_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH2_0_AddrHashBank0
pub const UMCCH2_0_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_0_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH2_0_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH2_0_AddrHashBank1
pub const UMCCH2_0_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_0_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH2_0_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH2_0_AddrHashBank2
pub const UMCCH2_0_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_0_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH2_0_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH2_0_AddrHashBank3
pub const UMCCH2_0_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_0_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH2_0_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH2_0_AddrHashBank4
pub const UMCCH2_0_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_0_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH2_0_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH2_0_AddrHashBank5
pub const UMCCH2_0_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_0_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH2_0_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH2_0_UMC_CONFIG
pub const UMCCH2_0_UMC_CONFIG__DDR_TYPE__SHIFT: u64 = 0x0;
pub const UMCCH2_0_UMC_CONFIG__BurstLength__SHIFT: u64 = 0x8;
pub const UMCCH2_0_UMC_CONFIG__BurstCtrl__SHIFT: u64 = 0xa;
pub const UMCCH2_0_UMC_CONFIG__DramReady__SHIFT: u64 = 0x1f;
pub const : u64 = 0x00000007;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00000C00;
pub const : u64 = 0x80000000;
//UMCCH2_0_EccCtrl
pub const UMCCH2_0_EccCtrl__WrEccEn__SHIFT: u64 = 0x0;
pub const UMCCH2_0_EccCtrl__EccReplayEn__SHIFT: u64 = 0x1;
pub const UMCCH2_0_EccCtrl__UCFatalEn__SHIFT: u64 = 0x8;
pub const UMCCH2_0_EccCtrl__RdEccEn__SHIFT: u64 = 0xa;
pub const UMCCH2_0_EccCtrl__PoisonFatalDis__SHIFT: u64 = 0xc;
pub const UMCCH2_0_EccCtrl__PoisonInhibit__SHIFT: u64 = 0xd;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00000002;
pub const : u64 = 0x00000100;
pub const : u64 = 0x00000400;
pub const : u64 = 0x00001000;
pub const : u64 = 0x00002000;
//UMCCH2_0_UmcLocalCap
pub const UMCCH2_0_UmcLocalCap__EccDis__SHIFT: u64 = 0x0;
pub const UMCCH2_0_UmcLocalCap__Spare__SHIFT: u64 = 0x1;
pub const UMCCH2_0_UmcLocalCap__WrDis__SHIFT: u64 = 0x1f;
pub const : u64 = 0x00000001;
pub const : u64 = 0x0000003E;
pub const : u64 = 0x80000000;
//UMCCH2_0_EccErrCntSel
pub const UMCCH2_0_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH2_0_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH2_0_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH2_0_EccErrCnt
pub const UMCCH2_0_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH2_0_PerfMonCtlClk
pub const UMCCH2_0_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH2_0_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH2_0_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH2_0_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH2_0_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH2_0_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH2_0_PerfMonCtrClk_Lo
pub const UMCCH2_0_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_0_PerfMonCtrClk_Hi
pub const UMCCH2_0_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_0_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH2_0_PerfMonCtl1
pub const UMCCH2_0_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_0_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_0_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_0_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_0_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_0_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_0_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_0_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_0_PerfMonCtr1_Lo
pub const UMCCH2_0_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_0_PerfMonCtr1_Hi
pub const UMCCH2_0_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_0_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_0_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_0_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_0_PerfMonCtl2
pub const UMCCH2_0_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_0_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_0_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_0_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_0_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_0_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_0_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_0_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_0_PerfMonCtr2_Lo
pub const UMCCH2_0_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_0_PerfMonCtr2_Hi
pub const UMCCH2_0_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_0_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_0_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_0_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_0_PerfMonCtl3
pub const UMCCH2_0_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_0_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_0_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_0_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_0_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_0_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_0_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_0_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_0_PerfMonCtr3_Lo
pub const UMCCH2_0_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_0_PerfMonCtr3_Hi
pub const UMCCH2_0_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_0_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_0_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_0_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_0_PerfMonCtl4
pub const UMCCH2_0_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_0_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_0_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_0_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_0_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_0_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_0_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_0_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_0_PerfMonCtr4_Lo
pub const UMCCH2_0_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_0_PerfMonCtr4_Hi
pub const UMCCH2_0_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_0_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_0_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_0_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_0_PerfMonCtl5
pub const UMCCH2_0_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_0_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_0_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_0_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_0_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_0_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_0_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_0_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_0_PerfMonCtr5_Lo
pub const UMCCH2_0_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_0_PerfMonCtr5_Hi
pub const UMCCH2_0_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_0_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_0_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_0_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_0_PerfMonCtl6
pub const UMCCH2_0_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_0_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_0_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_0_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_0_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_0_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_0_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_0_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_0_PerfMonCtr6_Lo
pub const UMCCH2_0_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_0_PerfMonCtr6_Hi
pub const UMCCH2_0_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_0_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_0_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_0_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_0_PerfMonCtl7
pub const UMCCH2_0_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_0_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_0_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_0_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_0_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_0_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_0_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_0_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_0_PerfMonCtr7_Lo
pub const UMCCH2_0_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_0_PerfMonCtr7_Hi
pub const UMCCH2_0_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_0_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_0_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_0_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_0_PerfMonCtl8
pub const UMCCH2_0_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_0_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_0_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_0_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_0_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_0_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_0_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_0_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_0_PerfMonCtr8_Lo
pub const UMCCH2_0_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_0_PerfMonCtr8_Hi
pub const UMCCH2_0_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_0_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_0_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_0_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc0_umcch3_umcchdec
//UMCCH3_0_BaseAddrCS0
pub const UMCCH3_0_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_0_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH3_0_AddrMaskCS01
pub const UMCCH3_0_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH3_0_AddrSelCS01
pub const UMCCH3_0_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH3_0_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH3_0_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH3_0_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH3_0_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH3_0_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH3_0_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH3_0_AddrHashBank0
pub const UMCCH3_0_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_0_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH3_0_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH3_0_AddrHashBank1
pub const UMCCH3_0_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_0_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH3_0_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH3_0_AddrHashBank2
pub const UMCCH3_0_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_0_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH3_0_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH3_0_AddrHashBank3
pub const UMCCH3_0_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_0_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH3_0_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH3_0_AddrHashBank4
pub const UMCCH3_0_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_0_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH3_0_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH3_0_AddrHashBank5
pub const UMCCH3_0_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_0_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH3_0_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH3_0_UMC_CONFIG
pub const UMCCH3_0_UMC_CONFIG__DDR_TYPE__SHIFT: u64 = 0x0;
pub const UMCCH3_0_UMC_CONFIG__BurstLength__SHIFT: u64 = 0x8;
pub const UMCCH3_0_UMC_CONFIG__BurstCtrl__SHIFT: u64 = 0xa;
pub const UMCCH3_0_UMC_CONFIG__DramReady__SHIFT: u64 = 0x1f;
pub const : u64 = 0x00000007;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00000C00;
pub const : u64 = 0x80000000;
//UMCCH3_0_EccCtrl
pub const UMCCH3_0_EccCtrl__WrEccEn__SHIFT: u64 = 0x0;
pub const UMCCH3_0_EccCtrl__EccReplayEn__SHIFT: u64 = 0x1;
pub const UMCCH3_0_EccCtrl__UCFatalEn__SHIFT: u64 = 0x8;
pub const UMCCH3_0_EccCtrl__RdEccEn__SHIFT: u64 = 0xa;
pub const UMCCH3_0_EccCtrl__PoisonFatalDis__SHIFT: u64 = 0xc;
pub const UMCCH3_0_EccCtrl__PoisonInhibit__SHIFT: u64 = 0xd;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00000002;
pub const : u64 = 0x00000100;
pub const : u64 = 0x00000400;
pub const : u64 = 0x00001000;
pub const : u64 = 0x00002000;
//UMCCH3_0_UmcLocalCap
pub const UMCCH3_0_UmcLocalCap__EccDis__SHIFT: u64 = 0x0;
pub const UMCCH3_0_UmcLocalCap__Spare__SHIFT: u64 = 0x1;
pub const UMCCH3_0_UmcLocalCap__WrDis__SHIFT: u64 = 0x1f;
pub const : u64 = 0x00000001;
pub const : u64 = 0x0000003E;
pub const : u64 = 0x80000000;
//UMCCH3_0_EccErrCntSel
pub const UMCCH3_0_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH3_0_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH3_0_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH3_0_EccErrCnt
pub const UMCCH3_0_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH3_0_PerfMonCtlClk
pub const UMCCH3_0_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH3_0_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH3_0_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH3_0_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH3_0_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH3_0_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH3_0_PerfMonCtrClk_Lo
pub const UMCCH3_0_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_0_PerfMonCtrClk_Hi
pub const UMCCH3_0_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_0_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH3_0_PerfMonCtl1
pub const UMCCH3_0_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_0_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_0_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_0_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_0_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_0_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_0_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_0_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_0_PerfMonCtr1_Lo
pub const UMCCH3_0_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_0_PerfMonCtr1_Hi
pub const UMCCH3_0_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_0_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_0_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_0_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_0_PerfMonCtl2
pub const UMCCH3_0_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_0_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_0_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_0_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_0_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_0_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_0_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_0_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_0_PerfMonCtr2_Lo
pub const UMCCH3_0_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_0_PerfMonCtr2_Hi
pub const UMCCH3_0_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_0_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_0_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_0_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_0_PerfMonCtl3
pub const UMCCH3_0_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_0_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_0_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_0_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_0_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_0_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_0_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_0_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_0_PerfMonCtr3_Lo
pub const UMCCH3_0_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_0_PerfMonCtr3_Hi
pub const UMCCH3_0_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_0_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_0_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_0_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_0_PerfMonCtl4
pub const UMCCH3_0_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_0_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_0_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_0_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_0_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_0_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_0_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_0_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_0_PerfMonCtr4_Lo
pub const UMCCH3_0_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_0_PerfMonCtr4_Hi
pub const UMCCH3_0_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_0_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_0_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_0_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_0_PerfMonCtl5
pub const UMCCH3_0_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_0_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_0_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_0_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_0_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_0_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_0_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_0_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_0_PerfMonCtr5_Lo
pub const UMCCH3_0_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_0_PerfMonCtr5_Hi
pub const UMCCH3_0_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_0_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_0_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_0_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_0_PerfMonCtl6
pub const UMCCH3_0_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_0_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_0_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_0_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_0_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_0_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_0_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_0_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_0_PerfMonCtr6_Lo
pub const UMCCH3_0_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_0_PerfMonCtr6_Hi
pub const UMCCH3_0_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_0_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_0_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_0_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_0_PerfMonCtl7
pub const UMCCH3_0_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_0_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_0_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_0_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_0_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_0_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_0_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_0_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_0_PerfMonCtr7_Lo
pub const UMCCH3_0_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_0_PerfMonCtr7_Hi
pub const UMCCH3_0_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_0_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_0_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_0_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_0_PerfMonCtl8
pub const UMCCH3_0_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_0_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_0_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_0_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_0_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_0_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_0_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_0_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_0_PerfMonCtr8_Lo
pub const UMCCH3_0_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_0_PerfMonCtr8_Hi
pub const UMCCH3_0_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_0_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_0_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_0_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc0_umcch4_umcchdec
//UMCCH4_0_BaseAddrCS0
pub const UMCCH4_0_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_0_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH4_0_AddrMaskCS01
pub const UMCCH4_0_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH4_0_AddrSelCS01
pub const UMCCH4_0_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH4_0_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH4_0_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH4_0_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH4_0_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH4_0_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH4_0_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH4_0_AddrHashBank0
pub const UMCCH4_0_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_0_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH4_0_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH4_0_AddrHashBank1
pub const UMCCH4_0_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_0_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH4_0_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH4_0_AddrHashBank2
pub const UMCCH4_0_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_0_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH4_0_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH4_0_AddrHashBank3
pub const UMCCH4_0_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_0_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH4_0_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH4_0_AddrHashBank4
pub const UMCCH4_0_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_0_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH4_0_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH4_0_AddrHashBank5
pub const UMCCH4_0_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_0_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH4_0_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH4_0_EccErrCntSel
pub const UMCCH4_0_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH4_0_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH4_0_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH4_0_EccErrCnt
pub const UMCCH4_0_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH4_0_PerfMonCtlClk
pub const UMCCH4_0_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH4_0_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH4_0_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH4_0_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH4_0_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH4_0_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH4_0_PerfMonCtrClk_Lo
pub const UMCCH4_0_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_0_PerfMonCtrClk_Hi
pub const UMCCH4_0_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_0_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH4_0_PerfMonCtl1
pub const UMCCH4_0_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_0_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_0_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_0_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_0_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_0_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_0_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_0_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_0_PerfMonCtr1_Lo
pub const UMCCH4_0_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_0_PerfMonCtr1_Hi
pub const UMCCH4_0_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_0_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_0_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_0_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_0_PerfMonCtl2
pub const UMCCH4_0_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_0_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_0_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_0_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_0_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_0_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_0_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_0_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_0_PerfMonCtr2_Lo
pub const UMCCH4_0_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_0_PerfMonCtr2_Hi
pub const UMCCH4_0_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_0_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_0_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_0_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_0_PerfMonCtl3
pub const UMCCH4_0_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_0_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_0_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_0_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_0_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_0_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_0_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_0_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_0_PerfMonCtr3_Lo
pub const UMCCH4_0_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_0_PerfMonCtr3_Hi
pub const UMCCH4_0_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_0_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_0_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_0_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_0_PerfMonCtl4
pub const UMCCH4_0_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_0_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_0_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_0_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_0_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_0_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_0_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_0_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_0_PerfMonCtr4_Lo
pub const UMCCH4_0_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_0_PerfMonCtr4_Hi
pub const UMCCH4_0_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_0_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_0_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_0_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_0_PerfMonCtl5
pub const UMCCH4_0_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_0_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_0_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_0_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_0_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_0_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_0_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_0_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_0_PerfMonCtr5_Lo
pub const UMCCH4_0_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_0_PerfMonCtr5_Hi
pub const UMCCH4_0_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_0_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_0_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_0_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_0_PerfMonCtl6
pub const UMCCH4_0_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_0_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_0_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_0_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_0_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_0_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_0_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_0_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_0_PerfMonCtr6_Lo
pub const UMCCH4_0_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_0_PerfMonCtr6_Hi
pub const UMCCH4_0_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_0_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_0_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_0_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_0_PerfMonCtl7
pub const UMCCH4_0_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_0_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_0_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_0_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_0_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_0_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_0_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_0_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_0_PerfMonCtr7_Lo
pub const UMCCH4_0_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_0_PerfMonCtr7_Hi
pub const UMCCH4_0_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_0_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_0_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_0_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_0_PerfMonCtl8
pub const UMCCH4_0_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_0_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_0_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_0_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_0_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_0_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_0_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_0_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_0_PerfMonCtr8_Lo
pub const UMCCH4_0_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_0_PerfMonCtr8_Hi
pub const UMCCH4_0_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_0_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_0_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_0_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc0_umcch5_umcchdec
//UMCCH5_0_BaseAddrCS0
pub const UMCCH5_0_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_0_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH5_0_AddrMaskCS01
pub const UMCCH5_0_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH5_0_AddrSelCS01
pub const UMCCH5_0_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH5_0_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH5_0_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH5_0_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH5_0_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH5_0_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH5_0_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH5_0_AddrHashBank0
pub const UMCCH5_0_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_0_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH5_0_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH5_0_AddrHashBank1
pub const UMCCH5_0_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_0_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH5_0_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH5_0_AddrHashBank2
pub const UMCCH5_0_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_0_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH5_0_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH5_0_AddrHashBank3
pub const UMCCH5_0_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_0_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH5_0_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH5_0_AddrHashBank4
pub const UMCCH5_0_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_0_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH5_0_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH5_0_AddrHashBank5
pub const UMCCH5_0_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_0_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH5_0_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH5_0_EccErrCntSel
pub const UMCCH5_0_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH5_0_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH5_0_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH5_0_EccErrCnt
pub const UMCCH5_0_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH5_0_PerfMonCtlClk
pub const UMCCH5_0_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH5_0_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH5_0_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH5_0_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH5_0_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH5_0_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH5_0_PerfMonCtrClk_Lo
pub const UMCCH5_0_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_0_PerfMonCtrClk_Hi
pub const UMCCH5_0_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_0_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH5_0_PerfMonCtl1
pub const UMCCH5_0_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_0_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_0_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_0_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_0_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_0_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_0_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_0_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_0_PerfMonCtr1_Lo
pub const UMCCH5_0_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_0_PerfMonCtr1_Hi
pub const UMCCH5_0_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_0_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_0_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_0_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_0_PerfMonCtl2
pub const UMCCH5_0_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_0_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_0_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_0_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_0_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_0_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_0_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_0_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_0_PerfMonCtr2_Lo
pub const UMCCH5_0_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_0_PerfMonCtr2_Hi
pub const UMCCH5_0_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_0_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_0_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_0_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_0_PerfMonCtl3
pub const UMCCH5_0_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_0_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_0_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_0_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_0_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_0_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_0_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_0_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_0_PerfMonCtr3_Lo
pub const UMCCH5_0_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_0_PerfMonCtr3_Hi
pub const UMCCH5_0_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_0_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_0_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_0_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_0_PerfMonCtl4
pub const UMCCH5_0_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_0_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_0_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_0_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_0_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_0_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_0_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_0_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_0_PerfMonCtr4_Lo
pub const UMCCH5_0_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_0_PerfMonCtr4_Hi
pub const UMCCH5_0_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_0_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_0_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_0_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_0_PerfMonCtl5
pub const UMCCH5_0_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_0_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_0_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_0_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_0_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_0_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_0_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_0_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_0_PerfMonCtr5_Lo
pub const UMCCH5_0_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_0_PerfMonCtr5_Hi
pub const UMCCH5_0_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_0_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_0_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_0_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_0_PerfMonCtl6
pub const UMCCH5_0_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_0_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_0_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_0_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_0_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_0_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_0_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_0_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_0_PerfMonCtr6_Lo
pub const UMCCH5_0_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_0_PerfMonCtr6_Hi
pub const UMCCH5_0_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_0_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_0_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_0_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_0_PerfMonCtl7
pub const UMCCH5_0_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_0_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_0_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_0_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_0_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_0_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_0_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_0_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_0_PerfMonCtr7_Lo
pub const UMCCH5_0_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_0_PerfMonCtr7_Hi
pub const UMCCH5_0_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_0_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_0_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_0_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_0_PerfMonCtl8
pub const UMCCH5_0_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_0_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_0_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_0_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_0_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_0_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_0_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_0_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_0_PerfMonCtr8_Lo
pub const UMCCH5_0_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_0_PerfMonCtr8_Hi
pub const UMCCH5_0_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_0_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_0_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_0_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc0_umcch6_umcchdec
//UMCCH6_0_BaseAddrCS0
pub const UMCCH6_0_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_0_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH6_0_AddrMaskCS01
pub const UMCCH6_0_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH6_0_AddrSelCS01
pub const UMCCH6_0_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH6_0_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH6_0_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH6_0_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH6_0_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH6_0_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH6_0_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH6_0_AddrHashBank0
pub const UMCCH6_0_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_0_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH6_0_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH6_0_AddrHashBank1
pub const UMCCH6_0_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_0_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH6_0_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH6_0_AddrHashBank2
pub const UMCCH6_0_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_0_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH6_0_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH6_0_AddrHashBank3
pub const UMCCH6_0_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_0_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH6_0_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH6_0_AddrHashBank4
pub const UMCCH6_0_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_0_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH6_0_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH6_0_AddrHashBank5
pub const UMCCH6_0_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_0_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH6_0_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH6_0_EccErrCntSel
pub const UMCCH6_0_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH6_0_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH6_0_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH6_0_EccErrCnt
pub const UMCCH6_0_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH6_0_PerfMonCtlClk
pub const UMCCH6_0_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH6_0_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH6_0_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH6_0_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH6_0_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH6_0_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH6_0_PerfMonCtrClk_Lo
pub const UMCCH6_0_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_0_PerfMonCtrClk_Hi
pub const UMCCH6_0_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_0_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH6_0_PerfMonCtl1
pub const UMCCH6_0_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_0_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_0_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_0_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_0_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_0_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_0_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_0_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_0_PerfMonCtr1_Lo
pub const UMCCH6_0_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_0_PerfMonCtr1_Hi
pub const UMCCH6_0_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_0_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_0_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_0_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_0_PerfMonCtl2
pub const UMCCH6_0_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_0_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_0_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_0_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_0_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_0_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_0_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_0_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_0_PerfMonCtr2_Lo
pub const UMCCH6_0_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_0_PerfMonCtr2_Hi
pub const UMCCH6_0_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_0_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_0_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_0_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_0_PerfMonCtl3
pub const UMCCH6_0_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_0_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_0_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_0_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_0_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_0_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_0_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_0_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_0_PerfMonCtr3_Lo
pub const UMCCH6_0_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_0_PerfMonCtr3_Hi
pub const UMCCH6_0_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_0_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_0_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_0_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_0_PerfMonCtl4
pub const UMCCH6_0_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_0_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_0_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_0_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_0_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_0_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_0_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_0_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_0_PerfMonCtr4_Lo
pub const UMCCH6_0_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_0_PerfMonCtr4_Hi
pub const UMCCH6_0_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_0_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_0_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_0_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_0_PerfMonCtl5
pub const UMCCH6_0_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_0_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_0_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_0_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_0_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_0_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_0_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_0_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_0_PerfMonCtr5_Lo
pub const UMCCH6_0_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_0_PerfMonCtr5_Hi
pub const UMCCH6_0_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_0_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_0_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_0_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_0_PerfMonCtl6
pub const UMCCH6_0_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_0_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_0_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_0_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_0_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_0_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_0_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_0_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_0_PerfMonCtr6_Lo
pub const UMCCH6_0_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_0_PerfMonCtr6_Hi
pub const UMCCH6_0_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_0_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_0_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_0_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_0_PerfMonCtl7
pub const UMCCH6_0_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_0_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_0_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_0_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_0_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_0_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_0_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_0_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_0_PerfMonCtr7_Lo
pub const UMCCH6_0_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_0_PerfMonCtr7_Hi
pub const UMCCH6_0_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_0_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_0_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_0_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_0_PerfMonCtl8
pub const UMCCH6_0_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_0_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_0_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_0_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_0_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_0_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_0_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_0_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_0_PerfMonCtr8_Lo
pub const UMCCH6_0_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_0_PerfMonCtr8_Hi
pub const UMCCH6_0_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_0_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_0_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_0_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc0_umcch7_umcchdec
//UMCCH7_0_BaseAddrCS0
pub const UMCCH7_0_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_0_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH7_0_AddrMaskCS01
pub const UMCCH7_0_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH7_0_AddrSelCS01
pub const UMCCH7_0_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH7_0_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH7_0_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH7_0_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH7_0_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH7_0_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH7_0_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH7_0_AddrHashBank0
pub const UMCCH7_0_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_0_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH7_0_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH7_0_AddrHashBank1
pub const UMCCH7_0_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_0_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH7_0_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH7_0_AddrHashBank2
pub const UMCCH7_0_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_0_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH7_0_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH7_0_AddrHashBank3
pub const UMCCH7_0_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_0_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH7_0_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH7_0_AddrHashBank4
pub const UMCCH7_0_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_0_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH7_0_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH7_0_AddrHashBank5
pub const UMCCH7_0_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_0_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH7_0_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH7_0_EccErrCntSel
pub const UMCCH7_0_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH7_0_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH7_0_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH7_0_EccErrCnt
pub const UMCCH7_0_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH7_0_PerfMonCtlClk
pub const UMCCH7_0_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH7_0_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH7_0_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH7_0_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH7_0_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH7_0_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH7_0_PerfMonCtrClk_Lo
pub const UMCCH7_0_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_0_PerfMonCtrClk_Hi
pub const UMCCH7_0_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_0_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH7_0_PerfMonCtl1
pub const UMCCH7_0_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_0_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_0_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_0_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_0_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_0_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_0_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_0_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_0_PerfMonCtr1_Lo
pub const UMCCH7_0_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_0_PerfMonCtr1_Hi
pub const UMCCH7_0_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_0_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_0_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_0_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_0_PerfMonCtl2
pub const UMCCH7_0_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_0_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_0_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_0_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_0_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_0_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_0_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_0_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_0_PerfMonCtr2_Lo
pub const UMCCH7_0_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_0_PerfMonCtr2_Hi
pub const UMCCH7_0_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_0_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_0_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_0_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_0_PerfMonCtl3
pub const UMCCH7_0_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_0_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_0_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_0_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_0_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_0_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_0_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_0_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_0_PerfMonCtr3_Lo
pub const UMCCH7_0_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_0_PerfMonCtr3_Hi
pub const UMCCH7_0_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_0_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_0_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_0_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_0_PerfMonCtl4
pub const UMCCH7_0_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_0_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_0_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_0_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_0_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_0_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_0_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_0_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_0_PerfMonCtr4_Lo
pub const UMCCH7_0_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_0_PerfMonCtr4_Hi
pub const UMCCH7_0_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_0_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_0_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_0_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_0_PerfMonCtl5
pub const UMCCH7_0_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_0_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_0_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_0_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_0_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_0_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_0_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_0_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_0_PerfMonCtr5_Lo
pub const UMCCH7_0_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_0_PerfMonCtr5_Hi
pub const UMCCH7_0_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_0_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_0_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_0_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_0_PerfMonCtl6
pub const UMCCH7_0_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_0_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_0_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_0_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_0_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_0_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_0_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_0_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_0_PerfMonCtr6_Lo
pub const UMCCH7_0_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_0_PerfMonCtr6_Hi
pub const UMCCH7_0_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_0_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_0_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_0_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_0_PerfMonCtl7
pub const UMCCH7_0_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_0_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_0_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_0_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_0_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_0_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_0_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_0_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_0_PerfMonCtr7_Lo
pub const UMCCH7_0_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_0_PerfMonCtr7_Hi
pub const UMCCH7_0_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_0_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_0_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_0_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_0_PerfMonCtl8
pub const UMCCH7_0_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_0_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_0_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_0_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_0_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_0_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_0_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_0_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_0_PerfMonCtr8_Lo
pub const UMCCH7_0_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_0_PerfMonCtr8_Hi
pub const UMCCH7_0_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_0_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_0_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_0_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc1_umcch0_umcchdec
//UMCCH0_1_BaseAddrCS0
pub const UMCCH0_1_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_1_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH0_1_AddrMaskCS01
pub const UMCCH0_1_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH0_1_AddrSelCS01
pub const UMCCH0_1_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH0_1_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH0_1_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH0_1_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH0_1_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH0_1_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH0_1_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH0_1_AddrHashBank0
pub const UMCCH0_1_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_1_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH0_1_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH0_1_AddrHashBank1
pub const UMCCH0_1_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_1_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH0_1_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH0_1_AddrHashBank2
pub const UMCCH0_1_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_1_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH0_1_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH0_1_AddrHashBank3
pub const UMCCH0_1_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_1_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH0_1_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH0_1_AddrHashBank4
pub const UMCCH0_1_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_1_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH0_1_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH0_1_AddrHashBank5
pub const UMCCH0_1_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_1_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH0_1_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH0_1_EccErrCntSel
pub const UMCCH0_1_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH0_1_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH0_1_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH0_1_EccErrCnt
pub const UMCCH0_1_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH0_1_PerfMonCtlClk
pub const UMCCH0_1_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH0_1_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH0_1_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH0_1_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH0_1_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH0_1_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH0_1_PerfMonCtrClk_Lo
pub const UMCCH0_1_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_1_PerfMonCtrClk_Hi
pub const UMCCH0_1_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_1_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH0_1_PerfMonCtl1
pub const UMCCH0_1_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_1_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_1_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_1_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_1_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_1_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_1_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_1_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_1_PerfMonCtr1_Lo
pub const UMCCH0_1_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_1_PerfMonCtr1_Hi
pub const UMCCH0_1_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_1_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_1_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_1_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_1_PerfMonCtl2
pub const UMCCH0_1_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_1_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_1_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_1_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_1_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_1_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_1_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_1_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_1_PerfMonCtr2_Lo
pub const UMCCH0_1_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_1_PerfMonCtr2_Hi
pub const UMCCH0_1_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_1_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_1_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_1_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_1_PerfMonCtl3
pub const UMCCH0_1_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_1_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_1_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_1_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_1_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_1_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_1_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_1_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_1_PerfMonCtr3_Lo
pub const UMCCH0_1_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_1_PerfMonCtr3_Hi
pub const UMCCH0_1_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_1_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_1_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_1_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_1_PerfMonCtl4
pub const UMCCH0_1_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_1_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_1_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_1_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_1_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_1_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_1_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_1_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_1_PerfMonCtr4_Lo
pub const UMCCH0_1_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_1_PerfMonCtr4_Hi
pub const UMCCH0_1_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_1_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_1_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_1_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_1_PerfMonCtl5
pub const UMCCH0_1_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_1_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_1_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_1_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_1_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_1_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_1_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_1_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_1_PerfMonCtr5_Lo
pub const UMCCH0_1_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_1_PerfMonCtr5_Hi
pub const UMCCH0_1_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_1_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_1_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_1_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_1_PerfMonCtl6
pub const UMCCH0_1_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_1_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_1_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_1_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_1_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_1_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_1_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_1_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_1_PerfMonCtr6_Lo
pub const UMCCH0_1_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_1_PerfMonCtr6_Hi
pub const UMCCH0_1_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_1_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_1_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_1_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_1_PerfMonCtl7
pub const UMCCH0_1_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_1_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_1_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_1_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_1_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_1_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_1_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_1_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_1_PerfMonCtr7_Lo
pub const UMCCH0_1_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_1_PerfMonCtr7_Hi
pub const UMCCH0_1_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_1_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_1_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_1_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_1_PerfMonCtl8
pub const UMCCH0_1_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_1_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_1_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_1_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_1_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_1_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_1_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_1_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_1_PerfMonCtr8_Lo
pub const UMCCH0_1_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_1_PerfMonCtr8_Hi
pub const UMCCH0_1_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_1_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_1_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_1_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc1_umcch1_umcchdec
//UMCCH1_1_BaseAddrCS0
pub const UMCCH1_1_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_1_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH1_1_AddrMaskCS01
pub const UMCCH1_1_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH1_1_AddrSelCS01
pub const UMCCH1_1_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH1_1_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH1_1_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH1_1_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH1_1_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH1_1_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH1_1_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH1_1_AddrHashBank0
pub const UMCCH1_1_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_1_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH1_1_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH1_1_AddrHashBank1
pub const UMCCH1_1_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_1_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH1_1_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH1_1_AddrHashBank2
pub const UMCCH1_1_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_1_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH1_1_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH1_1_AddrHashBank3
pub const UMCCH1_1_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_1_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH1_1_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH1_1_AddrHashBank4
pub const UMCCH1_1_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_1_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH1_1_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH1_1_AddrHashBank5
pub const UMCCH1_1_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_1_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH1_1_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH1_1_EccErrCntSel
pub const UMCCH1_1_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH1_1_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH1_1_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH1_1_EccErrCnt
pub const UMCCH1_1_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH1_1_PerfMonCtlClk
pub const UMCCH1_1_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH1_1_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH1_1_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH1_1_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH1_1_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH1_1_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH1_1_PerfMonCtrClk_Lo
pub const UMCCH1_1_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_1_PerfMonCtrClk_Hi
pub const UMCCH1_1_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_1_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH1_1_PerfMonCtl1
pub const UMCCH1_1_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_1_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_1_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_1_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_1_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_1_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_1_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_1_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_1_PerfMonCtr1_Lo
pub const UMCCH1_1_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_1_PerfMonCtr1_Hi
pub const UMCCH1_1_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_1_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_1_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_1_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_1_PerfMonCtl2
pub const UMCCH1_1_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_1_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_1_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_1_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_1_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_1_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_1_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_1_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_1_PerfMonCtr2_Lo
pub const UMCCH1_1_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_1_PerfMonCtr2_Hi
pub const UMCCH1_1_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_1_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_1_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_1_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_1_PerfMonCtl3
pub const UMCCH1_1_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_1_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_1_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_1_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_1_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_1_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_1_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_1_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_1_PerfMonCtr3_Lo
pub const UMCCH1_1_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_1_PerfMonCtr3_Hi
pub const UMCCH1_1_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_1_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_1_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_1_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_1_PerfMonCtl4
pub const UMCCH1_1_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_1_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_1_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_1_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_1_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_1_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_1_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_1_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_1_PerfMonCtr4_Lo
pub const UMCCH1_1_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_1_PerfMonCtr4_Hi
pub const UMCCH1_1_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_1_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_1_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_1_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_1_PerfMonCtl5
pub const UMCCH1_1_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_1_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_1_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_1_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_1_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_1_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_1_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_1_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_1_PerfMonCtr5_Lo
pub const UMCCH1_1_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_1_PerfMonCtr5_Hi
pub const UMCCH1_1_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_1_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_1_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_1_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_1_PerfMonCtl6
pub const UMCCH1_1_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_1_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_1_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_1_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_1_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_1_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_1_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_1_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_1_PerfMonCtr6_Lo
pub const UMCCH1_1_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_1_PerfMonCtr6_Hi
pub const UMCCH1_1_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_1_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_1_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_1_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_1_PerfMonCtl7
pub const UMCCH1_1_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_1_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_1_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_1_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_1_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_1_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_1_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_1_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_1_PerfMonCtr7_Lo
pub const UMCCH1_1_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_1_PerfMonCtr7_Hi
pub const UMCCH1_1_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_1_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_1_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_1_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_1_PerfMonCtl8
pub const UMCCH1_1_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_1_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_1_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_1_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_1_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_1_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_1_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_1_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_1_PerfMonCtr8_Lo
pub const UMCCH1_1_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_1_PerfMonCtr8_Hi
pub const UMCCH1_1_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_1_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_1_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_1_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc1_umcch2_umcchdec
//UMCCH2_1_BaseAddrCS0
pub const UMCCH2_1_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_1_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH2_1_AddrMaskCS01
pub const UMCCH2_1_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH2_1_AddrSelCS01
pub const UMCCH2_1_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH2_1_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH2_1_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH2_1_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH2_1_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH2_1_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH2_1_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH2_1_AddrHashBank0
pub const UMCCH2_1_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_1_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH2_1_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH2_1_AddrHashBank1
pub const UMCCH2_1_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_1_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH2_1_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH2_1_AddrHashBank2
pub const UMCCH2_1_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_1_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH2_1_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH2_1_AddrHashBank3
pub const UMCCH2_1_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_1_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH2_1_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH2_1_AddrHashBank4
pub const UMCCH2_1_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_1_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH2_1_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH2_1_AddrHashBank5
pub const UMCCH2_1_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_1_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH2_1_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH2_1_EccErrCntSel
pub const UMCCH2_1_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH2_1_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH2_1_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH2_1_EccErrCnt
pub const UMCCH2_1_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH2_1_PerfMonCtlClk
pub const UMCCH2_1_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH2_1_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH2_1_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH2_1_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH2_1_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH2_1_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH2_1_PerfMonCtrClk_Lo
pub const UMCCH2_1_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_1_PerfMonCtrClk_Hi
pub const UMCCH2_1_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_1_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH2_1_PerfMonCtl1
pub const UMCCH2_1_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_1_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_1_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_1_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_1_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_1_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_1_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_1_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_1_PerfMonCtr1_Lo
pub const UMCCH2_1_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_1_PerfMonCtr1_Hi
pub const UMCCH2_1_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_1_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_1_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_1_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_1_PerfMonCtl2
pub const UMCCH2_1_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_1_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_1_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_1_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_1_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_1_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_1_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_1_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_1_PerfMonCtr2_Lo
pub const UMCCH2_1_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_1_PerfMonCtr2_Hi
pub const UMCCH2_1_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_1_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_1_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_1_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_1_PerfMonCtl3
pub const UMCCH2_1_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_1_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_1_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_1_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_1_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_1_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_1_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_1_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_1_PerfMonCtr3_Lo
pub const UMCCH2_1_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_1_PerfMonCtr3_Hi
pub const UMCCH2_1_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_1_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_1_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_1_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_1_PerfMonCtl4
pub const UMCCH2_1_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_1_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_1_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_1_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_1_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_1_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_1_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_1_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_1_PerfMonCtr4_Lo
pub const UMCCH2_1_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_1_PerfMonCtr4_Hi
pub const UMCCH2_1_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_1_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_1_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_1_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_1_PerfMonCtl5
pub const UMCCH2_1_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_1_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_1_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_1_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_1_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_1_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_1_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_1_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_1_PerfMonCtr5_Lo
pub const UMCCH2_1_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_1_PerfMonCtr5_Hi
pub const UMCCH2_1_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_1_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_1_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_1_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_1_PerfMonCtl6
pub const UMCCH2_1_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_1_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_1_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_1_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_1_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_1_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_1_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_1_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_1_PerfMonCtr6_Lo
pub const UMCCH2_1_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_1_PerfMonCtr6_Hi
pub const UMCCH2_1_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_1_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_1_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_1_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_1_PerfMonCtl7
pub const UMCCH2_1_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_1_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_1_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_1_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_1_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_1_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_1_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_1_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_1_PerfMonCtr7_Lo
pub const UMCCH2_1_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_1_PerfMonCtr7_Hi
pub const UMCCH2_1_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_1_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_1_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_1_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_1_PerfMonCtl8
pub const UMCCH2_1_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_1_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_1_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_1_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_1_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_1_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_1_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_1_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_1_PerfMonCtr8_Lo
pub const UMCCH2_1_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_1_PerfMonCtr8_Hi
pub const UMCCH2_1_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_1_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_1_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_1_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc1_umcch3_umcchdec
//UMCCH3_1_BaseAddrCS0
pub const UMCCH3_1_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_1_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH3_1_AddrMaskCS01
pub const UMCCH3_1_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH3_1_AddrSelCS01
pub const UMCCH3_1_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH3_1_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH3_1_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH3_1_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH3_1_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH3_1_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH3_1_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH3_1_AddrHashBank0
pub const UMCCH3_1_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_1_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH3_1_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH3_1_AddrHashBank1
pub const UMCCH3_1_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_1_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH3_1_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH3_1_AddrHashBank2
pub const UMCCH3_1_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_1_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH3_1_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH3_1_AddrHashBank3
pub const UMCCH3_1_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_1_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH3_1_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH3_1_AddrHashBank4
pub const UMCCH3_1_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_1_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH3_1_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH3_1_AddrHashBank5
pub const UMCCH3_1_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_1_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH3_1_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH3_1_EccErrCntSel
pub const UMCCH3_1_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH3_1_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH3_1_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH3_1_EccErrCnt
pub const UMCCH3_1_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH3_1_PerfMonCtlClk
pub const UMCCH3_1_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH3_1_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH3_1_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH3_1_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH3_1_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH3_1_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH3_1_PerfMonCtrClk_Lo
pub const UMCCH3_1_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_1_PerfMonCtrClk_Hi
pub const UMCCH3_1_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_1_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH3_1_PerfMonCtl1
pub const UMCCH3_1_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_1_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_1_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_1_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_1_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_1_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_1_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_1_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_1_PerfMonCtr1_Lo
pub const UMCCH3_1_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_1_PerfMonCtr1_Hi
pub const UMCCH3_1_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_1_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_1_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_1_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_1_PerfMonCtl2
pub const UMCCH3_1_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_1_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_1_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_1_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_1_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_1_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_1_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_1_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_1_PerfMonCtr2_Lo
pub const UMCCH3_1_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_1_PerfMonCtr2_Hi
pub const UMCCH3_1_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_1_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_1_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_1_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_1_PerfMonCtl3
pub const UMCCH3_1_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_1_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_1_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_1_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_1_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_1_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_1_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_1_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_1_PerfMonCtr3_Lo
pub const UMCCH3_1_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_1_PerfMonCtr3_Hi
pub const UMCCH3_1_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_1_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_1_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_1_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_1_PerfMonCtl4
pub const UMCCH3_1_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_1_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_1_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_1_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_1_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_1_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_1_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_1_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_1_PerfMonCtr4_Lo
pub const UMCCH3_1_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_1_PerfMonCtr4_Hi
pub const UMCCH3_1_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_1_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_1_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_1_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_1_PerfMonCtl5
pub const UMCCH3_1_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_1_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_1_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_1_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_1_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_1_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_1_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_1_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_1_PerfMonCtr5_Lo
pub const UMCCH3_1_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_1_PerfMonCtr5_Hi
pub const UMCCH3_1_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_1_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_1_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_1_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_1_PerfMonCtl6
pub const UMCCH3_1_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_1_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_1_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_1_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_1_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_1_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_1_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_1_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_1_PerfMonCtr6_Lo
pub const UMCCH3_1_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_1_PerfMonCtr6_Hi
pub const UMCCH3_1_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_1_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_1_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_1_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_1_PerfMonCtl7
pub const UMCCH3_1_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_1_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_1_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_1_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_1_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_1_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_1_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_1_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_1_PerfMonCtr7_Lo
pub const UMCCH3_1_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_1_PerfMonCtr7_Hi
pub const UMCCH3_1_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_1_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_1_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_1_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_1_PerfMonCtl8
pub const UMCCH3_1_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_1_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_1_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_1_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_1_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_1_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_1_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_1_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_1_PerfMonCtr8_Lo
pub const UMCCH3_1_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_1_PerfMonCtr8_Hi
pub const UMCCH3_1_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_1_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_1_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_1_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc1_umcch4_umcchdec
//UMCCH4_1_BaseAddrCS0
pub const UMCCH4_1_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_1_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH4_1_AddrMaskCS01
pub const UMCCH4_1_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH4_1_AddrSelCS01
pub const UMCCH4_1_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH4_1_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH4_1_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH4_1_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH4_1_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH4_1_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH4_1_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH4_1_AddrHashBank0
pub const UMCCH4_1_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_1_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH4_1_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH4_1_AddrHashBank1
pub const UMCCH4_1_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_1_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH4_1_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH4_1_AddrHashBank2
pub const UMCCH4_1_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_1_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH4_1_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH4_1_AddrHashBank3
pub const UMCCH4_1_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_1_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH4_1_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH4_1_AddrHashBank4
pub const UMCCH4_1_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_1_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH4_1_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH4_1_AddrHashBank5
pub const UMCCH4_1_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_1_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH4_1_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH4_1_EccErrCntSel
pub const UMCCH4_1_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH4_1_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH4_1_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH4_1_EccErrCnt
pub const UMCCH4_1_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH4_1_PerfMonCtlClk
pub const UMCCH4_1_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH4_1_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH4_1_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH4_1_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH4_1_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH4_1_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH4_1_PerfMonCtrClk_Lo
pub const UMCCH4_1_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_1_PerfMonCtrClk_Hi
pub const UMCCH4_1_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_1_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH4_1_PerfMonCtl1
pub const UMCCH4_1_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_1_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_1_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_1_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_1_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_1_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_1_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_1_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_1_PerfMonCtr1_Lo
pub const UMCCH4_1_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_1_PerfMonCtr1_Hi
pub const UMCCH4_1_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_1_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_1_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_1_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_1_PerfMonCtl2
pub const UMCCH4_1_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_1_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_1_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_1_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_1_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_1_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_1_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_1_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_1_PerfMonCtr2_Lo
pub const UMCCH4_1_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_1_PerfMonCtr2_Hi
pub const UMCCH4_1_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_1_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_1_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_1_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_1_PerfMonCtl3
pub const UMCCH4_1_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_1_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_1_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_1_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_1_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_1_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_1_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_1_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_1_PerfMonCtr3_Lo
pub const UMCCH4_1_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_1_PerfMonCtr3_Hi
pub const UMCCH4_1_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_1_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_1_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_1_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_1_PerfMonCtl4
pub const UMCCH4_1_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_1_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_1_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_1_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_1_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_1_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_1_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_1_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_1_PerfMonCtr4_Lo
pub const UMCCH4_1_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_1_PerfMonCtr4_Hi
pub const UMCCH4_1_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_1_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_1_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_1_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_1_PerfMonCtl5
pub const UMCCH4_1_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_1_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_1_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_1_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_1_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_1_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_1_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_1_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_1_PerfMonCtr5_Lo
pub const UMCCH4_1_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_1_PerfMonCtr5_Hi
pub const UMCCH4_1_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_1_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_1_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_1_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_1_PerfMonCtl6
pub const UMCCH4_1_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_1_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_1_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_1_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_1_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_1_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_1_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_1_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_1_PerfMonCtr6_Lo
pub const UMCCH4_1_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_1_PerfMonCtr6_Hi
pub const UMCCH4_1_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_1_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_1_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_1_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_1_PerfMonCtl7
pub const UMCCH4_1_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_1_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_1_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_1_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_1_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_1_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_1_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_1_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_1_PerfMonCtr7_Lo
pub const UMCCH4_1_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_1_PerfMonCtr7_Hi
pub const UMCCH4_1_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_1_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_1_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_1_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_1_PerfMonCtl8
pub const UMCCH4_1_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_1_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_1_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_1_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_1_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_1_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_1_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_1_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_1_PerfMonCtr8_Lo
pub const UMCCH4_1_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_1_PerfMonCtr8_Hi
pub const UMCCH4_1_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_1_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_1_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_1_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc1_umcch5_umcchdec
//UMCCH5_1_BaseAddrCS0
pub const UMCCH5_1_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_1_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH5_1_AddrMaskCS01
pub const UMCCH5_1_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH5_1_AddrSelCS01
pub const UMCCH5_1_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH5_1_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH5_1_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH5_1_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH5_1_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH5_1_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH5_1_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH5_1_AddrHashBank0
pub const UMCCH5_1_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_1_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH5_1_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH5_1_AddrHashBank1
pub const UMCCH5_1_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_1_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH5_1_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH5_1_AddrHashBank2
pub const UMCCH5_1_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_1_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH5_1_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH5_1_AddrHashBank3
pub const UMCCH5_1_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_1_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH5_1_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH5_1_AddrHashBank4
pub const UMCCH5_1_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_1_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH5_1_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH5_1_AddrHashBank5
pub const UMCCH5_1_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_1_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH5_1_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH5_1_EccErrCntSel
pub const UMCCH5_1_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH5_1_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH5_1_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH5_1_EccErrCnt
pub const UMCCH5_1_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH5_1_PerfMonCtlClk
pub const UMCCH5_1_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH5_1_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH5_1_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH5_1_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH5_1_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH5_1_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH5_1_PerfMonCtrClk_Lo
pub const UMCCH5_1_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_1_PerfMonCtrClk_Hi
pub const UMCCH5_1_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_1_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH5_1_PerfMonCtl1
pub const UMCCH5_1_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_1_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_1_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_1_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_1_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_1_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_1_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_1_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_1_PerfMonCtr1_Lo
pub const UMCCH5_1_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_1_PerfMonCtr1_Hi
pub const UMCCH5_1_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_1_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_1_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_1_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_1_PerfMonCtl2
pub const UMCCH5_1_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_1_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_1_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_1_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_1_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_1_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_1_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_1_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_1_PerfMonCtr2_Lo
pub const UMCCH5_1_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_1_PerfMonCtr2_Hi
pub const UMCCH5_1_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_1_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_1_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_1_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_1_PerfMonCtl3
pub const UMCCH5_1_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_1_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_1_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_1_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_1_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_1_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_1_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_1_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_1_PerfMonCtr3_Lo
pub const UMCCH5_1_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_1_PerfMonCtr3_Hi
pub const UMCCH5_1_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_1_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_1_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_1_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_1_PerfMonCtl4
pub const UMCCH5_1_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_1_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_1_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_1_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_1_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_1_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_1_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_1_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_1_PerfMonCtr4_Lo
pub const UMCCH5_1_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_1_PerfMonCtr4_Hi
pub const UMCCH5_1_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_1_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_1_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_1_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_1_PerfMonCtl5
pub const UMCCH5_1_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_1_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_1_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_1_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_1_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_1_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_1_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_1_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_1_PerfMonCtr5_Lo
pub const UMCCH5_1_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_1_PerfMonCtr5_Hi
pub const UMCCH5_1_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_1_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_1_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_1_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_1_PerfMonCtl6
pub const UMCCH5_1_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_1_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_1_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_1_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_1_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_1_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_1_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_1_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_1_PerfMonCtr6_Lo
pub const UMCCH5_1_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_1_PerfMonCtr6_Hi
pub const UMCCH5_1_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_1_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_1_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_1_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_1_PerfMonCtl7
pub const UMCCH5_1_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_1_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_1_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_1_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_1_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_1_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_1_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_1_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_1_PerfMonCtr7_Lo
pub const UMCCH5_1_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_1_PerfMonCtr7_Hi
pub const UMCCH5_1_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_1_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_1_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_1_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_1_PerfMonCtl8
pub const UMCCH5_1_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_1_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_1_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_1_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_1_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_1_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_1_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_1_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_1_PerfMonCtr8_Lo
pub const UMCCH5_1_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_1_PerfMonCtr8_Hi
pub const UMCCH5_1_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_1_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_1_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_1_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc1_umcch6_umcchdec
//UMCCH6_1_BaseAddrCS0
pub const UMCCH6_1_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_1_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH6_1_AddrMaskCS01
pub const UMCCH6_1_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH6_1_AddrSelCS01
pub const UMCCH6_1_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH6_1_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH6_1_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH6_1_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH6_1_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH6_1_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH6_1_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH6_1_AddrHashBank0
pub const UMCCH6_1_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_1_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH6_1_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH6_1_AddrHashBank1
pub const UMCCH6_1_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_1_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH6_1_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH6_1_AddrHashBank2
pub const UMCCH6_1_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_1_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH6_1_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH6_1_AddrHashBank3
pub const UMCCH6_1_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_1_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH6_1_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH6_1_AddrHashBank4
pub const UMCCH6_1_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_1_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH6_1_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH6_1_AddrHashBank5
pub const UMCCH6_1_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_1_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH6_1_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH6_1_EccErrCntSel
pub const UMCCH6_1_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH6_1_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH6_1_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH6_1_EccErrCnt
pub const UMCCH6_1_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH6_1_PerfMonCtlClk
pub const UMCCH6_1_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH6_1_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH6_1_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH6_1_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH6_1_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH6_1_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH6_1_PerfMonCtrClk_Lo
pub const UMCCH6_1_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_1_PerfMonCtrClk_Hi
pub const UMCCH6_1_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_1_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH6_1_PerfMonCtl1
pub const UMCCH6_1_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_1_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_1_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_1_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_1_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_1_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_1_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_1_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_1_PerfMonCtr1_Lo
pub const UMCCH6_1_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_1_PerfMonCtr1_Hi
pub const UMCCH6_1_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_1_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_1_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_1_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_1_PerfMonCtl2
pub const UMCCH6_1_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_1_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_1_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_1_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_1_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_1_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_1_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_1_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_1_PerfMonCtr2_Lo
pub const UMCCH6_1_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_1_PerfMonCtr2_Hi
pub const UMCCH6_1_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_1_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_1_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_1_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_1_PerfMonCtl3
pub const UMCCH6_1_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_1_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_1_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_1_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_1_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_1_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_1_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_1_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_1_PerfMonCtr3_Lo
pub const UMCCH6_1_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_1_PerfMonCtr3_Hi
pub const UMCCH6_1_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_1_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_1_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_1_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_1_PerfMonCtl4
pub const UMCCH6_1_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_1_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_1_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_1_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_1_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_1_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_1_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_1_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_1_PerfMonCtr4_Lo
pub const UMCCH6_1_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_1_PerfMonCtr4_Hi
pub const UMCCH6_1_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_1_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_1_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_1_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_1_PerfMonCtl5
pub const UMCCH6_1_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_1_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_1_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_1_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_1_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_1_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_1_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_1_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_1_PerfMonCtr5_Lo
pub const UMCCH6_1_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_1_PerfMonCtr5_Hi
pub const UMCCH6_1_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_1_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_1_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_1_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_1_PerfMonCtl6
pub const UMCCH6_1_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_1_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_1_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_1_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_1_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_1_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_1_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_1_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_1_PerfMonCtr6_Lo
pub const UMCCH6_1_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_1_PerfMonCtr6_Hi
pub const UMCCH6_1_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_1_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_1_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_1_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_1_PerfMonCtl7
pub const UMCCH6_1_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_1_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_1_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_1_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_1_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_1_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_1_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_1_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_1_PerfMonCtr7_Lo
pub const UMCCH6_1_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_1_PerfMonCtr7_Hi
pub const UMCCH6_1_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_1_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_1_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_1_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_1_PerfMonCtl8
pub const UMCCH6_1_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_1_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_1_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_1_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_1_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_1_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_1_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_1_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_1_PerfMonCtr8_Lo
pub const UMCCH6_1_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_1_PerfMonCtr8_Hi
pub const UMCCH6_1_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_1_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_1_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_1_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc1_umcch7_umcchdec
//UMCCH7_1_BaseAddrCS0
pub const UMCCH7_1_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_1_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH7_1_AddrMaskCS01
pub const UMCCH7_1_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH7_1_AddrSelCS01
pub const UMCCH7_1_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH7_1_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH7_1_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH7_1_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH7_1_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH7_1_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH7_1_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH7_1_AddrHashBank0
pub const UMCCH7_1_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_1_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH7_1_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH7_1_AddrHashBank1
pub const UMCCH7_1_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_1_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH7_1_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH7_1_AddrHashBank2
pub const UMCCH7_1_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_1_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH7_1_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH7_1_AddrHashBank3
pub const UMCCH7_1_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_1_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH7_1_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH7_1_AddrHashBank4
pub const UMCCH7_1_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_1_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH7_1_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH7_1_AddrHashBank5
pub const UMCCH7_1_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_1_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH7_1_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH7_1_EccErrCntSel
pub const UMCCH7_1_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH7_1_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH7_1_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH7_1_EccErrCnt
pub const UMCCH7_1_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH7_1_PerfMonCtlClk
pub const UMCCH7_1_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH7_1_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH7_1_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH7_1_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH7_1_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH7_1_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH7_1_PerfMonCtrClk_Lo
pub const UMCCH7_1_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_1_PerfMonCtrClk_Hi
pub const UMCCH7_1_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_1_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH7_1_PerfMonCtl1
pub const UMCCH7_1_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_1_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_1_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_1_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_1_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_1_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_1_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_1_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_1_PerfMonCtr1_Lo
pub const UMCCH7_1_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_1_PerfMonCtr1_Hi
pub const UMCCH7_1_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_1_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_1_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_1_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_1_PerfMonCtl2
pub const UMCCH7_1_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_1_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_1_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_1_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_1_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_1_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_1_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_1_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_1_PerfMonCtr2_Lo
pub const UMCCH7_1_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_1_PerfMonCtr2_Hi
pub const UMCCH7_1_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_1_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_1_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_1_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_1_PerfMonCtl3
pub const UMCCH7_1_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_1_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_1_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_1_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_1_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_1_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_1_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_1_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_1_PerfMonCtr3_Lo
pub const UMCCH7_1_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_1_PerfMonCtr3_Hi
pub const UMCCH7_1_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_1_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_1_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_1_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_1_PerfMonCtl4
pub const UMCCH7_1_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_1_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_1_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_1_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_1_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_1_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_1_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_1_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_1_PerfMonCtr4_Lo
pub const UMCCH7_1_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_1_PerfMonCtr4_Hi
pub const UMCCH7_1_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_1_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_1_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_1_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_1_PerfMonCtl5
pub const UMCCH7_1_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_1_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_1_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_1_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_1_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_1_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_1_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_1_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_1_PerfMonCtr5_Lo
pub const UMCCH7_1_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_1_PerfMonCtr5_Hi
pub const UMCCH7_1_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_1_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_1_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_1_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_1_PerfMonCtl6
pub const UMCCH7_1_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_1_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_1_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_1_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_1_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_1_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_1_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_1_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_1_PerfMonCtr6_Lo
pub const UMCCH7_1_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_1_PerfMonCtr6_Hi
pub const UMCCH7_1_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_1_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_1_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_1_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_1_PerfMonCtl7
pub const UMCCH7_1_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_1_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_1_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_1_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_1_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_1_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_1_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_1_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_1_PerfMonCtr7_Lo
pub const UMCCH7_1_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_1_PerfMonCtr7_Hi
pub const UMCCH7_1_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_1_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_1_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_1_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_1_PerfMonCtl8
pub const UMCCH7_1_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_1_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_1_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_1_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_1_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_1_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_1_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_1_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_1_PerfMonCtr8_Lo
pub const UMCCH7_1_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_1_PerfMonCtr8_Hi
pub const UMCCH7_1_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_1_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_1_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_1_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc2_umcch0_umcchdec
//UMCCH0_2_BaseAddrCS0
pub const UMCCH0_2_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_2_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH0_2_AddrMaskCS01
pub const UMCCH0_2_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH0_2_AddrSelCS01
pub const UMCCH0_2_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH0_2_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH0_2_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH0_2_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH0_2_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH0_2_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH0_2_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH0_2_AddrHashBank0
pub const UMCCH0_2_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_2_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH0_2_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH0_2_AddrHashBank1
pub const UMCCH0_2_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_2_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH0_2_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH0_2_AddrHashBank2
pub const UMCCH0_2_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_2_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH0_2_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH0_2_AddrHashBank3
pub const UMCCH0_2_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_2_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH0_2_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH0_2_AddrHashBank4
pub const UMCCH0_2_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_2_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH0_2_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH0_2_AddrHashBank5
pub const UMCCH0_2_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_2_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH0_2_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH0_2_EccErrCntSel
pub const UMCCH0_2_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH0_2_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH0_2_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH0_2_EccErrCnt
pub const UMCCH0_2_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH0_2_PerfMonCtlClk
pub const UMCCH0_2_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH0_2_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH0_2_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH0_2_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH0_2_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH0_2_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH0_2_PerfMonCtrClk_Lo
pub const UMCCH0_2_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_2_PerfMonCtrClk_Hi
pub const UMCCH0_2_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_2_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH0_2_PerfMonCtl1
pub const UMCCH0_2_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_2_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_2_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_2_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_2_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_2_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_2_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_2_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_2_PerfMonCtr1_Lo
pub const UMCCH0_2_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_2_PerfMonCtr1_Hi
pub const UMCCH0_2_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_2_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_2_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_2_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_2_PerfMonCtl2
pub const UMCCH0_2_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_2_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_2_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_2_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_2_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_2_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_2_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_2_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_2_PerfMonCtr2_Lo
pub const UMCCH0_2_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_2_PerfMonCtr2_Hi
pub const UMCCH0_2_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_2_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_2_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_2_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_2_PerfMonCtl3
pub const UMCCH0_2_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_2_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_2_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_2_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_2_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_2_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_2_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_2_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_2_PerfMonCtr3_Lo
pub const UMCCH0_2_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_2_PerfMonCtr3_Hi
pub const UMCCH0_2_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_2_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_2_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_2_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_2_PerfMonCtl4
pub const UMCCH0_2_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_2_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_2_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_2_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_2_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_2_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_2_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_2_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_2_PerfMonCtr4_Lo
pub const UMCCH0_2_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_2_PerfMonCtr4_Hi
pub const UMCCH0_2_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_2_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_2_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_2_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_2_PerfMonCtl5
pub const UMCCH0_2_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_2_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_2_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_2_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_2_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_2_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_2_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_2_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_2_PerfMonCtr5_Lo
pub const UMCCH0_2_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_2_PerfMonCtr5_Hi
pub const UMCCH0_2_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_2_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_2_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_2_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_2_PerfMonCtl6
pub const UMCCH0_2_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_2_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_2_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_2_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_2_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_2_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_2_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_2_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_2_PerfMonCtr6_Lo
pub const UMCCH0_2_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_2_PerfMonCtr6_Hi
pub const UMCCH0_2_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_2_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_2_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_2_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_2_PerfMonCtl7
pub const UMCCH0_2_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_2_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_2_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_2_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_2_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_2_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_2_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_2_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_2_PerfMonCtr7_Lo
pub const UMCCH0_2_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_2_PerfMonCtr7_Hi
pub const UMCCH0_2_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_2_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_2_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_2_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_2_PerfMonCtl8
pub const UMCCH0_2_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_2_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_2_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_2_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_2_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_2_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_2_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_2_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_2_PerfMonCtr8_Lo
pub const UMCCH0_2_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_2_PerfMonCtr8_Hi
pub const UMCCH0_2_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_2_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_2_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_2_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc2_umcch1_umcchdec
//UMCCH1_2_BaseAddrCS0
pub const UMCCH1_2_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_2_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH1_2_AddrMaskCS01
pub const UMCCH1_2_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH1_2_AddrSelCS01
pub const UMCCH1_2_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH1_2_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH1_2_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH1_2_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH1_2_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH1_2_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH1_2_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH1_2_AddrHashBank0
pub const UMCCH1_2_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_2_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH1_2_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH1_2_AddrHashBank1
pub const UMCCH1_2_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_2_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH1_2_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH1_2_AddrHashBank2
pub const UMCCH1_2_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_2_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH1_2_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH1_2_AddrHashBank3
pub const UMCCH1_2_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_2_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH1_2_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH1_2_AddrHashBank4
pub const UMCCH1_2_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_2_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH1_2_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH1_2_AddrHashBank5
pub const UMCCH1_2_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_2_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH1_2_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH1_2_EccErrCntSel
pub const UMCCH1_2_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH1_2_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH1_2_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH1_2_EccErrCnt
pub const UMCCH1_2_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH1_2_PerfMonCtlClk
pub const UMCCH1_2_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH1_2_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH1_2_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH1_2_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH1_2_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH1_2_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH1_2_PerfMonCtrClk_Lo
pub const UMCCH1_2_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_2_PerfMonCtrClk_Hi
pub const UMCCH1_2_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_2_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH1_2_PerfMonCtl1
pub const UMCCH1_2_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_2_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_2_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_2_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_2_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_2_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_2_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_2_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_2_PerfMonCtr1_Lo
pub const UMCCH1_2_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_2_PerfMonCtr1_Hi
pub const UMCCH1_2_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_2_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_2_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_2_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_2_PerfMonCtl2
pub const UMCCH1_2_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_2_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_2_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_2_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_2_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_2_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_2_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_2_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_2_PerfMonCtr2_Lo
pub const UMCCH1_2_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_2_PerfMonCtr2_Hi
pub const UMCCH1_2_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_2_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_2_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_2_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_2_PerfMonCtl3
pub const UMCCH1_2_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_2_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_2_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_2_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_2_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_2_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_2_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_2_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_2_PerfMonCtr3_Lo
pub const UMCCH1_2_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_2_PerfMonCtr3_Hi
pub const UMCCH1_2_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_2_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_2_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_2_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_2_PerfMonCtl4
pub const UMCCH1_2_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_2_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_2_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_2_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_2_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_2_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_2_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_2_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_2_PerfMonCtr4_Lo
pub const UMCCH1_2_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_2_PerfMonCtr4_Hi
pub const UMCCH1_2_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_2_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_2_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_2_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_2_PerfMonCtl5
pub const UMCCH1_2_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_2_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_2_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_2_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_2_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_2_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_2_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_2_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_2_PerfMonCtr5_Lo
pub const UMCCH1_2_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_2_PerfMonCtr5_Hi
pub const UMCCH1_2_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_2_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_2_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_2_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_2_PerfMonCtl6
pub const UMCCH1_2_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_2_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_2_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_2_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_2_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_2_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_2_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_2_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_2_PerfMonCtr6_Lo
pub const UMCCH1_2_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_2_PerfMonCtr6_Hi
pub const UMCCH1_2_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_2_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_2_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_2_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_2_PerfMonCtl7
pub const UMCCH1_2_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_2_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_2_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_2_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_2_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_2_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_2_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_2_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_2_PerfMonCtr7_Lo
pub const UMCCH1_2_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_2_PerfMonCtr7_Hi
pub const UMCCH1_2_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_2_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_2_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_2_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_2_PerfMonCtl8
pub const UMCCH1_2_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_2_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_2_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_2_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_2_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_2_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_2_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_2_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_2_PerfMonCtr8_Lo
pub const UMCCH1_2_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_2_PerfMonCtr8_Hi
pub const UMCCH1_2_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_2_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_2_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_2_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc2_umcch2_umcchdec
//UMCCH2_2_BaseAddrCS0
pub const UMCCH2_2_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_2_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH2_2_AddrMaskCS01
pub const UMCCH2_2_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH2_2_AddrSelCS01
pub const UMCCH2_2_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH2_2_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH2_2_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH2_2_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH2_2_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH2_2_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH2_2_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH2_2_AddrHashBank0
pub const UMCCH2_2_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_2_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH2_2_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH2_2_AddrHashBank1
pub const UMCCH2_2_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_2_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH2_2_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH2_2_AddrHashBank2
pub const UMCCH2_2_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_2_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH2_2_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH2_2_AddrHashBank3
pub const UMCCH2_2_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_2_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH2_2_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH2_2_AddrHashBank4
pub const UMCCH2_2_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_2_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH2_2_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH2_2_AddrHashBank5
pub const UMCCH2_2_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_2_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH2_2_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH2_2_EccErrCntSel
pub const UMCCH2_2_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH2_2_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH2_2_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH2_2_EccErrCnt
pub const UMCCH2_2_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH2_2_PerfMonCtlClk
pub const UMCCH2_2_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH2_2_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH2_2_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH2_2_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH2_2_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH2_2_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH2_2_PerfMonCtrClk_Lo
pub const UMCCH2_2_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_2_PerfMonCtrClk_Hi
pub const UMCCH2_2_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_2_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH2_2_PerfMonCtl1
pub const UMCCH2_2_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_2_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_2_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_2_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_2_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_2_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_2_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_2_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_2_PerfMonCtr1_Lo
pub const UMCCH2_2_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_2_PerfMonCtr1_Hi
pub const UMCCH2_2_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_2_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_2_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_2_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_2_PerfMonCtl2
pub const UMCCH2_2_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_2_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_2_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_2_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_2_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_2_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_2_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_2_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_2_PerfMonCtr2_Lo
pub const UMCCH2_2_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_2_PerfMonCtr2_Hi
pub const UMCCH2_2_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_2_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_2_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_2_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_2_PerfMonCtl3
pub const UMCCH2_2_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_2_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_2_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_2_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_2_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_2_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_2_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_2_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_2_PerfMonCtr3_Lo
pub const UMCCH2_2_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_2_PerfMonCtr3_Hi
pub const UMCCH2_2_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_2_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_2_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_2_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_2_PerfMonCtl4
pub const UMCCH2_2_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_2_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_2_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_2_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_2_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_2_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_2_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_2_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_2_PerfMonCtr4_Lo
pub const UMCCH2_2_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_2_PerfMonCtr4_Hi
pub const UMCCH2_2_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_2_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_2_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_2_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_2_PerfMonCtl5
pub const UMCCH2_2_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_2_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_2_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_2_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_2_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_2_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_2_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_2_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_2_PerfMonCtr5_Lo
pub const UMCCH2_2_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_2_PerfMonCtr5_Hi
pub const UMCCH2_2_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_2_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_2_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_2_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_2_PerfMonCtl6
pub const UMCCH2_2_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_2_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_2_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_2_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_2_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_2_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_2_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_2_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_2_PerfMonCtr6_Lo
pub const UMCCH2_2_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_2_PerfMonCtr6_Hi
pub const UMCCH2_2_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_2_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_2_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_2_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_2_PerfMonCtl7
pub const UMCCH2_2_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_2_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_2_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_2_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_2_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_2_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_2_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_2_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_2_PerfMonCtr7_Lo
pub const UMCCH2_2_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_2_PerfMonCtr7_Hi
pub const UMCCH2_2_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_2_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_2_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_2_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_2_PerfMonCtl8
pub const UMCCH2_2_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_2_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_2_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_2_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_2_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_2_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_2_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_2_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_2_PerfMonCtr8_Lo
pub const UMCCH2_2_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_2_PerfMonCtr8_Hi
pub const UMCCH2_2_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_2_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_2_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_2_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc2_umcch3_umcchdec
//UMCCH3_2_BaseAddrCS0
pub const UMCCH3_2_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_2_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH3_2_AddrMaskCS01
pub const UMCCH3_2_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH3_2_AddrSelCS01
pub const UMCCH3_2_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH3_2_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH3_2_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH3_2_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH3_2_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH3_2_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH3_2_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH3_2_AddrHashBank0
pub const UMCCH3_2_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_2_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH3_2_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH3_2_AddrHashBank1
pub const UMCCH3_2_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_2_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH3_2_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH3_2_AddrHashBank2
pub const UMCCH3_2_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_2_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH3_2_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH3_2_AddrHashBank3
pub const UMCCH3_2_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_2_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH3_2_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH3_2_AddrHashBank4
pub const UMCCH3_2_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_2_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH3_2_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH3_2_AddrHashBank5
pub const UMCCH3_2_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_2_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH3_2_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH3_2_EccErrCntSel
pub const UMCCH3_2_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH3_2_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH3_2_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH3_2_EccErrCnt
pub const UMCCH3_2_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH3_2_PerfMonCtlClk
pub const UMCCH3_2_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH3_2_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH3_2_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH3_2_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH3_2_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH3_2_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH3_2_PerfMonCtrClk_Lo
pub const UMCCH3_2_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_2_PerfMonCtrClk_Hi
pub const UMCCH3_2_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_2_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH3_2_PerfMonCtl1
pub const UMCCH3_2_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_2_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_2_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_2_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_2_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_2_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_2_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_2_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_2_PerfMonCtr1_Lo
pub const UMCCH3_2_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_2_PerfMonCtr1_Hi
pub const UMCCH3_2_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_2_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_2_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_2_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_2_PerfMonCtl2
pub const UMCCH3_2_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_2_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_2_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_2_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_2_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_2_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_2_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_2_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_2_PerfMonCtr2_Lo
pub const UMCCH3_2_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_2_PerfMonCtr2_Hi
pub const UMCCH3_2_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_2_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_2_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_2_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_2_PerfMonCtl3
pub const UMCCH3_2_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_2_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_2_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_2_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_2_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_2_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_2_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_2_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_2_PerfMonCtr3_Lo
pub const UMCCH3_2_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_2_PerfMonCtr3_Hi
pub const UMCCH3_2_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_2_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_2_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_2_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_2_PerfMonCtl4
pub const UMCCH3_2_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_2_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_2_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_2_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_2_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_2_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_2_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_2_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_2_PerfMonCtr4_Lo
pub const UMCCH3_2_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_2_PerfMonCtr4_Hi
pub const UMCCH3_2_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_2_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_2_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_2_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_2_PerfMonCtl5
pub const UMCCH3_2_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_2_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_2_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_2_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_2_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_2_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_2_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_2_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_2_PerfMonCtr5_Lo
pub const UMCCH3_2_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_2_PerfMonCtr5_Hi
pub const UMCCH3_2_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_2_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_2_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_2_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_2_PerfMonCtl6
pub const UMCCH3_2_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_2_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_2_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_2_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_2_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_2_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_2_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_2_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_2_PerfMonCtr6_Lo
pub const UMCCH3_2_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_2_PerfMonCtr6_Hi
pub const UMCCH3_2_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_2_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_2_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_2_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_2_PerfMonCtl7
pub const UMCCH3_2_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_2_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_2_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_2_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_2_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_2_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_2_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_2_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_2_PerfMonCtr7_Lo
pub const UMCCH3_2_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_2_PerfMonCtr7_Hi
pub const UMCCH3_2_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_2_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_2_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_2_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_2_PerfMonCtl8
pub const UMCCH3_2_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_2_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_2_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_2_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_2_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_2_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_2_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_2_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_2_PerfMonCtr8_Lo
pub const UMCCH3_2_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_2_PerfMonCtr8_Hi
pub const UMCCH3_2_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_2_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_2_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_2_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc2_umcch4_umcchdec
//UMCCH4_2_BaseAddrCS0
pub const UMCCH4_2_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_2_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH4_2_AddrMaskCS01
pub const UMCCH4_2_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH4_2_AddrSelCS01
pub const UMCCH4_2_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH4_2_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH4_2_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH4_2_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH4_2_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH4_2_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH4_2_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH4_2_AddrHashBank0
pub const UMCCH4_2_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_2_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH4_2_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH4_2_AddrHashBank1
pub const UMCCH4_2_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_2_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH4_2_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH4_2_AddrHashBank2
pub const UMCCH4_2_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_2_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH4_2_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH4_2_AddrHashBank3
pub const UMCCH4_2_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_2_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH4_2_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH4_2_AddrHashBank4
pub const UMCCH4_2_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_2_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH4_2_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH4_2_AddrHashBank5
pub const UMCCH4_2_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_2_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH4_2_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH4_2_EccErrCntSel
pub const UMCCH4_2_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH4_2_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH4_2_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH4_2_EccErrCnt
pub const UMCCH4_2_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH4_2_PerfMonCtlClk
pub const UMCCH4_2_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH4_2_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH4_2_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH4_2_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH4_2_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH4_2_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH4_2_PerfMonCtrClk_Lo
pub const UMCCH4_2_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_2_PerfMonCtrClk_Hi
pub const UMCCH4_2_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_2_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH4_2_PerfMonCtl1
pub const UMCCH4_2_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_2_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_2_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_2_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_2_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_2_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_2_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_2_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_2_PerfMonCtr1_Lo
pub const UMCCH4_2_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_2_PerfMonCtr1_Hi
pub const UMCCH4_2_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_2_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_2_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_2_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_2_PerfMonCtl2
pub const UMCCH4_2_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_2_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_2_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_2_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_2_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_2_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_2_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_2_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_2_PerfMonCtr2_Lo
pub const UMCCH4_2_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_2_PerfMonCtr2_Hi
pub const UMCCH4_2_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_2_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_2_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_2_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_2_PerfMonCtl3
pub const UMCCH4_2_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_2_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_2_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_2_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_2_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_2_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_2_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_2_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_2_PerfMonCtr3_Lo
pub const UMCCH4_2_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_2_PerfMonCtr3_Hi
pub const UMCCH4_2_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_2_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_2_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_2_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_2_PerfMonCtl4
pub const UMCCH4_2_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_2_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_2_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_2_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_2_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_2_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_2_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_2_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_2_PerfMonCtr4_Lo
pub const UMCCH4_2_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_2_PerfMonCtr4_Hi
pub const UMCCH4_2_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_2_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_2_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_2_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_2_PerfMonCtl5
pub const UMCCH4_2_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_2_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_2_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_2_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_2_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_2_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_2_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_2_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_2_PerfMonCtr5_Lo
pub const UMCCH4_2_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_2_PerfMonCtr5_Hi
pub const UMCCH4_2_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_2_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_2_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_2_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_2_PerfMonCtl6
pub const UMCCH4_2_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_2_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_2_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_2_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_2_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_2_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_2_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_2_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_2_PerfMonCtr6_Lo
pub const UMCCH4_2_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_2_PerfMonCtr6_Hi
pub const UMCCH4_2_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_2_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_2_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_2_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_2_PerfMonCtl7
pub const UMCCH4_2_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_2_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_2_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_2_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_2_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_2_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_2_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_2_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_2_PerfMonCtr7_Lo
pub const UMCCH4_2_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_2_PerfMonCtr7_Hi
pub const UMCCH4_2_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_2_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_2_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_2_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_2_PerfMonCtl8
pub const UMCCH4_2_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_2_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_2_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_2_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_2_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_2_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_2_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_2_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_2_PerfMonCtr8_Lo
pub const UMCCH4_2_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_2_PerfMonCtr8_Hi
pub const UMCCH4_2_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_2_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_2_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_2_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc2_umcch5_umcchdec
//UMCCH5_2_BaseAddrCS0
pub const UMCCH5_2_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_2_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH5_2_AddrMaskCS01
pub const UMCCH5_2_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH5_2_AddrSelCS01
pub const UMCCH5_2_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH5_2_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH5_2_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH5_2_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH5_2_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH5_2_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH5_2_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH5_2_AddrHashBank0
pub const UMCCH5_2_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_2_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH5_2_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH5_2_AddrHashBank1
pub const UMCCH5_2_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_2_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH5_2_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH5_2_AddrHashBank2
pub const UMCCH5_2_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_2_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH5_2_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH5_2_AddrHashBank3
pub const UMCCH5_2_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_2_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH5_2_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH5_2_AddrHashBank4
pub const UMCCH5_2_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_2_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH5_2_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH5_2_AddrHashBank5
pub const UMCCH5_2_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_2_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH5_2_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH5_2_EccErrCntSel
pub const UMCCH5_2_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH5_2_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH5_2_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH5_2_EccErrCnt
pub const UMCCH5_2_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH5_2_PerfMonCtlClk
pub const UMCCH5_2_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH5_2_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH5_2_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH5_2_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH5_2_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH5_2_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH5_2_PerfMonCtrClk_Lo
pub const UMCCH5_2_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_2_PerfMonCtrClk_Hi
pub const UMCCH5_2_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_2_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH5_2_PerfMonCtl1
pub const UMCCH5_2_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_2_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_2_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_2_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_2_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_2_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_2_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_2_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_2_PerfMonCtr1_Lo
pub const UMCCH5_2_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_2_PerfMonCtr1_Hi
pub const UMCCH5_2_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_2_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_2_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_2_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_2_PerfMonCtl2
pub const UMCCH5_2_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_2_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_2_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_2_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_2_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_2_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_2_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_2_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_2_PerfMonCtr2_Lo
pub const UMCCH5_2_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_2_PerfMonCtr2_Hi
pub const UMCCH5_2_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_2_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_2_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_2_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_2_PerfMonCtl3
pub const UMCCH5_2_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_2_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_2_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_2_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_2_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_2_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_2_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_2_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_2_PerfMonCtr3_Lo
pub const UMCCH5_2_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_2_PerfMonCtr3_Hi
pub const UMCCH5_2_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_2_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_2_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_2_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_2_PerfMonCtl4
pub const UMCCH5_2_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_2_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_2_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_2_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_2_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_2_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_2_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_2_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_2_PerfMonCtr4_Lo
pub const UMCCH5_2_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_2_PerfMonCtr4_Hi
pub const UMCCH5_2_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_2_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_2_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_2_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_2_PerfMonCtl5
pub const UMCCH5_2_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_2_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_2_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_2_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_2_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_2_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_2_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_2_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_2_PerfMonCtr5_Lo
pub const UMCCH5_2_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_2_PerfMonCtr5_Hi
pub const UMCCH5_2_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_2_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_2_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_2_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_2_PerfMonCtl6
pub const UMCCH5_2_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_2_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_2_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_2_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_2_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_2_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_2_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_2_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_2_PerfMonCtr6_Lo
pub const UMCCH5_2_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_2_PerfMonCtr6_Hi
pub const UMCCH5_2_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_2_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_2_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_2_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_2_PerfMonCtl7
pub const UMCCH5_2_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_2_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_2_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_2_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_2_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_2_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_2_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_2_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_2_PerfMonCtr7_Lo
pub const UMCCH5_2_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_2_PerfMonCtr7_Hi
pub const UMCCH5_2_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_2_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_2_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_2_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_2_PerfMonCtl8
pub const UMCCH5_2_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_2_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_2_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_2_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_2_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_2_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_2_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_2_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_2_PerfMonCtr8_Lo
pub const UMCCH5_2_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_2_PerfMonCtr8_Hi
pub const UMCCH5_2_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_2_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_2_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_2_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc2_umcch6_umcchdec
//UMCCH6_2_BaseAddrCS0
pub const UMCCH6_2_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_2_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH6_2_AddrMaskCS01
pub const UMCCH6_2_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH6_2_AddrSelCS01
pub const UMCCH6_2_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH6_2_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH6_2_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH6_2_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH6_2_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH6_2_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH6_2_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH6_2_AddrHashBank0
pub const UMCCH6_2_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_2_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH6_2_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH6_2_AddrHashBank1
pub const UMCCH6_2_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_2_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH6_2_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH6_2_AddrHashBank2
pub const UMCCH6_2_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_2_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH6_2_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH6_2_AddrHashBank3
pub const UMCCH6_2_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_2_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH6_2_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH6_2_AddrHashBank4
pub const UMCCH6_2_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_2_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH6_2_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH6_2_AddrHashBank5
pub const UMCCH6_2_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_2_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH6_2_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH6_2_EccErrCntSel
pub const UMCCH6_2_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH6_2_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH6_2_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH6_2_EccErrCnt
pub const UMCCH6_2_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH6_2_PerfMonCtlClk
pub const UMCCH6_2_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH6_2_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH6_2_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH6_2_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH6_2_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH6_2_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH6_2_PerfMonCtrClk_Lo
pub const UMCCH6_2_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_2_PerfMonCtrClk_Hi
pub const UMCCH6_2_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_2_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH6_2_PerfMonCtl1
pub const UMCCH6_2_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_2_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_2_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_2_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_2_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_2_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_2_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_2_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_2_PerfMonCtr1_Lo
pub const UMCCH6_2_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_2_PerfMonCtr1_Hi
pub const UMCCH6_2_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_2_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_2_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_2_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_2_PerfMonCtl2
pub const UMCCH6_2_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_2_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_2_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_2_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_2_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_2_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_2_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_2_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_2_PerfMonCtr2_Lo
pub const UMCCH6_2_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_2_PerfMonCtr2_Hi
pub const UMCCH6_2_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_2_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_2_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_2_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_2_PerfMonCtl3
pub const UMCCH6_2_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_2_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_2_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_2_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_2_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_2_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_2_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_2_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_2_PerfMonCtr3_Lo
pub const UMCCH6_2_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_2_PerfMonCtr3_Hi
pub const UMCCH6_2_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_2_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_2_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_2_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_2_PerfMonCtl4
pub const UMCCH6_2_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_2_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_2_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_2_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_2_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_2_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_2_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_2_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_2_PerfMonCtr4_Lo
pub const UMCCH6_2_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_2_PerfMonCtr4_Hi
pub const UMCCH6_2_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_2_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_2_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_2_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_2_PerfMonCtl5
pub const UMCCH6_2_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_2_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_2_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_2_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_2_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_2_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_2_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_2_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_2_PerfMonCtr5_Lo
pub const UMCCH6_2_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_2_PerfMonCtr5_Hi
pub const UMCCH6_2_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_2_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_2_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_2_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_2_PerfMonCtl6
pub const UMCCH6_2_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_2_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_2_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_2_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_2_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_2_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_2_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_2_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_2_PerfMonCtr6_Lo
pub const UMCCH6_2_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_2_PerfMonCtr6_Hi
pub const UMCCH6_2_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_2_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_2_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_2_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_2_PerfMonCtl7
pub const UMCCH6_2_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_2_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_2_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_2_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_2_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_2_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_2_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_2_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_2_PerfMonCtr7_Lo
pub const UMCCH6_2_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_2_PerfMonCtr7_Hi
pub const UMCCH6_2_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_2_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_2_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_2_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_2_PerfMonCtl8
pub const UMCCH6_2_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_2_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_2_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_2_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_2_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_2_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_2_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_2_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_2_PerfMonCtr8_Lo
pub const UMCCH6_2_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_2_PerfMonCtr8_Hi
pub const UMCCH6_2_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_2_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_2_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_2_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc2_umcch7_umcchdec
//UMCCH7_2_BaseAddrCS0
pub const UMCCH7_2_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_2_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH7_2_AddrMaskCS01
pub const UMCCH7_2_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH7_2_AddrSelCS01
pub const UMCCH7_2_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH7_2_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH7_2_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH7_2_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH7_2_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH7_2_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH7_2_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH7_2_AddrHashBank0
pub const UMCCH7_2_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_2_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH7_2_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH7_2_AddrHashBank1
pub const UMCCH7_2_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_2_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH7_2_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH7_2_AddrHashBank2
pub const UMCCH7_2_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_2_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH7_2_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH7_2_AddrHashBank3
pub const UMCCH7_2_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_2_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH7_2_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH7_2_AddrHashBank4
pub const UMCCH7_2_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_2_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH7_2_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH7_2_AddrHashBank5
pub const UMCCH7_2_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_2_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH7_2_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH7_2_EccErrCntSel
pub const UMCCH7_2_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH7_2_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH7_2_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH7_2_EccErrCnt
pub const UMCCH7_2_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH7_2_PerfMonCtlClk
pub const UMCCH7_2_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH7_2_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH7_2_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH7_2_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH7_2_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH7_2_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH7_2_PerfMonCtrClk_Lo
pub const UMCCH7_2_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_2_PerfMonCtrClk_Hi
pub const UMCCH7_2_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_2_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH7_2_PerfMonCtl1
pub const UMCCH7_2_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_2_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_2_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_2_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_2_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_2_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_2_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_2_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_2_PerfMonCtr1_Lo
pub const UMCCH7_2_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_2_PerfMonCtr1_Hi
pub const UMCCH7_2_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_2_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_2_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_2_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_2_PerfMonCtl2
pub const UMCCH7_2_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_2_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_2_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_2_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_2_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_2_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_2_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_2_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_2_PerfMonCtr2_Lo
pub const UMCCH7_2_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_2_PerfMonCtr2_Hi
pub const UMCCH7_2_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_2_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_2_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_2_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_2_PerfMonCtl3
pub const UMCCH7_2_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_2_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_2_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_2_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_2_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_2_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_2_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_2_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_2_PerfMonCtr3_Lo
pub const UMCCH7_2_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_2_PerfMonCtr3_Hi
pub const UMCCH7_2_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_2_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_2_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_2_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_2_PerfMonCtl4
pub const UMCCH7_2_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_2_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_2_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_2_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_2_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_2_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_2_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_2_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_2_PerfMonCtr4_Lo
pub const UMCCH7_2_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_2_PerfMonCtr4_Hi
pub const UMCCH7_2_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_2_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_2_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_2_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_2_PerfMonCtl5
pub const UMCCH7_2_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_2_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_2_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_2_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_2_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_2_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_2_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_2_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_2_PerfMonCtr5_Lo
pub const UMCCH7_2_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_2_PerfMonCtr5_Hi
pub const UMCCH7_2_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_2_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_2_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_2_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_2_PerfMonCtl6
pub const UMCCH7_2_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_2_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_2_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_2_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_2_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_2_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_2_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_2_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_2_PerfMonCtr6_Lo
pub const UMCCH7_2_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_2_PerfMonCtr6_Hi
pub const UMCCH7_2_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_2_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_2_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_2_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_2_PerfMonCtl7
pub const UMCCH7_2_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_2_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_2_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_2_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_2_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_2_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_2_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_2_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_2_PerfMonCtr7_Lo
pub const UMCCH7_2_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_2_PerfMonCtr7_Hi
pub const UMCCH7_2_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_2_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_2_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_2_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_2_PerfMonCtl8
pub const UMCCH7_2_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_2_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_2_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_2_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_2_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_2_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_2_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_2_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_2_PerfMonCtr8_Lo
pub const UMCCH7_2_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_2_PerfMonCtr8_Hi
pub const UMCCH7_2_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_2_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_2_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_2_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc3_umcch0_umcchdec
//UMCCH0_3_BaseAddrCS0
pub const UMCCH0_3_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_3_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH0_3_AddrMaskCS01
pub const UMCCH0_3_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH0_3_AddrSelCS01
pub const UMCCH0_3_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH0_3_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH0_3_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH0_3_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH0_3_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH0_3_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH0_3_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH0_3_AddrHashBank0
pub const UMCCH0_3_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_3_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH0_3_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH0_3_AddrHashBank1
pub const UMCCH0_3_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_3_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH0_3_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH0_3_AddrHashBank2
pub const UMCCH0_3_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_3_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH0_3_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH0_3_AddrHashBank3
pub const UMCCH0_3_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_3_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH0_3_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH0_3_AddrHashBank4
pub const UMCCH0_3_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_3_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH0_3_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH0_3_AddrHashBank5
pub const UMCCH0_3_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH0_3_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH0_3_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH0_3_EccErrCntSel
pub const UMCCH0_3_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH0_3_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH0_3_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH0_3_EccErrCnt
pub const UMCCH0_3_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH0_3_PerfMonCtlClk
pub const UMCCH0_3_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH0_3_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH0_3_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH0_3_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH0_3_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH0_3_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH0_3_PerfMonCtrClk_Lo
pub const UMCCH0_3_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_3_PerfMonCtrClk_Hi
pub const UMCCH0_3_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_3_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH0_3_PerfMonCtl1
pub const UMCCH0_3_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_3_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_3_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_3_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_3_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_3_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_3_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_3_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_3_PerfMonCtr1_Lo
pub const UMCCH0_3_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_3_PerfMonCtr1_Hi
pub const UMCCH0_3_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_3_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_3_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_3_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_3_PerfMonCtl2
pub const UMCCH0_3_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_3_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_3_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_3_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_3_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_3_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_3_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_3_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_3_PerfMonCtr2_Lo
pub const UMCCH0_3_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_3_PerfMonCtr2_Hi
pub const UMCCH0_3_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_3_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_3_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_3_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_3_PerfMonCtl3
pub const UMCCH0_3_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_3_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_3_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_3_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_3_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_3_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_3_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_3_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_3_PerfMonCtr3_Lo
pub const UMCCH0_3_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_3_PerfMonCtr3_Hi
pub const UMCCH0_3_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_3_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_3_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_3_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_3_PerfMonCtl4
pub const UMCCH0_3_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_3_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_3_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_3_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_3_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_3_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_3_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_3_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_3_PerfMonCtr4_Lo
pub const UMCCH0_3_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_3_PerfMonCtr4_Hi
pub const UMCCH0_3_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_3_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_3_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_3_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_3_PerfMonCtl5
pub const UMCCH0_3_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_3_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_3_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_3_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_3_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_3_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_3_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_3_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_3_PerfMonCtr5_Lo
pub const UMCCH0_3_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_3_PerfMonCtr5_Hi
pub const UMCCH0_3_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_3_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_3_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_3_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_3_PerfMonCtl6
pub const UMCCH0_3_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_3_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_3_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_3_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_3_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_3_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_3_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_3_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_3_PerfMonCtr6_Lo
pub const UMCCH0_3_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_3_PerfMonCtr6_Hi
pub const UMCCH0_3_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_3_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_3_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_3_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_3_PerfMonCtl7
pub const UMCCH0_3_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_3_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_3_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_3_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_3_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_3_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_3_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_3_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_3_PerfMonCtr7_Lo
pub const UMCCH0_3_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_3_PerfMonCtr7_Hi
pub const UMCCH0_3_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_3_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_3_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_3_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH0_3_PerfMonCtl8
pub const UMCCH0_3_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH0_3_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH0_3_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH0_3_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH0_3_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH0_3_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH0_3_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH0_3_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH0_3_PerfMonCtr8_Lo
pub const UMCCH0_3_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH0_3_PerfMonCtr8_Hi
pub const UMCCH0_3_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH0_3_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH0_3_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH0_3_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc3_umcch1_umcchdec
//UMCCH1_3_BaseAddrCS0
pub const UMCCH1_3_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_3_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH1_3_AddrMaskCS01
pub const UMCCH1_3_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH1_3_AddrSelCS01
pub const UMCCH1_3_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH1_3_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH1_3_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH1_3_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH1_3_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH1_3_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH1_3_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH1_3_AddrHashBank0
pub const UMCCH1_3_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_3_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH1_3_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH1_3_AddrHashBank1
pub const UMCCH1_3_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_3_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH1_3_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH1_3_AddrHashBank2
pub const UMCCH1_3_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_3_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH1_3_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH1_3_AddrHashBank3
pub const UMCCH1_3_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_3_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH1_3_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH1_3_AddrHashBank4
pub const UMCCH1_3_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_3_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH1_3_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH1_3_AddrHashBank5
pub const UMCCH1_3_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH1_3_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH1_3_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH1_3_EccErrCntSel
pub const UMCCH1_3_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH1_3_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH1_3_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH1_3_EccErrCnt
pub const UMCCH1_3_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH1_3_PerfMonCtlClk
pub const UMCCH1_3_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH1_3_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH1_3_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH1_3_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH1_3_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH1_3_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH1_3_PerfMonCtrClk_Lo
pub const UMCCH1_3_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_3_PerfMonCtrClk_Hi
pub const UMCCH1_3_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_3_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH1_3_PerfMonCtl1
pub const UMCCH1_3_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_3_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_3_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_3_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_3_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_3_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_3_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_3_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_3_PerfMonCtr1_Lo
pub const UMCCH1_3_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_3_PerfMonCtr1_Hi
pub const UMCCH1_3_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_3_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_3_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_3_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_3_PerfMonCtl2
pub const UMCCH1_3_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_3_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_3_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_3_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_3_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_3_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_3_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_3_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_3_PerfMonCtr2_Lo
pub const UMCCH1_3_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_3_PerfMonCtr2_Hi
pub const UMCCH1_3_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_3_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_3_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_3_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_3_PerfMonCtl3
pub const UMCCH1_3_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_3_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_3_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_3_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_3_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_3_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_3_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_3_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_3_PerfMonCtr3_Lo
pub const UMCCH1_3_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_3_PerfMonCtr3_Hi
pub const UMCCH1_3_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_3_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_3_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_3_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_3_PerfMonCtl4
pub const UMCCH1_3_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_3_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_3_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_3_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_3_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_3_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_3_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_3_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_3_PerfMonCtr4_Lo
pub const UMCCH1_3_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_3_PerfMonCtr4_Hi
pub const UMCCH1_3_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_3_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_3_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_3_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_3_PerfMonCtl5
pub const UMCCH1_3_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_3_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_3_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_3_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_3_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_3_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_3_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_3_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_3_PerfMonCtr5_Lo
pub const UMCCH1_3_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_3_PerfMonCtr5_Hi
pub const UMCCH1_3_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_3_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_3_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_3_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_3_PerfMonCtl6
pub const UMCCH1_3_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_3_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_3_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_3_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_3_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_3_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_3_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_3_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_3_PerfMonCtr6_Lo
pub const UMCCH1_3_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_3_PerfMonCtr6_Hi
pub const UMCCH1_3_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_3_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_3_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_3_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_3_PerfMonCtl7
pub const UMCCH1_3_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_3_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_3_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_3_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_3_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_3_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_3_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_3_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_3_PerfMonCtr7_Lo
pub const UMCCH1_3_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_3_PerfMonCtr7_Hi
pub const UMCCH1_3_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_3_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_3_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_3_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH1_3_PerfMonCtl8
pub const UMCCH1_3_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH1_3_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH1_3_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH1_3_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH1_3_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH1_3_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH1_3_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH1_3_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH1_3_PerfMonCtr8_Lo
pub const UMCCH1_3_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH1_3_PerfMonCtr8_Hi
pub const UMCCH1_3_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH1_3_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH1_3_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH1_3_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc3_umcch2_umcchdec
//UMCCH2_3_BaseAddrCS0
pub const UMCCH2_3_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_3_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH2_3_AddrMaskCS01
pub const UMCCH2_3_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH2_3_AddrSelCS01
pub const UMCCH2_3_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH2_3_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH2_3_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH2_3_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH2_3_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH2_3_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH2_3_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH2_3_AddrHashBank0
pub const UMCCH2_3_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_3_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH2_3_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH2_3_AddrHashBank1
pub const UMCCH2_3_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_3_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH2_3_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH2_3_AddrHashBank2
pub const UMCCH2_3_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_3_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH2_3_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH2_3_AddrHashBank3
pub const UMCCH2_3_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_3_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH2_3_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH2_3_AddrHashBank4
pub const UMCCH2_3_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_3_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH2_3_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH2_3_AddrHashBank5
pub const UMCCH2_3_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH2_3_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH2_3_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH2_3_EccErrCntSel
pub const UMCCH2_3_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH2_3_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH2_3_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH2_3_EccErrCnt
pub const UMCCH2_3_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH2_3_PerfMonCtlClk
pub const UMCCH2_3_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH2_3_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH2_3_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH2_3_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH2_3_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH2_3_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH2_3_PerfMonCtrClk_Lo
pub const UMCCH2_3_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_3_PerfMonCtrClk_Hi
pub const UMCCH2_3_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_3_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH2_3_PerfMonCtl1
pub const UMCCH2_3_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_3_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_3_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_3_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_3_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_3_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_3_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_3_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_3_PerfMonCtr1_Lo
pub const UMCCH2_3_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_3_PerfMonCtr1_Hi
pub const UMCCH2_3_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_3_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_3_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_3_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_3_PerfMonCtl2
pub const UMCCH2_3_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_3_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_3_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_3_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_3_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_3_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_3_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_3_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_3_PerfMonCtr2_Lo
pub const UMCCH2_3_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_3_PerfMonCtr2_Hi
pub const UMCCH2_3_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_3_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_3_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_3_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_3_PerfMonCtl3
pub const UMCCH2_3_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_3_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_3_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_3_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_3_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_3_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_3_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_3_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_3_PerfMonCtr3_Lo
pub const UMCCH2_3_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_3_PerfMonCtr3_Hi
pub const UMCCH2_3_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_3_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_3_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_3_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_3_PerfMonCtl4
pub const UMCCH2_3_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_3_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_3_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_3_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_3_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_3_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_3_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_3_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_3_PerfMonCtr4_Lo
pub const UMCCH2_3_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_3_PerfMonCtr4_Hi
pub const UMCCH2_3_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_3_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_3_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_3_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_3_PerfMonCtl5
pub const UMCCH2_3_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_3_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_3_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_3_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_3_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_3_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_3_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_3_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_3_PerfMonCtr5_Lo
pub const UMCCH2_3_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_3_PerfMonCtr5_Hi
pub const UMCCH2_3_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_3_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_3_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_3_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_3_PerfMonCtl6
pub const UMCCH2_3_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_3_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_3_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_3_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_3_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_3_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_3_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_3_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_3_PerfMonCtr6_Lo
pub const UMCCH2_3_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_3_PerfMonCtr6_Hi
pub const UMCCH2_3_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_3_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_3_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_3_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_3_PerfMonCtl7
pub const UMCCH2_3_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_3_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_3_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_3_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_3_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_3_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_3_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_3_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_3_PerfMonCtr7_Lo
pub const UMCCH2_3_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_3_PerfMonCtr7_Hi
pub const UMCCH2_3_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_3_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_3_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_3_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH2_3_PerfMonCtl8
pub const UMCCH2_3_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH2_3_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH2_3_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH2_3_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH2_3_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH2_3_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH2_3_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH2_3_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH2_3_PerfMonCtr8_Lo
pub const UMCCH2_3_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH2_3_PerfMonCtr8_Hi
pub const UMCCH2_3_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH2_3_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH2_3_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH2_3_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc3_umcch3_umcchdec
//UMCCH3_3_BaseAddrCS0
pub const UMCCH3_3_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_3_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH3_3_AddrMaskCS01
pub const UMCCH3_3_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH3_3_AddrSelCS01
pub const UMCCH3_3_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH3_3_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH3_3_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH3_3_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH3_3_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH3_3_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH3_3_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH3_3_AddrHashBank0
pub const UMCCH3_3_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_3_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH3_3_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH3_3_AddrHashBank1
pub const UMCCH3_3_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_3_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH3_3_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH3_3_AddrHashBank2
pub const UMCCH3_3_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_3_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH3_3_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH3_3_AddrHashBank3
pub const UMCCH3_3_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_3_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH3_3_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH3_3_AddrHashBank4
pub const UMCCH3_3_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_3_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH3_3_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH3_3_AddrHashBank5
pub const UMCCH3_3_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH3_3_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH3_3_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH3_3_EccErrCntSel
pub const UMCCH3_3_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH3_3_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH3_3_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH3_3_EccErrCnt
pub const UMCCH3_3_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH3_3_PerfMonCtlClk
pub const UMCCH3_3_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH3_3_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH3_3_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH3_3_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH3_3_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH3_3_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH3_3_PerfMonCtrClk_Lo
pub const UMCCH3_3_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_3_PerfMonCtrClk_Hi
pub const UMCCH3_3_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_3_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH3_3_PerfMonCtl1
pub const UMCCH3_3_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_3_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_3_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_3_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_3_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_3_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_3_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_3_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_3_PerfMonCtr1_Lo
pub const UMCCH3_3_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_3_PerfMonCtr1_Hi
pub const UMCCH3_3_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_3_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_3_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_3_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_3_PerfMonCtl2
pub const UMCCH3_3_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_3_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_3_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_3_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_3_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_3_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_3_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_3_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_3_PerfMonCtr2_Lo
pub const UMCCH3_3_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_3_PerfMonCtr2_Hi
pub const UMCCH3_3_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_3_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_3_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_3_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_3_PerfMonCtl3
pub const UMCCH3_3_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_3_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_3_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_3_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_3_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_3_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_3_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_3_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_3_PerfMonCtr3_Lo
pub const UMCCH3_3_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_3_PerfMonCtr3_Hi
pub const UMCCH3_3_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_3_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_3_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_3_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_3_PerfMonCtl4
pub const UMCCH3_3_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_3_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_3_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_3_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_3_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_3_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_3_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_3_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_3_PerfMonCtr4_Lo
pub const UMCCH3_3_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_3_PerfMonCtr4_Hi
pub const UMCCH3_3_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_3_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_3_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_3_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_3_PerfMonCtl5
pub const UMCCH3_3_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_3_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_3_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_3_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_3_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_3_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_3_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_3_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_3_PerfMonCtr5_Lo
pub const UMCCH3_3_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_3_PerfMonCtr5_Hi
pub const UMCCH3_3_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_3_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_3_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_3_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_3_PerfMonCtl6
pub const UMCCH3_3_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_3_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_3_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_3_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_3_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_3_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_3_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_3_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_3_PerfMonCtr6_Lo
pub const UMCCH3_3_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_3_PerfMonCtr6_Hi
pub const UMCCH3_3_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_3_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_3_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_3_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_3_PerfMonCtl7
pub const UMCCH3_3_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_3_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_3_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_3_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_3_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_3_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_3_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_3_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_3_PerfMonCtr7_Lo
pub const UMCCH3_3_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_3_PerfMonCtr7_Hi
pub const UMCCH3_3_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_3_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_3_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_3_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH3_3_PerfMonCtl8
pub const UMCCH3_3_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH3_3_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH3_3_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH3_3_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH3_3_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH3_3_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH3_3_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH3_3_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH3_3_PerfMonCtr8_Lo
pub const UMCCH3_3_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH3_3_PerfMonCtr8_Hi
pub const UMCCH3_3_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH3_3_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH3_3_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH3_3_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc3_umcch4_umcchdec
//UMCCH4_3_BaseAddrCS0
pub const UMCCH4_3_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_3_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH4_3_AddrMaskCS01
pub const UMCCH4_3_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH4_3_AddrSelCS01
pub const UMCCH4_3_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH4_3_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH4_3_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH4_3_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH4_3_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH4_3_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH4_3_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH4_3_AddrHashBank0
pub const UMCCH4_3_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_3_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH4_3_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH4_3_AddrHashBank1
pub const UMCCH4_3_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_3_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH4_3_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH4_3_AddrHashBank2
pub const UMCCH4_3_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_3_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH4_3_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH4_3_AddrHashBank3
pub const UMCCH4_3_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_3_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH4_3_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH4_3_AddrHashBank4
pub const UMCCH4_3_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_3_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH4_3_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH4_3_AddrHashBank5
pub const UMCCH4_3_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH4_3_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH4_3_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH4_3_EccErrCntSel
pub const UMCCH4_3_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH4_3_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH4_3_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH4_3_EccErrCnt
pub const UMCCH4_3_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH4_3_PerfMonCtlClk
pub const UMCCH4_3_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH4_3_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH4_3_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH4_3_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH4_3_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH4_3_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH4_3_PerfMonCtrClk_Lo
pub const UMCCH4_3_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_3_PerfMonCtrClk_Hi
pub const UMCCH4_3_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_3_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH4_3_PerfMonCtl1
pub const UMCCH4_3_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_3_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_3_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_3_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_3_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_3_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_3_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_3_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_3_PerfMonCtr1_Lo
pub const UMCCH4_3_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_3_PerfMonCtr1_Hi
pub const UMCCH4_3_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_3_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_3_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_3_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_3_PerfMonCtl2
pub const UMCCH4_3_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_3_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_3_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_3_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_3_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_3_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_3_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_3_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_3_PerfMonCtr2_Lo
pub const UMCCH4_3_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_3_PerfMonCtr2_Hi
pub const UMCCH4_3_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_3_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_3_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_3_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_3_PerfMonCtl3
pub const UMCCH4_3_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_3_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_3_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_3_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_3_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_3_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_3_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_3_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_3_PerfMonCtr3_Lo
pub const UMCCH4_3_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_3_PerfMonCtr3_Hi
pub const UMCCH4_3_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_3_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_3_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_3_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_3_PerfMonCtl4
pub const UMCCH4_3_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_3_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_3_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_3_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_3_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_3_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_3_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_3_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_3_PerfMonCtr4_Lo
pub const UMCCH4_3_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_3_PerfMonCtr4_Hi
pub const UMCCH4_3_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_3_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_3_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_3_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_3_PerfMonCtl5
pub const UMCCH4_3_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_3_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_3_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_3_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_3_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_3_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_3_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_3_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_3_PerfMonCtr5_Lo
pub const UMCCH4_3_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_3_PerfMonCtr5_Hi
pub const UMCCH4_3_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_3_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_3_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_3_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_3_PerfMonCtl6
pub const UMCCH4_3_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_3_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_3_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_3_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_3_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_3_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_3_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_3_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_3_PerfMonCtr6_Lo
pub const UMCCH4_3_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_3_PerfMonCtr6_Hi
pub const UMCCH4_3_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_3_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_3_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_3_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_3_PerfMonCtl7
pub const UMCCH4_3_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_3_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_3_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_3_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_3_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_3_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_3_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_3_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_3_PerfMonCtr7_Lo
pub const UMCCH4_3_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_3_PerfMonCtr7_Hi
pub const UMCCH4_3_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_3_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_3_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_3_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH4_3_PerfMonCtl8
pub const UMCCH4_3_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH4_3_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH4_3_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH4_3_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH4_3_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH4_3_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH4_3_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH4_3_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH4_3_PerfMonCtr8_Lo
pub const UMCCH4_3_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH4_3_PerfMonCtr8_Hi
pub const UMCCH4_3_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH4_3_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH4_3_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH4_3_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc3_umcch5_umcchdec
//UMCCH5_3_BaseAddrCS0
pub const UMCCH5_3_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_3_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH5_3_AddrMaskCS01
pub const UMCCH5_3_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH5_3_AddrSelCS01
pub const UMCCH5_3_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH5_3_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH5_3_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH5_3_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH5_3_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH5_3_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH5_3_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH5_3_AddrHashBank0
pub const UMCCH5_3_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_3_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH5_3_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH5_3_AddrHashBank1
pub const UMCCH5_3_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_3_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH5_3_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH5_3_AddrHashBank2
pub const UMCCH5_3_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_3_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH5_3_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH5_3_AddrHashBank3
pub const UMCCH5_3_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_3_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH5_3_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH5_3_AddrHashBank4
pub const UMCCH5_3_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_3_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH5_3_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH5_3_AddrHashBank5
pub const UMCCH5_3_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH5_3_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH5_3_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH5_3_EccErrCntSel
pub const UMCCH5_3_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH5_3_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH5_3_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH5_3_EccErrCnt
pub const UMCCH5_3_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH5_3_PerfMonCtlClk
pub const UMCCH5_3_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH5_3_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH5_3_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH5_3_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH5_3_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH5_3_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH5_3_PerfMonCtrClk_Lo
pub const UMCCH5_3_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_3_PerfMonCtrClk_Hi
pub const UMCCH5_3_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_3_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH5_3_PerfMonCtl1
pub const UMCCH5_3_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_3_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_3_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_3_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_3_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_3_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_3_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_3_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_3_PerfMonCtr1_Lo
pub const UMCCH5_3_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_3_PerfMonCtr1_Hi
pub const UMCCH5_3_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_3_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_3_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_3_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_3_PerfMonCtl2
pub const UMCCH5_3_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_3_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_3_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_3_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_3_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_3_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_3_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_3_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_3_PerfMonCtr2_Lo
pub const UMCCH5_3_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_3_PerfMonCtr2_Hi
pub const UMCCH5_3_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_3_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_3_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_3_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_3_PerfMonCtl3
pub const UMCCH5_3_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_3_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_3_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_3_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_3_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_3_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_3_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_3_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_3_PerfMonCtr3_Lo
pub const UMCCH5_3_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_3_PerfMonCtr3_Hi
pub const UMCCH5_3_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_3_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_3_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_3_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_3_PerfMonCtl4
pub const UMCCH5_3_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_3_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_3_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_3_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_3_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_3_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_3_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_3_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_3_PerfMonCtr4_Lo
pub const UMCCH5_3_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_3_PerfMonCtr4_Hi
pub const UMCCH5_3_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_3_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_3_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_3_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_3_PerfMonCtl5
pub const UMCCH5_3_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_3_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_3_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_3_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_3_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_3_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_3_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_3_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_3_PerfMonCtr5_Lo
pub const UMCCH5_3_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_3_PerfMonCtr5_Hi
pub const UMCCH5_3_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_3_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_3_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_3_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_3_PerfMonCtl6
pub const UMCCH5_3_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_3_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_3_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_3_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_3_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_3_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_3_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_3_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_3_PerfMonCtr6_Lo
pub const UMCCH5_3_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_3_PerfMonCtr6_Hi
pub const UMCCH5_3_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_3_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_3_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_3_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_3_PerfMonCtl7
pub const UMCCH5_3_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_3_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_3_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_3_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_3_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_3_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_3_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_3_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_3_PerfMonCtr7_Lo
pub const UMCCH5_3_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_3_PerfMonCtr7_Hi
pub const UMCCH5_3_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_3_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_3_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_3_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH5_3_PerfMonCtl8
pub const UMCCH5_3_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH5_3_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH5_3_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH5_3_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH5_3_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH5_3_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH5_3_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH5_3_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH5_3_PerfMonCtr8_Lo
pub const UMCCH5_3_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH5_3_PerfMonCtr8_Hi
pub const UMCCH5_3_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH5_3_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH5_3_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH5_3_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc3_umcch6_umcchdec
//UMCCH6_3_BaseAddrCS0
pub const UMCCH6_3_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_3_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH6_3_AddrMaskCS01
pub const UMCCH6_3_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH6_3_AddrSelCS01
pub const UMCCH6_3_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH6_3_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH6_3_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH6_3_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH6_3_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH6_3_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH6_3_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH6_3_AddrHashBank0
pub const UMCCH6_3_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_3_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH6_3_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH6_3_AddrHashBank1
pub const UMCCH6_3_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_3_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH6_3_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH6_3_AddrHashBank2
pub const UMCCH6_3_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_3_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH6_3_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH6_3_AddrHashBank3
pub const UMCCH6_3_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_3_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH6_3_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH6_3_AddrHashBank4
pub const UMCCH6_3_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_3_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH6_3_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH6_3_AddrHashBank5
pub const UMCCH6_3_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH6_3_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH6_3_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH6_3_EccErrCntSel
pub const UMCCH6_3_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH6_3_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH6_3_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH6_3_EccErrCnt
pub const UMCCH6_3_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH6_3_PerfMonCtlClk
pub const UMCCH6_3_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH6_3_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH6_3_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH6_3_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH6_3_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH6_3_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH6_3_PerfMonCtrClk_Lo
pub const UMCCH6_3_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_3_PerfMonCtrClk_Hi
pub const UMCCH6_3_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_3_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH6_3_PerfMonCtl1
pub const UMCCH6_3_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_3_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_3_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_3_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_3_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_3_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_3_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_3_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_3_PerfMonCtr1_Lo
pub const UMCCH6_3_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_3_PerfMonCtr1_Hi
pub const UMCCH6_3_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_3_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_3_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_3_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_3_PerfMonCtl2
pub const UMCCH6_3_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_3_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_3_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_3_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_3_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_3_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_3_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_3_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_3_PerfMonCtr2_Lo
pub const UMCCH6_3_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_3_PerfMonCtr2_Hi
pub const UMCCH6_3_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_3_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_3_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_3_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_3_PerfMonCtl3
pub const UMCCH6_3_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_3_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_3_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_3_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_3_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_3_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_3_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_3_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_3_PerfMonCtr3_Lo
pub const UMCCH6_3_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_3_PerfMonCtr3_Hi
pub const UMCCH6_3_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_3_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_3_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_3_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_3_PerfMonCtl4
pub const UMCCH6_3_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_3_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_3_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_3_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_3_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_3_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_3_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_3_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_3_PerfMonCtr4_Lo
pub const UMCCH6_3_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_3_PerfMonCtr4_Hi
pub const UMCCH6_3_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_3_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_3_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_3_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_3_PerfMonCtl5
pub const UMCCH6_3_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_3_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_3_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_3_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_3_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_3_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_3_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_3_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_3_PerfMonCtr5_Lo
pub const UMCCH6_3_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_3_PerfMonCtr5_Hi
pub const UMCCH6_3_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_3_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_3_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_3_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_3_PerfMonCtl6
pub const UMCCH6_3_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_3_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_3_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_3_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_3_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_3_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_3_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_3_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_3_PerfMonCtr6_Lo
pub const UMCCH6_3_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_3_PerfMonCtr6_Hi
pub const UMCCH6_3_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_3_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_3_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_3_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_3_PerfMonCtl7
pub const UMCCH6_3_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_3_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_3_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_3_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_3_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_3_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_3_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_3_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_3_PerfMonCtr7_Lo
pub const UMCCH6_3_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_3_PerfMonCtr7_Hi
pub const UMCCH6_3_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_3_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_3_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_3_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH6_3_PerfMonCtl8
pub const UMCCH6_3_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH6_3_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH6_3_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH6_3_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH6_3_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH6_3_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH6_3_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH6_3_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH6_3_PerfMonCtr8_Lo
pub const UMCCH6_3_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH6_3_PerfMonCtr8_Hi
pub const UMCCH6_3_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH6_3_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH6_3_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH6_3_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;


// addressBlock: umc_w_phy_umc3_umcch7_umcchdec
//UMCCH7_3_BaseAddrCS0
pub const UMCCH7_3_BaseAddrCS0__CSEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_3_BaseAddrCS0__BaseAddr__SHIFT: u64 = 0x1;
pub const : u64 = 0x00000001;
pub const : u64 = 0xFFFFFFFE;
//UMCCH7_3_AddrMaskCS01
pub const UMCCH7_3_AddrMaskCS01__AddrMask__SHIFT: u64 = 0x1;
pub const : u64 = 0xFFFFFFFE;
//UMCCH7_3_AddrSelCS01
pub const UMCCH7_3_AddrSelCS01__BankBit0__SHIFT: u64 = 0x0;
pub const UMCCH7_3_AddrSelCS01__BankBit1__SHIFT: u64 = 0x4;
pub const UMCCH7_3_AddrSelCS01__BankBit2__SHIFT: u64 = 0x8;
pub const UMCCH7_3_AddrSelCS01__BankBit3__SHIFT: u64 = 0xc;
pub const UMCCH7_3_AddrSelCS01__BankBit4__SHIFT: u64 = 0x10;
pub const UMCCH7_3_AddrSelCS01__RowLo__SHIFT: u64 = 0x18;
pub const UMCCH7_3_AddrSelCS01__RowHi__SHIFT: u64 = 0x1c;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x000000F0;
pub const : u64 = 0x00000F00;
pub const : u64 = 0x0000F000;
pub const : u64 = 0x001F0000;
pub const : u64 = 0x0F000000;
pub const : u64 = 0xF0000000;
//UMCCH7_3_AddrHashBank0
pub const UMCCH7_3_AddrHashBank0__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_3_AddrHashBank0__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH7_3_AddrHashBank0__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH7_3_AddrHashBank1
pub const UMCCH7_3_AddrHashBank1__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_3_AddrHashBank1__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH7_3_AddrHashBank1__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH7_3_AddrHashBank2
pub const UMCCH7_3_AddrHashBank2__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_3_AddrHashBank2__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH7_3_AddrHashBank2__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH7_3_AddrHashBank3
pub const UMCCH7_3_AddrHashBank3__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_3_AddrHashBank3__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH7_3_AddrHashBank3__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH7_3_AddrHashBank4
pub const UMCCH7_3_AddrHashBank4__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_3_AddrHashBank4__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH7_3_AddrHashBank4__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH7_3_AddrHashBank5
pub const UMCCH7_3_AddrHashBank5__XorEnable__SHIFT: u64 = 0x0;
pub const UMCCH7_3_AddrHashBank5__ColXor__SHIFT: u64 = 0x1;
pub const UMCCH7_3_AddrHashBank5__RowXor__SHIFT: u64 = 0xe;
pub const : u64 = 0x00000001;
pub const : u64 = 0x00003FFE;
pub const : u64 = 0xFFFFC000;
//UMCCH7_3_EccErrCntSel
pub const UMCCH7_3_EccErrCntSel__EccErrCntCsSel__SHIFT: u64 = 0x0;
pub const UMCCH7_3_EccErrCntSel__EccErrInt__SHIFT: u64 = 0xc;
pub const UMCCH7_3_EccErrCntSel__EccErrCntEn__SHIFT: u64 = 0xf;
pub const : u64 = 0x0000000F;
pub const : u64 = 0x00003000;
pub const : u64 = 0x00008000;
//UMCCH7_3_EccErrCnt
pub const UMCCH7_3_EccErrCnt__EccErrCnt__SHIFT: u64 = 0x0;
pub const : u64 = 0x0000FFFF;
//UMCCH7_3_PerfMonCtlClk
pub const UMCCH7_3_PerfMonCtlClk__GlblResetMsk__SHIFT: u64 = 0x0;
pub const UMCCH7_3_PerfMonCtlClk__ClkGate__SHIFT: u64 = 0x16;
pub const UMCCH7_3_PerfMonCtlClk__GlblReset__SHIFT: u64 = 0x18;
pub const UMCCH7_3_PerfMonCtlClk__GlblMonEn__SHIFT: u64 = 0x19;
pub const UMCCH7_3_PerfMonCtlClk__NumCounters__SHIFT: u64 = 0x1a;
pub const UMCCH7_3_PerfMonCtlClk__CtrClkEn__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000001FF;
pub const : u64 = 0x00400000;
pub const : u64 = 0x01000000;
pub const : u64 = 0x02000000;
pub const : u64 = 0x3C000000;
pub const : u64 = 0x80000000;
//UMCCH7_3_PerfMonCtrClk_Lo
pub const UMCCH7_3_PerfMonCtrClk_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_3_PerfMonCtrClk_Hi
pub const UMCCH7_3_PerfMonCtrClk_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_3_PerfMonCtrClk_Hi__Overflow__SHIFT: u64 = 0x10;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
//UMCCH7_3_PerfMonCtl1
pub const UMCCH7_3_PerfMonCtl1__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_3_PerfMonCtl1__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_3_PerfMonCtl1__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_3_PerfMonCtl1__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_3_PerfMonCtl1__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_3_PerfMonCtl1__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_3_PerfMonCtl1__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_3_PerfMonCtl1__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_3_PerfMonCtr1_Lo
pub const UMCCH7_3_PerfMonCtr1_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_3_PerfMonCtr1_Hi
pub const UMCCH7_3_PerfMonCtr1_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_3_PerfMonCtr1_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_3_PerfMonCtr1_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_3_PerfMonCtr1_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_3_PerfMonCtl2
pub const UMCCH7_3_PerfMonCtl2__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_3_PerfMonCtl2__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_3_PerfMonCtl2__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_3_PerfMonCtl2__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_3_PerfMonCtl2__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_3_PerfMonCtl2__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_3_PerfMonCtl2__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_3_PerfMonCtl2__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_3_PerfMonCtr2_Lo
pub const UMCCH7_3_PerfMonCtr2_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_3_PerfMonCtr2_Hi
pub const UMCCH7_3_PerfMonCtr2_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_3_PerfMonCtr2_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_3_PerfMonCtr2_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_3_PerfMonCtr2_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_3_PerfMonCtl3
pub const UMCCH7_3_PerfMonCtl3__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_3_PerfMonCtl3__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_3_PerfMonCtl3__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_3_PerfMonCtl3__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_3_PerfMonCtl3__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_3_PerfMonCtl3__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_3_PerfMonCtl3__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_3_PerfMonCtl3__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_3_PerfMonCtr3_Lo
pub const UMCCH7_3_PerfMonCtr3_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_3_PerfMonCtr3_Hi
pub const UMCCH7_3_PerfMonCtr3_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_3_PerfMonCtr3_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_3_PerfMonCtr3_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_3_PerfMonCtr3_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_3_PerfMonCtl4
pub const UMCCH7_3_PerfMonCtl4__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_3_PerfMonCtl4__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_3_PerfMonCtl4__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_3_PerfMonCtl4__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_3_PerfMonCtl4__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_3_PerfMonCtl4__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_3_PerfMonCtl4__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_3_PerfMonCtl4__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_3_PerfMonCtr4_Lo
pub const UMCCH7_3_PerfMonCtr4_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_3_PerfMonCtr4_Hi
pub const UMCCH7_3_PerfMonCtr4_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_3_PerfMonCtr4_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_3_PerfMonCtr4_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_3_PerfMonCtr4_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_3_PerfMonCtl5
pub const UMCCH7_3_PerfMonCtl5__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_3_PerfMonCtl5__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_3_PerfMonCtl5__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_3_PerfMonCtl5__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_3_PerfMonCtl5__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_3_PerfMonCtl5__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_3_PerfMonCtl5__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_3_PerfMonCtl5__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_3_PerfMonCtr5_Lo
pub const UMCCH7_3_PerfMonCtr5_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_3_PerfMonCtr5_Hi
pub const UMCCH7_3_PerfMonCtr5_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_3_PerfMonCtr5_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_3_PerfMonCtr5_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_3_PerfMonCtr5_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_3_PerfMonCtl6
pub const UMCCH7_3_PerfMonCtl6__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_3_PerfMonCtl6__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_3_PerfMonCtl6__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_3_PerfMonCtl6__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_3_PerfMonCtl6__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_3_PerfMonCtl6__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_3_PerfMonCtl6__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_3_PerfMonCtl6__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_3_PerfMonCtr6_Lo
pub const UMCCH7_3_PerfMonCtr6_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_3_PerfMonCtr6_Hi
pub const UMCCH7_3_PerfMonCtr6_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_3_PerfMonCtr6_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_3_PerfMonCtr6_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_3_PerfMonCtr6_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_3_PerfMonCtl7
pub const UMCCH7_3_PerfMonCtl7__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_3_PerfMonCtl7__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_3_PerfMonCtl7__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_3_PerfMonCtl7__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_3_PerfMonCtl7__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_3_PerfMonCtl7__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_3_PerfMonCtl7__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_3_PerfMonCtl7__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_3_PerfMonCtr7_Lo
pub const UMCCH7_3_PerfMonCtr7_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_3_PerfMonCtr7_Hi
pub const UMCCH7_3_PerfMonCtr7_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_3_PerfMonCtr7_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_3_PerfMonCtr7_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_3_PerfMonCtr7_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;
//UMCCH7_3_PerfMonCtl8
pub const UMCCH7_3_PerfMonCtl8__EventSelect__SHIFT: u64 = 0x0;
pub const UMCCH7_3_PerfMonCtl8__RdWrMask__SHIFT: u64 = 0x8;
pub const UMCCH7_3_PerfMonCtl8__PriorityMask__SHIFT: u64 = 0xa;
pub const UMCCH7_3_PerfMonCtl8__ReqSizeMask__SHIFT: u64 = 0xe;
pub const UMCCH7_3_PerfMonCtl8__BankSel__SHIFT: u64 = 0x10;
pub const UMCCH7_3_PerfMonCtl8__VCSel__SHIFT: u64 = 0x18;
pub const UMCCH7_3_PerfMonCtl8__SubChanMask__SHIFT: u64 = 0x1d;
pub const UMCCH7_3_PerfMonCtl8__Enable__SHIFT: u64 = 0x1f;
pub const : u64 = 0x000000FF;
pub const : u64 = 0x00000300;
pub const : u64 = 0x00003C00;
pub const : u64 = 0x0000C000;
pub const : u64 = 0x00FF0000;
pub const : u64 = 0x1F000000;
pub const : u64 = 0x60000000;
pub const : u64 = 0x80000000;
//UMCCH7_3_PerfMonCtr8_Lo
pub const UMCCH7_3_PerfMonCtr8_Lo__Data__SHIFT: u64 = 0x0;
pub const : u64 = 0xFFFFFFFF;
//UMCCH7_3_PerfMonCtr8_Hi
pub const UMCCH7_3_PerfMonCtr8_Hi__Data__SHIFT: u64 = 0x0;
pub const UMCCH7_3_PerfMonCtr8_Hi__Overflow__SHIFT: u64 = 0x10;
pub const UMCCH7_3_PerfMonCtr8_Hi__ThreshCntEn__SHIFT: u64 = 0x12;
pub const UMCCH7_3_PerfMonCtr8_Hi__ThreshCnt__SHIFT: u64 = 0x14;
pub const : u64 = 0x0000FFFF;
pub const : u64 = 0x00010000;
pub const : u64 = 0x000C0000;
pub const : u64 = 0xFFF00000;



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
