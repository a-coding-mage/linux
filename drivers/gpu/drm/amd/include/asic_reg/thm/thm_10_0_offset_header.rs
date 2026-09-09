+/*
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



// addressBlock: thm_thm_SmuThmDec
// base address: 0x59800
pub const mmTHM_TCON_CUR_TMP: u32 = 0x0000;
pub const mmTHM_TCON_CUR_TMP_BASE_IDX: u32 = 0;
pub const mmTHM_TCON_HTC: u32 = 0x0001;
pub const mmTHM_TCON_HTC_BASE_IDX: u32 = 0;
pub const mmTHM_TCON_THERM_TRIP: u32 = 0x0002;
pub const mmTHM_TCON_THERM_TRIP_BASE_IDX: u32 = 0;
pub const mmTHM_CTF_DELAY: u32 = 0x0003;
pub const mmTHM_CTF_DELAY_BASE_IDX: u32 = 0;
pub const mmTHM_GPIO_PROCHOT_CTRL: u32 = 0x0004;
pub const mmTHM_GPIO_PROCHOT_CTRL_BASE_IDX: u32 = 0;
pub const mmTHM_THERMAL_INT_ENA: u32 = 0x000a;
pub const mmTHM_THERMAL_INT_ENA_BASE_IDX: u32 = 0;
pub const mmTHM_THERMAL_INT_CTRL: u32 = 0x000b;
pub const mmTHM_THERMAL_INT_CTRL_BASE_IDX: u32 = 0;
pub const mmTHM_THERMAL_INT_STATUS: u32 = 0x000c;
pub const mmTHM_THERMAL_INT_STATUS_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIL0_DATA: u32 = 0x000d;
pub const mmTHM_TMON0_RDIL0_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIL1_DATA: u32 = 0x000e;
pub const mmTHM_TMON0_RDIL1_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIL2_DATA: u32 = 0x000f;
pub const mmTHM_TMON0_RDIL2_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIL3_DATA: u32 = 0x0010;
pub const mmTHM_TMON0_RDIL3_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIL4_DATA: u32 = 0x0011;
pub const mmTHM_TMON0_RDIL4_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIL5_DATA: u32 = 0x0012;
pub const mmTHM_TMON0_RDIL5_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIL6_DATA: u32 = 0x0013;
pub const mmTHM_TMON0_RDIL6_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIL7_DATA: u32 = 0x0014;
pub const mmTHM_TMON0_RDIL7_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIL8_DATA: u32 = 0x0015;
pub const mmTHM_TMON0_RDIL8_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIL9_DATA: u32 = 0x0016;
pub const mmTHM_TMON0_RDIL9_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIL10_DATA: u32 = 0x0017;
pub const mmTHM_TMON0_RDIL10_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIL11_DATA: u32 = 0x0018;
pub const mmTHM_TMON0_RDIL11_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIL12_DATA: u32 = 0x0019;
pub const mmTHM_TMON0_RDIL12_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIL13_DATA: u32 = 0x001a;
pub const mmTHM_TMON0_RDIL13_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIL14_DATA: u32 = 0x001b;
pub const mmTHM_TMON0_RDIL14_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIL15_DATA: u32 = 0x001c;
pub const mmTHM_TMON0_RDIL15_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIR0_DATA: u32 = 0x001d;
pub const mmTHM_TMON0_RDIR0_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIR1_DATA: u32 = 0x001e;
pub const mmTHM_TMON0_RDIR1_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIR2_DATA: u32 = 0x001f;
pub const mmTHM_TMON0_RDIR2_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIR3_DATA: u32 = 0x0020;
pub const mmTHM_TMON0_RDIR3_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIR4_DATA: u32 = 0x0021;
pub const mmTHM_TMON0_RDIR4_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIR5_DATA: u32 = 0x0022;
pub const mmTHM_TMON0_RDIR5_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIR6_DATA: u32 = 0x0023;
pub const mmTHM_TMON0_RDIR6_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIR7_DATA: u32 = 0x0024;
pub const mmTHM_TMON0_RDIR7_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIR8_DATA: u32 = 0x0025;
pub const mmTHM_TMON0_RDIR8_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIR9_DATA: u32 = 0x0026;
pub const mmTHM_TMON0_RDIR9_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIR10_DATA: u32 = 0x0027;
pub const mmTHM_TMON0_RDIR10_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIR11_DATA: u32 = 0x0028;
pub const mmTHM_TMON0_RDIR11_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIR12_DATA: u32 = 0x0029;
pub const mmTHM_TMON0_RDIR12_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIR13_DATA: u32 = 0x002a;
pub const mmTHM_TMON0_RDIR13_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIR14_DATA: u32 = 0x002b;
pub const mmTHM_TMON0_RDIR14_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_RDIR15_DATA: u32 = 0x002c;
pub const mmTHM_TMON0_RDIR15_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_INT_DATA: u32 = 0x002d;
pub const mmTHM_TMON0_INT_DATA_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_CTRL: u32 = 0x002e;
pub const mmTHM_TMON0_CTRL_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_CTRL2: u32 = 0x002f;
pub const mmTHM_TMON0_CTRL2_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_DEBUG: u32 = 0x0030;
pub const mmTHM_TMON0_DEBUG_BASE_IDX: u32 = 0;
pub const mmTHM_DIE1_TEMP: u32 = 0x0055;
pub const mmTHM_DIE1_TEMP_BASE_IDX: u32 = 0;
pub const mmTHM_DIE2_TEMP: u32 = 0x0056;
pub const mmTHM_DIE2_TEMP_BASE_IDX: u32 = 0;
pub const mmTHM_DIE3_TEMP: u32 = 0x0057;
pub const mmTHM_DIE3_TEMP_BASE_IDX: u32 = 0;
pub const mmTHM_SW_TEMP: u32 = 0x0058;
pub const mmTHM_SW_TEMP_BASE_IDX: u32 = 0;
pub const mmCG_MULT_THERMAL_CTRL: u32 = 0x0059;
pub const mmCG_MULT_THERMAL_CTRL_BASE_IDX: u32 = 0;
pub const mmCG_MULT_THERMAL_STATUS: u32 = 0x005a;
pub const mmCG_MULT_THERMAL_STATUS_BASE_IDX: u32 = 0;
pub const mmCG_THERMAL_RANGE: u32 = 0x005b;
pub const mmCG_THERMAL_RANGE_BASE_IDX: u32 = 0;
pub const mmTHM_TMON_CONFIG: u32 = 0x005c;
pub const mmTHM_TMON_CONFIG_BASE_IDX: u32 = 0;
pub const mmTHM_TMON_CONFIG2: u32 = 0x005d;
pub const mmTHM_TMON_CONFIG2_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_COEFF: u32 = 0x005e;
pub const mmTHM_TMON0_COEFF_BASE_IDX: u32 = 0;
pub const mmTHM_TCON_LOCAL0: u32 = 0x006e;
pub const mmTHM_TCON_LOCAL0_BASE_IDX: u32 = 0;
pub const mmTHM_TCON_LOCAL1: u32 = 0x006f;
pub const mmTHM_TCON_LOCAL1_BASE_IDX: u32 = 0;
pub const mmTHM_TCON_LOCAL2: u32 = 0x0070;
pub const mmTHM_TCON_LOCAL2_BASE_IDX: u32 = 0;
pub const mmTHM_TCON_LOCAL3: u32 = 0x0071;
pub const mmTHM_TCON_LOCAL3_BASE_IDX: u32 = 0;
pub const mmTHM_TCON_LOCAL4: u32 = 0x0072;
pub const mmTHM_TCON_LOCAL4_BASE_IDX: u32 = 0;
pub const mmTHM_TCON_LOCAL5: u32 = 0x0073;
pub const mmTHM_TCON_LOCAL5_BASE_IDX: u32 = 0;
pub const mmTHM_TCON_LOCAL6: u32 = 0x0074;
pub const mmTHM_TCON_LOCAL6_BASE_IDX: u32 = 0;
pub const mmTHM_TCON_LOCAL7: u32 = 0x0075;
pub const mmTHM_TCON_LOCAL7_BASE_IDX: u32 = 0;
pub const mmTHM_TCON_LOCAL8: u32 = 0x0076;
pub const mmTHM_TCON_LOCAL8_BASE_IDX: u32 = 0;
pub const mmTHM_TCON_LOCAL9: u32 = 0x0077;
pub const mmTHM_TCON_LOCAL9_BASE_IDX: u32 = 0;
pub const mmTHM_TCON_LOCAL10: u32 = 0x0078;
pub const mmTHM_TCON_LOCAL10_BASE_IDX: u32 = 0;
pub const mmTHM_TCON_LOCAL11: u32 = 0x0079;
pub const mmTHM_TCON_LOCAL11_BASE_IDX: u32 = 0;
pub const mmTHM_TCON_LOCAL12: u32 = 0x007a;
pub const mmTHM_TCON_LOCAL12_BASE_IDX: u32 = 0;
pub const mmTHM_TCON_LOCAL13: u32 = 0x007b;
pub const mmTHM_TCON_LOCAL13_BASE_IDX: u32 = 0;
pub const mmTHM_PWRMGT: u32 = 0x007d;
pub const mmTHM_PWRMGT_BASE_IDX: u32 = 0;
pub const mmSMUSBI_SBIREGADDR: u32 = 0x0080;
pub const mmSMUSBI_SBIREGADDR_BASE_IDX: u32 = 0;
pub const mmSMUSBI_SBIREGDATA: u32 = 0x0081;
pub const mmSMUSBI_SBIREGDATA_BASE_IDX: u32 = 0;
pub const mmSMUSBI_ERRATA_STAT_REG: u32 = 0x0085;
pub const mmSMUSBI_ERRATA_STAT_REG_BASE_IDX: u32 = 0;
pub const mmSMUSBI_SBICTRL: u32 = 0x0086;
pub const mmSMUSBI_SBICTRL_BASE_IDX: u32 = 0;
pub const mmSMUSBI_CKNBIRESET: u32 = 0x0087;
pub const mmSMUSBI_CKNBIRESET_BASE_IDX: u32 = 0;
pub const mmSMUSBI_TIMING: u32 = 0x0088;
pub const mmSMUSBI_TIMING_BASE_IDX: u32 = 0;
pub const mmSMUSBI_HS_TIMING: u32 = 0x0089;
pub const mmSMUSBI_HS_TIMING_BASE_IDX: u32 = 0;
pub const mmSBTSI_REMOTE_TEMP: u32 = 0x008a;
pub const mmSBTSI_REMOTE_TEMP_BASE_IDX: u32 = 0;
pub const mmSBRMI_CONTROL: u32 = 0x008b;
pub const mmSBRMI_CONTROL_BASE_IDX: u32 = 0;
pub const mmSBRMI_COMMAND: u32 = 0x008c;
pub const mmSBRMI_COMMAND_BASE_IDX: u32 = 0;
pub const mmSBRMI_WRITE_DATA0: u32 = 0x008d;
pub const mmSBRMI_WRITE_DATA0_BASE_IDX: u32 = 0;
pub const mmSBRMI_WRITE_DATA1: u32 = 0x008e;
pub const mmSBRMI_WRITE_DATA1_BASE_IDX: u32 = 0;
pub const mmSBRMI_WRITE_DATA2: u32 = 0x008f;
pub const mmSBRMI_WRITE_DATA2_BASE_IDX: u32 = 0;
pub const mmSBRMI_READ_DATA0: u32 = 0x0090;
pub const mmSBRMI_READ_DATA0_BASE_IDX: u32 = 0;
pub const mmSBRMI_READ_DATA1: u32 = 0x0091;
pub const mmSBRMI_READ_DATA1_BASE_IDX: u32 = 0;
pub const mmSBRMI_CORE_EN_NUMBER: u32 = 0x0092;
pub const mmSBRMI_CORE_EN_NUMBER_BASE_IDX: u32 = 0;
pub const mmSBRMI_CORE_EN_STATUS0: u32 = 0x0093;
pub const mmSBRMI_CORE_EN_STATUS0_BASE_IDX: u32 = 0;
pub const mmSBRMI_CORE_EN_STATUS1: u32 = 0x0094;
pub const mmSBRMI_CORE_EN_STATUS1_BASE_IDX: u32 = 0;
pub const mmSBRMI_APIC_STATUS0: u32 = 0x0095;
pub const mmSBRMI_APIC_STATUS0_BASE_IDX: u32 = 0;
pub const mmSBRMI_APIC_STATUS1: u32 = 0x0096;
pub const mmSBRMI_APIC_STATUS1_BASE_IDX: u32 = 0;
pub const mmSBRMI_MCE_STATUS0: u32 = 0x0097;
pub const mmSBRMI_MCE_STATUS0_BASE_IDX: u32 = 0;
pub const mmSBRMI_MCE_STATUS1: u32 = 0x0098;
pub const mmSBRMI_MCE_STATUS1_BASE_IDX: u32 = 0;
pub const mmSMBUS_CNTL0: u32 = 0x0099;
pub const mmSMBUS_CNTL0_BASE_IDX: u32 = 0;
pub const mmSMBUS_CNTL1: u32 = 0x009a;
pub const mmSMBUS_CNTL1_BASE_IDX: u32 = 0;
pub const mmSMBUS_BLKWR_CMD_CTRL0: u32 = 0x009b;
pub const mmSMBUS_BLKWR_CMD_CTRL0_BASE_IDX: u32 = 0;
pub const mmSMBUS_BLKWR_CMD_CTRL1: u32 = 0x009c;
pub const mmSMBUS_BLKWR_CMD_CTRL1_BASE_IDX: u32 = 0;
pub const mmSMBUS_BLKRD_CMD_CTRL0: u32 = 0x009d;
pub const mmSMBUS_BLKRD_CMD_CTRL0_BASE_IDX: u32 = 0;
pub const mmSMBUS_BLKRD_CMD_CTRL1: u32 = 0x009e;
pub const mmSMBUS_BLKRD_CMD_CTRL1_BASE_IDX: u32 = 0;
pub const mmSMBUS_TIMING_CNTL0: u32 = 0x009f;
pub const mmSMBUS_TIMING_CNTL0_BASE_IDX: u32 = 0;
pub const mmSMBUS_TIMING_CNTL1: u32 = 0x00a0;
pub const mmSMBUS_TIMING_CNTL1_BASE_IDX: u32 = 0;
pub const mmSMBUS_TIMING_CNTL2: u32 = 0x00a1;
pub const mmSMBUS_TIMING_CNTL2_BASE_IDX: u32 = 0;
pub const mmSMBUS_TRIGGER_CNTL: u32 = 0x00a2;
pub const mmSMBUS_TRIGGER_CNTL_BASE_IDX: u32 = 0;
pub const mmSMBUS_UDID_CNTL0: u32 = 0x00a3;
pub const mmSMBUS_UDID_CNTL0_BASE_IDX: u32 = 0;
pub const mmSMBUS_UDID_CNTL1: u32 = 0x00a4;
pub const mmSMBUS_UDID_CNTL1_BASE_IDX: u32 = 0;
pub const mmSMBUS_UDID_CNTL2: u32 = 0x00a5;
pub const mmSMBUS_UDID_CNTL2_BASE_IDX: u32 = 0;
pub const mmSMUSBI_SMBUS: u32 = 0x00a6;
pub const mmSMUSBI_SMBUS_BASE_IDX: u32 = 0;
pub const mmSMUSBI_ALERT: u32 = 0x00a7;
pub const mmSMUSBI_ALERT_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_REMOTE_START: u32 = 0x0100;
pub const mmTHM_TMON0_REMOTE_START_BASE_IDX: u32 = 0;
pub const mmTHM_TMON0_REMOTE_END: u32 = 0x013f;
pub const mmTHM_TMON0_REMOTE_END_BASE_IDX: u32 = 0;
pub const mmTHM_TMON1_REMOTE_START: u32 = 0x0140;
pub const mmTHM_TMON1_REMOTE_START_BASE_IDX: u32 = 0;
pub const mmTHM_TMON1_REMOTE_END: u32 = 0x017f;
pub const mmTHM_TMON1_REMOTE_END_BASE_IDX: u32 = 0;
pub const mmTHM_TMON2_REMOTE_START: u32 = 0x0180;
pub const mmTHM_TMON2_REMOTE_START_BASE_IDX: u32 = 0;
pub const mmTHM_TMON2_REMOTE_END: u32 = 0x01bf;
pub const mmTHM_TMON2_REMOTE_END_BASE_IDX: u32 = 0;
pub const mmTHM_TMON3_REMOTE_START: u32 = 0x01c0;
pub const mmTHM_TMON3_REMOTE_START_BASE_IDX: u32 = 0;
pub const mmTHM_TMON3_REMOTE_END: u32 = 0x01ff;
pub const mmTHM_TMON3_REMOTE_END_BASE_IDX: u32 = 0;



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
