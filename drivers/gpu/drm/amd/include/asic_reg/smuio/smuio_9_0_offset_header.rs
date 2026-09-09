/*
 * Copyright (C) 2017  Advanced Micro Devices, Inc.
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

// addressBlock: smuio_smuio_SmuSmuioDec
// base address: 0x5a000

pub const mmROM_CNTL: u32 = 0x0024;
pub const mmROM_CNTL_BASE_IDX: u32 = 0;
pub const mmROM_STATUS: u32 = 0x0026;
pub const mmROM_STATUS_BASE_IDX: u32 = 0;
pub const mmCGTT_ROM_CLK_CTRL0: u32 = 0x0027;
pub const mmCGTT_ROM_CLK_CTRL0_BASE_IDX: u32 = 0;
pub const mmROM_INDEX: u32 = 0x0028;
pub const mmROM_INDEX_BASE_IDX: u32 = 0;
pub const mmROM_DATA: u32 = 0x0029;
pub const mmROM_DATA_BASE_IDX: u32 = 0;
pub const mmROM_START: u32 = 0x002a;
pub const mmROM_START_BASE_IDX: u32 = 0;
pub const mmROM_SW_CNTL: u32 = 0x002b;
pub const mmROM_SW_CNTL_BASE_IDX: u32 = 0;
pub const mmROM_SW_STATUS: u32 = 0x002c;
pub const mmROM_SW_STATUS_BASE_IDX: u32 = 0;
pub const mmROM_SW_COMMAND: u32 = 0x002d;
pub const mmROM_SW_COMMAND_BASE_IDX: u32 = 0;

pub const mmROM_SW_DATA_1: u32 = 0x002e;
pub const mmROM_SW_DATA_1_BASE_IDX: u32 = 0;

// The software data registers are consecutive and retain their source names.
pub const mmROM_SW_DATA_2: u32 = 0x002f;
pub const mmROM_SW_DATA_2_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_3: u32 = 0x0030;
pub const mmROM_SW_DATA_3_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_4: u32 = 0x0031;
pub const mmROM_SW_DATA_4_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_5: u32 = 0x0032;
pub const mmROM_SW_DATA_5_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_6: u32 = 0x0033;
pub const mmROM_SW_DATA_6_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_7: u32 = 0x0034;
pub const mmROM_SW_DATA_7_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_8: u32 = 0x0035;
pub const mmROM_SW_DATA_8_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_9: u32 = 0x0036;
pub const mmROM_SW_DATA_9_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_10: u32 = 0x0037;
pub const mmROM_SW_DATA_10_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_11: u32 = 0x0038;
pub const mmROM_SW_DATA_11_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_12: u32 = 0x0039;
pub const mmROM_SW_DATA_12_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_13: u32 = 0x003a;
pub const mmROM_SW_DATA_13_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_14: u32 = 0x003b;
pub const mmROM_SW_DATA_14_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_15: u32 = 0x003c;
pub const mmROM_SW_DATA_15_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_16: u32 = 0x003d;
pub const mmROM_SW_DATA_16_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_17: u32 = 0x003e;
pub const mmROM_SW_DATA_17_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_18: u32 = 0x003f;
pub const mmROM_SW_DATA_18_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_19: u32 = 0x0040;
pub const mmROM_SW_DATA_19_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_20: u32 = 0x0041;
pub const mmROM_SW_DATA_20_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_21: u32 = 0x0042;
pub const mmROM_SW_DATA_21_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_22: u32 = 0x0043;
pub const mmROM_SW_DATA_22_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_23: u32 = 0x0044;
pub const mmROM_SW_DATA_23_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_24: u32 = 0x0045;
pub const mmROM_SW_DATA_24_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_25: u32 = 0x0046;
pub const mmROM_SW_DATA_25_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_26: u32 = 0x0047;
pub const mmROM_SW_DATA_26_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_27: u32 = 0x0048;
pub const mmROM_SW_DATA_27_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_28: u32 = 0x0049;
pub const mmROM_SW_DATA_28_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_29: u32 = 0x004a;
pub const mmROM_SW_DATA_29_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_30: u32 = 0x004b;
pub const mmROM_SW_DATA_30_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_31: u32 = 0x004c;
pub const mmROM_SW_DATA_31_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_32: u32 = 0x004d;
pub const mmROM_SW_DATA_32_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_33: u32 = 0x004e;
pub const mmROM_SW_DATA_33_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_34: u32 = 0x004f;
pub const mmROM_SW_DATA_34_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_35: u32 = 0x0050;
pub const mmROM_SW_DATA_35_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_36: u32 = 0x0051;
pub const mmROM_SW_DATA_36_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_37: u32 = 0x0052;
pub const mmROM_SW_DATA_37_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_38: u32 = 0x0053;
pub const mmROM_SW_DATA_38_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_39: u32 = 0x0054;
pub const mmROM_SW_DATA_39_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_40: u32 = 0x0055;
pub const mmROM_SW_DATA_40_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_41: u32 = 0x0056;
pub const mmROM_SW_DATA_41_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_42: u32 = 0x0057;
pub const mmROM_SW_DATA_42_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_43: u32 = 0x0058;
pub const mmROM_SW_DATA_43_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_44: u32 = 0x0059;
pub const mmROM_SW_DATA_44_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_45: u32 = 0x005a;
pub const mmROM_SW_DATA_45_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_46: u32 = 0x005b;
pub const mmROM_SW_DATA_46_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_47: u32 = 0x005c;
pub const mmROM_SW_DATA_47_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_48: u32 = 0x005d;
pub const mmROM_SW_DATA_48_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_49: u32 = 0x005e;
pub const mmROM_SW_DATA_49_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_50: u32 = 0x005f;
pub const mmROM_SW_DATA_50_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_51: u32 = 0x0060;
pub const mmROM_SW_DATA_51_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_52: u32 = 0x0061;
pub const mmROM_SW_DATA_52_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_53: u32 = 0x0062;
pub const mmROM_SW_DATA_53_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_54: u32 = 0x0063;
pub const mmROM_SW_DATA_54_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_55: u32 = 0x0064;
pub const mmROM_SW_DATA_55_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_56: u32 = 0x0065;
pub const mmROM_SW_DATA_56_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_57: u32 = 0x0066;
pub const mmROM_SW_DATA_57_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_58: u32 = 0x0067;
pub const mmROM_SW_DATA_58_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_59: u32 = 0x0068;
pub const mmROM_SW_DATA_59_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_60: u32 = 0x0069;
pub const mmROM_SW_DATA_60_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_61: u32 = 0x006a;
pub const mmROM_SW_DATA_61_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_62: u32 = 0x006b;
pub const mmROM_SW_DATA_62_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_63: u32 = 0x006c;
pub const mmROM_SW_DATA_63_BASE_IDX: u32 = 0;
pub const mmROM_SW_DATA_64: u32 = 0x006d;
pub const mmROM_SW_DATA_64_BASE_IDX: u32 = 0;

pub const mmSMUSVI0_PLANE0_CURRENTVID_BASE_IDX: u32 = 0;
pub const mmSMUSVI0_PLANE0_CURRENTVID: u32 = 0x0013;
pub const mmSMUSVI0_TEL_PLANE0_BASE_IDX: u32 = 0;
pub const mmSMUSVI0_TEL_PLANE0: u32 = 0x0004;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
