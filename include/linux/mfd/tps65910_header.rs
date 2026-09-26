/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * tps65910.h  --  TI TPS6591x
 *
 * Copyright 2010-2011 Texas Instruments Inc.
 *
 * Author: Graeme Gregory <gg@slimlogic.co.uk>
 * Author: Jorge Eduardo Candelaria <jedu@slimlogic.co.uk>
 * Author: Arnaud Deconinck <a-deconinck@ti.com>
 */



/* TPS chip id list */
pub const TPS65910: u32 = 0;
pub const TPS65911: u32 = 1;

/* TPS regulator type list */
pub const REGULATOR_LDO: u32 = 0;
pub const REGULATOR_DCDC: u32 = 1;

/*
 * List of registers for component TPS65910
 *
 */

pub const TPS65910_SECONDS: u32 = 0x0;
pub const TPS65910_MINUTES: u32 = 0x1;
pub const TPS65910_HOURS: u32 = 0x2;
pub const TPS65910_DAYS: u32 = 0x3;
pub const TPS65910_MONTHS: u32 = 0x4;
pub const TPS65910_YEARS: u32 = 0x5;
pub const TPS65910_WEEKS: u32 = 0x6;
pub const TPS65910_ALARM_SECONDS: u32 = 0x8;
pub const TPS65910_ALARM_MINUTES: u32 = 0x9;
pub const TPS65910_ALARM_HOURS: u32 = 0xA;
pub const TPS65910_ALARM_DAYS: u32 = 0xB;
pub const TPS65910_ALARM_MONTHS: u32 = 0xC;
pub const TPS65910_ALARM_YEARS: u32 = 0xD;
pub const TPS65910_RTC_CTRL: u32 = 0x10;
pub const TPS65910_RTC_STATUS: u32 = 0x11;
pub const TPS65910_RTC_INTERRUPTS: u32 = 0x12;
pub const TPS65910_RTC_COMP_LSB: u32 = 0x13;
pub const TPS65910_RTC_COMP_MSB: u32 = 0x14;
pub const TPS65910_RTC_RES_PROG: u32 = 0x15;
pub const TPS65910_RTC_RESET_STATUS: u32 = 0x16;
pub const TPS65910_BCK1: u32 = 0x17;
pub const TPS65910_BCK2: u32 = 0x18;
pub const TPS65910_BCK3: u32 = 0x19;
pub const TPS65910_BCK4: u32 = 0x1A;
pub const TPS65910_BCK5: u32 = 0x1B;
pub const TPS65910_PUADEN: u32 = 0x1C;
pub const TPS65910_REF: u32 = 0x1D;
pub const TPS65910_VRTC: u32 = 0x1E;
pub const TPS65910_VIO: u32 = 0x20;
pub const TPS65910_VDD1: u32 = 0x21;
pub const TPS65910_VDD1_OP: u32 = 0x22;
pub const TPS65910_VDD1_SR: u32 = 0x23;
pub const TPS65910_VDD2: u32 = 0x24;
pub const TPS65910_VDD2_OP: u32 = 0x25;
pub const TPS65910_VDD2_SR: u32 = 0x26;
pub const TPS65910_VDD3: u32 = 0x27;
pub const TPS65910_VDIG1: u32 = 0x30;
pub const TPS65910_VDIG2: u32 = 0x31;
pub const TPS65910_VAUX1: u32 = 0x32;
pub const TPS65910_VAUX2: u32 = 0x33;
pub const TPS65910_VAUX33: u32 = 0x34;
pub const TPS65910_VMMC: u32 = 0x35;
pub const TPS65910_VPLL: u32 = 0x36;
pub const TPS65910_VDAC: u32 = 0x37;
pub const TPS65910_THERM: u32 = 0x38;
pub const TPS65910_BBCH: u32 = 0x39;
pub const TPS65910_DCDCCTRL: u32 = 0x3E;
pub const TPS65910_DEVCTRL: u32 = 0x3F;
pub const TPS65910_DEVCTRL2: u32 = 0x40;
pub const TPS65910_SLEEP_KEEP_LDO_ON: u32 = 0x41;
pub const TPS65910_SLEEP_KEEP_RES_ON: u32 = 0x42;
pub const TPS65910_SLEEP_SET_LDO_OFF: u32 = 0x43;
pub const TPS65910_SLEEP_SET_RES_OFF: u32 = 0x44;
pub const TPS65910_EN1_LDO_ASS: u32 = 0x45;
pub const TPS65910_EN1_SMPS_ASS: u32 = 0x46;
pub const TPS65910_EN2_LDO_ASS: u32 = 0x47;
pub const TPS65910_EN2_SMPS_ASS: u32 = 0x48;
pub const TPS65910_EN3_LDO_ASS: u32 = 0x49;
pub const TPS65910_SPARE: u32 = 0x4A;
pub const TPS65910_INT_STS: u32 = 0x50;
pub const TPS65910_INT_MSK: u32 = 0x51;
pub const TPS65910_INT_STS2: u32 = 0x52;
pub const TPS65910_INT_MSK2: u32 = 0x53;
pub const TPS65910_INT_STS3: u32 = 0x54;
pub const TPS65910_INT_MSK3: u32 = 0x55;
pub const TPS65910_GPIO0: u32 = 0x60;
pub const TPS65910_GPIO1: u32 = 0x61;
pub const TPS65910_GPIO2: u32 = 0x62;
pub const TPS65910_GPIO3: u32 = 0x63;
pub const TPS65910_GPIO4: u32 = 0x64;
pub const TPS65910_GPIO5: u32 = 0x65;
pub const TPS65910_GPIO6: u32 = 0x66;
pub const TPS65910_GPIO7: u32 = 0x67;
pub const TPS65910_GPIO8: u32 = 0x68;
pub const TPS65910_JTAGVERNUM: u32 = 0x80;
pub const TPS65910_MAX_REGISTER: u32 = 0x80;

/*
 * List of registers specific to TPS65911
 */
pub const TPS65911_VDDCTRL: u32 = 0x27;
pub const TPS65911_VDDCTRL_OP: u32 = 0x28;
pub const TPS65911_VDDCTRL_SR: u32 = 0x29;
pub const TPS65911_LDO1: u32 = 0x30;
pub const TPS65911_LDO2: u32 = 0x31;
pub const TPS65911_LDO5: u32 = 0x32;
pub const TPS65911_LDO8: u32 = 0x33;
pub const TPS65911_LDO7: u32 = 0x34;
pub const TPS65911_LDO6: u32 = 0x35;
pub const TPS65911_LDO4: u32 = 0x36;
pub const TPS65911_LDO3: u32 = 0x37;
pub const TPS65911_VMBCH: u32 = 0x6A;
pub const TPS65911_VMBCH2: u32 = 0x6B;

/*
 * List of register bitfields for component TPS65910
 *
 */

/* RTC_CTRL_REG bitfields */
pub const TPS65910_RTC_CTRL_STOP_RTC: u32 = 0x01;  /*0=stop, 1=run */
pub const TPS65910_RTC_CTRL_AUTO_COMP: u32 = 0x04;
pub const TPS65910_RTC_CTRL_GET_TIME: u32 = 0x40;

/* RTC_STATUS_REG bitfields */
pub const TPS65910_RTC_STATUS_ALARM: u32 = 0x40;

/* RTC_INTERRUPTS_REG bitfields */
pub const TPS65910_RTC_INTERRUPTS_EVERY: u32 = 0x03;
pub const TPS65910_RTC_INTERRUPTS_IT_ALARM: u32 = 0x08;

/*Register BCK1  (0x80) register.RegisterDescription */
pub const BCK1_BCKUP_MASK: u32 = 0xFF;
pub const BCK1_BCKUP_SHIFT: u32 = 0;

/*Register BCK2  (0x80) register.RegisterDescription */
pub const BCK2_BCKUP_MASK: u32 = 0xFF;
pub const BCK2_BCKUP_SHIFT: u32 = 0;

/*Register BCK3  (0x80) register.RegisterDescription */
pub const BCK3_BCKUP_MASK: u32 = 0xFF;
pub const BCK3_BCKUP_SHIFT: u32 = 0;

/*Register BCK4  (0x80) register.RegisterDescription */
pub const BCK4_BCKUP_MASK: u32 = 0xFF;
pub const BCK4_BCKUP_SHIFT: u32 = 0;

/*Register BCK5  (0x80) register.RegisterDescription */
pub const BCK5_BCKUP_MASK: u32 = 0xFF;
pub const BCK5_BCKUP_SHIFT: u32 = 0;

/*Register PUADEN  (0x80) register.RegisterDescription */
pub const PUADEN_EN3P_MASK: u32 = 0x80;
pub const PUADEN_EN3P_SHIFT: u32 = 7;
pub const PUADEN_I2CCTLP_MASK: u32 = 0x40;
pub const PUADEN_I2CCTLP_SHIFT: u32 = 6;
pub const PUADEN_I2CSRP_MASK: u32 = 0x20;
pub const PUADEN_I2CSRP_SHIFT: u32 = 5;
pub const PUADEN_PWRONP_MASK: u32 = 0x10;
pub const PUADEN_PWRONP_SHIFT: u32 = 4;
pub const PUADEN_SLEEPP_MASK: u32 = 0x08;
pub const PUADEN_SLEEPP_SHIFT: u32 = 3;
pub const PUADEN_PWRHOLDP_MASK: u32 = 0x04;
pub const PUADEN_PWRHOLDP_SHIFT: u32 = 2;
pub const PUADEN_BOOT1P_MASK: u32 = 0x02;
pub const PUADEN_BOOT1P_SHIFT: u32 = 1;
pub const PUADEN_BOOT0P_MASK: u32 = 0x01;
pub const PUADEN_BOOT0P_SHIFT: u32 = 0;

/*Register REF	(0x80) register.RegisterDescription */
pub const REF_VMBCH_SEL_MASK: u32 = 0x0C;
pub const REF_VMBCH_SEL_SHIFT: u32 = 2;
pub const REF_ST_MASK: u32 = 0x03;
pub const REF_ST_SHIFT: u32 = 0;

/*Register VRTC  (0x80) register.RegisterDescription */
pub const VRTC_VRTC_OFFMASK_MASK: u32 = 0x08;
pub const VRTC_VRTC_OFFMASK_SHIFT: u32 = 3;
pub const VRTC_ST_MASK: u32 = 0x03;
pub const VRTC_ST_SHIFT: u32 = 0;

/*Register VIO	(0x80) register.RegisterDescription */
pub const VIO_ILMAX_MASK: u32 = 0xC0;
pub const VIO_ILMAX_SHIFT: u32 = 6;
pub const VIO_SEL_MASK: u32 = 0x0C;
pub const VIO_SEL_SHIFT: u32 = 2;
pub const VIO_ST_MASK: u32 = 0x03;
pub const VIO_ST_SHIFT: u32 = 0;

/*Register VDD1  (0x80) register.RegisterDescription */
pub const VDD1_VGAIN_SEL_MASK: u32 = 0xC0;
pub const VDD1_VGAIN_SEL_SHIFT: u32 = 6;
pub const VDD1_ILMAX_MASK: u32 = 0x20;
pub const VDD1_ILMAX_SHIFT: u32 = 5;
pub const VDD1_TSTEP_MASK: u32 = 0x1C;
pub const VDD1_TSTEP_SHIFT: u32 = 2;
pub const VDD1_ST_MASK: u32 = 0x03;
pub const VDD1_ST_SHIFT: u32 = 0;

/*Register VDD1_OP  (0x80) register.RegisterDescription */
pub const VDD1_OP_CMD_MASK: u32 = 0x80;
pub const VDD1_OP_CMD_SHIFT: u32 = 7;
pub const VDD1_OP_SEL_MASK: u32 = 0x7F;
pub const VDD1_OP_SEL_SHIFT: u32 = 0;

/*Register VDD1_SR  (0x80) register.RegisterDescription */
pub const VDD1_SR_SEL_MASK: u32 = 0x7F;
pub const VDD1_SR_SEL_SHIFT: u32 = 0;

/*Register VDD2  (0x80) register.RegisterDescription */
pub const VDD2_VGAIN_SEL_MASK: u32 = 0xC0;
pub const VDD2_VGAIN_SEL_SHIFT: u32 = 6;
pub const VDD2_ILMAX_MASK: u32 = 0x20;
pub const VDD2_ILMAX_SHIFT: u32 = 5;
pub const VDD2_TSTEP_MASK: u32 = 0x1C;
pub const VDD2_TSTEP_SHIFT: u32 = 2;
pub const VDD2_ST_MASK: u32 = 0x03;
pub const VDD2_ST_SHIFT: u32 = 0;

/*Register VDD2_OP  (0x80) register.RegisterDescription */
pub const VDD2_OP_CMD_MASK: u32 = 0x80;
pub const VDD2_OP_CMD_SHIFT: u32 = 7;
pub const VDD2_OP_SEL_MASK: u32 = 0x7F;
pub const VDD2_OP_SEL_SHIFT: u32 = 0;

/*Register VDD2_SR  (0x80) register.RegisterDescription */
pub const VDD2_SR_SEL_MASK: u32 = 0x7F;
pub const VDD2_SR_SEL_SHIFT: u32 = 0;

/*Registers VDD1, VDD2 voltage values definitions */
pub const VDD1_2_NUM_VOLT_FINE: u32 = 73;
pub const VDD1_2_NUM_VOLT_COARSE: u32 = 3;
pub const VDD1_2_MIN_VOLT: u32 = 6000;
pub const VDD1_2_OFFSET: u32 = 125;

/*Register VDD3  (0x80) register.RegisterDescription */
pub const VDD3_CKINEN_MASK: u32 = 0x04;
pub const VDD3_CKINEN_SHIFT: u32 = 2;
pub const VDD3_ST_MASK: u32 = 0x03;
pub const VDD3_ST_SHIFT: u32 = 0;
pub const VDDCTRL_MIN_VOLT: u32 = 6000;
pub const VDDCTRL_OFFSET: u32 = 125;

/*Registers VDIG (0x80) to VDAC register.RegisterDescription */
pub const LDO_SEL_MASK: u32 = 0x0C;
pub const LDO_SEL_SHIFT: u32 = 2;
pub const LDO_ST_MASK: u32 = 0x03;
pub const LDO_ST_SHIFT: u32 = 0;
pub const LDO_ST_ON_BIT: u32 = 0x01;
pub const LDO_ST_MODE_BIT: u32 = 0x02;

/* Registers LDO1 to LDO8 in tps65910 */
pub const LDO1_SEL_MASK: u32 = 0xFC;
pub const LDO3_SEL_MASK: u32 = 0x7C;
pub const LDO_MIN_VOLT: u32 = 1000;
pub const LDO_MAX_VOLT: u32 = 3300;

/*Register VDIG1  (0x80) register.RegisterDescription */
pub const VDIG1_SEL_MASK: u32 = 0x0C;
pub const VDIG1_SEL_SHIFT: u32 = 2;
pub const VDIG1_ST_MASK: u32 = 0x03;
pub const VDIG1_ST_SHIFT: u32 = 0;

/*Register VDIG2  (0x80) register.RegisterDescription */
pub const VDIG2_SEL_MASK: u32 = 0x0C;
pub const VDIG2_SEL_SHIFT: u32 = 2;
pub const VDIG2_ST_MASK: u32 = 0x03;
pub const VDIG2_ST_SHIFT: u32 = 0;

/*Register VAUX1  (0x80) register.RegisterDescription */
pub const VAUX1_SEL_MASK: u32 = 0x0C;
pub const VAUX1_SEL_SHIFT: u32 = 2;
pub const VAUX1_ST_MASK: u32 = 0x03;
pub const VAUX1_ST_SHIFT: u32 = 0;

/*Register VAUX2  (0x80) register.RegisterDescription */
pub const VAUX2_SEL_MASK: u32 = 0x0C;
pub const VAUX2_SEL_SHIFT: u32 = 2;
pub const VAUX2_ST_MASK: u32 = 0x03;
pub const VAUX2_ST_SHIFT: u32 = 0;

/*Register VAUX33  (0x80) register.RegisterDescription */
pub const VAUX33_SEL_MASK: u32 = 0x0C;
pub const VAUX33_SEL_SHIFT: u32 = 2;
pub const VAUX33_ST_MASK: u32 = 0x03;
pub const VAUX33_ST_SHIFT: u32 = 0;

/*Register VMMC  (0x80) register.RegisterDescription */
pub const VMMC_SEL_MASK: u32 = 0x0C;
pub const VMMC_SEL_SHIFT: u32 = 2;
pub const VMMC_ST_MASK: u32 = 0x03;
pub const VMMC_ST_SHIFT: u32 = 0;

/*Register VPLL  (0x80) register.RegisterDescription */
pub const VPLL_SEL_MASK: u32 = 0x0C;
pub const VPLL_SEL_SHIFT: u32 = 2;
pub const VPLL_ST_MASK: u32 = 0x03;
pub const VPLL_ST_SHIFT: u32 = 0;

/*Register VDAC  (0x80) register.RegisterDescription */
pub const VDAC_SEL_MASK: u32 = 0x0C;
pub const VDAC_SEL_SHIFT: u32 = 2;
pub const VDAC_ST_MASK: u32 = 0x03;
pub const VDAC_ST_SHIFT: u32 = 0;

/*Register THERM  (0x80) register.RegisterDescription */
pub const THERM_THERM_HD_MASK: u32 = 0x20;
pub const THERM_THERM_HD_SHIFT: u32 = 5;
pub const THERM_THERM_TS_MASK: u32 = 0x10;
pub const THERM_THERM_TS_SHIFT: u32 = 4;
pub const THERM_THERM_HDSEL_MASK: u32 = 0x0C;
pub const THERM_THERM_HDSEL_SHIFT: u32 = 2;
pub const THERM_RSVD1_MASK: u32 = 0x02;
pub const THERM_RSVD1_SHIFT: u32 = 1;
pub const THERM_THERM_STATE_MASK: u32 = 0x01;
pub const THERM_THERM_STATE_SHIFT: u32 = 0;

/*Register BBCH  (0x80) register.RegisterDescription */
pub const BBCH_BBSEL_MASK: u32 = 0x06;
pub const BBCH_BBSEL_SHIFT: u32 = 1;

/*Register DCDCCTRL  (0x80) register.RegisterDescription */
pub const DCDCCTRL_VDD2_PSKIP_MASK: u32 = 0x20;
pub const DCDCCTRL_VDD2_PSKIP_SHIFT: u32 = 5;
pub const DCDCCTRL_VDD1_PSKIP_MASK: u32 = 0x10;
pub const DCDCCTRL_VDD1_PSKIP_SHIFT: u32 = 4;
pub const DCDCCTRL_VIO_PSKIP_MASK: u32 = 0x08;
pub const DCDCCTRL_VIO_PSKIP_SHIFT: u32 = 3;
pub const DCDCCTRL_DCDCCKEXT_MASK: u32 = 0x04;
pub const DCDCCTRL_DCDCCKEXT_SHIFT: u32 = 2;
pub const DCDCCTRL_DCDCCKSYNC_MASK: u32 = 0x03;
pub const DCDCCTRL_DCDCCKSYNC_SHIFT: u32 = 0;

/*Register DEVCTRL  (0x80) register.RegisterDescription */
pub const DEVCTRL_PWR_OFF_MASK: u32 = 0x80;
pub const DEVCTRL_PWR_OFF_SHIFT: u32 = 7;
pub const DEVCTRL_RTC_PWDN_MASK: u32 = 0x40;
pub const DEVCTRL_RTC_PWDN_SHIFT: u32 = 6;
pub const DEVCTRL_CK32K_CTRL_MASK: u32 = 0x20;
pub const DEVCTRL_CK32K_CTRL_SHIFT: u32 = 5;
pub const DEVCTRL_SR_CTL_I2C_SEL_MASK: u32 = 0x10;
pub const DEVCTRL_SR_CTL_I2C_SEL_SHIFT: u32 = 4;
pub const DEVCTRL_DEV_OFF_RST_MASK: u32 = 0x08;
pub const DEVCTRL_DEV_OFF_RST_SHIFT: u32 = 3;
pub const DEVCTRL_DEV_ON_MASK: u32 = 0x04;
pub const DEVCTRL_DEV_ON_SHIFT: u32 = 2;
pub const DEVCTRL_DEV_SLP_MASK: u32 = 0x02;
pub const DEVCTRL_DEV_SLP_SHIFT: u32 = 1;
pub const DEVCTRL_DEV_OFF_MASK: u32 = 0x01;
pub const DEVCTRL_DEV_OFF_SHIFT: u32 = 0;

/*Register DEVCTRL2  (0x80) register.RegisterDescription */
pub const DEVCTRL2_TSLOT_LENGTH_MASK: u32 = 0x30;
pub const DEVCTRL2_TSLOT_LENGTH_SHIFT: u32 = 4;
pub const DEVCTRL2_SLEEPSIG_POL_MASK: u32 = 0x08;
pub const DEVCTRL2_SLEEPSIG_POL_SHIFT: u32 = 3;
pub const DEVCTRL2_PWON_LP_OFF_MASK: u32 = 0x04;
pub const DEVCTRL2_PWON_LP_OFF_SHIFT: u32 = 2;
pub const DEVCTRL2_PWON_LP_RST_MASK: u32 = 0x02;
pub const DEVCTRL2_PWON_LP_RST_SHIFT: u32 = 1;
pub const DEVCTRL2_IT_POL_MASK: u32 = 0x01;
pub const DEVCTRL2_IT_POL_SHIFT: u32 = 0;

/*Register SLEEP_KEEP_LDO_ON  (0x80) register.RegisterDescription */
pub const SLEEP_KEEP_LDO_ON_VDAC_KEEPON_MASK: u32 = 0x80;
pub const SLEEP_KEEP_LDO_ON_VDAC_KEEPON_SHIFT: u32 = 7;
pub const SLEEP_KEEP_LDO_ON_VPLL_KEEPON_MASK: u32 = 0x40;
pub const SLEEP_KEEP_LDO_ON_VPLL_KEEPON_SHIFT: u32 = 6;
pub const SLEEP_KEEP_LDO_ON_VAUX33_KEEPON_MASK: u32 = 0x20;
pub const SLEEP_KEEP_LDO_ON_VAUX33_KEEPON_SHIFT: u32 = 5;
pub const SLEEP_KEEP_LDO_ON_VAUX2_KEEPON_MASK: u32 = 0x10;
pub const SLEEP_KEEP_LDO_ON_VAUX2_KEEPON_SHIFT: u32 = 4;
pub const SLEEP_KEEP_LDO_ON_VAUX1_KEEPON_MASK: u32 = 0x08;
pub const SLEEP_KEEP_LDO_ON_VAUX1_KEEPON_SHIFT: u32 = 3;
pub const SLEEP_KEEP_LDO_ON_VDIG2_KEEPON_MASK: u32 = 0x04;
pub const SLEEP_KEEP_LDO_ON_VDIG2_KEEPON_SHIFT: u32 = 2;
pub const SLEEP_KEEP_LDO_ON_VDIG1_KEEPON_MASK: u32 = 0x02;
pub const SLEEP_KEEP_LDO_ON_VDIG1_KEEPON_SHIFT: u32 = 1;
pub const SLEEP_KEEP_LDO_ON_VMMC_KEEPON_MASK: u32 = 0x01;
pub const SLEEP_KEEP_LDO_ON_VMMC_KEEPON_SHIFT: u32 = 0;

/*Register SLEEP_KEEP_RES_ON  (0x80) register.RegisterDescription */
pub const SLEEP_KEEP_RES_ON_THERM_KEEPON_MASK: u32 = 0x80;
pub const SLEEP_KEEP_RES_ON_THERM_KEEPON_SHIFT: u32 = 7;
pub const SLEEP_KEEP_RES_ON_CLKOUT32K_KEEPON_MASK: u32 = 0x40;
pub const SLEEP_KEEP_RES_ON_CLKOUT32K_KEEPON_SHIFT: u32 = 6;
pub const SLEEP_KEEP_RES_ON_VRTC_KEEPON_MASK: u32 = 0x20;
pub const SLEEP_KEEP_RES_ON_VRTC_KEEPON_SHIFT: u32 = 5;
pub const SLEEP_KEEP_RES_ON_I2CHS_KEEPON_MASK: u32 = 0x10;
pub const SLEEP_KEEP_RES_ON_I2CHS_KEEPON_SHIFT: u32 = 4;
pub const SLEEP_KEEP_RES_ON_VDD3_KEEPON_MASK: u32 = 0x08;
pub const SLEEP_KEEP_RES_ON_VDD3_KEEPON_SHIFT: u32 = 3;
pub const SLEEP_KEEP_RES_ON_VDD2_KEEPON_MASK: u32 = 0x04;
pub const SLEEP_KEEP_RES_ON_VDD2_KEEPON_SHIFT: u32 = 2;
pub const SLEEP_KEEP_RES_ON_VDD1_KEEPON_MASK: u32 = 0x02;
pub const SLEEP_KEEP_RES_ON_VDD1_KEEPON_SHIFT: u32 = 1;
pub const SLEEP_KEEP_RES_ON_VIO_KEEPON_MASK: u32 = 0x01;
pub const SLEEP_KEEP_RES_ON_VIO_KEEPON_SHIFT: u32 = 0;

/*Register SLEEP_SET_LDO_OFF  (0x80) register.RegisterDescription */
pub const SLEEP_SET_LDO_OFF_VDAC_SETOFF_MASK: u32 = 0x80;
pub const SLEEP_SET_LDO_OFF_VDAC_SETOFF_SHIFT: u32 = 7;
pub const SLEEP_SET_LDO_OFF_VPLL_SETOFF_MASK: u32 = 0x40;
pub const SLEEP_SET_LDO_OFF_VPLL_SETOFF_SHIFT: u32 = 6;
pub const SLEEP_SET_LDO_OFF_VAUX33_SETOFF_MASK: u32 = 0x20;
pub const SLEEP_SET_LDO_OFF_VAUX33_SETOFF_SHIFT: u32 = 5;
pub const SLEEP_SET_LDO_OFF_VAUX2_SETOFF_MASK: u32 = 0x10;
pub const SLEEP_SET_LDO_OFF_VAUX2_SETOFF_SHIFT: u32 = 4;
pub const SLEEP_SET_LDO_OFF_VAUX1_SETOFF_MASK: u32 = 0x08;
pub const SLEEP_SET_LDO_OFF_VAUX1_SETOFF_SHIFT: u32 = 3;
pub const SLEEP_SET_LDO_OFF_VDIG2_SETOFF_MASK: u32 = 0x04;
pub const SLEEP_SET_LDO_OFF_VDIG2_SETOFF_SHIFT: u32 = 2;
pub const SLEEP_SET_LDO_OFF_VDIG1_SETOFF_MASK: u32 = 0x02;
pub const SLEEP_SET_LDO_OFF_VDIG1_SETOFF_SHIFT: u32 = 1;
pub const SLEEP_SET_LDO_OFF_VMMC_SETOFF_MASK: u32 = 0x01;
pub const SLEEP_SET_LDO_OFF_VMMC_SETOFF_SHIFT: u32 = 0;

/*Register SLEEP_SET_RES_OFF  (0x80) register.RegisterDescription */
pub const SLEEP_SET_RES_OFF_DEFAULT_VOLT_MASK: u32 = 0x80;
pub const SLEEP_SET_RES_OFF_DEFAULT_VOLT_SHIFT: u32 = 7;
pub const SLEEP_SET_RES_OFF_RSVD_MASK: u32 = 0x60;
pub const SLEEP_SET_RES_OFF_RSVD_SHIFT: u32 = 5;
pub const SLEEP_SET_RES_OFF_SPARE_SETOFF_MASK: u32 = 0x10;
pub const SLEEP_SET_RES_OFF_SPARE_SETOFF_SHIFT: u32 = 4;
pub const SLEEP_SET_RES_OFF_VDD3_SETOFF_MASK: u32 = 0x08;
pub const SLEEP_SET_RES_OFF_VDD3_SETOFF_SHIFT: u32 = 3;
pub const SLEEP_SET_RES_OFF_VDD2_SETOFF_MASK: u32 = 0x04;
pub const SLEEP_SET_RES_OFF_VDD2_SETOFF_SHIFT: u32 = 2;
pub const SLEEP_SET_RES_OFF_VDD1_SETOFF_MASK: u32 = 0x02;
pub const SLEEP_SET_RES_OFF_VDD1_SETOFF_SHIFT: u32 = 1;
pub const SLEEP_SET_RES_OFF_VIO_SETOFF_MASK: u32 = 0x01;
pub const SLEEP_SET_RES_OFF_VIO_SETOFF_SHIFT: u32 = 0;

/*Register EN1_LDO_ASS	(0x80) register.RegisterDescription */
pub const EN1_LDO_ASS_VDAC_EN1_MASK: u32 = 0x80;
pub const EN1_LDO_ASS_VDAC_EN1_SHIFT: u32 = 7;
pub const EN1_LDO_ASS_VPLL_EN1_MASK: u32 = 0x40;
pub const EN1_LDO_ASS_VPLL_EN1_SHIFT: u32 = 6;
pub const EN1_LDO_ASS_VAUX33_EN1_MASK: u32 = 0x20;
pub const EN1_LDO_ASS_VAUX33_EN1_SHIFT: u32 = 5;
pub const EN1_LDO_ASS_VAUX2_EN1_MASK: u32 = 0x10;
pub const EN1_LDO_ASS_VAUX2_EN1_SHIFT: u32 = 4;
pub const EN1_LDO_ASS_VAUX1_EN1_MASK: u32 = 0x08;
pub const EN1_LDO_ASS_VAUX1_EN1_SHIFT: u32 = 3;
pub const EN1_LDO_ASS_VDIG2_EN1_MASK: u32 = 0x04;
pub const EN1_LDO_ASS_VDIG2_EN1_SHIFT: u32 = 2;
pub const EN1_LDO_ASS_VDIG1_EN1_MASK: u32 = 0x02;
pub const EN1_LDO_ASS_VDIG1_EN1_SHIFT: u32 = 1;
pub const EN1_LDO_ASS_VMMC_EN1_MASK: u32 = 0x01;
pub const EN1_LDO_ASS_VMMC_EN1_SHIFT: u32 = 0;

/*Register EN1_SMPS_ASS  (0x80) register.RegisterDescription */
pub const EN1_SMPS_ASS_RSVD_MASK: u32 = 0xE0;
pub const EN1_SMPS_ASS_RSVD_SHIFT: u32 = 5;
pub const EN1_SMPS_ASS_SPARE_EN1_MASK: u32 = 0x10;
pub const EN1_SMPS_ASS_SPARE_EN1_SHIFT: u32 = 4;
pub const EN1_SMPS_ASS_VDD3_EN1_MASK: u32 = 0x08;
pub const EN1_SMPS_ASS_VDD3_EN1_SHIFT: u32 = 3;
pub const EN1_SMPS_ASS_VDD2_EN1_MASK: u32 = 0x04;
pub const EN1_SMPS_ASS_VDD2_EN1_SHIFT: u32 = 2;
pub const EN1_SMPS_ASS_VDD1_EN1_MASK: u32 = 0x02;
pub const EN1_SMPS_ASS_VDD1_EN1_SHIFT: u32 = 1;
pub const EN1_SMPS_ASS_VIO_EN1_MASK: u32 = 0x01;
pub const EN1_SMPS_ASS_VIO_EN1_SHIFT: u32 = 0;

/*Register EN2_LDO_ASS	(0x80) register.RegisterDescription */
pub const EN2_LDO_ASS_VDAC_EN2_MASK: u32 = 0x80;
pub const EN2_LDO_ASS_VDAC_EN2_SHIFT: u32 = 7;
pub const EN2_LDO_ASS_VPLL_EN2_MASK: u32 = 0x40;
pub const EN2_LDO_ASS_VPLL_EN2_SHIFT: u32 = 6;
pub const EN2_LDO_ASS_VAUX33_EN2_MASK: u32 = 0x20;
pub const EN2_LDO_ASS_VAUX33_EN2_SHIFT: u32 = 5;
pub const EN2_LDO_ASS_VAUX2_EN2_MASK: u32 = 0x10;
pub const EN2_LDO_ASS_VAUX2_EN2_SHIFT: u32 = 4;
pub const EN2_LDO_ASS_VAUX1_EN2_MASK: u32 = 0x08;
pub const EN2_LDO_ASS_VAUX1_EN2_SHIFT: u32 = 3;
pub const EN2_LDO_ASS_VDIG2_EN2_MASK: u32 = 0x04;
pub const EN2_LDO_ASS_VDIG2_EN2_SHIFT: u32 = 2;
pub const EN2_LDO_ASS_VDIG1_EN2_MASK: u32 = 0x02;
pub const EN2_LDO_ASS_VDIG1_EN2_SHIFT: u32 = 1;
pub const EN2_LDO_ASS_VMMC_EN2_MASK: u32 = 0x01;
pub const EN2_LDO_ASS_VMMC_EN2_SHIFT: u32 = 0;

/*Register EN2_SMPS_ASS  (0x80) register.RegisterDescription */
pub const EN2_SMPS_ASS_RSVD_MASK: u32 = 0xE0;
pub const EN2_SMPS_ASS_RSVD_SHIFT: u32 = 5;
pub const EN2_SMPS_ASS_SPARE_EN2_MASK: u32 = 0x10;
pub const EN2_SMPS_ASS_SPARE_EN2_SHIFT: u32 = 4;
pub const EN2_SMPS_ASS_VDD3_EN2_MASK: u32 = 0x08;
pub const EN2_SMPS_ASS_VDD3_EN2_SHIFT: u32 = 3;
pub const EN2_SMPS_ASS_VDD2_EN2_MASK: u32 = 0x04;
pub const EN2_SMPS_ASS_VDD2_EN2_SHIFT: u32 = 2;
pub const EN2_SMPS_ASS_VDD1_EN2_MASK: u32 = 0x02;
pub const EN2_SMPS_ASS_VDD1_EN2_SHIFT: u32 = 1;
pub const EN2_SMPS_ASS_VIO_EN2_MASK: u32 = 0x01;
pub const EN2_SMPS_ASS_VIO_EN2_SHIFT: u32 = 0;

/*Register EN3_LDO_ASS	(0x80) register.RegisterDescription */
pub const EN3_LDO_ASS_VDAC_EN3_MASK: u32 = 0x80;
pub const EN3_LDO_ASS_VDAC_EN3_SHIFT: u32 = 7;
pub const EN3_LDO_ASS_VPLL_EN3_MASK: u32 = 0x40;
pub const EN3_LDO_ASS_VPLL_EN3_SHIFT: u32 = 6;
pub const EN3_LDO_ASS_VAUX33_EN3_MASK: u32 = 0x20;
pub const EN3_LDO_ASS_VAUX33_EN3_SHIFT: u32 = 5;
pub const EN3_LDO_ASS_VAUX2_EN3_MASK: u32 = 0x10;
pub const EN3_LDO_ASS_VAUX2_EN3_SHIFT: u32 = 4;
pub const EN3_LDO_ASS_VAUX1_EN3_MASK: u32 = 0x08;
pub const EN3_LDO_ASS_VAUX1_EN3_SHIFT: u32 = 3;
pub const EN3_LDO_ASS_VDIG2_EN3_MASK: u32 = 0x04;
pub const EN3_LDO_ASS_VDIG2_EN3_SHIFT: u32 = 2;
pub const EN3_LDO_ASS_VDIG1_EN3_MASK: u32 = 0x02;
pub const EN3_LDO_ASS_VDIG1_EN3_SHIFT: u32 = 1;
pub const EN3_LDO_ASS_VMMC_EN3_MASK: u32 = 0x01;
pub const EN3_LDO_ASS_VMMC_EN3_SHIFT: u32 = 0;

/*Register SPARE  (0x80) register.RegisterDescription */
pub const SPARE_SPARE_MASK: u32 = 0xFF;
pub const SPARE_SPARE_SHIFT: u32 = 0;

pub const TPS65910_INT_STS_RTC_PERIOD_IT_MASK: u32 = 0x80;
pub const TPS65910_INT_STS_RTC_PERIOD_IT_SHIFT: u32 = 7;
pub const TPS65910_INT_STS_RTC_ALARM_IT_MASK: u32 = 0x40;
pub const TPS65910_INT_STS_RTC_ALARM_IT_SHIFT: u32 = 6;
pub const TPS65910_INT_STS_HOTDIE_IT_MASK: u32 = 0x20;
pub const TPS65910_INT_STS_HOTDIE_IT_SHIFT: u32 = 5;
pub const TPS65910_INT_STS_PWRHOLD_F_IT_MASK: u32 = 0x10;
pub const TPS65910_INT_STS_PWRHOLD_F_IT_SHIFT: u32 = 4;
pub const TPS65910_INT_STS_PWRON_LP_IT_MASK: u32 = 0x08;
pub const TPS65910_INT_STS_PWRON_LP_IT_SHIFT: u32 = 3;
pub const TPS65910_INT_STS_PWRON_IT_MASK: u32 = 0x04;
pub const TPS65910_INT_STS_PWRON_IT_SHIFT: u32 = 2;
pub const TPS65910_INT_STS_VMBHI_IT_MASK: u32 = 0x02;
pub const TPS65910_INT_STS_VMBHI_IT_SHIFT: u32 = 1;
pub const TPS65910_INT_STS_VMBDCH_IT_MASK: u32 = 0x01;
pub const TPS65910_INT_STS_VMBDCH_IT_SHIFT: u32 = 0;

pub const TPS65910_INT_MSK_RTC_PERIOD_IT_MSK_MASK: u32 = 0x80;
pub const TPS65910_INT_MSK_RTC_PERIOD_IT_MSK_SHIFT: u32 = 7;
pub const TPS65910_INT_MSK_RTC_ALARM_IT_MSK_MASK: u32 = 0x40;
pub const TPS65910_INT_MSK_RTC_ALARM_IT_MSK_SHIFT: u32 = 6;
pub const TPS65910_INT_MSK_HOTDIE_IT_MSK_MASK: u32 = 0x20;
pub const TPS65910_INT_MSK_HOTDIE_IT_MSK_SHIFT: u32 = 5;
pub const TPS65910_INT_MSK_PWRHOLD_IT_MSK_MASK: u32 = 0x10;
pub const TPS65910_INT_MSK_PWRHOLD_IT_MSK_SHIFT: u32 = 4;
pub const TPS65910_INT_MSK_PWRON_LP_IT_MSK_MASK: u32 = 0x08;
pub const TPS65910_INT_MSK_PWRON_LP_IT_MSK_SHIFT: u32 = 3;
pub const TPS65910_INT_MSK_PWRON_IT_MSK_MASK: u32 = 0x04;
pub const TPS65910_INT_MSK_PWRON_IT_MSK_SHIFT: u32 = 2;
pub const TPS65910_INT_MSK_VMBHI_IT_MSK_MASK: u32 = 0x02;
pub const TPS65910_INT_MSK_VMBHI_IT_MSK_SHIFT: u32 = 1;
pub const TPS65910_INT_MSK_VMBDCH_IT_MSK_MASK: u32 = 0x01;
pub const TPS65910_INT_MSK_VMBDCH_IT_MSK_SHIFT: u32 = 0;

pub const TPS65910_INT_STS2_GPIO0_F_IT_SHIFT: u32 = 2;
pub const TPS65910_INT_STS2_GPIO0_F_IT_MASK: u32 = 0x02;
pub const TPS65910_INT_STS2_GPIO0_R_IT_SHIFT: u32 = 1;
pub const TPS65910_INT_STS2_GPIO0_R_IT_MASK: u32 = 0x01;

pub const TPS65910_INT_MSK2_GPIO0_F_IT_MSK_SHIFT: u32 = 2;
pub const TPS65910_INT_MSK2_GPIO0_F_IT_MSK_MASK: u32 = 0x02;
pub const TPS65910_INT_MSK2_GPIO0_R_IT_MSK_SHIFT: u32 = 1;
pub const TPS65910_INT_MSK2_GPIO0_R_IT_MSK_MASK: u32 = 0x01;

/*Register INT_STS  (0x80) register.RegisterDescription */
pub const INT_STS_RTC_PERIOD_IT_MASK: u32 = 0x80;
pub const INT_STS_RTC_PERIOD_IT_SHIFT: u32 = 7;
pub const INT_STS_RTC_ALARM_IT_MASK: u32 = 0x40;
pub const INT_STS_RTC_ALARM_IT_SHIFT: u32 = 6;
pub const INT_STS_HOTDIE_IT_MASK: u32 = 0x20;
pub const INT_STS_HOTDIE_IT_SHIFT: u32 = 5;
pub const INT_STS_PWRHOLD_R_IT_MASK: u32 = 0x10;
pub const INT_STS_PWRHOLD_R_IT_SHIFT: u32 = 4;
pub const INT_STS_PWRON_LP_IT_MASK: u32 = 0x08;
pub const INT_STS_PWRON_LP_IT_SHIFT: u32 = 3;
pub const INT_STS_PWRON_IT_MASK: u32 = 0x04;
pub const INT_STS_PWRON_IT_SHIFT: u32 = 2;
pub const INT_STS_VMBHI_IT_MASK: u32 = 0x02;
pub const INT_STS_VMBHI_IT_SHIFT: u32 = 1;
pub const INT_STS_PWRHOLD_F_IT_MASK: u32 = 0x01;
pub const INT_STS_PWRHOLD_F_IT_SHIFT: u32 = 0;

/*Register INT_MSK  (0x80) register.RegisterDescription */
pub const INT_MSK_RTC_PERIOD_IT_MSK_MASK: u32 = 0x80;
pub const INT_MSK_RTC_PERIOD_IT_MSK_SHIFT: u32 = 7;
pub const INT_MSK_RTC_ALARM_IT_MSK_MASK: u32 = 0x40;
pub const INT_MSK_RTC_ALARM_IT_MSK_SHIFT: u32 = 6;
pub const INT_MSK_HOTDIE_IT_MSK_MASK: u32 = 0x20;
pub const INT_MSK_HOTDIE_IT_MSK_SHIFT: u32 = 5;
pub const INT_MSK_PWRHOLD_R_IT_MSK_MASK: u32 = 0x10;
pub const INT_MSK_PWRHOLD_R_IT_MSK_SHIFT: u32 = 4;
pub const INT_MSK_PWRON_LP_IT_MSK_MASK: u32 = 0x08;
pub const INT_MSK_PWRON_LP_IT_MSK_SHIFT: u32 = 3;
pub const INT_MSK_PWRON_IT_MSK_MASK: u32 = 0x04;
pub const INT_MSK_PWRON_IT_MSK_SHIFT: u32 = 2;
pub const INT_MSK_VMBHI_IT_MSK_MASK: u32 = 0x02;
pub const INT_MSK_VMBHI_IT_MSK_SHIFT: u32 = 1;
pub const INT_MSK_PWRHOLD_F_IT_MSK_MASK: u32 = 0x01;
pub const INT_MSK_PWRHOLD_F_IT_MSK_SHIFT: u32 = 0;

/*Register INT_STS2  (0x80) register.RegisterDescription */
pub const INT_STS2_GPIO3_F_IT_MASK: u32 = 0x80;
pub const INT_STS2_GPIO3_F_IT_SHIFT: u32 = 7;
pub const INT_STS2_GPIO3_R_IT_MASK: u32 = 0x40;
pub const INT_STS2_GPIO3_R_IT_SHIFT: u32 = 6;
pub const INT_STS2_GPIO2_F_IT_MASK: u32 = 0x20;
pub const INT_STS2_GPIO2_F_IT_SHIFT: u32 = 5;
pub const INT_STS2_GPIO2_R_IT_MASK: u32 = 0x10;
pub const INT_STS2_GPIO2_R_IT_SHIFT: u32 = 4;
pub const INT_STS2_GPIO1_F_IT_MASK: u32 = 0x08;
pub const INT_STS2_GPIO1_F_IT_SHIFT: u32 = 3;
pub const INT_STS2_GPIO1_R_IT_MASK: u32 = 0x04;
pub const INT_STS2_GPIO1_R_IT_SHIFT: u32 = 2;
pub const INT_STS2_GPIO0_F_IT_MASK: u32 = 0x02;
pub const INT_STS2_GPIO0_F_IT_SHIFT: u32 = 1;
pub const INT_STS2_GPIO0_R_IT_MASK: u32 = 0x01;
pub const INT_STS2_GPIO0_R_IT_SHIFT: u32 = 0;

/*Register INT_MSK2  (0x80) register.RegisterDescription */
pub const INT_MSK2_GPIO3_F_IT_MSK_MASK: u32 = 0x80;
pub const INT_MSK2_GPIO3_F_IT_MSK_SHIFT: u32 = 7;
pub const INT_MSK2_GPIO3_R_IT_MSK_MASK: u32 = 0x40;
pub const INT_MSK2_GPIO3_R_IT_MSK_SHIFT: u32 = 6;
pub const INT_MSK2_GPIO2_F_IT_MSK_MASK: u32 = 0x20;
pub const INT_MSK2_GPIO2_F_IT_MSK_SHIFT: u32 = 5;
pub const INT_MSK2_GPIO2_R_IT_MSK_MASK: u32 = 0x10;
pub const INT_MSK2_GPIO2_R_IT_MSK_SHIFT: u32 = 4;
pub const INT_MSK2_GPIO1_F_IT_MSK_MASK: u32 = 0x08;
pub const INT_MSK2_GPIO1_F_IT_MSK_SHIFT: u32 = 3;
pub const INT_MSK2_GPIO1_R_IT_MSK_MASK: u32 = 0x04;
pub const INT_MSK2_GPIO1_R_IT_MSK_SHIFT: u32 = 2;
pub const INT_MSK2_GPIO0_F_IT_MSK_MASK: u32 = 0x02;
pub const INT_MSK2_GPIO0_F_IT_MSK_SHIFT: u32 = 1;
pub const INT_MSK2_GPIO0_R_IT_MSK_MASK: u32 = 0x01;
pub const INT_MSK2_GPIO0_R_IT_MSK_SHIFT: u32 = 0;

/*Register INT_STS3  (0x80) register.RegisterDescription */
pub const INT_STS3_PWRDN_IT_MASK: u32 = 0x80;
pub const INT_STS3_PWRDN_IT_SHIFT: u32 = 7;
pub const INT_STS3_VMBCH2_L_IT_MASK: u32 = 0x40;
pub const INT_STS3_VMBCH2_L_IT_SHIFT: u32 = 6;
pub const INT_STS3_VMBCH2_H_IT_MASK: u32 = 0x20;
pub const INT_STS3_VMBCH2_H_IT_SHIFT: u32 = 5;
pub const INT_STS3_WTCHDG_IT_MASK: u32 = 0x10;
pub const INT_STS3_WTCHDG_IT_SHIFT: u32 = 4;
pub const INT_STS3_GPIO5_F_IT_MASK: u32 = 0x08;
pub const INT_STS3_GPIO5_F_IT_SHIFT: u32 = 3;
pub const INT_STS3_GPIO5_R_IT_MASK: u32 = 0x04;
pub const INT_STS3_GPIO5_R_IT_SHIFT: u32 = 2;
pub const INT_STS3_GPIO4_F_IT_MASK: u32 = 0x02;
pub const INT_STS3_GPIO4_F_IT_SHIFT: u32 = 1;
pub const INT_STS3_GPIO4_R_IT_MASK: u32 = 0x01;
pub const INT_STS3_GPIO4_R_IT_SHIFT: u32 = 0;

/*Register INT_MSK3  (0x80) register.RegisterDescription */
pub const INT_MSK3_PWRDN_IT_MSK_MASK: u32 = 0x80;
pub const INT_MSK3_PWRDN_IT_MSK_SHIFT: u32 = 7;
pub const INT_MSK3_VMBCH2_L_IT_MSK_MASK: u32 = 0x40;
pub const INT_MSK3_VMBCH2_L_IT_MSK_SHIFT: u32 = 6;
pub const INT_MSK3_VMBCH2_H_IT_MSK_MASK: u32 = 0x20;
pub const INT_MSK3_VMBCH2_H_IT_MSK_SHIFT: u32 = 5;
pub const INT_MSK3_WTCHDG_IT_MSK_MASK: u32 = 0x10;
pub const INT_MSK3_WTCHDG_IT_MSK_SHIFT: u32 = 4;
pub const INT_MSK3_GPIO5_F_IT_MSK_MASK: u32 = 0x08;
pub const INT_MSK3_GPIO5_F_IT_MSK_SHIFT: u32 = 3;
pub const INT_MSK3_GPIO5_R_IT_MSK_MASK: u32 = 0x04;
pub const INT_MSK3_GPIO5_R_IT_MSK_SHIFT: u32 = 2;
pub const INT_MSK3_GPIO4_F_IT_MSK_MASK: u32 = 0x02;
pub const INT_MSK3_GPIO4_F_IT_MSK_SHIFT: u32 = 1;
pub const INT_MSK3_GPIO4_R_IT_MSK_MASK: u32 = 0x01;
pub const INT_MSK3_GPIO4_R_IT_MSK_SHIFT: u32 = 0;

/*Register GPIO  (0x80) register.RegisterDescription */
pub const GPIO_SLEEP_MASK: u32 = 0x80;
pub const GPIO_SLEEP_SHIFT: u32 = 7;
pub const GPIO_DEB_MASK: u32 = 0x10;
pub const GPIO_DEB_SHIFT: u32 = 4;
pub const GPIO_PUEN_MASK: u32 = 0x08;
pub const GPIO_PUEN_SHIFT: u32 = 3;
pub const GPIO_CFG_MASK: u32 = 0x04;
pub const GPIO_CFG_SHIFT: u32 = 2;
pub const GPIO_STS_MASK: u32 = 0x02;
pub const GPIO_STS_SHIFT: u32 = 1;
pub const GPIO_SET_MASK: u32 = 0x01;
pub const GPIO_SET_SHIFT: u32 = 0;

/*Register JTAGVERNUM  (0x80) register.RegisterDescription */
pub const JTAGVERNUM_VERNUM_MASK: u32 = 0x0F;
pub const JTAGVERNUM_VERNUM_SHIFT: u32 = 0;

/* Register VDDCTRL (0x27) bit definitions */
pub const VDDCTRL_ST_MASK: u32 = 0x03;
pub const VDDCTRL_ST_SHIFT: u32 = 0;

/*Register VDDCTRL_OP  (0x28) bit definitions */
pub const VDDCTRL_OP_CMD_MASK: u32 = 0x80;
pub const VDDCTRL_OP_CMD_SHIFT: u32 = 7;
pub const VDDCTRL_OP_SEL_MASK: u32 = 0x7F;
pub const VDDCTRL_OP_SEL_SHIFT: u32 = 0;

/*Register VDDCTRL_SR  (0x29) bit definitions */
pub const VDDCTRL_SR_SEL_MASK: u32 = 0x7F;
pub const VDDCTRL_SR_SEL_SHIFT: u32 = 0;

/* IRQ Definitions */
pub const TPS65910_IRQ_VBAT_VMBDCH: u32 = 0;
pub const TPS65910_IRQ_VBAT_VMHI: u32 = 1;
pub const TPS65910_IRQ_PWRON: u32 = 2;
pub const TPS65910_IRQ_PWRON_LP: u32 = 3;
pub const TPS65910_IRQ_PWRHOLD: u32 = 4;
pub const TPS65910_IRQ_HOTDIE: u32 = 5;
pub const TPS65910_IRQ_RTC_ALARM: u32 = 6;
pub const TPS65910_IRQ_RTC_PERIOD: u32 = 7;
pub const TPS65910_IRQ_GPIO_R: u32 = 8;
pub const TPS65910_IRQ_GPIO_F: u32 = 9;
pub const TPS65910_NUM_IRQ: u32 = 10;

pub const TPS65911_IRQ_PWRHOLD_F: u32 = 0;
pub const TPS65911_IRQ_VBAT_VMHI: u32 = 1;
pub const TPS65911_IRQ_PWRON: u32 = 2;
pub const TPS65911_IRQ_PWRON_LP: u32 = 3;
pub const TPS65911_IRQ_PWRHOLD_R: u32 = 4;
pub const TPS65911_IRQ_HOTDIE: u32 = 5;
pub const TPS65911_IRQ_RTC_ALARM: u32 = 6;
pub const TPS65911_IRQ_RTC_PERIOD: u32 = 7;
pub const TPS65911_IRQ_GPIO0_R: u32 = 8;
pub const TPS65911_IRQ_GPIO0_F: u32 = 9;
pub const TPS65911_IRQ_GPIO1_R: u32 = 10;
pub const TPS65911_IRQ_GPIO1_F: u32 = 11;
pub const TPS65911_IRQ_GPIO2_R: u32 = 12;
pub const TPS65911_IRQ_GPIO2_F: u32 = 13;
pub const TPS65911_IRQ_GPIO3_R: u32 = 14;
pub const TPS65911_IRQ_GPIO3_F: u32 = 15;
pub const TPS65911_IRQ_GPIO4_R: u32 = 16;
pub const TPS65911_IRQ_GPIO4_F: u32 = 17;
pub const TPS65911_IRQ_GPIO5_R: u32 = 18;
pub const TPS65911_IRQ_GPIO5_F: u32 = 19;
pub const TPS65911_IRQ_WTCHDG: u32 = 20;
pub const TPS65911_IRQ_VMBCH2_H: u32 = 21;
pub const TPS65911_IRQ_VMBCH2_L: u32 = 22;
pub const TPS65911_IRQ_PWRDN: u32 = 23;

pub const TPS65911_NUM_IRQ: u32 = 24;

/* GPIO Register Definitions */
pub const TPS65910_GPIO_DEB: u32 = 1 << 2;
pub const TPS65910_GPIO_PUEN: u32 = 1 << 3;
pub const TPS65910_GPIO_CFG: u32 = 1 << 2;
pub const TPS65910_GPIO_STS: u32 = 1 << 1;
pub const TPS65910_GPIO_SET: u32 = 1 << 0;

/* Max number of TPS65910/11 GPIOs */
pub const TPS65910_NUM_GPIO: u32 = 6;
pub const TPS65911_NUM_GPIO: u32 = 9;
pub const TPS6591X_MAX_NUM_GPIO: u32 = 9;

/* Regulator Index Definitions */
pub const TPS65910_REG_VRTC: u32 = 0;
pub const TPS65910_REG_VIO: u32 = 1;
pub const TPS65910_REG_VDD1: u32 = 2;
pub const TPS65910_REG_VDD2: u32 = 3;
pub const TPS65910_REG_VDD3: u32 = 4;
pub const TPS65910_REG_VDIG1: u32 = 5;
pub const TPS65910_REG_VDIG2: u32 = 6;
pub const TPS65910_REG_VPLL: u32 = 7;
pub const TPS65910_REG_VDAC: u32 = 8;
pub const TPS65910_REG_VAUX1: u32 = 9;
pub const TPS65910_REG_VAUX2: u32 = 10;
pub const TPS65910_REG_VAUX33: u32 = 11;
pub const TPS65910_REG_VMMC: u32 = 12;
pub const TPS65910_REG_VBB: u32 = 13;

pub const TPS65911_REG_VDDCTRL: u32 = 4;
pub const TPS65911_REG_LDO1: u32 = 5;
pub const TPS65911_REG_LDO2: u32 = 6;
pub const TPS65911_REG_LDO3: u32 = 7;
pub const TPS65911_REG_LDO4: u32 = 8;
pub const TPS65911_REG_LDO5: u32 = 9;
pub const TPS65911_REG_LDO6: u32 = 10;
pub const TPS65911_REG_LDO7: u32 = 11;
pub const TPS65911_REG_LDO8: u32 = 12;

/* Max number of TPS65910/11 regulators */
pub const TPS65910_NUM_REGS: u32 = 14;

/* External sleep controls through EN1/EN2/EN3/SLEEP inputs */
pub const TPS65910_SLEEP_CONTROL_EXT_INPUT_EN1: u32 = 0x1;
pub const TPS65910_SLEEP_CONTROL_EXT_INPUT_EN2: u32 = 0x2;
pub const TPS65910_SLEEP_CONTROL_EXT_INPUT_EN3: u32 = 0x4;

/*
 * Sleep keepon data: Maintains the state in sleep mode
 * @therm_keepon: Keep on the thermal monitoring in sleep state.
 * @clkout32k_keepon: Keep on the 32KHz clock output in sleep state.
 * @i2chs_keepon: Keep on high speed internal clock in sleep state.
 */
/// The three one-bit `unsigned` bitfields share a single storage unit.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct tps65910_sleep_keepon_data {
    bits: u32,
}

impl tps65910_sleep_keepon_data {
    const THERM_KEEPON: u32 = 1 << 0;
    const CLKOUT32K_KEEPON: u32 = 1 << 1;
    const I2CHS_KEEPON: u32 = 1 << 2;

    fn get(&self, bit: u32) -> bool {
        self.bits & bit != 0
    }

    fn set(&mut self, bit: u32, on: bool) {
        if on {
            self.bits |= bit;
        } else {
            self.bits &= !bit;
        }
    }

    pub fn therm_keepon(&self) -> bool {
        self.get(Self::THERM_KEEPON)
    }

    pub fn set_therm_keepon(&mut self, on: bool) {
        self.set(Self::THERM_KEEPON, on);
    }

    pub fn clkout32k_keepon(&self) -> bool {
        self.get(Self::CLKOUT32K_KEEPON)
    }

    pub fn set_clkout32k_keepon(&mut self, on: bool) {
        self.set(Self::CLKOUT32K_KEEPON, on);
    }

    pub fn i2chs_keepon(&self) -> bool {
        self.get(Self::I2CHS_KEEPON)
    }

    pub fn set_i2chs_keepon(&mut self, on: bool) {
        self.set(Self::I2CHS_KEEPON, on);
    }
}

/**
 * struct tps65910_board
 * Board platform data may be used to initialize regulators.
 */
#[repr(C)]
pub struct tps65910_board {
    pub gpio_base: core::ffi::c_int,
    pub irq: core::ffi::c_int,
    pub irq_base: core::ffi::c_int,
    pub vmbch_threshold: core::ffi::c_int,
    pub vmbch2_threshold: core::ffi::c_int,
    pub en_ck32k_xtal: bool,
    pub en_dev_slp: bool,
    pub pm_off: bool,
    pub slp_keepon: tps65910_sleep_keepon_data,
    pub en_gpio_sleep: [bool; TPS6591X_MAX_NUM_GPIO as usize],
    pub regulator_ext_sleep_control: [core::ffi::c_ulong; TPS65910_NUM_REGS as usize],
    pub tps65910_pmic_init_data: [*mut kernel::bindings::regulator_init_data; TPS65910_NUM_REGS as usize],
}

/**
 * struct tps65910 - tps65910 sub-driver chip access routines
 */
#[repr(C)]
pub struct tps65910 {
    pub dev: *mut kernel::bindings::device,
    pub i2c_client: *mut kernel::bindings::i2c_client,
    pub regmap: *mut kernel::bindings::regmap,
    pub id: core::ffi::c_ulong,

    /* Device node parsed board data */
    pub of_plat_data: *mut tps65910_board,

    /* IRQ Handling */
    pub chip_irq: core::ffi::c_int,
    pub irq_data: *mut kernel::bindings::regmap_irq_chip_data,
}

#[repr(C)]
pub struct tps65910_platform_data {
    pub irq: core::ffi::c_int,
    pub irq_base: core::ffi::c_int,
}

/// # Safety
///
/// `tps65910` must point to a live, initialized `struct tps65910`.
#[inline]
pub unsafe fn tps65910_chip_id(tps65910: *const tps65910) -> core::ffi::c_int {
    // SAFETY: the caller guarantees `tps65910` is valid for reads.
    // C returns the `unsigned long` id through an `int`, truncating it.
    unsafe { (*tps65910).id as core::ffi::c_int }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
