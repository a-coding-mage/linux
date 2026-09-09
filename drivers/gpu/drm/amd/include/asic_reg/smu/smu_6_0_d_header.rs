/*
 *
 * Copyright (C) 2016 Advanced Micro Devices, Inc.
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

pub const ixLCAC_MC0_CNTL: u32 = 0x011C;
pub const ixLCAC_MC0_OVR_SEL: u32 = 0x011D;
pub const ixLCAC_MC0_OVR_VAL: u32 = 0x011E;
pub const ixLCAC_MC1_CNTL: u32 = 0x011F;
pub const ixLCAC_MC1_OVR_SEL: u32 = 0x0120;
pub const ixLCAC_MC1_OVR_VAL: u32 = 0x0121;
pub const ixLCAC_MC2_CNTL: u32 = 0x0122;
pub const ixLCAC_MC2_OVR_SEL: u32 = 0x0123;
pub const ixLCAC_MC2_OVR_VAL: u32 = 0x0124;
pub const ixLCAC_MC3_CNTL: u32 = 0x0125;
pub const ixLCAC_MC3_OVR_SEL: u32 = 0x0126;
pub const ixLCAC_MC3_OVR_VAL: u32 = 0x0127;
pub const ixLCAC_MC4_CNTL: u32 = 0x0128;
pub const ixLCAC_MC4_OVR_SEL: u32 = 0x0129;
pub const ixLCAC_MC4_OVR_VAL: u32 = 0x012A;
pub const ixLCAC_MC5_CNTL: u32 = 0x012B;
pub const ixLCAC_MC5_OVR_SEL: u32 = 0x012C;
pub const ixLCAC_MC5_OVR_VAL: u32 = 0x012D;

pub const mmCG_SPLL_FUNC_CNTL: u32 = 0x0180;
pub const mmCG_SPLL_FUNC_CNTL_2: u32 = 0x0181;
pub const mmCG_SPLL_FUNC_CNTL_3: u32 = 0x0182;
pub const mmCG_SPLL_FUNC_CNTL_4: u32 = 0x0183;
pub const mmCG_SPLL_STATUS: u32 = 0x0185;
pub const mmSPLL_CNTL_MODE: u32 = 0x0186;
pub const mmCG_SPLL_SPREAD_SPECTRUM: u32 = 0x0188;
pub const mmCG_SPLL_SPREAD_SPECTRUM_2: u32 = 0x0189;
pub const mmCG_SPLL_AUTOSCALE_CNTL: u32 = 0x018B;
pub const mmMPLL_BYPASSCLK_SEL: u32 = 0x0197;
pub const mmCG_CLKPIN_CNTL: u32 = 0x0198;
pub const mmCG_CLKPIN_CNTL_2: u32 = 0x0199;
pub const mmTHM_CLK_CNTL: u32 = 0x019B;
pub const mmMISC_CLK_CNTL: u32 = 0x019C;
pub const mmCG_THERMAL_CTRL: u32 = 0x01C0;
pub const mmCG_THERMAL_STATUS: u32 = 0x01C1;
pub const mmCG_THERMAL_INT: u32 = 0x01C2;
pub const mmCG_MULT_THERMAL_CTRL: u32 = 0x01C4;
pub const mmCG_MULT_THERMAL_STATUS: u32 = 0x01C5;
pub const mmCG_FDO_CTRL0: u32 = 0x01D5;
pub const mmCG_FDO_CTRL1: u32 = 0x01D6;
pub const mmCG_FDO_CTRL2: u32 = 0x01D7;
pub const mmCG_TACH_CTRL: u32 = 0x01DC;
pub const mmCG_TACH_STATUS: u32 = 0x01DD;
pub const mmGENERAL_PWRMGT: u32 = 0x1E0;
pub const mmCG_TPC: u32 = 0x1E1;
pub const mmSCLK_PWRMGT_CNTL: u32 = 0x1E2;
pub const mmTARGET_AND_CURRENT_PROFILE_INDEX: u32 = 0x01E6;
pub const mmCG_FTV: u32 = 0x01EF;
pub const mmCG_FFCT_0: u32 = 0x01F0;
pub const mmCG_BSP: u32 = 0x01FF;
pub const mmCG_AT: u32 = 0x0200;
pub const mmCG_GIT: u32 = 0x0201;
pub const mmCG_SSP: u32 = 0x0203;
pub const mmCG_DISPLAY_GAP_CNTL: u32 = 0x020A;
pub const mmCG_ULV_CONTROL: u32 = 0x021E;
pub const mmCG_ULV_PARAMETER: u32 = 0x021F;
pub const mmSMC_SCRATCH0: u32 = 0x0221;
pub const mmCG_CAC_CTRL: u32 = 0x022E;

pub const ixSMC_PC_C: u32 = 0x80000370;

pub const ixTHM_TMON0_DEBUG: u32 = 0x03F0;
pub const ixTHM_TMON0_INT_DATA: u32 = 0x0380;
pub const ixTHM_TMON0_RDIL0_DATA: u32 = 0x0300;
pub const ixTHM_TMON0_RDIL10_DATA: u32 = 0x030A;
pub const ixTHM_TMON0_RDIL11_DATA: u32 = 0x030B;
pub const ixTHM_TMON0_RDIL12_DATA: u32 = 0x030C;
pub const ixTHM_TMON0_RDIL13_DATA: u32 = 0x030D;
pub const ixTHM_TMON0_RDIL14_DATA: u32 = 0x030E;
pub const ixTHM_TMON0_RDIL15_DATA: u32 = 0x030F;
pub const ixTHM_TMON0_RDIL1_DATA: u32 = 0x0301;
pub const ixTHM_TMON0_RDIL2_DATA: u32 = 0x0302;
pub const ixTHM_TMON0_RDIL3_DATA: u32 = 0x0303;
pub const ixTHM_TMON0_RDIL4_DATA: u32 = 0x0304;
pub const ixTHM_TMON0_RDIL5_DATA: u32 = 0x0305;
pub const ixTHM_TMON0_RDIL6_DATA: u32 = 0x0306;
pub const ixTHM_TMON0_RDIL7_DATA: u32 = 0x0307;
pub const ixTHM_TMON0_RDIL8_DATA: u32 = 0x0308;
pub const ixTHM_TMON0_RDIL9_DATA: u32 = 0x0309;
pub const ixTHM_TMON0_RDIR0_DATA: u32 = 0x0310;
pub const ixTHM_TMON0_RDIR10_DATA: u32 = 0x031A;
pub const ixTHM_TMON0_RDIR11_DATA: u32 = 0x031B;
pub const ixTHM_TMON0_RDIR12_DATA: u32 = 0x031C;
pub const ixTHM_TMON0_RDIR13_DATA: u32 = 0x031D;
pub const ixTHM_TMON0_RDIR14_DATA: u32 = 0x031E;
pub const ixTHM_TMON0_RDIR15_DATA: u32 = 0x031F;
pub const ixTHM_TMON0_RDIR1_DATA: u32 = 0x0311;
pub const ixTHM_TMON0_RDIR2_DATA: u32 = 0x0312;
pub const ixTHM_TMON0_RDIR3_DATA: u32 = 0x0313;
pub const ixTHM_TMON0_RDIR4_DATA: u32 = 0x0314;
pub const ixTHM_TMON0_RDIR5_DATA: u32 = 0x0315;
pub const ixTHM_TMON0_RDIR6_DATA: u32 = 0x0316;
pub const ixTHM_TMON0_RDIR7_DATA: u32 = 0x0317;
pub const ixTHM_TMON0_RDIR8_DATA: u32 = 0x0318;
pub const ixTHM_TMON0_RDIR9_DATA: u32 = 0x0319;

pub const ixTHM_TMON1_DEBUG: u32 = 0x03F1;
pub const ixTHM_TMON1_INT_DATA: u32 = 0x0381;
pub const ixTHM_TMON1_RDIL0_DATA: u32 = 0x0320;
pub const ixTHM_TMON1_RDIL10_DATA: u32 = 0x032A;
pub const ixTHM_TMON1_RDIL11_DATA: u32 = 0x032B;
pub const ixTHM_TMON1_RDIL12_DATA: u32 = 0x032C;
pub const ixTHM_TMON1_RDIL13_DATA: u32 = 0x032D;
pub const ixTHM_TMON1_RDIL14_DATA: u32 = 0x032E;
pub const ixTHM_TMON1_RDIL15_DATA: u32 = 0x032F;
pub const ixTHM_TMON1_RDIL1_DATA: u32 = 0x0321;
pub const ixTHM_TMON1_RDIL2_DATA: u32 = 0x0322;
pub const ixTHM_TMON1_RDIL3_DATA: u32 = 0x0323;
pub const ixTHM_TMON1_RDIL4_DATA: u32 = 0x0324;
pub const ixTHM_TMON1_RDIL5_DATA: u32 = 0x0325;
pub const ixTHM_TMON1_RDIL6_DATA: u32 = 0x0326;
pub const ixTHM_TMON1_RDIL7_DATA: u32 = 0x0327;
pub const ixTHM_TMON1_RDIL8_DATA: u32 = 0x0328;
pub const ixTHM_TMON1_RDIL9_DATA: u32 = 0x0329;
pub const ixTHM_TMON1_RDIR0_DATA: u32 = 0x0330;
pub const ixTHM_TMON1_RDIR10_DATA: u32 = 0x033A;
pub const ixTHM_TMON1_RDIR11_DATA: u32 = 0x033B;
pub const ixTHM_TMON1_RDIR12_DATA: u32 = 0x033C;
pub const ixTHM_TMON1_RDIR13_DATA: u32 = 0x033D;
pub const ixTHM_TMON1_RDIR14_DATA: u32 = 0x033E;
pub const ixTHM_TMON1_RDIR15_DATA: u32 = 0x033F;
pub const ixTHM_TMON1_RDIR1_DATA: u32 = 0x0331;
pub const ixTHM_TMON1_RDIR2_DATA: u32 = 0x0332;
pub const ixTHM_TMON1_RDIR3_DATA: u32 = 0x0333;
pub const ixTHM_TMON1_RDIR4_DATA: u32 = 0x0334;
pub const ixTHM_TMON1_RDIR5_DATA: u32 = 0x0335;
pub const ixTHM_TMON1_RDIR6_DATA: u32 = 0x0336;
pub const ixTHM_TMON1_RDIR7_DATA: u32 = 0x0337;
pub const ixTHM_TMON1_RDIR8_DATA: u32 = 0x0338;
pub const ixTHM_TMON1_RDIR9_DATA: u32 = 0x0339;

pub const mmGPIOPAD_A: u32 = 0x05E7;
pub const mmGPIOPAD_EN: u32 = 0x05E8;
pub const mmGPIOPAD_EXTERN_TRIG_CNTL: u32 = 0x05F1;
pub const mmGPIOPAD_INT_EN: u32 = 0x05EE;
pub const mmGPIOPAD_INT_POLARITY: u32 = 0x05F0;
pub const mmGPIOPAD_INT_STAT: u32 = 0x05EC;
pub const mmGPIOPAD_INT_STAT_AK: u32 = 0x05ED;
pub const mmGPIOPAD_INT_STAT_EN: u32 = 0x05EB;
pub const mmGPIOPAD_INT_TYPE: u32 = 0x05EF;
pub const mmGPIOPAD_MASK: u32 = 0x05E6;
pub const mmGPIOPAD_PD_EN: u32 = 0x05F4;
pub const mmGPIOPAD_PINSTRAPS: u32 = 0x05EA;
pub const mmGPIOPAD_PU_EN: u32 = 0x05F3;
pub const mmGPIOPAD_RCVR_SEL: u32 = 0x05F2;
pub const mmGPIOPAD_STRENGTH: u32 = 0x05E5;
pub const mmGPIOPAD_SW_INT_STAT: u32 = 0x05E4;
pub const mmGPIOPAD_Y: u32 = 0x05E9;

pub const mmSMC_IND_ACCESS_CNTL: u32 = 0x008A;
pub const mmSMC_IND_DATA_0: u32 = 0x0081;
pub const mmSMC_IND_DATA: u32 = 0x0081;
pub const mmSMC_IND_DATA_1: u32 = 0x0083;
pub const mmSMC_IND_DATA_2: u32 = 0x0085;
pub const mmSMC_IND_DATA_3: u32 = 0x0087;
pub const mmSMC_IND_INDEX_0: u32 = 0x0080;
pub const mmSMC_IND_INDEX: u32 = 0x0080;
pub const mmSMC_IND_INDEX_1: u32 = 0x0082;
pub const mmSMC_IND_INDEX_2: u32 = 0x0084;
pub const mmSMC_IND_INDEX_3: u32 = 0x0086;
pub const mmSMC_MESSAGE_0: u32 = 0x008B;
pub const mmSMC_MESSAGE_1: u32 = 0x008D;
pub const mmSMC_MESSAGE_2: u32 = 0x008F;
pub const mmSMC_RESP_0: u32 = 0x008C;
pub const mmSMC_RESP_1: u32 = 0x008E;
pub const mmSMC_RESP_2: u32 = 0x0090;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
