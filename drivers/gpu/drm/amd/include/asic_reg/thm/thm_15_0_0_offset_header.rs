/*
 * Copyright (C) 2025  Advanced Micro Devices, Inc.
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

// addressBlock: thm_thm_SmuThmDec
// base address: 0x59800

pub const regTHM_TCON_CUR_TMP: u32 = 0x0000;
pub const regTHM_TCON_CUR_TMP_BASE_IDX: u32 = 0;
pub const regTHM_TCON_HTC: u32 = 0x0001;
pub const regTHM_TCON_HTC_BASE_IDX: u32 = 0;
pub const regTHM_TCON_THERM_TRIP: u32 = 0x0002;
pub const regTHM_TCON_THERM_TRIP_BASE_IDX: u32 = 0;
pub const regTHM_CTF_DELAY: u32 = 0x0004;
pub const regTHM_CTF_DELAY_BASE_IDX: u32 = 0;
pub const regTHM_GPIO_PROCHOT_CTRL: u32 = 0x0005;
pub const regTHM_GPIO_PROCHOT_CTRL_BASE_IDX: u32 = 0;
pub const regTHM_SW_TEMP: u32 = 0x0006;
pub const regTHM_SW_TEMP_BASE_IDX: u32 = 0;
pub const regCG_MULT_THERMAL_CTRL: u32 = 0x0007;
pub const regCG_MULT_THERMAL_CTRL_BASE_IDX: u32 = 0;
pub const regCG_MULT_THERMAL_STATUS: u32 = 0x0008;
pub const regCG_MULT_THERMAL_STATUS_BASE_IDX: u32 = 0;
pub const regCG_THERMAL_RANGE: u32 = 0x0009;
pub const regCG_THERMAL_RANGE_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL2: u32 = 0x000a;
pub const regTHM_TCON_LOCAL2_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL3: u32 = 0x000b;
pub const regTHM_TCON_LOCAL3_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL4: u32 = 0x000c;
pub const regTHM_TCON_LOCAL4_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL5: u32 = 0x000d;
pub const regTHM_TCON_LOCAL5_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL6: u32 = 0x000e;
pub const regTHM_TCON_LOCAL6_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL7: u32 = 0x000f;
pub const regTHM_TCON_LOCAL7_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL8: u32 = 0x0010;
pub const regTHM_TCON_LOCAL8_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL9: u32 = 0x0011;
pub const regTHM_TCON_LOCAL9_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL10: u32 = 0x0012;
pub const regTHM_TCON_LOCAL10_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL11: u32 = 0x0013;
pub const regTHM_TCON_LOCAL11_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL12: u32 = 0x0014;
pub const regTHM_TCON_LOCAL12_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL13: u32 = 0x0015;
pub const regTHM_TCON_LOCAL13_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL14: u32 = 0x0016;
pub const regTHM_TCON_LOCAL14_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL15: u32 = 0x0017;
pub const regTHM_TCON_LOCAL15_BASE_IDX: u32 = 0;
pub const regTHM_PWRMGT: u32 = 0x001b;
pub const regTHM_PWRMGT_BASE_IDX: u32 = 0;
pub const regTHM_DIE1_TEMP: u32 = 0x001c;
pub const regTHM_DIE1_TEMP_BASE_IDX: u32 = 0;
pub const regTHM_DIE2_TEMP: u32 = 0x001d;
pub const regTHM_DIE2_TEMP_BASE_IDX: u32 = 0;
pub const regTHM_DIE3_TEMP: u32 = 0x001e;
pub const regTHM_DIE3_TEMP_BASE_IDX: u32 = 0;
pub const regSMUSBI_SBIREGADDR: u32 = 0x0124;
pub const regSMUSBI_SBIREGADDR_BASE_IDX: u32 = 0;
pub const regSMUSBI_SBIREGDATA: u32 = 0x0125;
pub const regSMUSBI_SBIREGDATA_BASE_IDX: u32 = 0;
pub const regSMUSBI_ERRATA_STAT_REG: u32 = 0x0129;
pub const regSMUSBI_ERRATA_STAT_REG_BASE_IDX: u32 = 0;
pub const regSMUSBI_SBICTRL: u32 = 0x012a;
pub const regSMUSBI_SBICTRL_BASE_IDX: u32 = 0;
pub const regSMUSBI_CKNBIRESET: u32 = 0x012b;
pub const regSMUSBI_CKNBIRESET_BASE_IDX: u32 = 0;
pub const regSMUSBI_TIMING: u32 = 0x012c;
pub const regSMUSBI_TIMING_BASE_IDX: u32 = 0;
pub const regSMUSBI_HS_TIMING: u32 = 0x012d;
pub const regSMUSBI_HS_TIMING_BASE_IDX: u32 = 0;
pub const regSBTSI_REMOTE_TEMP: u32 = 0x012e;
pub const regSBTSI_REMOTE_TEMP_BASE_IDX: u32 = 0;
pub const regSBRMI_CONTROL: u32 = 0x012f;
pub const regSBRMI_CONTROL_BASE_IDX: u32 = 0;
pub const regSBRMI_COMMAND: u32 = 0x0130;
pub const regSBRMI_COMMAND_BASE_IDX: u32 = 0;
pub const regSBRMI_WRITE_DATA0: u32 = 0x0132;
pub const regSBRMI_WRITE_DATA0_BASE_IDX: u32 = 0;
pub const regSBRMI_WRITE_DATA1: u32 = 0x0133;
pub const regSBRMI_WRITE_DATA1_BASE_IDX: u32 = 0;
pub const regSBRMI_WRITE_DATA2: u32 = 0x0134;
pub const regSBRMI_WRITE_DATA2_BASE_IDX: u32 = 0;
pub const regSBRMI_READ_DATA0: u32 = 0x0136;
pub const regSBRMI_READ_DATA0_BASE_IDX: u32 = 0;
pub const regSBRMI_READ_DATA1: u32 = 0x0137;
pub const regSBRMI_READ_DATA1_BASE_IDX: u32 = 0;
pub const regSBRMI_CORE_EN_NUMBER: u32 = 0x0138;
pub const regSBRMI_CORE_EN_NUMBER_BASE_IDX: u32 = 0;
pub const regSBRMI_CORE_EN_STATUS0: u32 = 0x0139;
pub const regSBRMI_CORE_EN_STATUS0_BASE_IDX: u32 = 0;
pub const regSBRMI_CORE_EN_STATUS1: u32 = 0x013a;
pub const regSBRMI_CORE_EN_STATUS1_BASE_IDX: u32 = 0;
pub const regSBRMI_APIC_STATUS0: u32 = 0x013b;
pub const regSBRMI_APIC_STATUS0_BASE_IDX: u32 = 0;
pub const regSBRMI_APIC_STATUS1: u32 = 0x013c;
pub const regSBRMI_APIC_STATUS1_BASE_IDX: u32 = 0;
pub const regSBRMI_MCE_STATUS0: u32 = 0x013d;
pub const regSBRMI_MCE_STATUS0_BASE_IDX: u32 = 0;
pub const regSBRMI_MCE_STATUS1: u32 = 0x013e;
pub const regSBRMI_MCE_STATUS1_BASE_IDX: u32 = 0;
pub const regSMBUS_CNTL0: u32 = 0x013f;
pub const regSMBUS_CNTL0_BASE_IDX: u32 = 0;
pub const regSMBUS_CNTL1: u32 = 0x0140;
pub const regSMBUS_CNTL1_BASE_IDX: u32 = 0;
pub const regSMBUS_BLKWR_CMD_CTRL0: u32 = 0x0141;
pub const regSMBUS_BLKWR_CMD_CTRL0_BASE_IDX: u32 = 0;
pub const regSMBUS_BLKWR_CMD_CTRL1: u32 = 0x0142;
pub const regSMBUS_BLKWR_CMD_CTRL1_BASE_IDX: u32 = 0;
pub const regSMBUS_BLKRD_CMD_CTRL0: u32 = 0x0143;
pub const regSMBUS_BLKRD_CMD_CTRL0_BASE_IDX: u32 = 0;
pub const regSMBUS_BLKRD_CMD_CTRL1: u32 = 0x0144;
pub const regSMBUS_BLKRD_CMD_CTRL1_BASE_IDX: u32 = 0;
pub const regSMBUS_TIMING_CNTL0: u32 = 0x0145;
pub const regSMBUS_TIMING_CNTL0_BASE_IDX: u32 = 0;
pub const regSMBUS_TIMING_CNTL1: u32 = 0x0146;
pub const regSMBUS_TIMING_CNTL1_BASE_IDX: u32 = 0;
pub const regSMBUS_TIMING_CNTL2: u32 = 0x0147;
pub const regSMBUS_TIMING_CNTL2_BASE_IDX: u32 = 0;
pub const regSMBUS_TRIGGER_CNTL: u32 = 0x0148;
pub const regSMBUS_TRIGGER_CNTL_BASE_IDX: u32 = 0;
pub const regSMBUS_UDID_CNTL0: u32 = 0x0149;
pub const regSMBUS_UDID_CNTL0_BASE_IDX: u32 = 0;
pub const regSMBUS_UDID_CNTL1: u32 = 0x014a;
pub const regSMBUS_UDID_CNTL1_BASE_IDX: u32 = 0;
pub const regSMBUS_UDID_CNTL2: u32 = 0x014b;
pub const regSMBUS_UDID_CNTL2_BASE_IDX: u32 = 0;
pub const regSMUSBI_SMBUS: u32 = 0x014c;
pub const regSMUSBI_SMBUS_BASE_IDX: u32 = 0;
pub const regSMUSBI_ALERT: u32 = 0x014d;
pub const regSMUSBI_ALERT_BASE_IDX: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
