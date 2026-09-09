/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * core.h  --  Core Driver for Wolfson WM8350 PMIC
 *
 * Copyright 2007 Wolfson Microelectronics PLC
 */
/*
 * Register values.
 */
pub const WM8350_RESET_ID: u32 = 0x00;
pub const WM8350_ID: u32 = 0x01;
pub const WM8350_REVISION: u32 = 0x02;
pub const WM8350_SYSTEM_CONTROL_1: u32 = 0x03;
pub const WM8350_SYSTEM_CONTROL_2: u32 = 0x04;
pub const WM8350_SYSTEM_HIBERNATE: u32 = 0x05;
pub const WM8350_INTERFACE_CONTROL: u32 = 0x06;
pub const WM8350_POWER_MGMT_1: u32 = 0x08;
pub const WM8350_POWER_MGMT_2: u32 = 0x09;
pub const WM8350_POWER_MGMT_3: u32 = 0x0A;
pub const WM8350_POWER_MGMT_4: u32 = 0x0B;
pub const WM8350_POWER_MGMT_5: u32 = 0x0C;
pub const WM8350_POWER_MGMT_6: u32 = 0x0D;
pub const WM8350_POWER_MGMT_7: u32 = 0x0E;
pub const WM8350_SYSTEM_INTERRUPTS: u32 = 0x18;
pub const WM8350_INT_STATUS_1: u32 = 0x19;
pub const WM8350_INT_STATUS_2: u32 = 0x1A;
pub const WM8350_POWER_UP_INT_STATUS: u32 = 0x1B;
pub const WM8350_UNDER_VOLTAGE_INT_STATUS: u32 = 0x1C;
pub const WM8350_OVER_CURRENT_INT_STATUS: u32 = 0x1D;
pub const WM8350_GPIO_INT_STATUS: u32 = 0x1E;
pub const WM8350_COMPARATOR_INT_STATUS: u32 = 0x1F;
pub const WM8350_SYSTEM_INTERRUPTS_MASK: u32 = 0x20;
pub const WM8350_INT_STATUS_1_MASK: u32 = 0x21;
pub const WM8350_INT_STATUS_2_MASK: u32 = 0x22;
pub const WM8350_POWER_UP_INT_STATUS_MASK: u32 = 0x23;
pub const WM8350_UNDER_VOLTAGE_INT_STATUS_MASK: u32 = 0x24;
pub const WM8350_OVER_CURRENT_INT_STATUS_MASK: u32 = 0x25;
pub const WM8350_GPIO_INT_STATUS_MASK: u32 = 0x26;
pub const WM8350_COMPARATOR_INT_STATUS_MASK: u32 = 0x27;
pub const WM8350_CHARGER_OVERRIDES: u32 = 0xE2;
pub const WM8350_MISC_OVERRIDES: u32 = 0xE3;
pub const WM8350_COMPARATOR_OVERRIDES: u32 = 0xE7;
pub const WM8350_STATE_MACHINE_STATUS: u32 = 0xE9;
pub const WM8350_MAX_REGISTER: u32 = 0xFF;
pub const WM8350_UNLOCK_KEY: u32 = 0x0013;
pub const WM8350_LOCK_KEY: u32 = 0x0000;
/*
 * Field Definitions.
 */
/*
 * R0 (0x00) - Reset/ID
 */
pub const WM8350_SW_RESET_CHIP_ID_MASK: u32 = 0xFFFF;
/*
 * R1 (0x01) - ID
 */
pub const WM8350_CHIP_REV_MASK: u32 = 0x7000;
pub const WM8350_CONF_STS_MASK: u32 = 0x0C00;
pub const WM8350_CUST_ID_MASK: u32 = 0x00FF;
/*
 * R2 (0x02) - Revision
 */
pub const WM8350_MASK_REV_MASK: u32 = 0x00FF;
/*
 * R3 (0x03) - System Control 1
 */
pub const WM8350_CHIP_ON: u32 = 0x8000;
pub const WM8350_POWERCYCLE: u32 = 0x2000;
pub const WM8350_VCC_FAULT_OV: u32 = 0x1000;
pub const WM8350_REG_RSTB_TIME_MASK: u32 = 0x0C00;
pub const WM8350_BG_SLEEP: u32 = 0x0200;
pub const WM8350_MEM_VALID: u32 = 0x0020;
pub const WM8350_CHIP_SET_UP: u32 = 0x0010;
pub const WM8350_ON_DEB_T: u32 = 0x0008;
pub const WM8350_ON_POL: u32 = 0x0002;
pub const WM8350_IRQ_POL: u32 = 0x0001;
/*
 * R4 (0x04) - System Control 2
 */
pub const WM8350_USB_SUSPEND_8MA: u32 = 0x8000;
pub const WM8350_USB_SUSPEND: u32 = 0x4000;
pub const WM8350_USB_MSTR: u32 = 0x2000;
pub const WM8350_USB_MSTR_SRC: u32 = 0x1000;
pub const WM8350_USB_500MA: u32 = 0x0800;
pub const WM8350_USB_NOLIM: u32 = 0x0400;
/*
 * R5 (0x05) - System Hibernate
 */
pub const WM8350_HIBERNATE: u32 = 0x8000;
pub const WM8350_WDOG_HIB_MODE: u32 = 0x0080;
pub const WM8350_REG_HIB_STARTUP_SEQ: u32 = 0x0040;
pub const WM8350_REG_RESET_HIB_MODE: u32 = 0x0020;
pub const WM8350_RST_HIB_MODE: u32 = 0x0010;
pub const WM8350_IRQ_HIB_MODE: u32 = 0x0008;
pub const WM8350_MEMRST_HIB_MODE: u32 = 0x0004;
pub const WM8350_PCCOMP_HIB_MODE: u32 = 0x0002;
pub const WM8350_TEMPMON_HIB_MODE: u32 = 0x0001;
/*
 * R6 (0x06) - Interface Control
 */
pub const WM8350_USE_DEV_PINS: u32 = 0x8000;
pub const WM8350_USE_DEV_PINS_MASK: u32 = 0x8000;
pub const WM8350_USE_DEV_PINS_SHIFT: u32 = 15;
pub const WM8350_DEV_ADDR_MASK: u32 = 0x6000;
pub const WM8350_DEV_ADDR_SHIFT: u32 = 13;
pub const WM8350_CONFIG_DONE: u32 = 0x1000;
pub const WM8350_CONFIG_DONE_MASK: u32 = 0x1000;
pub const WM8350_CONFIG_DONE_SHIFT: u32 = 12;
pub const WM8350_RECONFIG_AT_ON: u32 = 0x0800;
pub const WM8350_RECONFIG_AT_ON_MASK: u32 = 0x0800;
pub const WM8350_RECONFIG_AT_ON_SHIFT: u32 = 11;
pub const WM8350_AUTOINC: u32 = 0x0200;
pub const WM8350_AUTOINC_MASK: u32 = 0x0200;
pub const WM8350_AUTOINC_SHIFT: u32 = 9;
pub const WM8350_ARA: u32 = 0x0100;
pub const WM8350_ARA_MASK: u32 = 0x0100;
pub const WM8350_ARA_SHIFT: u32 = 8;
pub const WM8350_SPI_CFG: u32 = 0x0008;
pub const WM8350_SPI_CFG_MASK: u32 = 0x0008;
pub const WM8350_SPI_CFG_SHIFT: u32 = 3;
pub const WM8350_SPI_4WIRE: u32 = 0x0004;
pub const WM8350_SPI_4WIRE_MASK: u32 = 0x0004;
pub const WM8350_SPI_4WIRE_SHIFT: u32 = 2;
pub const WM8350_SPI_3WIRE: u32 = 0x0002;
pub const WM8350_SPI_3WIRE_MASK: u32 = 0x0002;
pub const WM8350_SPI_3WIRE_SHIFT: u32 = 1;
/* Bit values for R06 (0x06) */
pub const WM8350_USE_DEV_PINS_PRIMARY: u32 = 0;
pub const WM8350_USE_DEV_PINS_DEV: u32 = 1;
pub const WM8350_DEV_ADDR_34: u32 = 0;
pub const WM8350_DEV_ADDR_36: u32 = 1;
pub const WM8350_DEV_ADDR_3C: u32 = 2;
pub const WM8350_DEV_ADDR_3E: u32 = 3;
pub const WM8350_CONFIG_DONE_OFF: u32 = 0;
pub const WM8350_CONFIG_DONE_DONE: u32 = 1;
pub const WM8350_RECONFIG_AT_ON_OFF: u32 = 0;
pub const WM8350_RECONFIG_AT_ON_ON: u32 = 1;
pub const WM8350_AUTOINC_OFF: u32 = 0;
pub const WM8350_AUTOINC_ON: u32 = 1;
pub const WM8350_ARA_OFF: u32 = 0;
pub const WM8350_ARA_ON: u32 = 1;
pub const WM8350_SPI_CFG_CMOS: u32 = 0;
pub const WM8350_SPI_CFG_OD: u32 = 1;
pub const WM8350_SPI_4WIRE_3WIRE: u32 = 0;
pub const WM8350_SPI_4WIRE_4WIRE: u32 = 1;
pub const WM8350_SPI_3WIRE_I2C: u32 = 0;
pub const WM8350_SPI_3WIRE_SPI: u32 = 1;
/*
 * R8 (0x08) - Power mgmt (1)
 */
pub const WM8350_CODEC_ISEL_MASK: u32 = 0xC000;
pub const WM8350_VBUFEN: u32 = 0x2000;
pub const WM8350_OUTPUT_DRAIN_EN: u32 = 0x0400;
pub const WM8350_MIC_DET_ENA: u32 = 0x0100;
pub const WM8350_BIASEN: u32 = 0x0020;
pub const WM8350_MICBEN: u32 = 0x0010;
pub const WM8350_VMIDEN: u32 = 0x0004;
pub const WM8350_VMID_MASK: u32 = 0x0003;
pub const WM8350_VMID_SHIFT: u32 = 0;
/*
 * R9 (0x09) - Power mgmt (2)
 */
pub const WM8350_IN3R_ENA: u32 = 0x0800;
pub const WM8350_IN3L_ENA: u32 = 0x0400;
pub const WM8350_INR_ENA: u32 = 0x0200;
pub const WM8350_INL_ENA: u32 = 0x0100;
pub const WM8350_MIXINR_ENA: u32 = 0x0080;
pub const WM8350_MIXINL_ENA: u32 = 0x0040;
pub const WM8350_OUT4_ENA: u32 = 0x0020;
pub const WM8350_OUT3_ENA: u32 = 0x0010;
pub const WM8350_MIXOUTR_ENA: u32 = 0x0002;
pub const WM8350_MIXOUTL_ENA: u32 = 0x0001;
/*
 * R10 (0x0A) - Power mgmt (3)
 */
pub const WM8350_IN3R_TO_OUT2R: u32 = 0x0080;
pub const WM8350_OUT2R_ENA: u32 = 0x0008;
pub const WM8350_OUT2L_ENA: u32 = 0x0004;
pub const WM8350_OUT1R_ENA: u32 = 0x0002;
pub const WM8350_OUT1L_ENA: u32 = 0x0001;
/*
 * R11 (0x0B) - Power mgmt (4)
 */
pub const WM8350_SYSCLK_ENA: u32 = 0x4000;
pub const WM8350_ADC_HPF_ENA: u32 = 0x2000;
pub const WM8350_FLL_ENA: u32 = 0x0800;
pub const WM8350_FLL_OSC_ENA: u32 = 0x0400;
pub const WM8350_TOCLK_ENA: u32 = 0x0100;
pub const WM8350_DACR_ENA: u32 = 0x0020;
pub const WM8350_DACL_ENA: u32 = 0x0010;
pub const WM8350_ADCR_ENA: u32 = 0x0008;
pub const WM8350_ADCL_ENA: u32 = 0x0004;
/*
 * R12 (0x0C) - Power mgmt (5)
 */
pub const WM8350_CODEC_ENA: u32 = 0x1000;
pub const WM8350_RTC_TICK_ENA: u32 = 0x0800;
pub const WM8350_OSC32K_ENA: u32 = 0x0400;
pub const WM8350_CHG_ENA: u32 = 0x0200;
pub const WM8350_ACC_DET_ENA: u32 = 0x0100;
pub const WM8350_AUXADC_ENA: u32 = 0x0080;
pub const WM8350_DCMP4_ENA: u32 = 0x0008;
pub const WM8350_DCMP3_ENA: u32 = 0x0004;
pub const WM8350_DCMP2_ENA: u32 = 0x0002;
pub const WM8350_DCMP1_ENA: u32 = 0x0001;
/*
 * R13 (0x0D) - Power mgmt (6)
 */
pub const WM8350_LS_ENA: u32 = 0x8000;
pub const WM8350_LDO4_ENA: u32 = 0x0800;
pub const WM8350_LDO3_ENA: u32 = 0x0400;
pub const WM8350_LDO2_ENA: u32 = 0x0200;
pub const WM8350_LDO1_ENA: u32 = 0x0100;
pub const WM8350_DC6_ENA: u32 = 0x0020;
pub const WM8350_DC5_ENA: u32 = 0x0010;
pub const WM8350_DC4_ENA: u32 = 0x0008;
pub const WM8350_DC3_ENA: u32 = 0x0004;
pub const WM8350_DC2_ENA: u32 = 0x0002;
pub const WM8350_DC1_ENA: u32 = 0x0001;
/*
 * R14 (0x0E) - Power mgmt (7)
 */
pub const WM8350_CS2_ENA: u32 = 0x0002;
pub const WM8350_CS1_ENA: u32 = 0x0001;
/*
 * R24 (0x18) - System Interrupts
 */
pub const WM8350_OC_INT: u32 = 0x2000;
pub const WM8350_UV_INT: u32 = 0x1000;
pub const WM8350_PUTO_INT: u32 = 0x0800;
pub const WM8350_CS_INT: u32 = 0x0200;
pub const WM8350_EXT_INT: u32 = 0x0100;
pub const WM8350_CODEC_INT: u32 = 0x0080;
pub const WM8350_GP_INT: u32 = 0x0040;
pub const WM8350_AUXADC_INT: u32 = 0x0020;
pub const WM8350_RTC_INT: u32 = 0x0010;
pub const WM8350_SYS_INT: u32 = 0x0008;
pub const WM8350_CHG_INT: u32 = 0x0004;
pub const WM8350_USB_INT: u32 = 0x0002;
pub const WM8350_WKUP_INT: u32 = 0x0001;
/*
 * R25 (0x19) - Interrupt Status 1
 */
pub const WM8350_CHG_BAT_HOT_EINT: u32 = 0x8000;
pub const WM8350_CHG_BAT_COLD_EINT: u32 = 0x4000;
pub const WM8350_CHG_BAT_FAIL_EINT: u32 = 0x2000;
pub const WM8350_CHG_TO_EINT: u32 = 0x1000;
pub const WM8350_CHG_END_EINT: u32 = 0x0800;
pub const WM8350_CHG_START_EINT: u32 = 0x0400;
pub const WM8350_CHG_FAST_RDY_EINT: u32 = 0x0200;
pub const WM8350_RTC_PER_EINT: u32 = 0x0080;
pub const WM8350_RTC_SEC_EINT: u32 = 0x0040;
pub const WM8350_RTC_ALM_EINT: u32 = 0x0020;
pub const WM8350_CHG_VBATT_LT_3P9_EINT: u32 = 0x0004;
pub const WM8350_CHG_VBATT_LT_3P1_EINT: u32 = 0x0002;
pub const WM8350_CHG_VBATT_LT_2P85_EINT: u32 = 0x0001;
/*
 * R26 (0x1A) - Interrupt Status 2
 */
pub const WM8350_CS1_EINT: u32 = 0x2000;
pub const WM8350_CS2_EINT: u32 = 0x1000;
pub const WM8350_USB_LIMIT_EINT: u32 = 0x0400;
pub const WM8350_AUXADC_DATARDY_EINT: u32 = 0x0100;
pub const WM8350_AUXADC_DCOMP4_EINT: u32 = 0x0080;
pub const WM8350_AUXADC_DCOMP3_EINT: u32 = 0x0040;
pub const WM8350_AUXADC_DCOMP2_EINT: u32 = 0x0020;
pub const WM8350_AUXADC_DCOMP1_EINT: u32 = 0x0010;
pub const WM8350_SYS_HYST_COMP_FAIL_EINT: u32 = 0x0008;
pub const WM8350_SYS_CHIP_GT115_EINT: u32 = 0x0004;
pub const WM8350_SYS_CHIP_GT140_EINT: u32 = 0x0002;
pub const WM8350_SYS_WDOG_TO_EINT: u32 = 0x0001;
/*
 * R27 (0x1B) - Power Up Interrupt Status
 */
pub const WM8350_PUTO_LDO4_EINT: u32 = 0x0800;
pub const WM8350_PUTO_LDO3_EINT: u32 = 0x0400;
pub const WM8350_PUTO_LDO2_EINT: u32 = 0x0200;
pub const WM8350_PUTO_LDO1_EINT: u32 = 0x0100;
pub const WM8350_PUTO_DC6_EINT: u32 = 0x0020;
pub const WM8350_PUTO_DC5_EINT: u32 = 0x0010;
pub const WM8350_PUTO_DC4_EINT: u32 = 0x0008;
pub const WM8350_PUTO_DC3_EINT: u32 = 0x0004;
pub const WM8350_PUTO_DC2_EINT: u32 = 0x0002;
pub const WM8350_PUTO_DC1_EINT: u32 = 0x0001;
/*
 * R28 (0x1C) - Under Voltage Interrupt status
 */
pub const WM8350_UV_LDO4_EINT: u32 = 0x0800;
pub const WM8350_UV_LDO3_EINT: u32 = 0x0400;
pub const WM8350_UV_LDO2_EINT: u32 = 0x0200;
pub const WM8350_UV_LDO1_EINT: u32 = 0x0100;
pub const WM8350_UV_DC6_EINT: u32 = 0x0020;
pub const WM8350_UV_DC5_EINT: u32 = 0x0010;
pub const WM8350_UV_DC4_EINT: u32 = 0x0008;
pub const WM8350_UV_DC3_EINT: u32 = 0x0004;
pub const WM8350_UV_DC2_EINT: u32 = 0x0002;
pub const WM8350_UV_DC1_EINT: u32 = 0x0001;
/*
 * R29 (0x1D) - Over Current Interrupt status
 */
pub const WM8350_OC_LS_EINT: u32 = 0x8000;
/*
 * R30 (0x1E) - GPIO Interrupt Status
 */
pub const WM8350_GP12_EINT: u32 = 0x1000;
pub const WM8350_GP11_EINT: u32 = 0x0800;
pub const WM8350_GP10_EINT: u32 = 0x0400;
pub const WM8350_GP9_EINT: u32 = 0x0200;
pub const WM8350_GP8_EINT: u32 = 0x0100;
pub const WM8350_GP7_EINT: u32 = 0x0080;
pub const WM8350_GP6_EINT: u32 = 0x0040;
pub const WM8350_GP5_EINT: u32 = 0x0020;
pub const WM8350_GP4_EINT: u32 = 0x0010;
pub const WM8350_GP3_EINT: u32 = 0x0008;
pub const WM8350_GP2_EINT: u32 = 0x0004;
pub const WM8350_GP1_EINT: u32 = 0x0002;
pub const WM8350_GP0_EINT: u32 = 0x0001;
/*
 * R31 (0x1F) - Comparator Interrupt Status
 */
pub const WM8350_EXT_USB_FB_EINT: u32 = 0x8000;
pub const WM8350_EXT_WALL_FB_EINT: u32 = 0x4000;
pub const WM8350_EXT_BAT_FB_EINT: u32 = 0x2000;
pub const WM8350_CODEC_JCK_DET_L_EINT: u32 = 0x0800;
pub const WM8350_CODEC_JCK_DET_R_EINT: u32 = 0x0400;
pub const WM8350_CODEC_MICSCD_EINT: u32 = 0x0200;
pub const WM8350_CODEC_MICD_EINT: u32 = 0x0100;
pub const WM8350_WKUP_OFF_STATE_EINT: u32 = 0x0040;
pub const WM8350_WKUP_HIB_STATE_EINT: u32 = 0x0020;
pub const WM8350_WKUP_CONV_FAULT_EINT: u32 = 0x0010;
pub const WM8350_WKUP_WDOG_RST_EINT: u32 = 0x0008;
pub const WM8350_WKUP_GP_PWR_ON_EINT: u32 = 0x0004;
pub const WM8350_WKUP_ONKEY_EINT: u32 = 0x0002;
pub const WM8350_WKUP_GP_WAKEUP_EINT: u32 = 0x0001;
/*
 * R32 (0x20) - System Interrupts Mask
 */
pub const WM8350_IM_OC_INT: u32 = 0x2000;
pub const WM8350_IM_UV_INT: u32 = 0x1000;
pub const WM8350_IM_PUTO_INT: u32 = 0x0800;
pub const WM8350_IM_SPARE_INT: u32 = 0x0400;
pub const WM8350_IM_CS_INT: u32 = 0x0200;
pub const WM8350_IM_EXT_INT: u32 = 0x0100;
pub const WM8350_IM_CODEC_INT: u32 = 0x0080;
pub const WM8350_IM_GP_INT: u32 = 0x0040;
pub const WM8350_IM_AUXADC_INT: u32 = 0x0020;
pub const WM8350_IM_RTC_INT: u32 = 0x0010;
pub const WM8350_IM_SYS_INT: u32 = 0x0008;
pub const WM8350_IM_CHG_INT: u32 = 0x0004;
pub const WM8350_IM_USB_INT: u32 = 0x0002;
pub const WM8350_IM_WKUP_INT: u32 = 0x0001;
/*
 * R33 (0x21) - Interrupt Status 1 Mask
 */
pub const WM8350_IM_CHG_BAT_HOT_EINT: u32 = 0x8000;
pub const WM8350_IM_CHG_BAT_COLD_EINT: u32 = 0x4000;
pub const WM8350_IM_CHG_BAT_FAIL_EINT: u32 = 0x2000;
pub const WM8350_IM_CHG_TO_EINT: u32 = 0x1000;
pub const WM8350_IM_CHG_END_EINT: u32 = 0x0800;
pub const WM8350_IM_CHG_START_EINT: u32 = 0x0400;
pub const WM8350_IM_CHG_FAST_RDY_EINT: u32 = 0x0200;
pub const WM8350_IM_RTC_PER_EINT: u32 = 0x0080;
pub const WM8350_IM_RTC_SEC_EINT: u32 = 0x0040;
pub const WM8350_IM_RTC_ALM_EINT: u32 = 0x0020;
pub const WM8350_IM_CHG_VBATT_LT_3P9_EINT: u32 = 0x0004;
pub const WM8350_IM_CHG_VBATT_LT_3P1_EINT: u32 = 0x0002;
pub const WM8350_IM_CHG_VBATT_LT_2P85_EINT: u32 = 0x0001;
/*
 * R34 (0x22) - Interrupt Status 2 Mask
 */
pub const WM8350_IM_SPARE2_EINT: u32 = 0x8000;
pub const WM8350_IM_SPARE1_EINT: u32 = 0x4000;
pub const WM8350_IM_CS1_EINT: u32 = 0x2000;
pub const WM8350_IM_CS2_EINT: u32 = 0x1000;
pub const WM8350_IM_USB_LIMIT_EINT: u32 = 0x0400;
pub const WM8350_IM_AUXADC_DATARDY_EINT: u32 = 0x0100;
pub const WM8350_IM_AUXADC_DCOMP4_EINT: u32 = 0x0080;
pub const WM8350_IM_AUXADC_DCOMP3_EINT: u32 = 0x0040;
pub const WM8350_IM_AUXADC_DCOMP2_EINT: u32 = 0x0020;
pub const WM8350_IM_AUXADC_DCOMP1_EINT: u32 = 0x0010;
pub const WM8350_IM_SYS_HYST_COMP_FAIL_EINT: u32 = 0x0008;
pub const WM8350_IM_SYS_CHIP_GT115_EINT: u32 = 0x0004;
pub const WM8350_IM_SYS_CHIP_GT140_EINT: u32 = 0x0002;
pub const WM8350_IM_SYS_WDOG_TO_EINT: u32 = 0x0001;
/*
 * R35 (0x23) - Power Up Interrupt Status Mask
 */
pub const WM8350_IM_PUTO_LDO4_EINT: u32 = 0x0800;
pub const WM8350_IM_PUTO_LDO3_EINT: u32 = 0x0400;
pub const WM8350_IM_PUTO_LDO2_EINT: u32 = 0x0200;
pub const WM8350_IM_PUTO_LDO1_EINT: u32 = 0x0100;
pub const WM8350_IM_PUTO_DC6_EINT: u32 = 0x0020;
pub const WM8350_IM_PUTO_DC5_EINT: u32 = 0x0010;
pub const WM8350_IM_PUTO_DC4_EINT: u32 = 0x0008;
pub const WM8350_IM_PUTO_DC3_EINT: u32 = 0x0004;
pub const WM8350_IM_PUTO_DC2_EINT: u32 = 0x0002;
pub const WM8350_IM_PUTO_DC1_EINT: u32 = 0x0001;
/*
 * R36 (0x24) - Under Voltage Interrupt status Mask
 */
pub const WM8350_IM_UV_LDO4_EINT: u32 = 0x0800;
pub const WM8350_IM_UV_LDO3_EINT: u32 = 0x0400;
pub const WM8350_IM_UV_LDO2_EINT: u32 = 0x0200;
pub const WM8350_IM_UV_LDO1_EINT: u32 = 0x0100;
pub const WM8350_IM_UV_DC6_EINT: u32 = 0x0020;
pub const WM8350_IM_UV_DC5_EINT: u32 = 0x0010;
pub const WM8350_IM_UV_DC4_EINT: u32 = 0x0008;
pub const WM8350_IM_UV_DC3_EINT: u32 = 0x0004;
pub const WM8350_IM_UV_DC2_EINT: u32 = 0x0002;
pub const WM8350_IM_UV_DC1_EINT: u32 = 0x0001;
/*
 * R37 (0x25) - Over Current Interrupt status Mask
 */
pub const WM8350_IM_OC_LS_EINT: u32 = 0x8000;
/*
 * R38 (0x26) - GPIO Interrupt Status Mask
 */
pub const WM8350_IM_GP12_EINT: u32 = 0x1000;
pub const WM8350_IM_GP11_EINT: u32 = 0x0800;
pub const WM8350_IM_GP10_EINT: u32 = 0x0400;
pub const WM8350_IM_GP9_EINT: u32 = 0x0200;
pub const WM8350_IM_GP8_EINT: u32 = 0x0100;
pub const WM8350_IM_GP7_EINT: u32 = 0x0080;
pub const WM8350_IM_GP6_EINT: u32 = 0x0040;
pub const WM8350_IM_GP5_EINT: u32 = 0x0020;
pub const WM8350_IM_GP4_EINT: u32 = 0x0010;
pub const WM8350_IM_GP3_EINT: u32 = 0x0008;
pub const WM8350_IM_GP2_EINT: u32 = 0x0004;
pub const WM8350_IM_GP1_EINT: u32 = 0x0002;
pub const WM8350_IM_GP0_EINT: u32 = 0x0001;
/*
 * R39 (0x27) - Comparator Interrupt Status Mask
 */
pub const WM8350_IM_EXT_USB_FB_EINT: u32 = 0x8000;
pub const WM8350_IM_EXT_WALL_FB_EINT: u32 = 0x4000;
pub const WM8350_IM_EXT_BAT_FB_EINT: u32 = 0x2000;
pub const WM8350_IM_CODEC_JCK_DET_L_EINT: u32 = 0x0800;
pub const WM8350_IM_CODEC_JCK_DET_R_EINT: u32 = 0x0400;
pub const WM8350_IM_CODEC_MICSCD_EINT: u32 = 0x0200;
pub const WM8350_IM_CODEC_MICD_EINT: u32 = 0x0100;
pub const WM8350_IM_WKUP_OFF_STATE_EINT: u32 = 0x0040;
pub const WM8350_IM_WKUP_HIB_STATE_EINT: u32 = 0x0020;
pub const WM8350_IM_WKUP_CONV_FAULT_EINT: u32 = 0x0010;
pub const WM8350_IM_WKUP_WDOG_RST_EINT: u32 = 0x0008;
pub const WM8350_IM_WKUP_GP_PWR_ON_EINT: u32 = 0x0004;
pub const WM8350_IM_WKUP_ONKEY_EINT: u32 = 0x0002;
pub const WM8350_IM_WKUP_GP_WAKEUP_EINT: u32 = 0x0001;
/*
 * R220 (0xDC) - RAM BIST 1
 */
pub const WM8350_READ_STATUS: u32 = 0x0800;
pub const WM8350_TSTRAM_CLK: u32 = 0x0100;
pub const WM8350_TSTRAM_CLK_ENA: u32 = 0x0080;
pub const WM8350_STARTSEQ: u32 = 0x0040;
pub const WM8350_READ_SRC: u32 = 0x0020;
pub const WM8350_COUNT_DIR: u32 = 0x0010;
pub const WM8350_TSTRAM_MODE_MASK: u32 = 0x000E;
pub const WM8350_TSTRAM_ENA: u32 = 0x0001;
/*
 * R225 (0xE1) - DCDC/LDO status
 */
pub const WM8350_LS_STS: u32 = 0x8000;
pub const WM8350_LDO4_STS: u32 = 0x0800;
pub const WM8350_LDO3_STS: u32 = 0x0400;
pub const WM8350_LDO2_STS: u32 = 0x0200;
pub const WM8350_LDO1_STS: u32 = 0x0100;
pub const WM8350_DC6_STS: u32 = 0x0020;
pub const WM8350_DC5_STS: u32 = 0x0010;
pub const WM8350_DC4_STS: u32 = 0x0008;
pub const WM8350_DC3_STS: u32 = 0x0004;
pub const WM8350_DC2_STS: u32 = 0x0002;
pub const WM8350_DC1_STS: u32 = 0x0001;
/*
 * R226 (0xE2) - Charger status
 */
pub const WM8350_CHG_BATT_HOT_OVRDE: u32 = 0x8000;
pub const WM8350_CHG_BATT_COLD_OVRDE: u32 = 0x4000;
/*
 * R227 (0xE3) - Misc Overrides
 */
pub const WM8350_USB_LIMIT_OVRDE: u32 = 0x0400;
/*
 * R227 (0xE7) - Comparator Overrides
 */
pub const WM8350_USB_FB_OVRDE: u32 = 0x8000;
pub const WM8350_WALL_FB_OVRDE: u32 = 0x4000;
pub const WM8350_BATT_FB_OVRDE: u32 = 0x2000;
/*
 * R233 (0xE9) - State Machinine Status
 */
pub const WM8350_USB_SM_MASK: u32 = 0x0700;
pub const WM8350_USB_SM_SHIFT: u32 = 8;
pub const WM8350_USB_SM_100_SLV: u32 = 1;
pub const WM8350_USB_SM_500_SLV: u32 = 5;
pub const WM8350_USB_SM_STDBY_SLV: u32 = 7;
/* WM8350 wake up conditions */
pub const WM8350_IRQ_WKUP_OFF_STATE: u32 = 43;
pub const WM8350_IRQ_WKUP_HIB_STATE: u32 = 44;
pub const WM8350_IRQ_WKUP_CONV_FAULT: u32 = 45;
pub const WM8350_IRQ_WKUP_WDOG_RST: u32 = 46;
pub const WM8350_IRQ_WKUP_GP_PWR_ON: u32 = 47;
pub const WM8350_IRQ_WKUP_ONKEY: u32 = 48;
pub const WM8350_IRQ_WKUP_GP_WAKEUP: u32 = 49;
/* wm8350 chip revisions */
pub const WM8350_REV_E: u32 = 0x4;
pub const WM8350_REV_F: u32 = 0x5;
pub const WM8350_REV_G: u32 = 0x6;
pub const WM8350_REV_H: u32 = 0x7;
pub const WM8350_NUM_IRQ: u32 = 63;
pub const WM8350_NUM_IRQ_REGS: u32 = 7;
/**
 * Data to be supplied by the platform to initialise the WM8350.
 *
 * @init: Function called during driver initialisation.  Should be
 *        used by the platform to configure GPIO functions and similar.
 * @irq_high: Set if WM8350 IRQ is active high.
 * @irq_base: Base IRQ for genirq (not currently used).
 * @gpio_base: Base for gpiolib.
 */
/*
 * WM8350 device initialisation and exit.
 */
/*
 * WM8350 device IO
 */
/*
 * WM8350 internal interrupts
 */
+

// External dependencies supplied by other translation units.
#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct platform_device { _private: [u8; 0] }
#[repr(C)]
pub struct regmap { _private: [u8; 0] }
#[repr(C)]
pub struct mutex { _private: [u8; 0] }
#[repr(C)]
pub struct completion { _private: [u8; 0] }

#[repr(C)]
pub struct wm8350_codec { _private: [u8; 0] }
#[repr(C)]
pub struct wm8350_gpio { _private: [u8; 0] }
#[repr(C)]
pub struct wm8350_pmic { _private: [u8; 0] }
#[repr(C)]
pub struct wm8350_power { _private: [u8; 0] }
#[repr(C)]
pub struct wm8350_rtc { _private: [u8; 0] }
#[repr(C)]
pub struct wm8350_wdt { _private: [u8; 0] }

#[repr(C)]
pub struct wm8350_hwmon {
    pub pdev: *mut platform_device,
    pub classdev: *mut device,
}

#[repr(C)]
pub struct wm8350 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub unlocked: bool,
    pub auxadc_mutex: mutex,
    pub auxadc_done: completion,
    pub irq_lock: mutex,
    pub chip_irq: i32,
    pub irq_base: i32,
    pub irq_masks: [u16; WM8350_NUM_IRQ_REGS as usize],
    pub codec: wm8350_codec,
    pub gpio: wm8350_gpio,
    pub hwmon: wm8350_hwmon,
    pub pmic: wm8350_pmic,
    pub power: wm8350_power,
    pub rtc: wm8350_rtc,
    pub wdt: wm8350_wdt,
}

#[repr(C)]
pub struct wm8350_platform_data {
    pub init: Option<unsafe extern "C" fn(*mut wm8350) -> i32>,
    pub irq_high: i32,
    pub irq_base: i32,
    pub gpio_base: i32,
}

extern "C" {
    pub static wm8350_regmap: regmap;
    pub fn wm8350_device_init(wm8350: *mut wm8350, irq: i32, pdata: *mut wm8350_platform_data) -> i32;
    pub fn wm8350_clear_bits(wm8350: *mut wm8350, reg: u16, mask: u16) -> i32;
    pub fn wm8350_set_bits(wm8350: *mut wm8350, reg: u16, mask: u16) -> i32;
    pub fn wm8350_reg_read(wm8350: *mut wm8350, reg: i32) -> u16;
    pub fn wm8350_reg_write(wm8350: *mut wm8350, reg: i32, val: u16) -> i32;
    pub fn wm8350_reg_lock(wm8350: *mut wm8350) -> i32;
    pub fn wm8350_reg_unlock(wm8350: *mut wm8350) -> i32;
    pub fn wm8350_block_read(wm8350: *mut wm8350, reg: i32, size: i32, dest: *mut u16) -> i32;
    pub fn wm8350_block_write(wm8350: *mut wm8350, reg: i32, size: i32, src: *mut u16) -> i32;
    pub fn wm8350_irq_init(wm8350: *mut wm8350, irq: i32, pdata: *mut wm8350_platform_data) -> i32;
    pub fn wm8350_irq_exit(wm8350: *mut wm8350) -> i32;
}

pub type irq_handler_t = Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> i32>;

pub unsafe fn wm8350_register_irq(wm8350: *mut wm8350, irq: i32, handler: irq_handler_t, flags: u64, name: *const core::ffi::c_char, data: *mut core::ffi::c_void) -> i32 {
    if (*wm8350).irq_base == 0 { return -19; /* -ENODEV */ }
    request_threaded_irq(irq + (*wm8350).irq_base, None, handler, flags | IRQF_ONESHOT, name, data)
}
pub unsafe fn wm8350_free_irq(wm8350: *mut wm8350, irq: i32, data: *mut core::ffi::c_void) { free_irq(irq + (*wm8350).irq_base, data); }
pub unsafe fn wm8350_mask_irq(wm8350: *mut wm8350, irq: i32) { disable_irq(irq + (*wm8350).irq_base); }
pub unsafe fn wm8350_unmask_irq(wm8350: *mut wm8350, irq: i32) { enable_irq(irq + (*wm8350).irq_base); }

extern "C" {
    fn request_threaded_irq(irq: i32, primary: irq_handler_t, threaded: irq_handler_t, flags: u64, name: *const core::ffi::c_char, data: *mut core::ffi::c_void) -> i32;
    fn free_irq(irq: i32, data: *mut core::ffi::c_void);
    fn disable_irq(irq: i32);
    fn enable_irq(irq: i32);
}

pub const IRQF_ONESHOT: u64 = 1 << 0;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
