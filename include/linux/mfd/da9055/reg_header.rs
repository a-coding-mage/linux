/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * DA9055 declarations for DA9055 PMICs.
 *
 * Copyright(c) 2012 Dialog Semiconductor Ltd.
 *
 * Author: David Dajun Chen <dchen@diasemi.com>
 */


/*
 * PMIC registers
 */
 /* PAGE0 */
pub const DA9055_REG_PAGE_CON: u32 = 0x00;

/* System Control and Event Registers */
pub const DA9055_REG_STATUS_A: u32 = 0x01;
pub const DA9055_REG_STATUS_B: u32 = 0x02;
pub const DA9055_REG_FAULT_LOG: u32 = 0x03;
pub const DA9055_REG_EVENT_A: u32 = 0x04;
pub const DA9055_REG_EVENT_B: u32 = 0x05;
pub const DA9055_REG_EVENT_C: u32 = 0x06;
pub const DA9055_REG_IRQ_MASK_A: u32 = 0x07;
pub const DA9055_REG_IRQ_MASK_B: u32 = 0x08;
pub const DA9055_REG_IRQ_MASK_C: u32 = 0x09;
pub const DA9055_REG_CONTROL_A: u32 = 0x0A;
pub const DA9055_REG_CONTROL_B: u32 = 0x0B;
pub const DA9055_REG_CONTROL_C: u32 = 0x0C;
pub const DA9055_REG_CONTROL_D: u32 = 0x0D;
pub const DA9055_REG_CONTROL_E: u32 = 0x0E;
pub const DA9055_REG_PD_DIS: u32 = 0x0F;

/* GPIO Control Registers */
pub const DA9055_REG_GPIO0_1: u32 = 0x10;
pub const DA9055_REG_GPIO2: u32 = 0x11;
pub const DA9055_REG_GPIO_MODE0_2: u32 = 0x12;

/* Regulator Control Registers */
pub const DA9055_REG_BCORE_CONT: u32 = 0x13;
pub const DA9055_REG_BMEM_CONT: u32 = 0x14;
pub const DA9055_REG_LDO1_CONT: u32 = 0x15;
pub const DA9055_REG_LDO2_CONT: u32 = 0x16;
pub const DA9055_REG_LDO3_CONT: u32 = 0x17;
pub const DA9055_REG_LDO4_CONT: u32 = 0x18;
pub const DA9055_REG_LDO5_CONT: u32 = 0x19;
pub const DA9055_REG_LDO6_CONT: u32 = 0x1A;

/* GP-ADC Control Registers */
pub const DA9055_REG_ADC_MAN: u32 = 0x1B;
pub const DA9055_REG_ADC_CONT: u32 = 0x1C;
pub const DA9055_REG_VSYS_MON: u32 = 0x1D;
pub const DA9055_REG_ADC_RES_L: u32 = 0x1E;
pub const DA9055_REG_ADC_RES_H: u32 = 0x1F;
pub const DA9055_REG_VSYS_RES: u32 = 0x20;
pub const DA9055_REG_ADCIN1_RES: u32 = 0x21;
pub const DA9055_REG_ADCIN2_RES: u32 = 0x22;
pub const DA9055_REG_ADCIN3_RES: u32 = 0x23;

/* Sequencer Control Registers */
pub const DA9055_REG_EN_32K: u32 = 0x35;

/* Regulator Setting Registers */
pub const DA9055_REG_BUCK_LIM: u32 = 0x37;
pub const DA9055_REG_BCORE_MODE: u32 = 0x38;
pub const DA9055_REG_VBCORE_A: u32 = 0x39;
pub const DA9055_REG_VBMEM_A: u32 = 0x3A;
pub const DA9055_REG_VLDO1_A: u32 = 0x3B;
pub const DA9055_REG_VLDO2_A: u32 = 0x3C;
pub const DA9055_REG_VLDO3_A: u32 = 0x3D;
pub const DA9055_REG_VLDO4_A: u32 = 0x3E;
pub const DA9055_REG_VLDO5_A: u32 = 0x3F;
pub const DA9055_REG_VLDO6_A: u32 = 0x40;
pub const DA9055_REG_VBCORE_B: u32 = 0x41;
pub const DA9055_REG_VBMEM_B: u32 = 0x42;
pub const DA9055_REG_VLDO1_B: u32 = 0x43;
pub const DA9055_REG_VLDO2_B: u32 = 0x44;
pub const DA9055_REG_VLDO3_B: u32 = 0x45;
pub const DA9055_REG_VLDO4_B: u32 = 0x46;
pub const DA9055_REG_VLDO5_B: u32 = 0x47;
pub const DA9055_REG_VLDO6_B: u32 = 0x48;

/* GP-ADC Threshold Registers */
pub const DA9055_REG_AUTO1_HIGH: u32 = 0x49;
pub const DA9055_REG_AUTO1_LOW: u32 = 0x4A;
pub const DA9055_REG_AUTO2_HIGH: u32 = 0x4B;
pub const DA9055_REG_AUTO2_LOW: u32 = 0x4C;
pub const DA9055_REG_AUTO3_HIGH: u32 = 0x4D;
pub const DA9055_REG_AUTO3_LOW: u32 = 0x4E;

/* OTP */
pub const DA9055_REG_OPT_COUNT: u32 = 0x50;
pub const DA9055_REG_OPT_ADDR: u32 = 0x51;
pub const DA9055_REG_OPT_DATA: u32 = 0x52;

/* RTC Calendar and Alarm Registers */
pub const DA9055_REG_COUNT_S: u32 = 0x53;
pub const DA9055_REG_COUNT_MI: u32 = 0x54;
pub const DA9055_REG_COUNT_H: u32 = 0x55;
pub const DA9055_REG_COUNT_D: u32 = 0x56;
pub const DA9055_REG_COUNT_MO: u32 = 0x57;
pub const DA9055_REG_COUNT_Y: u32 = 0x58;
pub const DA9055_REG_ALARM_MI: u32 = 0x59;
pub const DA9055_REG_ALARM_H: u32 = 0x5A;
pub const DA9055_REG_ALARM_D: u32 = 0x5B;
pub const DA9055_REG_ALARM_MO: u32 = 0x5C;
pub const DA9055_REG_ALARM_Y: u32 = 0x5D;
pub const DA9055_REG_SECOND_A: u32 = 0x5E;
pub const DA9055_REG_SECOND_B: u32 = 0x5F;
pub const DA9055_REG_SECOND_C: u32 = 0x60;
pub const DA9055_REG_SECOND_D: u32 = 0x61;

/* Customer Trim and Configuration */
pub const DA9055_REG_T_OFFSET: u32 = 0x63;
pub const DA9055_REG_INTERFACE: u32 = 0x64;
pub const DA9055_REG_CONFIG_A: u32 = 0x65;
pub const DA9055_REG_CONFIG_B: u32 = 0x66;
pub const DA9055_REG_CONFIG_C: u32 = 0x67;
pub const DA9055_REG_CONFIG_D: u32 = 0x68;
pub const DA9055_REG_CONFIG_E: u32 = 0x69;
pub const DA9055_REG_TRIM_CLDR: u32 = 0x6F;

/* General Purpose Registers */
pub const DA9055_REG_GP_ID_0: u32 = 0x70;
pub const DA9055_REG_GP_ID_1: u32 = 0x71;
pub const DA9055_REG_GP_ID_2: u32 = 0x72;
pub const DA9055_REG_GP_ID_3: u32 = 0x73;
pub const DA9055_REG_GP_ID_4: u32 = 0x74;
pub const DA9055_REG_GP_ID_5: u32 = 0x75;
pub const DA9055_REG_GP_ID_6: u32 = 0x76;
pub const DA9055_REG_GP_ID_7: u32 = 0x77;
pub const DA9055_REG_GP_ID_8: u32 = 0x78;
pub const DA9055_REG_GP_ID_9: u32 = 0x79;
pub const DA9055_REG_GP_ID_10: u32 = 0x7A;
pub const DA9055_REG_GP_ID_11: u32 = 0x7B;
pub const DA9055_REG_GP_ID_12: u32 = 0x7C;
pub const DA9055_REG_GP_ID_13: u32 = 0x7D;
pub const DA9055_REG_GP_ID_14: u32 = 0x7E;
pub const DA9055_REG_GP_ID_15: u32 = 0x7F;
pub const DA9055_REG_GP_ID_16: u32 = 0x80;
pub const DA9055_REG_GP_ID_17: u32 = 0x81;
pub const DA9055_REG_GP_ID_18: u32 = 0x82;
pub const DA9055_REG_GP_ID_19: u32 = 0x83;

pub const DA9055_MAX_REGISTER_CNT: u32 = DA9055_REG_GP_ID_19;

/*
 * PMIC registers bits
 */

/* DA9055_REG_PAGE_CON (addr=0x00) */
pub const DA9055_PAGE_WRITE_MODE: u32 = (0<<6);
pub const DA9055_REPEAT_WRITE_MODE: u32 = (1<<6);

/* DA9055_REG_STATUS_A (addr=0x01) */
pub const DA9055_NOKEY_STS: u32 = 0x01;
pub const DA9055_WAKE_STS: u32 = 0x02;
pub const DA9055_DVC_BUSY_STS: u32 = 0x04;
pub const DA9055_COMP1V2_STS: u32 = 0x08;
pub const DA9055_NJIG_STS: u32 = 0x10;
pub const DA9055_LDO5_LIM_STS: u32 = 0x20;
pub const DA9055_LDO6_LIM_STS: u32 = 0x40;

/* DA9055_REG_STATUS_B (addr=0x02) */
pub const DA9055_GPI0_STS: u32 = 0x01;
pub const DA9055_GPI1_STS: u32 = 0x02;
pub const DA9055_GPI2_STS: u32 = 0x04;

/* DA9055_REG_FAULT_LOG (addr=0x03) */
pub const DA9055_TWD_ERROR_FLG: u32 = 0x01;
pub const DA9055_POR_FLG: u32 = 0x02;
pub const DA9055_VDD_FAULT_FLG: u32 = 0x04;
pub const DA9055_VDD_START_FLG: u32 = 0x08;
pub const DA9055_TEMP_CRIT_FLG: u32 = 0x10;
pub const DA9055_KEY_RESET_FLG: u32 = 0x20;
pub const DA9055_WAIT_SHUT_FLG: u32 = 0x80;

/* DA9055_REG_EVENT_A (addr=0x04) */
pub const DA9055_NOKEY_EINT: u32 = 0x01;
pub const DA9055_ALARM_EINT: u32 = 0x02;
pub const DA9055_TICK_EINT: u32 = 0x04;
pub const DA9055_ADC_RDY_EINT: u32 = 0x08;
pub const DA9055_SEQ_RDY_EINT: u32 = 0x10;
pub const DA9055_EVENTS_B_EINT: u32 = 0x20;
pub const DA9055_EVENTS_C_EINT: u32 = 0x40;

/* DA9055_REG_EVENT_B (addr=0x05) */
pub const DA9055_E_WAKE_EINT: u32 = 0x01;
pub const DA9055_E_TEMP_EINT: u32 = 0x02;
pub const DA9055_E_COMP1V2_EINT: u32 = 0x04;
pub const DA9055_E_LDO_LIM_EINT: u32 = 0x08;
pub const DA9055_E_NJIG_EINT: u32 = 0x20;
pub const DA9055_E_VDD_MON_EINT: u32 = 0x40;
pub const DA9055_E_VDD_WARN_EINT: u32 = 0x80;

/* DA9055_REG_EVENT_C (addr=0x06) */
pub const DA9055_E_GPI0_EINT: u32 = 0x01;
pub const DA9055_E_GPI1_EINT: u32 = 0x02;
pub const DA9055_E_GPI2_EINT: u32 = 0x04;

/* DA9055_REG_IRQ_MASK_A (addr=0x07) */
pub const DA9055_M_NONKEY_EINT: u32 = 0x01;
pub const DA9055_M_ALARM_EINT: u32 = 0x02;
pub const DA9055_M_TICK_EINT: u32 = 0x04;
pub const DA9055_M_ADC_RDY_EINT: u32 = 0x08;
pub const DA9055_M_SEQ_RDY_EINT: u32 = 0x10;

/* DA9055_REG_IRQ_MASK_B (addr=0x08) */
pub const DA9055_M_WAKE_EINT: u32 = 0x01;
pub const DA9055_M_TEMP_EINT: u32 = 0x02;
pub const DA9055_M_COMP_1V2_EINT: u32 = 0x04;
pub const DA9055_M_LDO_LIM_EINT: u32 = 0x08;
pub const DA9055_M_NJIG_EINT: u32 = 0x20;
pub const DA9055_M_VDD_MON_EINT: u32 = 0x40;
pub const DA9055_M_VDD_WARN_EINT: u32 = 0x80;

/* DA9055_REG_IRQ_MASK_C (addr=0x09) */
pub const DA9055_M_GPI0_EINT: u32 = 0x01;
pub const DA9055_M_GPI1_EINT: u32 = 0x02;
pub const DA9055_M_GPI2_EINT: u32 = 0x04;

/* DA9055_REG_CONTROL_A (addr=0xA) */
pub const DA9055_DEBOUNCING_SHIFT: u32 = 0x00;
pub const DA9055_DEBOUNCING_MASK: u32 = 0x07;
pub const DA9055_NRES_MODE_SHIFT: u32 = 0x03;
pub const DA9055_NRES_MODE_MASK: u32 = 0x08;
pub const DA9055_SLEW_RATE_SHIFT: u32 = 0x04;
pub const DA9055_SLEW_RATE_MASK: u32 = 0x30;
pub const DA9055_NOKEY_LOCK_SHIFT: u32 = 0x06;
pub const DA9055_NOKEY_LOCK_MASK: u32 = 0x40;

/* DA9055_REG_CONTROL_B (addr=0xB) */
pub const DA9055_RTC_MODE_PD: u32 = 0x01;
pub const DA9055_RTC_MODE_SD_SHIFT: u32 = 0x01;
pub const DA9055_RTC_MODE_SD: u32 = 0x02;
pub const DA9055_RTC_EN: u32 = 0x04;
pub const DA9055_ECO_MODE_SHIFT: u32 = 0x03;
pub const DA9055_ECO_MODE_MASK: u32 = 0x08;
pub const DA9055_TWDSCALE_SHIFT: u32 = 4;
pub const DA9055_TWDSCALE_MASK: u32 = 0x70;
pub const DA9055_V_LOCK_SHIFT: u32 = 0x07;
pub const DA9055_V_LOCK_MASK: u32 = 0x80;

/* DA9055_REG_CONTROL_C (addr=0xC) */
pub const DA9055_SYSTEM_EN_SHIFT: u32 = 0x00;
pub const DA9055_SYSTEM_EN_MASK: u32 = 0x01;
pub const DA9055_POWERN_EN_SHIFT: u32 = 0x01;
pub const DA9055_POWERN_EN_MASK: u32 = 0x02;
pub const DA9055_POWER1_EN_SHIFT: u32 = 0x02;
pub const DA9055_POWER1_EN_MASK: u32 = 0x04;

/* DA9055_REG_CONTROL_D (addr=0xD) */
pub const DA9055_STANDBY_SHIFT: u32 = 0x02;
pub const DA9055_STANDBY_MASK: u32 = 0x08;
pub const DA9055_AUTO_BOOT_SHIFT: u32 = 0x03;
pub const DA9055_AUTO_BOOT_MASK: u32 = 0x04;

/* DA9055_REG_CONTROL_E (addr=0xE) */
pub const DA9055_WATCHDOG_SHIFT: u32 = 0x00;
pub const DA9055_WATCHDOG_MASK: u32 = 0x01;
pub const DA9055_SHUTDOWN_SHIFT: u32 = 0x01;
pub const DA9055_SHUTDOWN_MASK: u32 = 0x02;
pub const DA9055_WAKE_UP_SHIFT: u32 = 0x02;
pub const DA9055_WAKE_UP_MASK: u32 = 0x04;

/* DA9055_REG_GPIO (addr=0x10/0x11) */
pub const DA9055_GPIO0_PIN_SHIFT: u32 = 0x00;
pub const DA9055_GPIO0_PIN_MASK: u32 = 0x03;
pub const DA9055_GPIO0_TYPE_SHIFT: u32 = 0x02;
pub const DA9055_GPIO0_TYPE_MASK: u32 = 0x04;
pub const DA9055_GPIO0_WEN_SHIFT: u32 = 0x03;
pub const DA9055_GPIO0_WEN_MASK: u32 = 0x08;
pub const DA9055_GPIO1_PIN_SHIFT: u32 = 0x04;
pub const DA9055_GPIO1_PIN_MASK: u32 = 0x30;
pub const DA9055_GPIO1_TYPE_SHIFT: u32 = 0x06;
pub const DA9055_GPIO1_TYPE_MASK: u32 = 0x40;
pub const DA9055_GPIO1_WEN_SHIFT: u32 = 0x07;
pub const DA9055_GPIO1_WEN_MASK: u32 = 0x80;
pub const DA9055_GPIO2_PIN_SHIFT: u32 = 0x00;
pub const DA9055_GPIO2_PIN_MASK: u32 = 0x30;
pub const DA9055_GPIO2_TYPE_SHIFT: u32 = 0x02;
pub const DA9055_GPIO2_TYPE_MASK: u32 = 0x04;
pub const DA9055_GPIO2_WEN_SHIFT: u32 = 0x03;
pub const DA9055_GPIO2_WEN_MASK: u32 = 0x08;

/* DA9055_REG_GPIO_MODE (addr=0x12) */
pub const DA9055_GPIO0_MODE_SHIFT: u32 = 0x00;
pub const DA9055_GPIO0_MODE_MASK: u32 = 0x01;
pub const DA9055_GPIO1_MODE_SHIFT: u32 = 0x01;
pub const DA9055_GPIO1_MODE_MASK: u32 = 0x02;
pub const DA9055_GPIO2_MODE_SHIFT: u32 = 0x02;
pub const DA9055_GPIO2_MODE_MASK: u32 = 0x04;

/* DA9055_REG_BCORE_CONT (addr=0x13) */
pub const DA9055_BCORE_EN_SHIFT: u32 = 0x00;
pub const DA9055_BCORE_EN_MASK: u32 = 0x01;
pub const DA9055_BCORE_GPI_SHIFT: u32 = 0x01;
pub const DA9055_BCORE_GPI_MASK: u32 = 0x02;
pub const DA9055_BCORE_PD_DIS_SHIFT: u32 = 0x03;
pub const DA9055_BCORE_PD_DIS_MASK: u32 = 0x04;
pub const DA9055_VBCORE_SEL_SHIFT: u32 = 0x04;
pub const DA9055_SEL_REG_A: u32 = 0x0;
pub const DA9055_SEL_REG_B: u32 = 0x10;
pub const DA9055_VBCORE_SEL_MASK: u32 = 0x10;
pub const DA9055_V_GPI_MASK: u32 = 0x60;
pub const DA9055_V_GPI_SHIFT: u32 = 0x05;
pub const DA9055_E_GPI_MASK: u32 = 0x06;
pub const DA9055_E_GPI_SHIFT: u32 = 0x01;
pub const DA9055_VBCORE_GPI_SHIFT: u32 = 0x05;
pub const DA9055_VBCORE_GPI_MASK: u32 = 0x60;
pub const DA9055_BCORE_CONF_SHIFT: u32 = 0x07;
pub const DA9055_BCORE_CONF_MASK: u32 = 0x80;

/* DA9055_REG_BMEM_CONT (addr=0x14) */
pub const DA9055_BMEM_EN_SHIFT: u32 = 0x00;
pub const DA9055_BMEM_EN_MASK: u32 = 0x01;
pub const DA9055_BMEM_GPI_SHIFT: u32 = 0x01;
pub const DA9055_BMEM_GPI_MASK: u32 = 0x06;
pub const DA9055_BMEM_PD_DIS_SHIFT: u32 = 0x03;
pub const DA9055_BMEM_PD_DIS_MASK: u32 = 0x08;
pub const DA9055_VBMEM_SEL_SHIT: u32 = 0x04;
pub const DA9055_VBMEM_SEL_VBMEM_A: u32 = (0<<4);
pub const DA9055_VBMEM_SEL_VBMEM_B: u32 = (1<<4);
pub const DA9055_VBMEM_SEL_MASK: u32 = 0x10;
pub const DA9055_VBMEM_GPI_SHIFT: u32 = 0x05;
pub const DA9055_VBMEM_GPI_MASK: u32 = 0x60;
pub const DA9055_BMEM_CONF_SHIFT: u32 = 0x07;
pub const DA9055_BMEM_CONF_MASK: u32 = 0x80;

/* DA9055_REG_LDO_CONT (addr=0x15-0x1A) */
pub const DA9055_LDO_EN_SHIFT: u32 = 0x00;
pub const DA9055_LDO_EN_MASK: u32 = 0x01;
pub const DA9055_LDO_GPI_SHIFT: u32 = 0x01;
pub const DA9055_LDO_GPI_MASK: u32 = 0x06;
pub const DA9055_LDO_PD_DIS_SHIFT: u32 = 0x03;
pub const DA9055_LDO_PD_DIS_MASK: u32 = 0x08;
pub const DA9055_VLDO_SEL_SHIFT: u32 = 0x04;
pub const DA9055_VLDO_SEL_MASK: u32 = 0x10;
pub const DA9055_VLDO_SEL_VLDO_A: u32 = 0x00;
pub const DA9055_VLDO_SEL_VLDO_B: u32 = 0x01;
pub const DA9055_VLDO_GPI_SHIFT: u32 = 0x05;
pub const DA9055_VLDO_GPI_MASK: u32 = 0x60;
pub const DA9055_LDO_CONF_SHIFT: u32 = 0x07;
pub const DA9055_LDO_CONF_MASK: u32 = 0x80;
pub const DA9055_REGUALTOR_SET_A: u32 = 0x00;
pub const DA9055_REGUALTOR_SET_B: u32 = 0x10;

/* DA9055_REG_ADC_MAN (addr=0x1B) */
pub const DA9055_ADC_MUX_SHIFT: u32 = 0;
pub const DA9055_ADC_MUX_MASK: u32 = 0xF;
pub const DA9055_ADC_MUX_VSYS: u32 = 0x0;
pub const DA9055_ADC_MUX_ADCIN1: u32 = 0x01;
pub const DA9055_ADC_MUX_ADCIN2: u32 = 0x02;
pub const DA9055_ADC_MUX_ADCIN3: u32 = 0x03;
pub const DA9055_ADC_MUX_T_SENSE: u32 = 0x04;
pub const DA9055_ADC_MAN_SHIFT: u32 = 0x04;
pub const DA9055_ADC_MAN_CONV: u32 = 0x10;
pub const DA9055_ADC_LSB_MASK: u32 = 0x03;
pub const DA9055_ADC_MODE_MASK: u32 = 0x20;
pub const DA9055_ADC_MODE_SHIFT: u32 = 5;
pub const DA9055_ADC_MODE_1MS: u32 = (1<<5);
pub const DA9055_COMP1V2_EN_SHIFT: u32 = 7;

/* DA9055_REG_ADC_CONT (addr=0x1C) */
pub const DA9055_ADC_AUTO_VSYS_EN_SHIFT: u32 = 0;
pub const DA9055_ADC_AUTO_AD1_EN_SHIFT: u32 = 1;
pub const DA9055_ADC_AUTO_AD2_EN_SHIFT: u32 = 2;
pub const DA9055_ADC_AUTO_AD3_EN_SHIFT: u32 = 3;
pub const DA9055_ADC_ISRC_EN_SHIFT: u32 = 4;
pub const DA9055_ADC_ADCIN1_DEB_SHIFT: u32 = 5;
pub const DA9055_ADC_ADCIN2_DEB_SHIFT: u32 = 6;
pub const DA9055_ADC_ADCIN3_DEB_SHIFT: u32 = 7;
pub const DA9055_AD1_ISRC_MASK: u32 = 0x10;
pub const DA9055_AD1_ISRC_SHIFT: u32 = 4;

/* DA9055_REG_VSYS_MON (addr=0x1D) */
pub const DA9055_VSYS_VAL_SHIFT: u32 = 0;
pub const DA9055_VSYS_VAL_MASK: u32 = 0xFF;
pub const DA9055_VSYS_VAL_BASE: u32 = 0x00;
pub const DA9055_VSYS_VAL_MAX: u32 = DA9055_VSYS_VAL_MASK;
pub const DA9055_VSYS_VOLT_BASE: u32 = 2500;
pub const DA9055_VSYS_VOLT_INC: u32 = 10;
pub const DA9055_VSYS_STEPS: u32 = 255;
pub const DA9055_VSYS_VOLT_MIN: u32 = 2500;

/* DA9044_REG_XXX_RES (addr=0x20-0x23) */
pub const DA9055_ADC_VAL_SHIFT: u32 = 0;
pub const DA9055_ADC_VAL_MASK: u32 = 0xFF;
pub const DA9055_ADC_VAL_BASE: u32 = 0x00;
pub const DA9055_ADC_VAL_MAX: u32 = DA9055_ADC_VAL_MASK;
pub const DA9055_ADC_VOLT_BASE: u32 = 0;
pub const DA9055_ADC_VSYS_VOLT_BASE: u32 = 2500;
pub const DA9055_ADC_VOLT_INC: u32 = 10;
pub const DA9055_ADC_VSYS_VOLT_INC: u32 = 12;
pub const DA9055_ADC_STEPS: u32 = 255;

/* DA9055_REG_EN_32K  (addr=0x35)*/
pub const DA9055_STARTUP_TIME_MASK: u32 = 0x07;
pub const DA9055_STARTUP_TIME_0S: u32 = 0x0;
pub const DA9055_STARTUP_TIME_0_52S: u32 = 0x1;
pub const DA9055_STARTUP_TIME_1S: u32 = 0x2;
pub const DA9055_CRYSTAL_EN: u32 = 0x08;
pub const DA9055_DELAY_MODE_EN: u32 = 0x10;
pub const DA9055_OUT_CLCK_GATED: u32 = 0x20;
pub const DA9055_RTC_CLOCK_GATED: u32 = 0x40;
pub const DA9055_EN_32KOUT_BUF: u32 = 0x80;

/* DA9055_REG_RESET (addr=0x36) */
/* Timer up to 31.744 ms */
pub const DA9055_RESET_TIMER_VAL_SHIFT: u32 = 0;
pub const DA9055_RESET_LOW_VAL_MASK: u32 = 0x3F;
pub const DA9055_RESET_LOW_VAL_BASE: u32 = 0;
pub const DA9055_RESET_LOW_VAL_MAX: u32 = DA9055_RESET_LOW_VAL_MASK;
pub const DA9055_RESET_US_LOW_BASE: u32 = 1024 /* min val in units of us */;
pub const DA9055_RESET_US_LOW_INC: u32 = 1024 /* inc val in units of us */;
pub const DA9055_RESET_US_LOW_STEP: u32 = 30;

/* Timer up to 1048.576ms */
pub const DA9055_RESET_HIGH_VAL_MASK: u32 = 0x3F;
pub const DA9055_RESET_HIGH_VAL_BASE: u32 = 0;
pub const DA9055_RESET_HIGH_VAL_MAX: u32 = DA9055_RESET_HIGH_VAL_MASK;
pub const DA9055_RESET_US_HIGH_BASE: u32 = 32768 /* min val in units of us */;
pub const DA9055_RESET_US_HIGH_INC: u32 = 32768 /* inv val in units of us */;
pub const DA9055_RESET_US_HIGH_STEP: u32 = 31;

/* DA9055_REG_BUCK_ILIM (addr=0x37)*/
pub const DA9055_BMEM_ILIM_SHIFT: u32 = 0;
pub const DA9055_ILIM_MASK: u32 = 0x3;
pub const DA9055_ILIM_500MA: u32 = 0x0;
pub const DA9055_ILIM_600MA: u32 = 0x1;
pub const DA9055_ILIM_700MA: u32 = 0x2;
pub const DA9055_ILIM_800MA: u32 = 0x3;
pub const DA9055_BCORE_ILIM_SHIFT: u32 = 2;

/* DA9055_REG_BCORE_MODE (addr=0x38) */
pub const DA9055_BMEM_MODE_SHIFT: u32 = 0;
pub const DA9055_MODE_MASK: u32 = 0x3;
pub const DA9055_MODE_AB: u32 = 0x0;
pub const DA9055_MODE_SLEEP: u32 = 0x1;
pub const DA9055_MODE_SYNCHRO: u32 = 0x2;
pub const DA9055_MODE_AUTO: u32 = 0x3;
pub const DA9055_BCORE_MODE_SHIFT: u32 = 2;

/* DA9055_REG_VBCORE_A/B (addr=0x39/0x41)*/
pub const DA9055_VBCORE_VAL_SHIFT: u32 = 0;
pub const DA9055_VBCORE_VAL_MASK: u32 = 0x3F;
pub const DA9055_VBCORE_VAL_BASE: u32 = 0x09;
pub const DA9055_VBCORE_VAL_MAX: u32 = DA9055_VBCORE_VAL_MASK;
pub const DA9055_VBCORE_VOLT_BASE: u32 = 750;
pub const DA9055_VBCORE_VOLT_INC: u32 = 25;
pub const DA9055_VBCORE_STEPS: u32 = 53;
pub const DA9055_VBCORE_VOLT_MIN: u32 = DA9055_VBCORE_VOLT_BASE;
pub const DA9055_BCORE_SL_SYNCHRO: u32 = (0<<7);
pub const DA9055_BCORE_SL_SLEEP: u32 = (1<<7);

/* DA9055_REG_VBMEM_A/B (addr=0x3A/0x42)*/
pub const DA9055_VBMEM_VAL_SHIFT: u32 = 0;
pub const DA9055_VBMEM_VAL_MASK: u32 = 0x3F;
pub const DA9055_VBMEM_VAL_BASE: u32 = 0x00;
pub const DA9055_VBMEM_VAL_MAX: u32 = DA9055_VBMEM_VAL_MASK;
pub const DA9055_VBMEM_VOLT_BASE: u32 = 925;
pub const DA9055_VBMEM_VOLT_INC: u32 = 25;
pub const DA9055_VBMEM_STEPS: u32 = 63;
pub const DA9055_VBMEM_VOLT_MIN: u32 = DA9055_VBMEM_VOLT_BASE;
pub const DA9055_BCMEM_SL_SYNCHRO: u32 = (0<<7);
pub const DA9055_BCMEM_SL_SLEEP: u32 = (1<<7);


/* DA9055_REG_VLDO (addr=0x3B-0x40/0x43-0x48)*/
pub const DA9055_VLDO_VAL_SHIFT: u32 = 0;
pub const DA9055_VLDO_VAL_MASK: u32 = 0x3F;
pub const DA9055_VLDO6_VAL_MASK: u32 = 0x7F;
pub const DA9055_VLDO_VAL_BASE: u32 = 0x02;
pub const DA9055_VLDO2_VAL_BASE: u32 = 0x03;
pub const DA9055_VLDO6_VAL_BASE: u32 = 0x00;
pub const DA9055_VLDO_VAL_MAX: u32 = DA9055_VLDO_VAL_MASK;
pub const DA9055_VLDO6_VAL_MAX: u32 = DA9055_VLDO6_VAL_MASK;
pub const DA9055_VLDO_VOLT_BASE: u32 = 900;
pub const DA9055_VLDO_VOLT_INC: u32 = 50;
pub const DA9055_VLDO6_VOLT_INC: u32 = 20;
pub const DA9055_VLDO_STEPS: u32 = 48;
pub const DA9055_VLDO5_STEPS: u32 = 37;
pub const DA9055_VLDO6_STEPS: u32 = 120;
pub const DA9055_VLDO_VOLT_MIN: u32 = DA9055_VLDO_VOLT_BASE;
pub const DA9055_LDO_MODE_SHIFT: u32 = 7;
pub const DA9055_LDO_SL_NORMAL: u32 = 0;
pub const DA9055_LDO_SL_SLEEP: u32 = 1;

/* DA9055_REG_OTP_CONT (addr=0x50) */
pub const DA9055_OTP_TIM_NORMAL: u32 = (0<<0);
pub const DA9055_OTP_TIM_MARGINAL: u32 = (1<<0);
pub const DA9055_OTP_GP_RD_SHIFT: u32 = 1;
pub const DA9055_OTP_APPS_RD_SHIFT: u32 = 2;
pub const DA9055_PC_DONE_SHIFT: u32 = 3;
pub const DA9055_OTP_GP_LOCK_SHIFT: u32 = 4;
pub const DA9055_OTP_APPS_LOCK_SHIFT: u32 = 5;
pub const DA9055_OTP_CONF_LOCK_SHIFT: u32 = 6;
pub const DA9055_OTP_WRITE_DIS_SHIFT: u32 = 7;

/* DA9055_REG_COUNT_S (addr=0x53) */
pub const DA9055_RTC_SEC: u32 = 0x3F;
pub const DA9055_RTC_MONITOR_EN: u32 = 0x40;
pub const DA9055_RTC_READ: u32 = 0x80;

/* DA9055_REG_COUNT_MI (addr=0x54) */
pub const DA9055_RTC_MIN: u32 = 0x3F;

/* DA9055_REG_COUNT_H (addr=0x55) */
pub const DA9055_RTC_HOUR: u32 = 0x1F;

/* DA9055_REG_COUNT_D (addr=0x56) */
pub const DA9055_RTC_DAY: u32 = 0x1F;

/* DA9055_REG_COUNT_MO (addr=0x57) */
pub const DA9055_RTC_MONTH: u32 = 0x0F;

/* DA9055_REG_COUNT_Y (addr=0x58) */
pub const DA9055_RTC_YEAR: u32 = 0x3F;
pub const DA9055_RTC_YEAR_BASE: u32 = 2000;

/* DA9055_REG_ALARM_MI (addr=0x59) */
pub const DA9055_RTC_ALM_MIN: u32 = 0x3F;
pub const DA9055_ALARM_STATUS_SHIFT: u32 = 6;
pub const DA9055_ALARM_STATUS_MASK: u32 = 0x3;
pub const DA9055_ALARM_STATUS_NO_ALARM: u32 = 0x0;
pub const DA9055_ALARM_STATUS_TICK: u32 = 0x1;
pub const DA9055_ALARM_STATUS_TIMER_ALARM: u32 = 0x2;
pub const DA9055_ALARM_STATUS_BOTH: u32 = 0x3;

/* DA9055_REG_ALARM_H (addr=0x5A) */
pub const DA9055_RTC_ALM_HOUR: u32 = 0x1F;

/* DA9055_REG_ALARM_D (addr=0x5B) */
pub const DA9055_RTC_ALM_DAY: u32 = 0x1F;

/* DA9055_REG_ALARM_MO (addr=0x5C) */
pub const DA9055_RTC_ALM_MONTH: u32 = 0x0F;
pub const DA9055_RTC_TICK_WAKE_MASK: u32 = 0x20;
pub const DA9055_RTC_TICK_WAKE_SHIFT: u32 = 5;
pub const DA9055_RTC_TICK_TYPE: u32 = 0x10;
pub const DA9055_RTC_TICK_TYPE_SHIFT: u32 = 0x4;
pub const DA9055_RTC_TICK_SEC: u32 = 0x0;
pub const DA9055_RTC_TICK_MIN: u32 = 0x1;
pub const DA9055_ALARAM_TICK_WAKE: u32 = 0x20;

/* DA9055_REG_ALARM_Y (addr=0x5D) */
pub const DA9055_RTC_TICK_EN: u32 = 0x80;
pub const DA9055_RTC_ALM_EN: u32 = 0x40;
pub const DA9055_RTC_TICK_ALM_MASK: u32 = 0xC0;
pub const DA9055_RTC_ALM_YEAR: u32 = 0x3F;

/* DA9055_REG_TRIM_CLDR (addr=0x62) */
pub const DA9055_TRIM_32K_SHIFT: u32 = 0;
pub const DA9055_TRIM_32K_MASK: u32 = 0x7F;
pub const DA9055_TRIM_DECREMENT: u32 = (1<<7);
pub const DA9055_TRIM_INCREMENT: u32 = (0<<7);
pub const DA9055_TRIM_VAL_BASE: u32 = 0x0;
pub const DA9055_TRIM_PPM_BASE: u32 = 0x0 /* min val in units of 0.1PPM */;
pub const DA9055_TRIM_PPM_INC: u32 = 19 /* min inc in units of 0.1PPM */;
pub const DA9055_TRIM_STEPS: u32 = 127;

/* DA9055_REG_CONFIG_A (addr=0x65) */
pub const DA9055_PM_I_V_VDDCORE: u32 = (0<<0);
pub const DA9055_PM_I_V_VDD_IO: u32 = (1<<0);
pub const DA9055_VDD_FAULT_TYPE_ACT_LOW: u32 = (0<<1);
pub const DA9055_VDD_FAULT_TYPE_ACT_HIGH: u32 = (1<<1);
pub const DA9055_PM_O_TYPE_PUSH_PULL: u32 = (0<<2);
pub const DA9055_PM_O_TYPE_OPEN_DRAIN: u32 = (1<<2);
pub const DA9055_IRQ_TYPE_ACT_LOW: u32 = (0<<3);
pub const DA9055_IRQ_TYPE_ACT_HIGH: u32 = (1<<3);
pub const DA9055_NIRQ_MODE_IMM: u32 = (0<<4);
pub const DA9055_NIRQ_MODE_ACTIVE: u32 = (1<<4);
pub const DA9055_GPI_V_VDDCORE: u32 = (0<<5);
pub const DA9055_GPI_V_VDD_IO: u32 = (1<<5);
pub const DA9055_PM_IF_V_VDDCORE: u32 = (0<<6);
pub const DA9055_PM_IF_V_VDD_IO: u32 = (1<<6);

/* DA9055_REG_CONFIG_B (addr=0x66) */
pub const DA9055_VDD_FAULT_VAL_SHIFT: u32 = 0;
pub const DA9055_VDD_FAULT_VAL_MASK: u32 = 0xF;
pub const DA9055_VDD_FAULT_VAL_BASE: u32 = 0x0;
pub const DA9055_VDD_FAULT_VAL_MAX: u32 = DA9055_VDD_FAULT_VAL_MASK;
pub const DA9055_VDD_FAULT_VOLT_BASE: u32 = 2500;
pub const DA9055_VDD_FAULT_VOLT_INC: u32 = 50;
pub const DA9055_VDD_FAULT_STEPS: u32 = 15;

pub const DA9055_VDD_HYST_VAL_SHIFT: u32 = 4;
pub const DA9055_VDD_HYST_VAL_MASK: u32 = 0x7;
pub const DA9055_VDD_HYST_VAL_BASE: u32 = 0x0;
pub const DA9055_VDD_HYST_VAL_MAX: u32 = DA9055_VDD_HYST_VAL_MASK;
pub const DA9055_VDD_HYST_VOLT_BASE: u32 = 100;
pub const DA9055_VDD_HYST_VOLT_INC: u32 = 50;
pub const DA9055_VDD_HYST_STEPS: u32 = 7;
pub const DA9055_VDD_HYST_VOLT_MIN: u32 = DA9055_VDD_HYST_VOLT_BASE;

pub const DA9055_VDD_FAULT_EN_SHIFT: u32 = 7;

/* DA9055_REG_CONFIG_C (addr=0x67) */
pub const DA9055_BCORE_CLK_INV_SHIFT: u32 = 0;
pub const DA9055_BMEM_CLK_INV_SHIFT: u32 = 1;
pub const DA9055_NFAULT_CONF_SHIFT: u32 = 2;
pub const DA9055_LDO_SD_SHIFT: u32 = 4;
pub const DA9055_LDO5_BYP_SHIFT: u32 = 6;
pub const DA9055_LDO6_BYP_SHIFT: u32 = 7;

/* DA9055_REG_CONFIG_D (addr=0x68) */
pub const DA9055_NONKEY_PIN_SHIFT: u32 = 0;
pub const DA9055_NONKEY_PIN_MASK: u32 = 0x3;
pub const DA9055_NONKEY_PIN_PORT_MODE: u32 = 0x0;
pub const DA9055_NONKEY_PIN_KEY_MODE: u32 = 0x1;
pub const DA9055_NONKEY_PIN_MULTI_FUNC: u32 = 0x2;
pub const DA9055_NONKEY_PIN_DEDICT: u32 = 0x3;
pub const DA9055_NONKEY_SD_SHIFT: u32 = 2;
pub const DA9055_KEY_DELAY_SHIFT: u32 = 3;
pub const DA9055_KEY_DELAY_MASK: u32 = 0x3;
pub const DA9055_KEY_DELAY_4S: u32 = 0x0;
pub const DA9055_KEY_DELAY_6S: u32 = 0x1;
pub const DA9055_KEY_DELAY_8S: u32 = 0x2;
pub const DA9055_KEY_DELAY_10S: u32 = 0x3;

/* DA9055_REG_CONFIG_E (addr=0x69) */
pub const DA9055_GPIO_PUPD_PULL_UP: u32 = 0x0;
pub const DA9055_GPIO_PUPD_OPEN_DRAIN: u32 = 0x1;
pub const DA9055_GPIO0_PUPD_SHIFT: u32 = 0;
pub const DA9055_GPIO1_PUPD_SHIFT: u32 = 1;
pub const DA9055_GPIO2_PUPD_SHIFT: u32 = 2;
pub const DA9055_UVOV_DELAY_SHIFT: u32 = 4;
pub const DA9055_UVOV_DELAY_MASK: u32 = 0x3;
pub const DA9055_RESET_DURATION_SHIFT: u32 = 6;
pub const DA9055_RESET_DURATION_MASK: u32 = 0x3;
pub const DA9055_RESET_DURATION_0MS: u32 = 0x0;
pub const DA9055_RESET_DURATION_100MS: u32 = 0x1;
pub const DA9055_RESET_DURATION_500MS: u32 = 0x2;
pub const DA9055_RESET_DURATION_1000MS: u32 = 0x3;

/* DA9055_REG_MON_REG_1 (addr=0x6A) */
pub const DA9055_MON_THRES_SHIFT: u32 = 0;
pub const DA9055_MON_THRES_MASK: u32 = 0x3;
pub const DA9055_MON_RES_SHIFT: u32 = 2;
pub const DA9055_MON_DEB_SHIFT: u32 = 3;
pub const DA9055_MON_MODE_SHIFT: u32 = 4;
pub const DA9055_MON_MODE_MASK: u32 = 0x3;
pub const DA9055_START_MAX_SHIFT: u32 = 6;
pub const DA9055_START_MAX_MASK: u32 = 0x3;

/* DA9055_REG_MON_REG_2 (addr=0x6B) */
pub const DA9055_LDO1_MON_EN_SHIFT: u32 = 0;
pub const DA9055_LDO2_MON_EN_SHIFT: u32 = 1;
pub const DA9055_LDO3_MON_EN_SHIFT: u32 = 2;
pub const DA9055_LDO4_MON_EN_SHIFT: u32 = 3;
pub const DA9055_LDO5_MON_EN_SHIFT: u32 = 4;
pub const DA9055_LDO6_MON_EN_SHIFT: u32 = 5;
pub const DA9055_BCORE_MON_EN_SHIFT: u32 = 6;
pub const DA9055_BMEM_MON_EN_SHIFT: u32 = 7;

/* DA9055_REG_CONFIG_F (addr=0x6C) */
pub const DA9055_LDO1_DEF_SHIFT: u32 = 0;
pub const DA9055_LDO2_DEF_SHIFT: u32 = 1;
pub const DA9055_LDO3_DEF_SHIFT: u32 = 2;
pub const DA9055_LDO4_DEF_SHIFT: u32 = 3;
pub const DA9055_LDO5_DEF_SHIFT: u32 = 4;
pub const DA9055_LDO6_DEF_SHIFT: u32 = 5;
pub const DA9055_BCORE_DEF_SHIFT: u32 = 6;
pub const DA9055_BMEM_DEF_SHIFT: u32 = 7;

/* DA9055_REG_MON_REG_4 (addr=0x6D) */
pub const DA9055_MON_A8_IDX_SHIFT: u32 = 0;
pub const DA9055_MON_A89_IDX_MASK: u32 = 0x3;
pub const DA9055_MON_A89_IDX_NONE: u32 = 0x0;
pub const DA9055_MON_A89_IDX_BUCKCORE: u32 = 0x1;
pub const DA9055_MON_A89_IDX_LDO3: u32 = 0x2;
pub const DA9055_MON_A9_IDX_SHIFT: u32 = 5;

/* DA9055_REG_MON_REG_5 (addr=0x6E) */
pub const DA9055_MON_A10_IDX_SHIFT: u32 = 0;
pub const DA9055_MON_A10_IDX_MASK: u32 = 0x3;
pub const DA9055_MON_A10_IDX_NONE: u32 = 0x0;
pub const DA9055_MON_A10_IDX_LDO1: u32 = 0x1;
pub const DA9055_MON_A10_IDX_LDO2: u32 = 0x2;
pub const DA9055_MON_A10_IDX_LDO5: u32 = 0x3;
pub const DA9055_MON_A10_IDX_LDO6: u32 = 0x4;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
