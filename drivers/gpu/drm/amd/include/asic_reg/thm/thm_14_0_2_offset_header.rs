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
 *
 */



// addressBlock: thm_thm_SmuThmDec
// base address: 0x59800
pub const regTHM_TCON_CUR_TMP: u32 = 0x0000;
pub const regTHM_TCON_CUR_TMP_BASE_IDX: u32 = 0;
pub const regTHM_TCON_HTC: u32 = 0x0001;
pub const regTHM_TCON_HTC_BASE_IDX: u32 = 0;
pub const regTHM_TCON_THERM_TRIP: u32 = 0x0002;
pub const regTHM_TCON_THERM_TRIP_BASE_IDX: u32 = 0;
pub const regTHM_CTF_DELAY: u32 = 0x0003;
pub const regTHM_CTF_DELAY_BASE_IDX: u32 = 0;
pub const regTHM_GPIO_PROCHOT_CTRL: u32 = 0x0004;
pub const regTHM_GPIO_PROCHOT_CTRL_BASE_IDX: u32 = 0;
pub const regTHM_GPIO_THERMTRIP_CTRL: u32 = 0x0005;
pub const regTHM_GPIO_THERMTRIP_CTRL_BASE_IDX: u32 = 0;
pub const regTHM_GPIO_PWM_CTRL: u32 = 0x0006;
pub const regTHM_GPIO_PWM_CTRL_BASE_IDX: u32 = 0;
pub const regTHM_GPIO_TACHIN_CTRL: u32 = 0x0007;
pub const regTHM_GPIO_TACHIN_CTRL_BASE_IDX: u32 = 0;
pub const regTHM_GPIO_PUMPOUT_CTRL: u32 = 0x0008;
pub const regTHM_GPIO_PUMPOUT_CTRL_BASE_IDX: u32 = 0;
pub const regTHM_GPIO_PUMPIN_CTRL: u32 = 0x0009;
pub const regTHM_GPIO_PUMPIN_CTRL_BASE_IDX: u32 = 0;
pub const regTHM_THERMAL_INT_ENA: u32 = 0x000a;
pub const regTHM_THERMAL_INT_ENA_BASE_IDX: u32 = 0;
pub const regTHM_THERMAL_INT_CTRL: u32 = 0x000b;
pub const regTHM_THERMAL_INT_CTRL_BASE_IDX: u32 = 0;
pub const regTHM_THERMAL_INT_STATUS: u32 = 0x000c;
pub const regTHM_THERMAL_INT_STATUS_BASE_IDX: u32 = 0;
pub const regTHM_SW_TEMP: u32 = 0x000d;
pub const regTHM_SW_TEMP_BASE_IDX: u32 = 0;
pub const regCG_MULT_THERMAL_CTRL: u32 = 0x000e;
pub const regCG_MULT_THERMAL_CTRL_BASE_IDX: u32 = 0;
pub const regCG_MULT_THERMAL_STATUS: u32 = 0x000f;
pub const regCG_MULT_THERMAL_STATUS_BASE_IDX: u32 = 0;
pub const regCG_THERMAL_RANGE: u32 = 0x0010;
pub const regCG_THERMAL_RANGE_BASE_IDX: u32 = 0;
pub const regCG_FDO_CTRL0: u32 = 0x0011;
pub const regCG_FDO_CTRL0_BASE_IDX: u32 = 0;
pub const regCG_FDO_CTRL1: u32 = 0x0012;
pub const regCG_FDO_CTRL1_BASE_IDX: u32 = 0;
pub const regCG_FDO_CTRL2: u32 = 0x0013;
pub const regCG_FDO_CTRL2_BASE_IDX: u32 = 0;
pub const regCG_TACH_CTRL: u32 = 0x0014;
pub const regCG_TACH_CTRL_BASE_IDX: u32 = 0;
pub const regCG_TACH_STATUS: u32 = 0x0015;
pub const regCG_TACH_STATUS_BASE_IDX: u32 = 0;
pub const regCG_THERMAL_STATUS: u32 = 0x0016;
pub const regCG_THERMAL_STATUS_BASE_IDX: u32 = 0;
pub const regCG_PUMP_CTRL0: u32 = 0x0017;
pub const regCG_PUMP_CTRL0_BASE_IDX: u32 = 0;
pub const regCG_PUMP_CTRL1: u32 = 0x0018;
pub const regCG_PUMP_CTRL1_BASE_IDX: u32 = 0;
pub const regCG_PUMP_CTRL2: u32 = 0x0019;
pub const regCG_PUMP_CTRL2_BASE_IDX: u32 = 0;
pub const regCG_PUMP_TACH_CTRL: u32 = 0x001a;
pub const regCG_PUMP_TACH_CTRL_BASE_IDX: u32 = 0;
pub const regCG_PUMP_TACH_STATUS: u32 = 0x001b;
pub const regCG_PUMP_TACH_STATUS_BASE_IDX: u32 = 0;
pub const regCG_PUMP_STATUS: u32 = 0x001c;
pub const regCG_PUMP_STATUS_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL2: u32 = 0x001d;
pub const regTHM_TCON_LOCAL2_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL3: u32 = 0x001e;
pub const regTHM_TCON_LOCAL3_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL4: u32 = 0x001f;
pub const regTHM_TCON_LOCAL4_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL5: u32 = 0x0020;
pub const regTHM_TCON_LOCAL5_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL6: u32 = 0x0021;
pub const regTHM_TCON_LOCAL6_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL7: u32 = 0x0022;
pub const regTHM_TCON_LOCAL7_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL8: u32 = 0x0023;
pub const regTHM_TCON_LOCAL8_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL9: u32 = 0x0024;
pub const regTHM_TCON_LOCAL9_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL10: u32 = 0x0025;
pub const regTHM_TCON_LOCAL10_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL11: u32 = 0x0026;
pub const regTHM_TCON_LOCAL11_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL12: u32 = 0x0027;
pub const regTHM_TCON_LOCAL12_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL13: u32 = 0x0028;
pub const regTHM_TCON_LOCAL13_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL14: u32 = 0x0029;
pub const regTHM_TCON_LOCAL14_BASE_IDX: u32 = 0;
pub const regTHM_TCON_LOCAL15: u32 = 0x002a;
pub const regTHM_TCON_LOCAL15_BASE_IDX: u32 = 0;
pub const regTHM_BACO_CNTL: u32 = 0x002d;
pub const regTHM_BACO_CNTL_BASE_IDX: u32 = 0;
pub const regTHM_BACO_TIMING0: u32 = 0x002e;
pub const regTHM_BACO_TIMING0_BASE_IDX: u32 = 0;
pub const regTHM_BACO_TIMING1: u32 = 0x002f;
pub const regTHM_BACO_TIMING1_BASE_IDX: u32 = 0;
pub const regTHM_BACO_TIMING2: u32 = 0x0030;
pub const regTHM_BACO_TIMING2_BASE_IDX: u32 = 0;
pub const regTHM_BACO_TIMING: u32 = 0x0031;
pub const regTHM_BACO_TIMING_BASE_IDX: u32 = 0;
pub const regXTAL_CNTL: u32 = 0x0032;
pub const regXTAL_CNTL_BASE_IDX: u32 = 0;
pub const regTHM_PWRMGT: u32 = 0x0033;
pub const regTHM_PWRMGT_BASE_IDX: u32 = 0;
pub const regSMUSBI_SBIREGADDR: u32 = 0x0158;
pub const regSMUSBI_SBIREGADDR_BASE_IDX: u32 = 0;
pub const regSMUSBI_SBIREGDATA: u32 = 0x0159;
pub const regSMUSBI_SBIREGDATA_BASE_IDX: u32 = 0;
pub const regSMUSBI_ERRATA_STAT_REG: u32 = 0x015d;
pub const regSMUSBI_ERRATA_STAT_REG_BASE_IDX: u32 = 0;
pub const regSMUSBI_SBICTRL: u32 = 0x015e;
pub const regSMUSBI_SBICTRL_BASE_IDX: u32 = 0;
pub const regSMUSBI_CKNBIRESET: u32 = 0x015f;
pub const regSMUSBI_CKNBIRESET_BASE_IDX: u32 = 0;
pub const regSMUSBI_TIMING: u32 = 0x0160;
pub const regSMUSBI_TIMING_BASE_IDX: u32 = 0;
pub const regSMUSBI_HS_TIMING: u32 = 0x0161;
pub const regSMUSBI_HS_TIMING_BASE_IDX: u32 = 0;
pub const regSBTSI_REMOTE_TEMP: u32 = 0x0162;
pub const regSBTSI_REMOTE_TEMP_BASE_IDX: u32 = 0;
pub const regSBRMI_CONTROL: u32 = 0x0163;
pub const regSBRMI_CONTROL_BASE_IDX: u32 = 0;
pub const regSBRMI_COMMAND: u32 = 0x0164;
pub const regSBRMI_COMMAND_BASE_IDX: u32 = 0;
pub const regSBRMI_WRITE_DATA0: u32 = 0x0166;
pub const regSBRMI_WRITE_DATA0_BASE_IDX: u32 = 0;
pub const regSBRMI_WRITE_DATA1: u32 = 0x0167;
pub const regSBRMI_WRITE_DATA1_BASE_IDX: u32 = 0;
pub const regSBRMI_WRITE_DATA2: u32 = 0x0168;
pub const regSBRMI_WRITE_DATA2_BASE_IDX: u32 = 0;
pub const regSBRMI_READ_DATA0: u32 = 0x016a;
pub const regSBRMI_READ_DATA0_BASE_IDX: u32 = 0;
pub const regSBRMI_READ_DATA1: u32 = 0x016b;
pub const regSBRMI_READ_DATA1_BASE_IDX: u32 = 0;
pub const regSBRMI_CORE_EN_NUMBER: u32 = 0x016c;
pub const regSBRMI_CORE_EN_NUMBER_BASE_IDX: u32 = 0;
pub const regSBRMI_CORE_EN_STATUS0: u32 = 0x016d;
pub const regSBRMI_CORE_EN_STATUS0_BASE_IDX: u32 = 0;
pub const regSBRMI_CORE_EN_STATUS1: u32 = 0x016e;
pub const regSBRMI_CORE_EN_STATUS1_BASE_IDX: u32 = 0;
pub const regSBRMI_APIC_STATUS0: u32 = 0x016f;
pub const regSBRMI_APIC_STATUS0_BASE_IDX: u32 = 0;
pub const regSBRMI_APIC_STATUS1: u32 = 0x0170;
pub const regSBRMI_APIC_STATUS1_BASE_IDX: u32 = 0;
pub const regSBRMI_MCE_STATUS0: u32 = 0x0171;
pub const regSBRMI_MCE_STATUS0_BASE_IDX: u32 = 0;
pub const regSBRMI_MCE_STATUS1: u32 = 0x0172;
pub const regSBRMI_MCE_STATUS1_BASE_IDX: u32 = 0;
pub const regSMBUS_CNTL0: u32 = 0x0173;
pub const regSMBUS_CNTL0_BASE_IDX: u32 = 0;
pub const regSMBUS_CNTL1: u32 = 0x0174;
pub const regSMBUS_CNTL1_BASE_IDX: u32 = 0;
pub const regSMBUS_BLKWR_CMD_CTRL0: u32 = 0x0175;
pub const regSMBUS_BLKWR_CMD_CTRL0_BASE_IDX: u32 = 0;
pub const regSMBUS_BLKWR_CMD_CTRL1: u32 = 0x0176;
pub const regSMBUS_BLKWR_CMD_CTRL1_BASE_IDX: u32 = 0;
pub const regSMBUS_BLKRD_CMD_CTRL0: u32 = 0x0177;
pub const regSMBUS_BLKRD_CMD_CTRL0_BASE_IDX: u32 = 0;
pub const regSMBUS_BLKRD_CMD_CTRL1: u32 = 0x0178;
pub const regSMBUS_BLKRD_CMD_CTRL1_BASE_IDX: u32 = 0;
pub const regSMBUS_TIMING_CNTL0: u32 = 0x0179;
pub const regSMBUS_TIMING_CNTL0_BASE_IDX: u32 = 0;
pub const regSMBUS_TIMING_CNTL1: u32 = 0x017a;
pub const regSMBUS_TIMING_CNTL1_BASE_IDX: u32 = 0;
pub const regSMBUS_TIMING_CNTL2: u32 = 0x017b;
pub const regSMBUS_TIMING_CNTL2_BASE_IDX: u32 = 0;
pub const regSMBUS_TRIGGER_CNTL: u32 = 0x017c;
pub const regSMBUS_TRIGGER_CNTL_BASE_IDX: u32 = 0;
pub const regSMBUS_UDID_CNTL0: u32 = 0x017d;
pub const regSMBUS_UDID_CNTL0_BASE_IDX: u32 = 0;
pub const regSMBUS_UDID_CNTL1: u32 = 0x017e;
pub const regSMBUS_UDID_CNTL1_BASE_IDX: u32 = 0;
pub const regSMBUS_UDID_CNTL2: u32 = 0x017f;
pub const regSMBUS_UDID_CNTL2_BASE_IDX: u32 = 0;
pub const regSMUSBI_SMBUS: u32 = 0x0180;
pub const regSMUSBI_SMBUS_BASE_IDX: u32 = 0;
pub const regSMUSBI_ALERT: u32 = 0x0181;
pub const regSMUSBI_ALERT_BASE_IDX: u32 = 0;
pub const regSMBUS_BACO_DUMMY: u32 = 0x0182;
pub const regSMBUS_BACO_DUMMY_BASE_IDX: u32 = 0;
pub const regSMBUS_BACO_ADDR_RANGE0_LOW: u32 = 0x0183;
pub const regSMBUS_BACO_ADDR_RANGE0_LOW_BASE_IDX: u32 = 0;
pub const regSMBUS_BACO_ADDR_RANGE0_HIGH: u32 = 0x0184;
pub const regSMBUS_BACO_ADDR_RANGE0_HIGH_BASE_IDX: u32 = 0;
pub const regSMBUS_BACO_ADDR_RANGE1_LOW: u32 = 0x0185;
pub const regSMBUS_BACO_ADDR_RANGE1_LOW_BASE_IDX: u32 = 0;
pub const regSMBUS_BACO_ADDR_RANGE1_HIGH: u32 = 0x0186;
pub const regSMBUS_BACO_ADDR_RANGE1_HIGH_BASE_IDX: u32 = 0;
pub const regSMBUS_BACO_ADDR_RANGE2_LOW: u32 = 0x0187;
pub const regSMBUS_BACO_ADDR_RANGE2_LOW_BASE_IDX: u32 = 0;
pub const regSMBUS_BACO_ADDR_RANGE2_HIGH: u32 = 0x0188;
pub const regSMBUS_BACO_ADDR_RANGE2_HIGH_BASE_IDX: u32 = 0;
pub const regSMBUS_BACO_ADDR_RANGE3_LOW: u32 = 0x0189;
pub const regSMBUS_BACO_ADDR_RANGE3_LOW_BASE_IDX: u32 = 0;
pub const regSMBUS_BACO_ADDR_RANGE3_HIGH: u32 = 0x018a;
pub const regSMBUS_BACO_ADDR_RANGE3_HIGH_BASE_IDX: u32 = 0;
pub const regSMBUS_BACO_ADDR_RANGE4_LOW: u32 = 0x018b;
pub const regSMBUS_BACO_ADDR_RANGE4_LOW_BASE_IDX: u32 = 0;
pub const regSMBUS_BACO_ADDR_RANGE4_HIGH: u32 = 0x018c;
pub const regSMBUS_BACO_ADDR_RANGE4_HIGH_BASE_IDX: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
