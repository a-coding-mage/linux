// SPDX-License-Identifier: GPL-2.0-only
/*
 * wm2000.h  --  WM2000 Soc Audio driver
 */

pub const WM2000_REG_SYS_START: u16 = 0x8000;
pub const WM2000_REG_ANC_GAIN_CTRL: u16 = 0x8fa2;
pub const WM2000_REG_MSE_TH2: u16 = 0x8fdf;
pub const WM2000_REG_MSE_TH1: u16 = 0x8fe0;
pub const WM2000_REG_SPEECH_CLARITY: u16 = 0x8fef;
pub const WM2000_REG_SYS_WATCHDOG: u16 = 0x8ff6;
pub const WM2000_REG_ANA_VMID_PD_TIME: u16 = 0x8ff7;
pub const WM2000_REG_ANA_VMID_PU_TIME: u16 = 0x8ff8;
pub const WM2000_REG_CAT_FLTR_INDX: u16 = 0x8ff9;
pub const WM2000_REG_CAT_GAIN_0: u16 = 0x8ffa;
pub const WM2000_REG_SYS_STATUS: u16 = 0x8ffc;
pub const WM2000_REG_SYS_MODE_CNTRL: u16 = 0x8ffd;
pub const WM2000_REG_SYS_START0: u16 = 0x8ffe;
pub const WM2000_REG_SYS_START1: u16 = 0x8fff;
pub const WM2000_REG_ID1: u16 = 0xf000;
pub const WM2000_REG_ID2: u16 = 0xf001;
pub const WM2000_REG_REVISON: u16 = 0xf002;
pub const WM2000_REG_SYS_CTL1: u16 = 0xf003;
pub const WM2000_REG_SYS_CTL2: u16 = 0xf004;
pub const WM2000_REG_ANC_STAT: u16 = 0xf005;
pub const WM2000_REG_IF_CTL: u16 = 0xf006;
pub const WM2000_REG_ANA_MIC_CTL: u16 = 0xf028;
pub const WM2000_REG_SPK_CTL: u16 = 0xf034;

/* SPEECH_CLARITY */
pub const WM2000_SPEECH_CLARITY: u8 = 0x01;

/* SYS_STATUS */
pub const WM2000_STATUS_MOUSE_ACTIVE: u8 = 0x40;
pub const WM2000_STATUS_CAT_FREQ_COMPLETE: u8 = 0x20;
pub const WM2000_STATUS_CAT_GAIN_COMPLETE: u8 = 0x10;
pub const WM2000_STATUS_THERMAL_SHUTDOWN_COMPLETE: u8 = 0x08;
pub const WM2000_STATUS_ANC_DISABLED: u8 = 0x04;
pub const WM2000_STATUS_POWER_DOWN_COMPLETE: u8 = 0x02;
pub const WM2000_STATUS_BOOT_COMPLETE: u8 = 0x01;

/* SYS_MODE_CNTRL */
pub const WM2000_MODE_ANA_SEQ_INCLUDE: u8 = 0x80;
pub const WM2000_MODE_MOUSE_ENABLE: u8 = 0x40;
pub const WM2000_MODE_CAT_FREQ_ENABLE: u8 = 0x20;
pub const WM2000_MODE_CAT_GAIN_ENABLE: u8 = 0x10;
pub const WM2000_MODE_BYPASS_ENTRY: u8 = 0x08;
pub const WM2000_MODE_STANDBY_ENTRY: u8 = 0x04;
pub const WM2000_MODE_THERMAL_ENABLE: u8 = 0x02;
pub const WM2000_MODE_POWER_DOWN: u8 = 0x01;

/* SYS_CTL1 */
pub const WM2000_SYS_STBY: u8 = 0x01;

/* SYS_CTL2 */
pub const WM2000_MCLK_DIV2_ENA_CLR: u8 = 0x80;
pub const WM2000_MCLK_DIV2_ENA_SET: u8 = 0x40;
pub const WM2000_ANC_ENG_CLR: u8 = 0x20;
pub const WM2000_ANC_ENG_SET: u8 = 0x10;
pub const WM2000_ANC_INT_N_CLR: u8 = 0x08;
pub const WM2000_ANC_INT_N_SET: u8 = 0x04;
pub const WM2000_RAM_CLR: u8 = 0x02;
pub const WM2000_RAM_SET: u8 = 0x01;

/* ANC_STAT */
pub const WM2000_ANC_ENG_IDLE: u8 = 0x01;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
