/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * DA9150 MFD Driver - Registers
 *
 * Copyright (c) 2014 Dialog Semiconductor
 *
 * Author: Adam Thomson <Adam.Thomson.Opensource@diasemi.com>
 */



/* Registers */
pub const DA9150_PAGE_CON: u32 = 0x000;
pub const DA9150_STATUS_A: u32 = 0x068;
pub const DA9150_STATUS_B: u32 = 0x069;
pub const DA9150_STATUS_C: u32 = 0x06A;
pub const DA9150_STATUS_D: u32 = 0x06B;
pub const DA9150_STATUS_E: u32 = 0x06C;
pub const DA9150_STATUS_F: u32 = 0x06D;
pub const DA9150_STATUS_G: u32 = 0x06E;
pub const DA9150_STATUS_H: u32 = 0x06F;
pub const DA9150_STATUS_I: u32 = 0x070;
pub const DA9150_STATUS_J: u32 = 0x071;
pub const DA9150_STATUS_K: u32 = 0x072;
pub const DA9150_STATUS_L: u32 = 0x073;
pub const DA9150_STATUS_N: u32 = 0x074;
pub const DA9150_FAULT_LOG_A: u32 = 0x076;
pub const DA9150_FAULT_LOG_B: u32 = 0x077;
pub const DA9150_EVENT_E: u32 = 0x078;
pub const DA9150_EVENT_F: u32 = 0x079;
pub const DA9150_EVENT_G: u32 = 0x07A;
pub const DA9150_EVENT_H: u32 = 0x07B;
pub const DA9150_IRQ_MASK_E: u32 = 0x07C;
pub const DA9150_IRQ_MASK_F: u32 = 0x07D;
pub const DA9150_IRQ_MASK_G: u32 = 0x07E;
pub const DA9150_IRQ_MASK_H: u32 = 0x07F;
pub const DA9150_PAGE_CON_1: u32 = 0x080;
pub const DA9150_CONFIG_A: u32 = 0x0E0;
pub const DA9150_CONFIG_B: u32 = 0x0E1;
pub const DA9150_CONFIG_C: u32 = 0x0E2;
pub const DA9150_CONFIG_D: u32 = 0x0E3;
pub const DA9150_CONFIG_E: u32 = 0x0E4;
pub const DA9150_CONTROL_A: u32 = 0x0E5;
pub const DA9150_CONTROL_B: u32 = 0x0E6;
pub const DA9150_CONTROL_C: u32 = 0x0E7;
pub const DA9150_GPIO_A_B: u32 = 0x0E8;
pub const DA9150_GPIO_C_D: u32 = 0x0E9;
pub const DA9150_GPIO_MODE_CONT: u32 = 0x0EA;
pub const DA9150_GPIO_CTRL_B: u32 = 0x0EB;
pub const DA9150_GPIO_CTRL_A: u32 = 0x0EC;
pub const DA9150_GPIO_CTRL_C: u32 = 0x0ED;
pub const DA9150_GPIO_CFG_A: u32 = 0x0EE;
pub const DA9150_GPIO_CFG_B: u32 = 0x0EF;
pub const DA9150_GPIO_CFG_C: u32 = 0x0F0;
pub const DA9150_GPADC_MAN: u32 = 0x0F2;
pub const DA9150_GPADC_RES_A: u32 = 0x0F4;
pub const DA9150_GPADC_RES_B: u32 = 0x0F5;
pub const DA9150_PAGE_CON_2: u32 = 0x100;
pub const DA9150_OTP_CONT_SHARED: u32 = 0x101;
pub const DA9150_INTERFACE_SHARED: u32 = 0x105;
pub const DA9150_CONFIG_A_SHARED: u32 = 0x106;
pub const DA9150_CONFIG_D_SHARED: u32 = 0x109;
pub const DA9150_ADETVB_CFG_C: u32 = 0x150;
pub const DA9150_ADETD_STAT: u32 = 0x151;
pub const DA9150_ADET_CMPSTAT: u32 = 0x152;
pub const DA9150_ADET_CTRL_A: u32 = 0x153;
pub const DA9150_ADETVB_CFG_B: u32 = 0x154;
pub const DA9150_ADETVB_CFG_A: u32 = 0x155;
pub const DA9150_ADETAC_CFG_A: u32 = 0x156;
pub const DA9150_ADDETAC_CFG_B: u32 = 0x157;
pub const DA9150_ADETAC_CFG_C: u32 = 0x158;
pub const DA9150_ADETAC_CFG_D: u32 = 0x159;
pub const DA9150_ADETVB_CFG_D: u32 = 0x15A;
pub const DA9150_ADETID_CFG_A: u32 = 0x15B;
pub const DA9150_ADET_RID_PT_CHG_H: u32 = 0x15C;
pub const DA9150_ADET_RID_PT_CHG_L: u32 = 0x15D;
pub const DA9150_PPR_TCTR_B: u32 = 0x160;
pub const DA9150_PPR_BKCTRL_A: u32 = 0x163;
pub const DA9150_PPR_BKCFG_A: u32 = 0x164;
pub const DA9150_PPR_BKCFG_B: u32 = 0x165;
pub const DA9150_PPR_CHGCTRL_A: u32 = 0x166;
pub const DA9150_PPR_CHGCTRL_B: u32 = 0x167;
pub const DA9150_PPR_CHGCTRL_C: u32 = 0x168;
pub const DA9150_PPR_TCTR_A: u32 = 0x169;
pub const DA9150_PPR_CHGCTRL_D: u32 = 0x16A;
pub const DA9150_PPR_CHGCTRL_E: u32 = 0x16B;
pub const DA9150_PPR_CHGCTRL_F: u32 = 0x16C;
pub const DA9150_PPR_CHGCTRL_G: u32 = 0x16D;
pub const DA9150_PPR_CHGCTRL_H: u32 = 0x16E;
pub const DA9150_PPR_CHGCTRL_I: u32 = 0x16F;
pub const DA9150_PPR_CHGCTRL_J: u32 = 0x170;
pub const DA9150_PPR_CHGCTRL_K: u32 = 0x171;
pub const DA9150_PPR_CHGCTRL_L: u32 = 0x172;
pub const DA9150_PPR_CHGCTRL_M: u32 = 0x173;
pub const DA9150_PPR_THYST_A: u32 = 0x174;
pub const DA9150_PPR_THYST_B: u32 = 0x175;
pub const DA9150_PPR_THYST_C: u32 = 0x176;
pub const DA9150_PPR_THYST_D: u32 = 0x177;
pub const DA9150_PPR_THYST_E: u32 = 0x178;
pub const DA9150_PPR_THYST_F: u32 = 0x179;
pub const DA9150_PPR_THYST_G: u32 = 0x17A;
pub const DA9150_PAGE_CON_3: u32 = 0x180;
pub const DA9150_PAGE_CON_4: u32 = 0x200;
pub const DA9150_PAGE_CON_5: u32 = 0x280;
pub const DA9150_PAGE_CON_6: u32 = 0x300;
pub const DA9150_COREBTLD_STAT_A: u32 = 0x302;
pub const DA9150_COREBTLD_CTRL_A: u32 = 0x303;
pub const DA9150_CORE_CONFIG_A: u32 = 0x304;
pub const DA9150_CORE_CONFIG_C: u32 = 0x305;
pub const DA9150_CORE_CONFIG_B: u32 = 0x306;
pub const DA9150_CORE_CFG_DATA_A: u32 = 0x307;
pub const DA9150_CORE_CFG_DATA_B: u32 = 0x308;
pub const DA9150_CORE_CMD_A: u32 = 0x309;
pub const DA9150_CORE_DATA_A: u32 = 0x30A;
pub const DA9150_CORE_DATA_B: u32 = 0x30B;
pub const DA9150_CORE_DATA_C: u32 = 0x30C;
pub const DA9150_CORE_DATA_D: u32 = 0x30D;
pub const DA9150_CORE2WIRE_STAT_A: u32 = 0x310;
pub const DA9150_CORE2WIRE_CTRL_A: u32 = 0x311;
pub const DA9150_FW_CTRL_A: u32 = 0x312;
pub const DA9150_FW_CTRL_C: u32 = 0x313;
pub const DA9150_FW_CTRL_D: u32 = 0x314;
pub const DA9150_FG_CTRL_A: u32 = 0x315;
pub const DA9150_FG_CTRL_B: u32 = 0x316;
pub const DA9150_FW_CTRL_E: u32 = 0x317;
pub const DA9150_FW_CTRL_B: u32 = 0x318;
pub const DA9150_GPADC_CMAN: u32 = 0x320;
pub const DA9150_GPADC_CRES_A: u32 = 0x322;
pub const DA9150_GPADC_CRES_B: u32 = 0x323;
pub const DA9150_CC_CFG_A: u32 = 0x328;
pub const DA9150_CC_CFG_B: u32 = 0x329;
pub const DA9150_CC_ICHG_RES_A: u32 = 0x32A;
pub const DA9150_CC_ICHG_RES_B: u32 = 0x32B;
pub const DA9150_CC_IAVG_RES_A: u32 = 0x32C;
pub const DA9150_CC_IAVG_RES_B: u32 = 0x32D;
pub const DA9150_TAUX_CTRL_A: u32 = 0x330;
pub const DA9150_TAUX_RELOAD_H: u32 = 0x332;
pub const DA9150_TAUX_RELOAD_L: u32 = 0x333;
pub const DA9150_TAUX_VALUE_H: u32 = 0x334;
pub const DA9150_TAUX_VALUE_L: u32 = 0x335;
pub const DA9150_AUX_DATA_0: u32 = 0x338;
pub const DA9150_AUX_DATA_1: u32 = 0x339;
pub const DA9150_AUX_DATA_2: u32 = 0x33A;
pub const DA9150_AUX_DATA_3: u32 = 0x33B;
pub const DA9150_BIF_CTRL: u32 = 0x340;
pub const DA9150_TBAT_CTRL_A: u32 = 0x342;
pub const DA9150_TBAT_CTRL_B: u32 = 0x343;
pub const DA9150_TBAT_RES_A: u32 = 0x344;
pub const DA9150_TBAT_RES_B: u32 = 0x345;

/* DA9150_PAGE_CON = 0x000 */
pub const DA9150_PAGE_SHIFT: u32 = 0;
pub const DA9150_PAGE_MASK: u32 = (0x3f << 0);
pub const DA9150_I2C_PAGE_SHIFT: u32 = 1;
pub const DA9150_I2C_PAGE_MASK: u32 = (0x1f << 1);
pub const DA9150_WRITE_MODE_SHIFT: u32 = 6;
pub const DA9150_WRITE_MODE_MASK: u32 = (1u32 << 6);
pub const DA9150_REVERT_SHIFT: u32 = 7;
pub const DA9150_REVERT_MASK: u32 = (1u32 << 7);

/* DA9150_STATUS_A = 0x068 */
pub const DA9150_WKUP_STAT_SHIFT: u32 = 2;
pub const DA9150_WKUP_STAT_MASK: u32 = (0x0f << 2);
pub const DA9150_SLEEP_STAT_SHIFT: u32 = 6;
pub const DA9150_SLEEP_STAT_MASK: u32 = (0x03 << 6);

/* DA9150_STATUS_B = 0x069 */
pub const DA9150_VFAULT_STAT_SHIFT: u32 = 0;
pub const DA9150_VFAULT_STAT_MASK: u32 = (1u32 << 0);
pub const DA9150_TFAULT_STAT_SHIFT: u32 = 1;
pub const DA9150_TFAULT_STAT_MASK: u32 = (1u32 << 1);

/* DA9150_STATUS_C = 0x06A */
pub const DA9150_VDD33_STAT_SHIFT: u32 = 0;
pub const DA9150_VDD33_STAT_MASK: u32 = (1u32 << 0);
pub const DA9150_VDD33_SLEEP_SHIFT: u32 = 1;
pub const DA9150_VDD33_SLEEP_MASK: u32 = (1u32 << 1);
pub const DA9150_LFOSC_STAT_SHIFT: u32 = 7;
pub const DA9150_LFOSC_STAT_MASK: u32 = (1u32 << 7);

/* DA9150_STATUS_D = 0x06B */
pub const DA9150_GPIOA_STAT_SHIFT: u32 = 0;
pub const DA9150_GPIOA_STAT_MASK: u32 = (1u32 << 0);
pub const DA9150_GPIOB_STAT_SHIFT: u32 = 1;
pub const DA9150_GPIOB_STAT_MASK: u32 = (1u32 << 1);
pub const DA9150_GPIOC_STAT_SHIFT: u32 = 2;
pub const DA9150_GPIOC_STAT_MASK: u32 = (1u32 << 2);
pub const DA9150_GPIOD_STAT_SHIFT: u32 = 3;
pub const DA9150_GPIOD_STAT_MASK: u32 = (1u32 << 3);

/* DA9150_STATUS_E = 0x06C */
pub const DA9150_DTYPE_SHIFT: u32 = 0;
pub const DA9150_DTYPE_MASK: u32 = (0x1f << 0);
pub const DA9150_DTYPE_DT_NIL: u32 = (0x00 << 0);
pub const DA9150_DTYPE_DT_USB_OTG: u32 = (1u32 << 0);
pub const DA9150_DTYPE_DT_USB_STD: u32 = (0x02 << 0);
pub const DA9150_DTYPE_DT_USB_CHG: u32 = (0x03 << 0);
pub const DA9150_DTYPE_DT_ACA_CHG: u32 = (0x04 << 0);
pub const DA9150_DTYPE_DT_ACA_OTG: u32 = (0x05 << 0);
pub const DA9150_DTYPE_DT_ACA_DOC: u32 = (0x06 << 0);
pub const DA9150_DTYPE_DT_DED_CHG: u32 = (0x07 << 0);
pub const DA9150_DTYPE_DT_CR5_CHG: u32 = (0x08 << 0);
pub const DA9150_DTYPE_DT_CR4_CHG: u32 = (0x0c << 0);
pub const DA9150_DTYPE_DT_PT_CHG: u32 = (0x11 << 0);
pub const DA9150_DTYPE_DT_NN_ACC: u32 = (0x16 << 0);
pub const DA9150_DTYPE_DT_NN_CHG: u32 = (0x17 << 0);

/* DA9150_STATUS_F = 0x06D */
pub const DA9150_SESS_VLD_SHIFT: u32 = 0;
pub const DA9150_SESS_VLD_MASK: u32 = (1u32 << 0);
pub const DA9150_ID_ERR_SHIFT: u32 = 1;
pub const DA9150_ID_ERR_MASK: u32 = (1u32 << 1);
pub const DA9150_PT_CHG_SHIFT: u32 = 2;
pub const DA9150_PT_CHG_MASK: u32 = (1u32 << 2);

/* DA9150_STATUS_G = 0x06E */
pub const DA9150_RID_SHIFT: u32 = 0;
pub const DA9150_RID_MASK: u32 = (0xff << 0);

/* DA9150_STATUS_H = 0x06F */
pub const DA9150_VBUS_STAT_SHIFT: u32 = 0;
pub const DA9150_VBUS_STAT_MASK: u32 = (0x07 << 0);
pub const DA9150_VBUS_STAT_OFF: u32 = (0x00 << 0);
pub const DA9150_VBUS_STAT_WAIT: u32 = (1u32 << 0);
pub const DA9150_VBUS_STAT_CHG: u32 = (0x02 << 0);
pub const DA9150_VBUS_TRED_SHIFT: u32 = 3;
pub const DA9150_VBUS_TRED_MASK: u32 = (1u32 << 3);
pub const DA9150_VBUS_DROP_STAT_SHIFT: u32 = 4;
pub const DA9150_VBUS_DROP_STAT_MASK: u32 = (0x0f << 4);

/* DA9150_STATUS_I = 0x070 */
pub const DA9150_VBUS_ISET_STAT_SHIFT: u32 = 0;
pub const DA9150_VBUS_ISET_STAT_MASK: u32 = (0x1f << 0);
pub const DA9150_VBUS_OT_SHIFT: u32 = 7;
pub const DA9150_VBUS_OT_MASK: u32 = (1u32 << 7);

/* DA9150_STATUS_J = 0x071 */
pub const DA9150_CHG_STAT_SHIFT: u32 = 0;
pub const DA9150_CHG_STAT_MASK: u32 = (0x0f << 0);
pub const DA9150_CHG_STAT_OFF: u32 = (0x00 << 0);
pub const DA9150_CHG_STAT_SUSP: u32 = (1u32 << 0);
pub const DA9150_CHG_STAT_ACT: u32 = (0x02 << 0);
pub const DA9150_CHG_STAT_PRE: u32 = (0x03 << 0);
pub const DA9150_CHG_STAT_CC: u32 = (0x04 << 0);
pub const DA9150_CHG_STAT_CV: u32 = (0x05 << 0);
pub const DA9150_CHG_STAT_FULL: u32 = (0x06 << 0);
pub const DA9150_CHG_STAT_TEMP: u32 = (0x07 << 0);
pub const DA9150_CHG_STAT_TIME: u32 = (0x08 << 0);
pub const DA9150_CHG_STAT_BAT: u32 = (0x09 << 0);
pub const DA9150_CHG_TEMP_SHIFT: u32 = 4;
pub const DA9150_CHG_TEMP_MASK: u32 = (0x07 << 4);
pub const DA9150_CHG_TEMP_UNDER: u32 = (0x06 << 4);
pub const DA9150_CHG_TEMP_OVER: u32 = (0x07 << 4);
pub const DA9150_CHG_IEND_STAT_SHIFT: u32 = 7;
pub const DA9150_CHG_IEND_STAT_MASK: u32 = (1u32 << 7);

/* DA9150_STATUS_K = 0x072 */
pub const DA9150_CHG_IAV_H_SHIFT: u32 = 0;
pub const DA9150_CHG_IAV_H_MASK: u32 = (0xff << 0);

/* DA9150_STATUS_L = 0x073 */
pub const DA9150_CHG_IAV_L_SHIFT: u32 = 5;
pub const DA9150_CHG_IAV_L_MASK: u32 = (0x07 << 5);

/* DA9150_STATUS_N = 0x074 */
pub const DA9150_CHG_TIME_SHIFT: u32 = 1;
pub const DA9150_CHG_TIME_MASK: u32 = (1u32 << 1);
pub const DA9150_CHG_TRED_SHIFT: u32 = 2;
pub const DA9150_CHG_TRED_MASK: u32 = (1u32 << 2);
pub const DA9150_CHG_TJUNC_CLASS_SHIFT: u32 = 3;
pub const DA9150_CHG_TJUNC_CLASS_MASK: u32 = (0x07 << 3);
pub const DA9150_CHG_TJUNC_CLASS_6: u32 = (0x06 << 3);
pub const DA9150_EBS_STAT_SHIFT: u32 = 6;
pub const DA9150_EBS_STAT_MASK: u32 = (1u32 << 6);
pub const DA9150_CHG_BAT_REMOVED_SHIFT: u32 = 7;
pub const DA9150_CHG_BAT_REMOVED_MASK: u32 = (1u32 << 7);

/* DA9150_FAULT_LOG_A = 0x076 */
pub const DA9150_TEMP_FAULT_SHIFT: u32 = 0;
pub const DA9150_TEMP_FAULT_MASK: u32 = (1u32 << 0);
pub const DA9150_VSYS_FAULT_SHIFT: u32 = 1;
pub const DA9150_VSYS_FAULT_MASK: u32 = (1u32 << 1);
pub const DA9150_START_FAULT_SHIFT: u32 = 2;
pub const DA9150_START_FAULT_MASK: u32 = (1u32 << 2);
pub const DA9150_EXT_FAULT_SHIFT: u32 = 3;
pub const DA9150_EXT_FAULT_MASK: u32 = (1u32 << 3);
pub const DA9150_POR_FAULT_SHIFT: u32 = 4;
pub const DA9150_POR_FAULT_MASK: u32 = (1u32 << 4);

/* DA9150_FAULT_LOG_B = 0x077 */
pub const DA9150_VBUS_FAULT_SHIFT: u32 = 0;
pub const DA9150_VBUS_FAULT_MASK: u32 = (1u32 << 0);
pub const DA9150_OTG_FAULT_SHIFT: u32 = 1;
pub const DA9150_OTG_FAULT_MASK: u32 = (1u32 << 1);

/* DA9150_EVENT_E = 0x078 */
pub const DA9150_E_VBUS_SHIFT: u32 = 0;
pub const DA9150_E_VBUS_MASK: u32 = (1u32 << 0);
pub const DA9150_E_CHG_SHIFT: u32 = 1;
pub const DA9150_E_CHG_MASK: u32 = (1u32 << 1);
pub const DA9150_E_TCLASS_SHIFT: u32 = 2;
pub const DA9150_E_TCLASS_MASK: u32 = (1u32 << 2);
pub const DA9150_E_TJUNC_SHIFT: u32 = 3;
pub const DA9150_E_TJUNC_MASK: u32 = (1u32 << 3);
pub const DA9150_E_VFAULT_SHIFT: u32 = 4;
pub const DA9150_E_VFAULT_MASK: u32 = (1u32 << 4);
pub const DA9150_EVENTS_H_SHIFT: u32 = 5;
pub const DA9150_EVENTS_H_MASK: u32 = (1u32 << 5);
pub const DA9150_EVENTS_G_SHIFT: u32 = 6;
pub const DA9150_EVENTS_G_MASK: u32 = (1u32 << 6);
pub const DA9150_EVENTS_F_SHIFT: u32 = 7;
pub const DA9150_EVENTS_F_MASK: u32 = (1u32 << 7);

/* DA9150_EVENT_F = 0x079 */
pub const DA9150_E_CONF_SHIFT: u32 = 0;
pub const DA9150_E_CONF_MASK: u32 = (1u32 << 0);
pub const DA9150_E_DAT_SHIFT: u32 = 1;
pub const DA9150_E_DAT_MASK: u32 = (1u32 << 1);
pub const DA9150_E_DTYPE_SHIFT: u32 = 3;
pub const DA9150_E_DTYPE_MASK: u32 = (1u32 << 3);
pub const DA9150_E_ID_SHIFT: u32 = 4;
pub const DA9150_E_ID_MASK: u32 = (1u32 << 4);
pub const DA9150_E_ADP_SHIFT: u32 = 5;
pub const DA9150_E_ADP_MASK: u32 = (1u32 << 5);
pub const DA9150_E_SESS_END_SHIFT: u32 = 6;
pub const DA9150_E_SESS_END_MASK: u32 = (1u32 << 6);
pub const DA9150_E_SESS_VLD_SHIFT: u32 = 7;
pub const DA9150_E_SESS_VLD_MASK: u32 = (1u32 << 7);

/* DA9150_EVENT_G = 0x07A */
pub const DA9150_E_FG_SHIFT: u32 = 0;
pub const DA9150_E_FG_MASK: u32 = (1u32 << 0);
pub const DA9150_E_GP_SHIFT: u32 = 1;
pub const DA9150_E_GP_MASK: u32 = (1u32 << 1);
pub const DA9150_E_TBAT_SHIFT: u32 = 2;
pub const DA9150_E_TBAT_MASK: u32 = (1u32 << 2);
pub const DA9150_E_GPIOA_SHIFT: u32 = 3;
pub const DA9150_E_GPIOA_MASK: u32 = (1u32 << 3);
pub const DA9150_E_GPIOB_SHIFT: u32 = 4;
pub const DA9150_E_GPIOB_MASK: u32 = (1u32 << 4);
pub const DA9150_E_GPIOC_SHIFT: u32 = 5;
pub const DA9150_E_GPIOC_MASK: u32 = (1u32 << 5);
pub const DA9150_E_GPIOD_SHIFT: u32 = 6;
pub const DA9150_E_GPIOD_MASK: u32 = (1u32 << 6);
pub const DA9150_E_GPADC_SHIFT: u32 = 7;
pub const DA9150_E_GPADC_MASK: u32 = (1u32 << 7);

/* DA9150_EVENT_H = 0x07B */
pub const DA9150_E_WKUP_SHIFT: u32 = 0;
pub const DA9150_E_WKUP_MASK: u32 = (1u32 << 0);

/* DA9150_IRQ_MASK_E = 0x07C */
pub const DA9150_M_VBUS_SHIFT: u32 = 0;
pub const DA9150_M_VBUS_MASK: u32 = (1u32 << 0);
pub const DA9150_M_CHG_SHIFT: u32 = 1;
pub const DA9150_M_CHG_MASK: u32 = (1u32 << 1);
pub const DA9150_M_TJUNC_SHIFT: u32 = 3;
pub const DA9150_M_TJUNC_MASK: u32 = (1u32 << 3);
pub const DA9150_M_VFAULT_SHIFT: u32 = 4;
pub const DA9150_M_VFAULT_MASK: u32 = (1u32 << 4);

/* DA9150_IRQ_MASK_F = 0x07D */
pub const DA9150_M_CONF_SHIFT: u32 = 0;
pub const DA9150_M_CONF_MASK: u32 = (1u32 << 0);
pub const DA9150_M_DAT_SHIFT: u32 = 1;
pub const DA9150_M_DAT_MASK: u32 = (1u32 << 1);
pub const DA9150_M_DTYPE_SHIFT: u32 = 3;
pub const DA9150_M_DTYPE_MASK: u32 = (1u32 << 3);
pub const DA9150_M_ID_SHIFT: u32 = 4;
pub const DA9150_M_ID_MASK: u32 = (1u32 << 4);
pub const DA9150_M_ADP_SHIFT: u32 = 5;
pub const DA9150_M_ADP_MASK: u32 = (1u32 << 5);
pub const DA9150_M_SESS_END_SHIFT: u32 = 6;
pub const DA9150_M_SESS_END_MASK: u32 = (1u32 << 6);
pub const DA9150_M_SESS_VLD_SHIFT: u32 = 7;
pub const DA9150_M_SESS_VLD_MASK: u32 = (1u32 << 7);

/* DA9150_IRQ_MASK_G = 0x07E */
pub const DA9150_M_FG_SHIFT: u32 = 0;
pub const DA9150_M_FG_MASK: u32 = (1u32 << 0);
pub const DA9150_M_GP_SHIFT: u32 = 1;
pub const DA9150_M_GP_MASK: u32 = (1u32 << 1);
pub const DA9150_M_TBAT_SHIFT: u32 = 2;
pub const DA9150_M_TBAT_MASK: u32 = (1u32 << 2);
pub const DA9150_M_GPIOA_SHIFT: u32 = 3;
pub const DA9150_M_GPIOA_MASK: u32 = (1u32 << 3);
pub const DA9150_M_GPIOB_SHIFT: u32 = 4;
pub const DA9150_M_GPIOB_MASK: u32 = (1u32 << 4);
pub const DA9150_M_GPIOC_SHIFT: u32 = 5;
pub const DA9150_M_GPIOC_MASK: u32 = (1u32 << 5);
pub const DA9150_M_GPIOD_SHIFT: u32 = 6;
pub const DA9150_M_GPIOD_MASK: u32 = (1u32 << 6);
pub const DA9150_M_GPADC_SHIFT: u32 = 7;
pub const DA9150_M_GPADC_MASK: u32 = (1u32 << 7);

/* DA9150_IRQ_MASK_H = 0x07F */
pub const DA9150_M_WKUP_SHIFT: u32 = 0;
pub const DA9150_M_WKUP_MASK: u32 = (1u32 << 0);

/* DA9150_PAGE_CON_1 = 0x080 */
pub const DA9150_PAGE_SHIFT: u32 = 0;
pub const DA9150_PAGE_MASK: u32 = (0x3f << 0);
pub const DA9150_WRITE_MODE_SHIFT: u32 = 6;
pub const DA9150_WRITE_MODE_MASK: u32 = (1u32 << 6);
pub const DA9150_REVERT_SHIFT: u32 = 7;
pub const DA9150_REVERT_MASK: u32 = (1u32 << 7);

/* DA9150_CONFIG_A = 0x0E0 */
pub const DA9150_RESET_DUR_SHIFT: u32 = 0;
pub const DA9150_RESET_DUR_MASK: u32 = (0x03 << 0);
pub const DA9150_RESET_EXT_SHIFT: u32 = 2;
pub const DA9150_RESET_EXT_MASK: u32 = (0x03 << 2);
pub const DA9150_START_MAX_SHIFT: u32 = 4;
pub const DA9150_START_MAX_MASK: u32 = (0x03 << 4);
pub const DA9150_PS_WAIT_EN_SHIFT: u32 = 6;
pub const DA9150_PS_WAIT_EN_MASK: u32 = (1u32 << 6);
pub const DA9150_PS_DISABLE_DIRECT_SHIFT: u32 = 7;
pub const DA9150_PS_DISABLE_DIRECT_MASK: u32 = (1u32 << 7);

/* DA9150_CONFIG_B = 0x0E1 */
pub const DA9150_VFAULT_ADJ_SHIFT: u32 = 0;
pub const DA9150_VFAULT_ADJ_MASK: u32 = (0x0f << 0);
pub const DA9150_VFAULT_HYST_SHIFT: u32 = 4;
pub const DA9150_VFAULT_HYST_MASK: u32 = (0x07 << 4);
pub const DA9150_VFAULT_EN_SHIFT: u32 = 7;
pub const DA9150_VFAULT_EN_MASK: u32 = (1u32 << 7);

/* DA9150_CONFIG_C = 0x0E2 */
pub const DA9150_VSYS_MIN_SHIFT: u32 = 3;
pub const DA9150_VSYS_MIN_MASK: u32 = (0x1f << 3);

/* DA9150_CONFIG_D = 0x0E3 */
pub const DA9150_LFOSC_EXT_SHIFT: u32 = 0;
pub const DA9150_LFOSC_EXT_MASK: u32 = (1u32 << 0);
pub const DA9150_VDD33_DWN_SHIFT: u32 = 1;
pub const DA9150_VDD33_DWN_MASK: u32 = (1u32 << 1);
pub const DA9150_WKUP_PM_EN_SHIFT: u32 = 2;
pub const DA9150_WKUP_PM_EN_MASK: u32 = (1u32 << 2);
pub const DA9150_WKUP_CE_SEL_SHIFT: u32 = 3;
pub const DA9150_WKUP_CE_SEL_MASK: u32 = (0x03 << 3);
pub const DA9150_WKUP_CLK32K_EN_SHIFT: u32 = 5;
pub const DA9150_WKUP_CLK32K_EN_MASK: u32 = (1u32 << 5);
pub const DA9150_DISABLE_DEL_SHIFT: u32 = 7;
pub const DA9150_DISABLE_DEL_MASK: u32 = (1u32 << 7);

/* DA9150_CONFIG_E = 0x0E4 */
pub const DA9150_PM_SPKSUP_DIS_SHIFT: u32 = 0;
pub const DA9150_PM_SPKSUP_DIS_MASK: u32 = (1u32 << 0);
pub const DA9150_PM_MERGE_SHIFT: u32 = 1;
pub const DA9150_PM_MERGE_MASK: u32 = (1u32 << 1);
pub const DA9150_PM_SR_OFF_SHIFT: u32 = 2;
pub const DA9150_PM_SR_OFF_MASK: u32 = (1u32 << 2);
pub const DA9150_PM_TIMEOUT_EN_SHIFT: u32 = 3;
pub const DA9150_PM_TIMEOUT_EN_MASK: u32 = (1u32 << 3);
pub const DA9150_PM_DLY_SEL_SHIFT: u32 = 4;
pub const DA9150_PM_DLY_SEL_MASK: u32 = (0x07 << 4);
pub const DA9150_PM_OUT_DLY_SEL_SHIFT: u32 = 7;
pub const DA9150_PM_OUT_DLY_SEL_MASK: u32 = (1u32 << 7);

/* DA9150_CONTROL_A = 0x0E5 */
pub const DA9150_VDD33_SL_SHIFT: u32 = 0;
pub const DA9150_VDD33_SL_MASK: u32 = (1u32 << 0);
pub const DA9150_VDD33_LPM_SHIFT: u32 = 1;
pub const DA9150_VDD33_LPM_MASK: u32 = (0x03 << 1);
pub const DA9150_VDD33_EN_SHIFT: u32 = 3;
pub const DA9150_VDD33_EN_MASK: u32 = (1u32 << 3);
pub const DA9150_GPI_LPM_SHIFT: u32 = 6;
pub const DA9150_GPI_LPM_MASK: u32 = (1u32 << 6);
pub const DA9150_PM_IF_LPM_SHIFT: u32 = 7;
pub const DA9150_PM_IF_LPM_MASK: u32 = (1u32 << 7);

/* DA9150_CONTROL_B = 0x0E6 */
pub const DA9150_LPM_SHIFT: u32 = 0;
pub const DA9150_LPM_MASK: u32 = (1u32 << 0);
pub const DA9150_RESET_SHIFT: u32 = 1;
pub const DA9150_RESET_MASK: u32 = (1u32 << 1);
pub const DA9150_RESET_USRCONF_EN_SHIFT: u32 = 2;
pub const DA9150_RESET_USRCONF_EN_MASK: u32 = (1u32 << 2);

/* DA9150_CONTROL_C = 0x0E7 */
pub const DA9150_DISABLE_SHIFT: u32 = 0;
pub const DA9150_DISABLE_MASK: u32 = (1u32 << 0);

/* DA9150_GPIO_A_B = 0x0E8 */
pub const DA9150_GPIOA_PIN_SHIFT: u32 = 0;
pub const DA9150_GPIOA_PIN_MASK: u32 = (0x07 << 0);
pub const DA9150_GPIOA_PIN_GPI: u32 = (0x00 << 0);
pub const DA9150_GPIOA_PIN_GPO_OD: u32 = (1u32 << 0);
pub const DA9150_GPIOA_TYPE_SHIFT: u32 = 3;
pub const DA9150_GPIOA_TYPE_MASK: u32 = (1u32 << 3);
pub const DA9150_GPIOB_PIN_SHIFT: u32 = 4;
pub const DA9150_GPIOB_PIN_MASK: u32 = (0x07 << 4);
pub const DA9150_GPIOB_PIN_GPI: u32 = (0x00 << 4);
pub const DA9150_GPIOB_PIN_GPO_OD: u32 = (1u32 << 4);
pub const DA9150_GPIOB_TYPE_SHIFT: u32 = 7;
pub const DA9150_GPIOB_TYPE_MASK: u32 = (1u32 << 7);

/* DA9150_GPIO_C_D = 0x0E9 */
pub const DA9150_GPIOC_PIN_SHIFT: u32 = 0;
pub const DA9150_GPIOC_PIN_MASK: u32 = (0x07 << 0);
pub const DA9150_GPIOC_PIN_GPI: u32 = (0x00 << 0);
pub const DA9150_GPIOC_PIN_GPO_OD: u32 = (1u32 << 0);
pub const DA9150_GPIOC_TYPE_SHIFT: u32 = 3;
pub const DA9150_GPIOC_TYPE_MASK: u32 = (1u32 << 3);
pub const DA9150_GPIOD_PIN_SHIFT: u32 = 4;
pub const DA9150_GPIOD_PIN_MASK: u32 = (0x07 << 4);
pub const DA9150_GPIOD_PIN_GPI: u32 = (0x00 << 4);
pub const DA9150_GPIOD_PIN_GPO_OD: u32 = (1u32 << 4);
pub const DA9150_GPIOD_TYPE_SHIFT: u32 = 7;
pub const DA9150_GPIOD_TYPE_MASK: u32 = (1u32 << 7);

/* DA9150_GPIO_MODE_CONT = 0x0EA */
pub const DA9150_GPIOA_MODE_SHIFT: u32 = 0;
pub const DA9150_GPIOA_MODE_MASK: u32 = (1u32 << 0);
pub const DA9150_GPIOB_MODE_SHIFT: u32 = 1;
pub const DA9150_GPIOB_MODE_MASK: u32 = (1u32 << 1);
pub const DA9150_GPIOC_MODE_SHIFT: u32 = 2;
pub const DA9150_GPIOC_MODE_MASK: u32 = (1u32 << 2);
pub const DA9150_GPIOD_MODE_SHIFT: u32 = 3;
pub const DA9150_GPIOD_MODE_MASK: u32 = (1u32 << 3);
pub const DA9150_GPIOA_CONT_SHIFT: u32 = 4;
pub const DA9150_GPIOA_CONT_MASK: u32 = (1u32 << 4);
pub const DA9150_GPIOB_CONT_SHIFT: u32 = 5;
pub const DA9150_GPIOB_CONT_MASK: u32 = (1u32 << 5);
pub const DA9150_GPIOC_CONT_SHIFT: u32 = 6;
pub const DA9150_GPIOC_CONT_MASK: u32 = (1u32 << 6);
pub const DA9150_GPIOD_CONT_SHIFT: u32 = 7;
pub const DA9150_GPIOD_CONT_MASK: u32 = (1u32 << 7);

/* DA9150_GPIO_CTRL_B = 0x0EB */
pub const DA9150_WAKE_PIN_SHIFT: u32 = 0;
pub const DA9150_WAKE_PIN_MASK: u32 = (0x03 << 0);
pub const DA9150_WAKE_MODE_SHIFT: u32 = 2;
pub const DA9150_WAKE_MODE_MASK: u32 = (1u32 << 2);
pub const DA9150_WAKE_CONT_SHIFT: u32 = 3;
pub const DA9150_WAKE_CONT_MASK: u32 = (1u32 << 3);
pub const DA9150_WAKE_DLY_SHIFT: u32 = 4;
pub const DA9150_WAKE_DLY_MASK: u32 = (1u32 << 4);

/* DA9150_GPIO_CTRL_A = 0x0EC */
pub const DA9150_GPIOA_ANAEN_SHIFT: u32 = 0;
pub const DA9150_GPIOA_ANAEN_MASK: u32 = (1u32 << 0);
pub const DA9150_GPIOB_ANAEN_SHIFT: u32 = 1;
pub const DA9150_GPIOB_ANAEN_MASK: u32 = (1u32 << 1);
pub const DA9150_GPIOC_ANAEN_SHIFT: u32 = 2;
pub const DA9150_GPIOC_ANAEN_MASK: u32 = (1u32 << 2);
pub const DA9150_GPIOD_ANAEN_SHIFT: u32 = 3;
pub const DA9150_GPIOD_ANAEN_MASK: u32 = (1u32 << 3);
pub const DA9150_GPIO_ANAEN: u32 = 0x01;
pub const DA9150_GPIO_ANAEN_MASK: u32 = 0x0F;
pub const DA9150_CHGLED_PIN_SHIFT: u32 = 5;
pub const DA9150_CHGLED_PIN_MASK: u32 = (0x07 << 5);

/* DA9150_GPIO_CTRL_C = 0x0ED */
pub const DA9150_CHGBL_DUR_SHIFT: u32 = 0;
pub const DA9150_CHGBL_DUR_MASK: u32 = (0x03 << 0);
pub const DA9150_CHGBL_DBL_SHIFT: u32 = 2;
pub const DA9150_CHGBL_DBL_MASK: u32 = (1u32 << 2);
pub const DA9150_CHGBL_FRQ_SHIFT: u32 = 3;
pub const DA9150_CHGBL_FRQ_MASK: u32 = (0x03 << 3);
pub const DA9150_CHGBL_FLKR_SHIFT: u32 = 5;
pub const DA9150_CHGBL_FLKR_MASK: u32 = (1u32 << 5);

/* DA9150_GPIO_CFG_A = 0x0EE */
pub const DA9150_CE_LPM_DEB_SHIFT: u32 = 0;
pub const DA9150_CE_LPM_DEB_MASK: u32 = (0x07 << 0);

/* DA9150_GPIO_CFG_B = 0x0EF */
pub const DA9150_GPIOA_PUPD_SHIFT: u32 = 0;
pub const DA9150_GPIOA_PUPD_MASK: u32 = (1u32 << 0);
pub const DA9150_GPIOB_PUPD_SHIFT: u32 = 1;
pub const DA9150_GPIOB_PUPD_MASK: u32 = (1u32 << 1);
pub const DA9150_GPIOC_PUPD_SHIFT: u32 = 2;
pub const DA9150_GPIOC_PUPD_MASK: u32 = (1u32 << 2);
pub const DA9150_GPIOD_PUPD_SHIFT: u32 = 3;
pub const DA9150_GPIOD_PUPD_MASK: u32 = (1u32 << 3);
pub const DA9150_GPIO_PUPD_MASK: u32 = (0xF << 0);
pub const DA9150_GPI_DEB_SHIFT: u32 = 4;
pub const DA9150_GPI_DEB_MASK: u32 = (0x07 << 4);
pub const DA9150_LPM_EN_SHIFT: u32 = 7;
pub const DA9150_LPM_EN_MASK: u32 = (1u32 << 7);

/* DA9150_GPIO_CFG_C = 0x0F0 */
pub const DA9150_GPI_V_SHIFT: u32 = 0;
pub const DA9150_GPI_V_MASK: u32 = (1u32 << 0);
pub const DA9150_VDDIO_INT_SHIFT: u32 = 1;
pub const DA9150_VDDIO_INT_MASK: u32 = (1u32 << 1);
pub const DA9150_FAULT_PIN_SHIFT: u32 = 3;
pub const DA9150_FAULT_PIN_MASK: u32 = (0x07 << 3);
pub const DA9150_FAULT_TYPE_SHIFT: u32 = 6;
pub const DA9150_FAULT_TYPE_MASK: u32 = (1u32 << 6);
pub const DA9150_NIRQ_PUPD_SHIFT: u32 = 7;
pub const DA9150_NIRQ_PUPD_MASK: u32 = (1u32 << 7);

/* DA9150_GPADC_MAN = 0x0F2 */
pub const DA9150_GPADC_EN_SHIFT: u32 = 0;
pub const DA9150_GPADC_EN_MASK: u32 = (1u32 << 0);
pub const DA9150_GPADC_MUX_SHIFT: u32 = 1;
pub const DA9150_GPADC_MUX_MASK: u32 = (0x1f << 1);

/* DA9150_GPADC_RES_A = 0x0F4 */
pub const DA9150_GPADC_RES_H_SHIFT: u32 = 0;
pub const DA9150_GPADC_RES_H_MASK: u32 = (0xff << 0);

/* DA9150_GPADC_RES_B = 0x0F5 */
pub const DA9150_GPADC_RUN_SHIFT: u32 = 0;
pub const DA9150_GPADC_RUN_MASK: u32 = (1u32 << 0);
pub const DA9150_GPADC_RES_L_SHIFT: u32 = 6;
pub const DA9150_GPADC_RES_L_MASK: u32 = (0x03 << 6);
pub const DA9150_GPADC_RES_L_BITS: u32 = 2;

/* DA9150_PAGE_CON_2 = 0x100 */
pub const DA9150_PAGE_SHIFT: u32 = 0;
pub const DA9150_PAGE_MASK: u32 = (0x3f << 0);
pub const DA9150_WRITE_MODE_SHIFT: u32 = 6;
pub const DA9150_WRITE_MODE_MASK: u32 = (1u32 << 6);
pub const DA9150_REVERT_SHIFT: u32 = 7;
pub const DA9150_REVERT_MASK: u32 = (1u32 << 7);

/* DA9150_OTP_CONT_SHARED = 0x101 */
pub const DA9150_PC_DONE_SHIFT: u32 = 3;
pub const DA9150_PC_DONE_MASK: u32 = (1u32 << 3);

/* DA9150_INTERFACE_SHARED = 0x105 */
pub const DA9150_IF_BASE_ADDR_SHIFT: u32 = 4;
pub const DA9150_IF_BASE_ADDR_MASK: u32 = (0x0f << 4);

/* DA9150_CONFIG_A_SHARED = 0x106 */
pub const DA9150_NIRQ_VDD_SHIFT: u32 = 1;
pub const DA9150_NIRQ_VDD_MASK: u32 = (1u32 << 1);
pub const DA9150_NIRQ_PIN_SHIFT: u32 = 2;
pub const DA9150_NIRQ_PIN_MASK: u32 = (1u32 << 2);
pub const DA9150_NIRQ_TYPE_SHIFT: u32 = 3;
pub const DA9150_NIRQ_TYPE_MASK: u32 = (1u32 << 3);
pub const DA9150_PM_IF_V_SHIFT: u32 = 4;
pub const DA9150_PM_IF_V_MASK: u32 = (1u32 << 4);
pub const DA9150_PM_IF_FMP_SHIFT: u32 = 5;
pub const DA9150_PM_IF_FMP_MASK: u32 = (1u32 << 5);
pub const DA9150_PM_IF_HSM_SHIFT: u32 = 6;
pub const DA9150_PM_IF_HSM_MASK: u32 = (1u32 << 6);

/* DA9150_CONFIG_D_SHARED = 0x109 */
pub const DA9150_NIRQ_MODE_SHIFT: u32 = 1;
pub const DA9150_NIRQ_MODE_MASK: u32 = (1u32 << 1);

/* DA9150_ADETVB_CFG_C = 0x150 */
pub const DA9150_TADP_RISE_SHIFT: u32 = 0;
pub const DA9150_TADP_RISE_MASK: u32 = (0xff << 0);

/* DA9150_ADETD_STAT = 0x151 */
pub const DA9150_DCD_STAT_SHIFT: u32 = 0;
pub const DA9150_DCD_STAT_MASK: u32 = (1u32 << 0);
pub const DA9150_PCD_STAT_SHIFT: u32 = 1;
pub const DA9150_PCD_STAT_MASK: u32 = (0x03 << 1);
pub const DA9150_SCD_STAT_SHIFT: u32 = 3;
pub const DA9150_SCD_STAT_MASK: u32 = (0x03 << 3);
pub const DA9150_DP_STAT_SHIFT: u32 = 5;
pub const DA9150_DP_STAT_MASK: u32 = (1u32 << 5);
pub const DA9150_DM_STAT_SHIFT: u32 = 6;
pub const DA9150_DM_STAT_MASK: u32 = (1u32 << 6);

/* DA9150_ADET_CMPSTAT = 0x152 */
pub const DA9150_DP_COMP_SHIFT: u32 = 1;
pub const DA9150_DP_COMP_MASK: u32 = (1u32 << 1);
pub const DA9150_DM_COMP_SHIFT: u32 = 2;
pub const DA9150_DM_COMP_MASK: u32 = (1u32 << 2);
pub const DA9150_ADP_SNS_COMP_SHIFT: u32 = 3;
pub const DA9150_ADP_SNS_COMP_MASK: u32 = (1u32 << 3);
pub const DA9150_ADP_PRB_COMP_SHIFT: u32 = 4;
pub const DA9150_ADP_PRB_COMP_MASK: u32 = (1u32 << 4);
pub const DA9150_ID_COMP_SHIFT: u32 = 5;
pub const DA9150_ID_COMP_MASK: u32 = (1u32 << 5);

/* DA9150_ADET_CTRL_A = 0x153 */
pub const DA9150_AID_DAT_SHIFT: u32 = 0;
pub const DA9150_AID_DAT_MASK: u32 = (1u32 << 0);
pub const DA9150_AID_ID_SHIFT: u32 = 1;
pub const DA9150_AID_ID_MASK: u32 = (1u32 << 1);
pub const DA9150_AID_TRIG_SHIFT: u32 = 2;
pub const DA9150_AID_TRIG_MASK: u32 = (1u32 << 2);

/* DA9150_ADETVB_CFG_B = 0x154 */
pub const DA9150_VB_MODE_SHIFT: u32 = 0;
pub const DA9150_VB_MODE_MASK: u32 = (0x03 << 0);
pub const DA9150_VB_MODE_VB_SESS: u32 = (1u32 << 0);

pub const DA9150_TADP_PRB_SHIFT: u32 = 2;
pub const DA9150_TADP_PRB_MASK: u32 = (1u32 << 2);
pub const DA9150_DAT_RPD_EXT_SHIFT: u32 = 5;
pub const DA9150_DAT_RPD_EXT_MASK: u32 = (1u32 << 5);
pub const DA9150_CONF_RPD_SHIFT: u32 = 6;
pub const DA9150_CONF_RPD_MASK: u32 = (1u32 << 6);
pub const DA9150_CONF_SRP_SHIFT: u32 = 7;
pub const DA9150_CONF_SRP_MASK: u32 = (1u32 << 7);

/* DA9150_ADETVB_CFG_A = 0x155 */
pub const DA9150_AID_MODE_SHIFT: u32 = 0;
pub const DA9150_AID_MODE_MASK: u32 = (0x03 << 0);
pub const DA9150_AID_EXT_POL_SHIFT: u32 = 2;
pub const DA9150_AID_EXT_POL_MASK: u32 = (1u32 << 2);

/* DA9150_ADETAC_CFG_A = 0x156 */
pub const DA9150_ISET_CDP_SHIFT: u32 = 0;
pub const DA9150_ISET_CDP_MASK: u32 = (0x1f << 0);
pub const DA9150_CONF_DBP_SHIFT: u32 = 5;
pub const DA9150_CONF_DBP_MASK: u32 = (1u32 << 5);

/* DA9150_ADDETAC_CFG_B = 0x157 */
pub const DA9150_ISET_DCHG_SHIFT: u32 = 0;
pub const DA9150_ISET_DCHG_MASK: u32 = (0x1f << 0);
pub const DA9150_CONF_GPIOA_SHIFT: u32 = 5;
pub const DA9150_CONF_GPIOA_MASK: u32 = (1u32 << 5);
pub const DA9150_CONF_GPIOB_SHIFT: u32 = 6;
pub const DA9150_CONF_GPIOB_MASK: u32 = (1u32 << 6);
pub const DA9150_AID_VB_SHIFT: u32 = 7;
pub const DA9150_AID_VB_MASK: u32 = (1u32 << 7);

/* DA9150_ADETAC_CFG_C = 0x158 */
pub const DA9150_ISET_DEF_SHIFT: u32 = 0;
pub const DA9150_ISET_DEF_MASK: u32 = (0x1f << 0);
pub const DA9150_CONF_MODE_SHIFT: u32 = 5;
pub const DA9150_CONF_MODE_MASK: u32 = (0x03 << 5);
pub const DA9150_AID_CR_DIS_SHIFT: u32 = 7;
pub const DA9150_AID_CR_DIS_MASK: u32 = (1u32 << 7);

/* DA9150_ADETAC_CFG_D = 0x159 */
pub const DA9150_ISET_UNIT_SHIFT: u32 = 0;
pub const DA9150_ISET_UNIT_MASK: u32 = (0x1f << 0);
pub const DA9150_AID_UNCLAMP_SHIFT: u32 = 5;
pub const DA9150_AID_UNCLAMP_MASK: u32 = (1u32 << 5);

/* DA9150_ADETVB_CFG_D = 0x15A */
pub const DA9150_ID_MODE_SHIFT: u32 = 0;
pub const DA9150_ID_MODE_MASK: u32 = (0x03 << 0);
pub const DA9150_DAT_MODE_SHIFT: u32 = 2;
pub const DA9150_DAT_MODE_MASK: u32 = (0x0f << 2);
pub const DA9150_DAT_SWP_SHIFT: u32 = 6;
pub const DA9150_DAT_SWP_MASK: u32 = (1u32 << 6);
pub const DA9150_DAT_CLAMP_EXT_SHIFT: u32 = 7;
pub const DA9150_DAT_CLAMP_EXT_MASK: u32 = (1u32 << 7);

/* DA9150_ADETID_CFG_A = 0x15B */
pub const DA9150_TID_POLL_SHIFT: u32 = 0;
pub const DA9150_TID_POLL_MASK: u32 = (0x07 << 0);
pub const DA9150_RID_CONV_SHIFT: u32 = 3;
pub const DA9150_RID_CONV_MASK: u32 = (1u32 << 3);

/* DA9150_ADET_RID_PT_CHG_H = 0x15C */
pub const DA9150_RID_PT_CHG_H_SHIFT: u32 = 0;
pub const DA9150_RID_PT_CHG_H_MASK: u32 = (0xff << 0);

/* DA9150_ADET_RID_PT_CHG_L = 0x15D */
pub const DA9150_RID_PT_CHG_L_SHIFT: u32 = 6;
pub const DA9150_RID_PT_CHG_L_MASK: u32 = (0x03 << 6);

/* DA9150_PPR_TCTR_B = 0x160 */
pub const DA9150_CHG_TCTR_VAL_SHIFT: u32 = 0;
pub const DA9150_CHG_TCTR_VAL_MASK: u32 = (0xff << 0);

/* DA9150_PPR_BKCTRL_A = 0x163 */
pub const DA9150_VBUS_MODE_SHIFT: u32 = 0;
pub const DA9150_VBUS_MODE_MASK: u32 = (0x03 << 0);
pub const DA9150_VBUS_MODE_CHG: u32 = (1u32 << 0);
pub const DA9150_VBUS_MODE_OTG: u32 = (0x02 << 0);
pub const DA9150_VBUS_LPM_SHIFT: u32 = 2;
pub const DA9150_VBUS_LPM_MASK: u32 = (0x03 << 2);
pub const DA9150_VBUS_SUSP_SHIFT: u32 = 4;
pub const DA9150_VBUS_SUSP_MASK: u32 = (1u32 << 4);
pub const DA9150_VBUS_PWM_SHIFT: u32 = 5;
pub const DA9150_VBUS_PWM_MASK: u32 = (1u32 << 5);
pub const DA9150_VBUS_ISO_SHIFT: u32 = 6;
pub const DA9150_VBUS_ISO_MASK: u32 = (1u32 << 6);
pub const DA9150_VBUS_LDO_SHIFT: u32 = 7;
pub const DA9150_VBUS_LDO_MASK: u32 = (1u32 << 7);

/* DA9150_PPR_BKCFG_A = 0x164 */
pub const DA9150_VBUS_ISET_SHIFT: u32 = 0;
pub const DA9150_VBUS_ISET_MASK: u32 = (0x1f << 0);
pub const DA9150_VBUS_IMAX_SHIFT: u32 = 5;
pub const DA9150_VBUS_IMAX_MASK: u32 = (1u32 << 5);
pub const DA9150_VBUS_IOTG_SHIFT: u32 = 6;
pub const DA9150_VBUS_IOTG_MASK: u32 = (0x03 << 6);

/* DA9150_PPR_BKCFG_B = 0x165 */
pub const DA9150_VBUS_DROP_SHIFT: u32 = 0;
pub const DA9150_VBUS_DROP_MASK: u32 = (0x0f << 0);
pub const DA9150_VBUS_FAULT_DIS_SHIFT: u32 = 6;
pub const DA9150_VBUS_FAULT_DIS_MASK: u32 = (1u32 << 6);
pub const DA9150_OTG_FAULT_DIS_SHIFT: u32 = 7;
pub const DA9150_OTG_FAULT_DIS_MASK: u32 = (1u32 << 7);

/* DA9150_PPR_CHGCTRL_A = 0x166 */
pub const DA9150_CHG_EN_SHIFT: u32 = 0;
pub const DA9150_CHG_EN_MASK: u32 = (1u32 << 0);

/* DA9150_PPR_CHGCTRL_B = 0x167 */
pub const DA9150_CHG_VBAT_SHIFT: u32 = 0;
pub const DA9150_CHG_VBAT_MASK: u32 = (0x1f << 0);
pub const DA9150_CHG_VDROP_SHIFT: u32 = 6;
pub const DA9150_CHG_VDROP_MASK: u32 = (0x03 << 6);

/* DA9150_PPR_CHGCTRL_C = 0x168 */
pub const DA9150_CHG_VFAULT_SHIFT: u32 = 0;
pub const DA9150_CHG_VFAULT_MASK: u32 = (0x0f << 0);
pub const DA9150_CHG_IPRE_SHIFT: u32 = 4;
pub const DA9150_CHG_IPRE_MASK: u32 = (0x03 << 4);

/* DA9150_PPR_TCTR_A = 0x169 */
pub const DA9150_CHG_TCTR_SHIFT: u32 = 0;
pub const DA9150_CHG_TCTR_MASK: u32 = (0x07 << 0);
pub const DA9150_CHG_TCTR_MODE_SHIFT: u32 = 4;
pub const DA9150_CHG_TCTR_MODE_MASK: u32 = (1u32 << 4);

/* DA9150_PPR_CHGCTRL_D = 0x16A */
pub const DA9150_CHG_IBAT_SHIFT: u32 = 0;
pub const DA9150_CHG_IBAT_MASK: u32 = (0xff << 0);

/* DA9150_PPR_CHGCTRL_E = 0x16B */
pub const DA9150_CHG_IEND_SHIFT: u32 = 0;
pub const DA9150_CHG_IEND_MASK: u32 = (0xff << 0);

/* DA9150_PPR_CHGCTRL_F = 0x16C */
pub const DA9150_CHG_VCOLD_SHIFT: u32 = 0;
pub const DA9150_CHG_VCOLD_MASK: u32 = (0x1f << 0);
pub const DA9150_TBAT_TQA_EN_SHIFT: u32 = 6;
pub const DA9150_TBAT_TQA_EN_MASK: u32 = (1u32 << 6);
pub const DA9150_TBAT_TDP_EN_SHIFT: u32 = 7;
pub const DA9150_TBAT_TDP_EN_MASK: u32 = (1u32 << 7);

/* DA9150_PPR_CHGCTRL_G = 0x16D */
pub const DA9150_CHG_VWARM_SHIFT: u32 = 0;
pub const DA9150_CHG_VWARM_MASK: u32 = (0x1f << 0);

/* DA9150_PPR_CHGCTRL_H = 0x16E */
pub const DA9150_CHG_VHOT_SHIFT: u32 = 0;
pub const DA9150_CHG_VHOT_MASK: u32 = (0x1f << 0);

/* DA9150_PPR_CHGCTRL_I = 0x16F */
pub const DA9150_CHG_ICOLD_SHIFT: u32 = 0;
pub const DA9150_CHG_ICOLD_MASK: u32 = (0xff << 0);

/* DA9150_PPR_CHGCTRL_J = 0x170 */
pub const DA9150_CHG_IWARM_SHIFT: u32 = 0;
pub const DA9150_CHG_IWARM_MASK: u32 = (0xff << 0);

/* DA9150_PPR_CHGCTRL_K = 0x171 */
pub const DA9150_CHG_IHOT_SHIFT: u32 = 0;
pub const DA9150_CHG_IHOT_MASK: u32 = (0xff << 0);

/* DA9150_PPR_CHGCTRL_L = 0x172 */
pub const DA9150_CHG_IBAT_TRED_SHIFT: u32 = 0;
pub const DA9150_CHG_IBAT_TRED_MASK: u32 = (0xff << 0);

/* DA9150_PPR_CHGCTRL_M = 0x173 */
pub const DA9150_CHG_VFLOAT_SHIFT: u32 = 0;
pub const DA9150_CHG_VFLOAT_MASK: u32 = (0x0f << 0);
pub const DA9150_CHG_LPM_SHIFT: u32 = 5;
pub const DA9150_CHG_LPM_MASK: u32 = (1u32 << 5);
pub const DA9150_CHG_NBLO_SHIFT: u32 = 6;
pub const DA9150_CHG_NBLO_MASK: u32 = (1u32 << 6);
pub const DA9150_EBS_EN_SHIFT: u32 = 7;
pub const DA9150_EBS_EN_MASK: u32 = (1u32 << 7);

/* DA9150_PPR_THYST_A = 0x174 */
pub const DA9150_TBAT_T1_SHIFT: u32 = 0;
pub const DA9150_TBAT_T1_MASK: u32 = (0xff << 0);

/* DA9150_PPR_THYST_B = 0x175 */
pub const DA9150_TBAT_T2_SHIFT: u32 = 0;
pub const DA9150_TBAT_T2_MASK: u32 = (0xff << 0);

/* DA9150_PPR_THYST_C = 0x176 */
pub const DA9150_TBAT_T3_SHIFT: u32 = 0;
pub const DA9150_TBAT_T3_MASK: u32 = (0xff << 0);

/* DA9150_PPR_THYST_D = 0x177 */
pub const DA9150_TBAT_T4_SHIFT: u32 = 0;
pub const DA9150_TBAT_T4_MASK: u32 = (0xff << 0);

/* DA9150_PPR_THYST_E = 0x178 */
pub const DA9150_TBAT_T5_SHIFT: u32 = 0;
pub const DA9150_TBAT_T5_MASK: u32 = (0xff << 0);

/* DA9150_PPR_THYST_F = 0x179 */
pub const DA9150_TBAT_H1_SHIFT: u32 = 0;
pub const DA9150_TBAT_H1_MASK: u32 = (0xff << 0);

/* DA9150_PPR_THYST_G = 0x17A */
pub const DA9150_TBAT_H5_SHIFT: u32 = 0;
pub const DA9150_TBAT_H5_MASK: u32 = (0xff << 0);

/* DA9150_PAGE_CON_3 = 0x180 */
pub const DA9150_PAGE_SHIFT: u32 = 0;
pub const DA9150_PAGE_MASK: u32 = (0x3f << 0);
pub const DA9150_WRITE_MODE_SHIFT: u32 = 6;
pub const DA9150_WRITE_MODE_MASK: u32 = (1u32 << 6);
pub const DA9150_REVERT_SHIFT: u32 = 7;
pub const DA9150_REVERT_MASK: u32 = (1u32 << 7);

/* DA9150_PAGE_CON_4 = 0x200 */
pub const DA9150_PAGE_SHIFT: u32 = 0;
pub const DA9150_PAGE_MASK: u32 = (0x3f << 0);
pub const DA9150_WRITE_MODE_SHIFT: u32 = 6;
pub const DA9150_WRITE_MODE_MASK: u32 = (1u32 << 6);
pub const DA9150_REVERT_SHIFT: u32 = 7;
pub const DA9150_REVERT_MASK: u32 = (1u32 << 7);

/* DA9150_PAGE_CON_5 = 0x280 */
pub const DA9150_PAGE_SHIFT: u32 = 0;
pub const DA9150_PAGE_MASK: u32 = (0x3f << 0);
pub const DA9150_WRITE_MODE_SHIFT: u32 = 6;
pub const DA9150_WRITE_MODE_MASK: u32 = (1u32 << 6);
pub const DA9150_REVERT_SHIFT: u32 = 7;
pub const DA9150_REVERT_MASK: u32 = (1u32 << 7);

/* DA9150_PAGE_CON_6 = 0x300 */
pub const DA9150_PAGE_SHIFT: u32 = 0;
pub const DA9150_PAGE_MASK: u32 = (0x3f << 0);
pub const DA9150_WRITE_MODE_SHIFT: u32 = 6;
pub const DA9150_WRITE_MODE_MASK: u32 = (1u32 << 6);
pub const DA9150_REVERT_SHIFT: u32 = 7;
pub const DA9150_REVERT_MASK: u32 = (1u32 << 7);

/* DA9150_COREBTLD_STAT_A = 0x302 */
pub const DA9150_BOOTLD_STAT_SHIFT: u32 = 0;
pub const DA9150_BOOTLD_STAT_MASK: u32 = (0x03 << 0);
pub const DA9150_CORE_LOCKUP_SHIFT: u32 = 2;
pub const DA9150_CORE_LOCKUP_MASK: u32 = (1u32 << 2);

/* DA9150_COREBTLD_CTRL_A = 0x303 */
pub const DA9150_CORE_RESET_SHIFT: u32 = 0;
pub const DA9150_CORE_RESET_MASK: u32 = (1u32 << 0);
pub const DA9150_CORE_STOP_SHIFT: u32 = 1;
pub const DA9150_CORE_STOP_MASK: u32 = (1u32 << 1);

/* DA9150_CORE_CONFIG_A = 0x304 */
pub const DA9150_CORE_MEMMUX_SHIFT: u32 = 0;
pub const DA9150_CORE_MEMMUX_MASK: u32 = (0x03 << 0);
pub const DA9150_WDT_AUTO_START_SHIFT: u32 = 2;
pub const DA9150_WDT_AUTO_START_MASK: u32 = (1u32 << 2);
pub const DA9150_WDT_AUTO_LOCK_SHIFT: u32 = 3;
pub const DA9150_WDT_AUTO_LOCK_MASK: u32 = (1u32 << 3);
pub const DA9150_WDT_HLT_NO_CLK_SHIFT: u32 = 4;
pub const DA9150_WDT_HLT_NO_CLK_MASK: u32 = (1u32 << 4);

/* DA9150_CORE_CONFIG_C = 0x305 */
pub const DA9150_CORE_SW_SIZE_SHIFT: u32 = 0;
pub const DA9150_CORE_SW_SIZE_MASK: u32 = (0xff << 0);

/* DA9150_CORE_CONFIG_B = 0x306 */
pub const DA9150_BOOTLD_EN_SHIFT: u32 = 0;
pub const DA9150_BOOTLD_EN_MASK: u32 = (1u32 << 0);
pub const DA9150_CORE_EN_SHIFT: u32 = 2;
pub const DA9150_CORE_EN_MASK: u32 = (1u32 << 2);
pub const DA9150_CORE_SW_SRC_SHIFT: u32 = 3;
pub const DA9150_CORE_SW_SRC_MASK: u32 = (0x07 << 3);
pub const DA9150_DEEP_SLEEP_EN_SHIFT: u32 = 7;
pub const DA9150_DEEP_SLEEP_EN_MASK: u32 = (1u32 << 7);

/* DA9150_CORE_CFG_DATA_A = 0x307 */
pub const DA9150_CORE_CFG_DT_A_SHIFT: u32 = 0;
pub const DA9150_CORE_CFG_DT_A_MASK: u32 = (0xff << 0);

/* DA9150_CORE_CFG_DATA_B = 0x308 */
pub const DA9150_CORE_CFG_DT_B_SHIFT: u32 = 0;
pub const DA9150_CORE_CFG_DT_B_MASK: u32 = (0xff << 0);

/* DA9150_CORE_CMD_A = 0x309 */
pub const DA9150_CORE_CMD_SHIFT: u32 = 0;
pub const DA9150_CORE_CMD_MASK: u32 = (0xff << 0);

/* DA9150_CORE_DATA_A = 0x30A */
pub const DA9150_CORE_DATA_0_SHIFT: u32 = 0;
pub const DA9150_CORE_DATA_0_MASK: u32 = (0xff << 0);

/* DA9150_CORE_DATA_B = 0x30B */
pub const DA9150_CORE_DATA_1_SHIFT: u32 = 0;
pub const DA9150_CORE_DATA_1_MASK: u32 = (0xff << 0);

/* DA9150_CORE_DATA_C = 0x30C */
pub const DA9150_CORE_DATA_2_SHIFT: u32 = 0;
pub const DA9150_CORE_DATA_2_MASK: u32 = (0xff << 0);

/* DA9150_CORE_DATA_D = 0x30D */
pub const DA9150_CORE_DATA_3_SHIFT: u32 = 0;
pub const DA9150_CORE_DATA_3_MASK: u32 = (0xff << 0);

/* DA9150_CORE2WIRE_STAT_A = 0x310 */
pub const DA9150_FW_FWDL_ERR_SHIFT: u32 = 7;
pub const DA9150_FW_FWDL_ERR_MASK: u32 = (1u32 << 7);

/* DA9150_CORE2WIRE_CTRL_A = 0x311 */
pub const DA9150_FW_FWDL_EN_SHIFT: u32 = 0;
pub const DA9150_FW_FWDL_EN_MASK: u32 = (1u32 << 0);
pub const DA9150_FG_QIF_EN_SHIFT: u32 = 1;
pub const DA9150_FG_QIF_EN_MASK: u32 = (1u32 << 1);
pub const DA9150_CORE_BASE_ADDR_SHIFT: u32 = 4;
pub const DA9150_CORE_BASE_ADDR_MASK: u32 = (0x0f << 4);

/* DA9150_FW_CTRL_A = 0x312 */
pub const DA9150_FW_SEAL_SHIFT: u32 = 0;
pub const DA9150_FW_SEAL_MASK: u32 = (0xff << 0);

/* DA9150_FW_CTRL_C = 0x313 */
pub const DA9150_FW_FWDL_CRC_SHIFT: u32 = 0;
pub const DA9150_FW_FWDL_CRC_MASK: u32 = (0xff << 0);

/* DA9150_FW_CTRL_D = 0x314 */
pub const DA9150_FW_FWDL_BASE_SHIFT: u32 = 0;
pub const DA9150_FW_FWDL_BASE_MASK: u32 = (0x0f << 0);

/* DA9150_FG_CTRL_A = 0x315 */
pub const DA9150_FG_QIF_CODE_SHIFT: u32 = 0;
pub const DA9150_FG_QIF_CODE_MASK: u32 = (0xff << 0);

/* DA9150_FG_CTRL_B = 0x316 */
pub const DA9150_FG_QIF_VALUE_SHIFT: u32 = 0;
pub const DA9150_FG_QIF_VALUE_MASK: u32 = (0xff << 0);

/* DA9150_FW_CTRL_E = 0x317 */
pub const DA9150_FW_FWDL_SEG_SHIFT: u32 = 0;
pub const DA9150_FW_FWDL_SEG_MASK: u32 = (0xff << 0);

/* DA9150_FW_CTRL_B = 0x318 */
pub const DA9150_FW_FWDL_VALUE_SHIFT: u32 = 0;
pub const DA9150_FW_FWDL_VALUE_MASK: u32 = (0xff << 0);

/* DA9150_GPADC_CMAN = 0x320 */
pub const DA9150_GPADC_CEN_SHIFT: u32 = 0;
pub const DA9150_GPADC_CEN_MASK: u32 = (1u32 << 0);
pub const DA9150_GPADC_CMUX_SHIFT: u32 = 1;
pub const DA9150_GPADC_CMUX_MASK: u32 = (0x1f << 1);

/* DA9150_GPADC_CRES_A = 0x322 */
pub const DA9150_GPADC_CRES_H_SHIFT: u32 = 0;
pub const DA9150_GPADC_CRES_H_MASK: u32 = (0xff << 0);

/* DA9150_GPADC_CRES_B = 0x323 */
pub const DA9150_GPADC_CRUN_SHIFT: u32 = 0;
pub const DA9150_GPADC_CRUN_MASK: u32 = (1u32 << 0);
pub const DA9150_GPADC_CRES_L_SHIFT: u32 = 6;
pub const DA9150_GPADC_CRES_L_MASK: u32 = (0x03 << 6);

/* DA9150_CC_CFG_A = 0x328 */
pub const DA9150_CC_EN_SHIFT: u32 = 0;
pub const DA9150_CC_EN_MASK: u32 = (1u32 << 0);
pub const DA9150_CC_TIMEBASE_SHIFT: u32 = 1;
pub const DA9150_CC_TIMEBASE_MASK: u32 = (0x03 << 1);
pub const DA9150_CC_CFG_SHIFT: u32 = 5;
pub const DA9150_CC_CFG_MASK: u32 = (0x03 << 5);
pub const DA9150_CC_ENDLESS_MODE_SHIFT: u32 = 7;
pub const DA9150_CC_ENDLESS_MODE_MASK: u32 = (1u32 << 7);

/* DA9150_CC_CFG_B = 0x329 */
pub const DA9150_CC_OPT_SHIFT: u32 = 0;
pub const DA9150_CC_OPT_MASK: u32 = (0x03 << 0);
pub const DA9150_CC_PREAMP_SHIFT: u32 = 2;
pub const DA9150_CC_PREAMP_MASK: u32 = (0x03 << 2);

/* DA9150_CC_ICHG_RES_A = 0x32A */
pub const DA9150_CC_ICHG_RES_H_SHIFT: u32 = 0;
pub const DA9150_CC_ICHG_RES_H_MASK: u32 = (0xff << 0);

/* DA9150_CC_ICHG_RES_B = 0x32B */
pub const DA9150_CC_ICHG_RES_L_SHIFT: u32 = 3;
pub const DA9150_CC_ICHG_RES_L_MASK: u32 = (0x1f << 3);

/* DA9150_CC_IAVG_RES_A = 0x32C */
pub const DA9150_CC_IAVG_RES_H_SHIFT: u32 = 0;
pub const DA9150_CC_IAVG_RES_H_MASK: u32 = (0xff << 0);

/* DA9150_CC_IAVG_RES_B = 0x32D */
pub const DA9150_CC_IAVG_RES_L_SHIFT: u32 = 0;
pub const DA9150_CC_IAVG_RES_L_MASK: u32 = (0xff << 0);

/* DA9150_TAUX_CTRL_A = 0x330 */
pub const DA9150_TAUX_EN_SHIFT: u32 = 0;
pub const DA9150_TAUX_EN_MASK: u32 = (1u32 << 0);
pub const DA9150_TAUX_MOD_SHIFT: u32 = 1;
pub const DA9150_TAUX_MOD_MASK: u32 = (1u32 << 1);
pub const DA9150_TAUX_UPDATE_SHIFT: u32 = 2;
pub const DA9150_TAUX_UPDATE_MASK: u32 = (1u32 << 2);

/* DA9150_TAUX_RELOAD_H = 0x332 */
pub const DA9150_TAUX_RLD_H_SHIFT: u32 = 0;
pub const DA9150_TAUX_RLD_H_MASK: u32 = (0xff << 0);

/* DA9150_TAUX_RELOAD_L = 0x333 */
pub const DA9150_TAUX_RLD_L_SHIFT: u32 = 3;
pub const DA9150_TAUX_RLD_L_MASK: u32 = (0x1f << 3);

/* DA9150_TAUX_VALUE_H = 0x334 */
pub const DA9150_TAUX_VAL_H_SHIFT: u32 = 0;
pub const DA9150_TAUX_VAL_H_MASK: u32 = (0xff << 0);

/* DA9150_TAUX_VALUE_L = 0x335 */
pub const DA9150_TAUX_VAL_L_SHIFT: u32 = 3;
pub const DA9150_TAUX_VAL_L_MASK: u32 = (0x1f << 3);

/* DA9150_AUX_DATA_0 = 0x338 */
pub const DA9150_AUX_DAT_0_SHIFT: u32 = 0;
pub const DA9150_AUX_DAT_0_MASK: u32 = (0xff << 0);

/* DA9150_AUX_DATA_1 = 0x339 */
pub const DA9150_AUX_DAT_1_SHIFT: u32 = 0;
pub const DA9150_AUX_DAT_1_MASK: u32 = (0xff << 0);

/* DA9150_AUX_DATA_2 = 0x33A */
pub const DA9150_AUX_DAT_2_SHIFT: u32 = 0;
pub const DA9150_AUX_DAT_2_MASK: u32 = (0xff << 0);

/* DA9150_AUX_DATA_3 = 0x33B */
pub const DA9150_AUX_DAT_3_SHIFT: u32 = 0;
pub const DA9150_AUX_DAT_3_MASK: u32 = (0xff << 0);

/* DA9150_BIF_CTRL = 0x340 */
pub const DA9150_BIF_ISRC_EN_SHIFT: u32 = 0;
pub const DA9150_BIF_ISRC_EN_MASK: u32 = (1u32 << 0);

/* DA9150_TBAT_CTRL_A = 0x342 */
pub const DA9150_TBAT_EN_SHIFT: u32 = 0;
pub const DA9150_TBAT_EN_MASK: u32 = (1u32 << 0);
pub const DA9150_TBAT_SW1_SHIFT: u32 = 1;
pub const DA9150_TBAT_SW1_MASK: u32 = (1u32 << 1);
pub const DA9150_TBAT_SW2_SHIFT: u32 = 2;
pub const DA9150_TBAT_SW2_MASK: u32 = (1u32 << 2);

/* DA9150_TBAT_CTRL_B = 0x343 */
pub const DA9150_TBAT_SW_FRC_SHIFT: u32 = 0;
pub const DA9150_TBAT_SW_FRC_MASK: u32 = (1u32 << 0);
pub const DA9150_TBAT_STAT_SW1_SHIFT: u32 = 1;
pub const DA9150_TBAT_STAT_SW1_MASK: u32 = (1u32 << 1);
pub const DA9150_TBAT_STAT_SW2_SHIFT: u32 = 2;
pub const DA9150_TBAT_STAT_SW2_MASK: u32 = (1u32 << 2);
pub const DA9150_TBAT_HIGH_CURR_SHIFT: u32 = 3;
pub const DA9150_TBAT_HIGH_CURR_MASK: u32 = (1u32 << 3);

/* DA9150_TBAT_RES_A = 0x344 */
pub const DA9150_TBAT_RES_H_SHIFT: u32 = 0;
pub const DA9150_TBAT_RES_H_MASK: u32 = (0xff << 0);

/* DA9150_TBAT_RES_B = 0x345 */
pub const DA9150_TBAT_RES_DIS_SHIFT: u32 = 0;
pub const DA9150_TBAT_RES_DIS_MASK: u32 = (1u32 << 0);
pub const DA9150_TBAT_RES_L_SHIFT: u32 = 6;
pub const DA9150_TBAT_RES_L_MASK: u32 = (0x03 << 6);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
