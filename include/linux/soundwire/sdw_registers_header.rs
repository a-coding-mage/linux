/* SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause) */
/* Copyright(c) 2015-17 Intel Corporation. */

/* SDW registers as defined by MIPI 1.2 Spec. */
pub const SDW_REGADDR: u32 = 0x7fff;
pub const SDW_SCP_ADDRPAGE2_MASK: u32 = 0x7f8000;
pub const SDW_SCP_ADDRPAGE1_MASK: u32 = 0x7f800000;
pub const SDW_REG_NO_PAGE: u32 = 0x00008000;
pub const SDW_REG_OPTIONAL_PAGE: u32 = 0x00010000;
pub const SDW_REG_MAX: u32 = 0x48000000;
pub const SDW_DPN_SIZE: u32 = 0x100;
pub const SDW_BANK1_OFFSET: u32 = 0x10;

pub const SDW_DP0_INT: u32 = 0x0;
pub const SDW_DP0_INTMASK: u32 = 0x1;
pub const SDW_DP0_PORTCTRL: u32 = 0x2;
pub const SDW_DP0_BLOCKCTRL1: u32 = 0x3;
pub const SDW_DP0_PREPARESTATUS: u32 = 0x4;
pub const SDW_DP0_PREPARECTRL: u32 = 0x5;
pub const SDW_DP0_INT_TEST_FAIL: u32 = 1 << 0;
pub const SDW_DP0_INT_PORT_READY: u32 = 1 << 1;
pub const SDW_DP0_INT_BRA_FAILURE: u32 = 1 << 2;
pub const SDW_DP0_SDCA_CASCADE: u32 = 1 << 3;
pub const SDW_DP0_INT_IMPDEF1: u32 = 1 << 5;
pub const SDW_DP0_INT_IMPDEF2: u32 = 1 << 6;
pub const SDW_DP0_INT_IMPDEF3: u32 = 1 << 7;
pub const SDW_DP0_INTERRUPTS: u32 = SDW_DP0_INT_TEST_FAIL | SDW_DP0_INT_PORT_READY | SDW_DP0_INT_BRA_FAILURE | SDW_DP0_INT_IMPDEF1 | SDW_DP0_INT_IMPDEF2 | SDW_DP0_INT_IMPDEF3;
pub const SDW_DP0_PORTCTRL_DATAMODE: u32 = 0xc;
pub const SDW_DP0_PORTCTRL_NXTINVBANK: u32 = 1 << 4;
pub const SDW_DP0_PORTCTRL_BPT_PAYLD: u32 = 0xc0;
pub const SDW_DP0_CHANNELEN: u32 = 0x20;
pub const SDW_DP0_SAMPLECTRL1: u32 = 0x22;
pub const SDW_DP0_SAMPLECTRL2: u32 = 0x23;
pub const SDW_DP0_OFFSETCTRL1: u32 = 0x24;
pub const SDW_DP0_OFFSETCTRL2: u32 = 0x25;
pub const SDW_DP0_HCTRL: u32 = 0x26;
pub const SDW_DP0_LANECTRL: u32 = 0x28;

pub const SDW_SCP_INT1: u32 = 0x40;
pub const SDW_SCP_INTMASK1: u32 = 0x41;
pub const SDW_SCP_INT1_PARITY: u32 = 1;
pub const SDW_SCP_INT1_BUS_CLASH: u32 = 2;
pub const SDW_SCP_INT1_IMPL_DEF: u32 = 4;
pub const SDW_SCP_INT1_SCP2_CASCADE: u32 = 0x80;
pub const SDW_SCP_INT1_PORT0_3: u32 = 0x78;
pub const SDW_SCP_INTSTAT2: u32 = 0x42;
pub const SDW_SCP_INTSTAT2_SCP3_CASCADE: u32 = 0x80;
pub const SDW_SCP_INTSTAT2_PORT4_10: u32 = 0x7f;
pub const SDW_SCP_INTSTAT3: u32 = 0x43;
pub const SDW_SCP_INTSTAT3_PORT11_14: u32 = 0xf;
pub const SDW_NUM_INT_STAT_REGISTERS: u32 = 3;
pub const SDW_NUM_INT_CLEAR_REGISTERS: u32 = 1;
pub const SDW_SCP_CTRL: u32 = 0x44;
pub const SDW_SCP_CTRL_CLK_STP_NOW: u32 = 2;
pub const SDW_SCP_CTRL_FORCE_RESET: u32 = 0x80;
pub const SDW_SCP_STAT: u32 = 0x44;
pub const SDW_SCP_STAT_CLK_STP_NF: u32 = 1;
pub const SDW_SCP_STAT_HPHY_NOK: u32 = 0x20;
pub const SDW_SCP_STAT_CURR_BANK: u32 = 0x40;
pub const SDW_SCP_SYSTEMCTRL: u32 = 0x45;
pub const SDW_SCP_SYSTEMCTRL_CLK_STP_PREP: u32 = 1;
pub const SDW_SCP_SYSTEMCTRL_CLK_STP_MODE: u32 = 4;
pub const SDW_SCP_SYSTEMCTRL_WAKE_UP_EN: u32 = 8;
pub const SDW_SCP_SYSTEMCTRL_HIGH_PHY: u32 = 0x10;
pub const SDW_SCP_SYSTEMCTRL_CLK_STP_MODE0: u32 = 0;
pub const SDW_SCP_SYSTEMCTRL_CLK_STP_MODE1: u32 = 4;
pub const SDW_SCP_DEVNUMBER: u32 = 0x46;
pub const SDW_SCP_HIGH_PHY_CHECK: u32 = 0x47;
pub const SDW_SCP_ADDRPAGE1: u32 = 0x48;
pub const SDW_SCP_ADDRPAGE2: u32 = 0x49;
pub const SDW_SCP_KEEPEREN: u32 = 0x4a;
pub const SDW_SCP_BANKDELAY: u32 = 0x4b;
pub const SDW_SCP_COMMIT: u32 = 0x4c;
pub const SDW_SCP_BUS_CLOCK_BASE: u32 = 0x4d;
pub const SDW_SCP_BASE_CLOCK_FREQ: u32 = 7;
pub const SDW_SCP_BASE_CLOCK_UNKNOWN: u32 = 0;
pub const SDW_SCP_BASE_CLOCK_19200000_HZ: u32 = 1;
pub const SDW_SCP_BASE_CLOCK_24000000_HZ: u32 = 2;
pub const SDW_SCP_BASE_CLOCK_24576000_HZ: u32 = 3;
pub const SDW_SCP_BASE_CLOCK_22579200_HZ: u32 = 4;
pub const SDW_SCP_BASE_CLOCK_32000000_HZ: u32 = 5;
pub const SDW_SCP_BASE_CLOCK_RESERVED: u32 = 6;
pub const SDW_SCP_BASE_CLOCK_IMP_DEF: u32 = 7;
pub const SDW_SCP_TESTMODE: u32 = 0x4f;
pub const SDW_SCP_DEVID_0: u32 = 0x50;
pub const SDW_SCP_DEVID_1: u32 = 0x51;
pub const SDW_SCP_DEVID_2: u32 = 0x52;
pub const SDW_SCP_DEVID_3: u32 = 0x53;
pub const SDW_SCP_DEVID_4: u32 = 0x54;
pub const SDW_SCP_DEVID_5: u32 = 0x55;

pub const SDW_SCP_SDCA_INT1: u32 = 0x58;
pub const SDW_SCP_SDCA_INT2: u32 = 0x59;
pub const SDW_SCP_SDCA_INT3: u32 = 0x5a;
pub const SDW_SCP_SDCA_INT4: u32 = 0x5b;
pub const SDW_SCP_SDCA_INTMASK1: u32 = 0x5c;
pub const SDW_SCP_SDCA_INTMASK2: u32 = 0x5d;
pub const SDW_SCP_SDCA_INTMASK3: u32 = 0x5e;
pub const SDW_SCP_SDCA_INTMASK4: u32 = 0x5f;
pub const SDW_SCP_SDCA_INT_SDCA_0: u32 = 1;
pub const SDW_SCP_SDCA_INT_SDCA_1: u32 = 2;
pub const SDW_SCP_SDCA_INT_SDCA_2: u32 = 4;
pub const SDW_SCP_SDCA_INT_SDCA_3: u32 = 8;
pub const SDW_SCP_SDCA_INT_SDCA_4: u32 = 0x10;
pub const SDW_SCP_SDCA_INT_SDCA_5: u32 = 0x20;
pub const SDW_SCP_SDCA_INT_SDCA_6: u32 = 0x40;
pub const SDW_SCP_SDCA_INT_SDCA_7: u32 = 0x80;
pub const SDW_SCP_SDCA_INT_SDCA_8: u32 = 1;
pub const SDW_SCP_SDCA_INT_SDCA_9: u32 = 2;
pub const SDW_SCP_SDCA_INT_SDCA_10: u32 = 4;
pub const SDW_SCP_SDCA_INT_SDCA_11: u32 = 8;
pub const SDW_SCP_SDCA_INT_SDCA_12: u32 = 0x10;
pub const SDW_SCP_SDCA_INT_SDCA_13: u32 = 0x20;
pub const SDW_SCP_SDCA_INT_SDCA_14: u32 = 0x40;
pub const SDW_SCP_SDCA_INT_SDCA_15: u32 = 0x80;
pub const SDW_SCP_SDCA_INT_SDCA_16: u32 = 1;
pub const SDW_SCP_SDCA_INT_SDCA_17: u32 = 2;
pub const SDW_SCP_SDCA_INT_SDCA_18: u32 = 4;
pub const SDW_SCP_SDCA_INT_SDCA_19: u32 = 8;
pub const SDW_SCP_SDCA_INT_SDCA_20: u32 = 0x10;
pub const SDW_SCP_SDCA_INT_SDCA_21: u32 = 0x20;
pub const SDW_SCP_SDCA_INT_SDCA_22: u32 = 0x40;
pub const SDW_SCP_SDCA_INT_SDCA_23: u32 = 0x80;
pub const SDW_SCP_SDCA_INT_SDCA_24: u32 = 1;
pub const SDW_SCP_SDCA_INT_SDCA_25: u32 = 2;
pub const SDW_SCP_SDCA_INT_SDCA_26: u32 = 4;
pub const SDW_SCP_SDCA_INT_SDCA_27: u32 = 8;
pub const SDW_SCP_SDCA_INT_SDCA_28: u32 = 0x10;
pub const SDW_SCP_SDCA_INT_SDCA_29: u32 = 0x20;
pub const SDW_SCP_SDCA_INT_SDCA_30: u32 = 0x40;
pub const SDW_SCP_SDCA_INTMASK_SDCA_0: u32 = 1;
pub const SDW_SCP_SDCA_INTMASK_SDCA_1: u32 = 2;
pub const SDW_SCP_SDCA_INTMASK_SDCA_2: u32 = 4;
pub const SDW_SCP_SDCA_INTMASK_SDCA_3: u32 = 8;
pub const SDW_SCP_SDCA_INTMASK_SDCA_4: u32 = 0x10;
pub const SDW_SCP_SDCA_INTMASK_SDCA_5: u32 = 0x20;
pub const SDW_SCP_SDCA_INTMASK_SDCA_6: u32 = 0x40;
pub const SDW_SCP_SDCA_INTMASK_SDCA_7: u32 = 0x80;
pub const SDW_SCP_SDCA_INTMASK_SDCA_8: u32 = 1;
pub const SDW_SCP_SDCA_INTMASK_SDCA_9: u32 = 2;
pub const SDW_SCP_SDCA_INTMASK_SDCA_10: u32 = 4;
pub const SDW_SCP_SDCA_INTMASK_SDCA_11: u32 = 8;
pub const SDW_SCP_SDCA_INTMASK_SDCA_12: u32 = 0x10;
pub const SDW_SCP_SDCA_INTMASK_SDCA_13: u32 = 0x20;
pub const SDW_SCP_SDCA_INTMASK_SDCA_14: u32 = 0x40;
pub const SDW_SCP_SDCA_INTMASK_SDCA_15: u32 = 0x80;
pub const SDW_SCP_SDCA_INTMASK_SDCA_16: u32 = 1;
pub const SDW_SCP_SDCA_INTMASK_SDCA_17: u32 = 2;
pub const SDW_SCP_SDCA_INTMASK_SDCA_18: u32 = 4;
pub const SDW_SCP_SDCA_INTMASK_SDCA_19: u32 = 8;
pub const SDW_SCP_SDCA_INTMASK_SDCA_20: u32 = 0x10;
pub const SDW_SCP_SDCA_INTMASK_SDCA_21: u32 = 0x20;
pub const SDW_SCP_SDCA_INTMASK_SDCA_22: u32 = 0x40;
pub const SDW_SCP_SDCA_INTMASK_SDCA_23: u32 = 0x80;
pub const SDW_SCP_SDCA_INTMASK_SDCA_24: u32 = 1;
pub const SDW_SCP_SDCA_INTMASK_SDCA_25: u32 = 2;
pub const SDW_SCP_SDCA_INTMASK_SDCA_26: u32 = 4;
pub const SDW_SCP_SDCA_INTMASK_SDCA_27: u32 = 8;
pub const SDW_SCP_SDCA_INTMASK_SDCA_28: u32 = 0x10;
pub const SDW_SCP_SDCA_INTMASK_SDCA_29: u32 = 0x20;
pub const SDW_SCP_SDCA_INTMASK_SDCA_30: u32 = 0x40;

pub const SDW_SCP_FRAMECTRL_B0: u32 = 0x60;
pub const SDW_SCP_FRAMECTRL_B1: u32 = 0x70;
pub const SDW_SCP_NEXTFRAME_B0: u32 = 0x61;
pub const SDW_SCP_NEXTFRAME_B1: u32 = 0x71;
pub const SDW_SCP_BUSCLOCK_SCALE_B0: u32 = 0x62;
pub const SDW_SCP_BUSCLOCK_SCALE_B1: u32 = 0x72;
pub const SDW_SCP_CLOCK_SCALE: u32 = 0xf;
pub const SDW_SCP_PHY_OUT_CTRL_0: u32 = 0x80;
pub const SDW_SCP_PHY_OUT_CTRL_1: u32 = 0x81;
pub const SDW_SCP_PHY_OUT_CTRL_2: u32 = 0x82;
pub const SDW_SCP_PHY_OUT_CTRL_3: u32 = 0x83;
pub const SDW_SCP_PHY_OUT_CTRL_4: u32 = 0x84;
pub const SDW_SCP_PHY_OUT_CTRL_5: u32 = 0x85;
pub const SDW_SCP_PHY_OUT_CTRL_6: u32 = 0x86;
pub const SDW_SCP_PHY_OUT_CTRL_7: u32 = 0x87;
pub const SDW_SCP_CAP_LOAD_CTRL: u32 = 7;
pub const SDW_SCP_DRIVE_STRENGTH_CTRL: u32 = 0x38;
pub const SDW_SCP_SLEW_TIME_CTRL: u32 = 0xc0;

#[inline] pub const fn SDW_DPN_INT(n: u32) -> u32 { SDW_DPN_SIZE * n }
#[inline] pub const fn SDW_DPN_INTMASK(n: u32) -> u32 { 1 + SDW_DPN_SIZE * n }
#[inline] pub const fn SDW_DPN_PORTCTRL(n: u32) -> u32 { 2 + SDW_DPN_SIZE * n }
#[inline] pub const fn SDW_DPN_BLOCKCTRL1(n: u32) -> u32 { 3 + SDW_DPN_SIZE * n }
#[inline] pub const fn SDW_DPN_PREPARESTATUS(n: u32) -> u32 { 4 + SDW_DPN_SIZE * n }
#[inline] pub const fn SDW_DPN_PREPARECTRL(n: u32) -> u32 { 5 + SDW_DPN_SIZE * n }
pub const SDW_DPN_INT_TEST_FAIL: u32 = 1;
pub const SDW_DPN_INT_PORT_READY: u32 = 2;
pub const SDW_DPN_INT_IMPDEF1: u32 = 0x20;
pub const SDW_DPN_INT_IMPDEF2: u32 = 0x40;
pub const SDW_DPN_INT_IMPDEF3: u32 = 0x80;
pub const SDW_DPN_INTERRUPTS: u32 = 0xe3;
pub const SDW_DPN_PORTCTRL_FLOWMODE: u32 = 3;
pub const SDW_DPN_PORTCTRL_DATAMODE: u32 = 0xc;
pub const SDW_DPN_PORTCTRL_NXTINVBANK: u32 = 0x10;
pub const SDW_DPN_BLOCKCTRL1_WDLEN: u32 = 0x3f;
pub const SDW_DPN_PREPARECTRL_CH_PREP: u32 = 0xff;
#[inline] pub const fn SDW_DPN_CHANNELEN_B0(n: u32) -> u32 { 0x20 + SDW_DPN_SIZE * n }
#[inline] pub const fn SDW_DPN_CHANNELEN_B1(n: u32) -> u32 { 0x30 + SDW_DPN_SIZE * n }
#[inline] pub const fn SDW_DPN_BLOCKCTRL2_B0(n: u32) -> u32 { 0x21 + SDW_DPN_SIZE * n }
#[inline] pub const fn SDW_DPN_BLOCKCTRL2_B1(n: u32) -> u32 { 0x31 + SDW_DPN_SIZE * n }
#[inline] pub const fn SDW_DPN_SAMPLECTRL1_B0(n: u32) -> u32 { 0x22 + SDW_DPN_SIZE * n }
#[inline] pub const fn SDW_DPN_SAMPLECTRL1_B1(n: u32) -> u32 { 0x32 + SDW_DPN_SIZE * n }
#[inline] pub const fn SDW_DPN_SAMPLECTRL2_B0(n: u32) -> u32 { 0x23 + SDW_DPN_SIZE * n }
#[inline] pub const fn SDW_DPN_SAMPLECTRL2_B1(n: u32) -> u32 { 0x33 + SDW_DPN_SIZE * n }
#[inline] pub const fn SDW_DPN_OFFSETCTRL1_B0(n: u32) -> u32 { 0x24 + SDW_DPN_SIZE * n }
#[inline] pub const fn SDW_DPN_OFFSETCTRL1_B1(n: u32) -> u32 { 0x34 + SDW_DPN_SIZE * n }
#[inline] pub const fn SDW_DPN_OFFSETCTRL2_B0(n: u32) -> u32 { 0x25 + SDW_DPN_SIZE * n }
#[inline] pub const fn SDW_DPN_OFFSETCTRL2_B1(n: u32) -> u32 { 0x35 + SDW_DPN_SIZE * n }
#[inline] pub const fn SDW_DPN_HCTRL_B0(n: u32) -> u32 { 0x26 + SDW_DPN_SIZE * n }
#[inline] pub const fn SDW_DPN_HCTRL_B1(n: u32) -> u32 { 0x36 + SDW_DPN_SIZE * n }
#[inline] pub const fn SDW_DPN_BLOCKCTRL3_B0(n: u32) -> u32 { 0x27 + SDW_DPN_SIZE * n }
#[inline] pub const fn SDW_DPN_BLOCKCTRL3_B1(n: u32) -> u32 { 0x37 + SDW_DPN_SIZE * n }
#[inline] pub const fn SDW_DPN_LANECTRL_B0(n: u32) -> u32 { 0x28 + SDW_DPN_SIZE * n }
#[inline] pub const fn SDW_DPN_LANECTRL_B1(n: u32) -> u32 { 0x38 + SDW_DPN_SIZE * n }
pub const SDW_DPN_SAMPLECTRL_LOW: u32 = 0xff;
pub const SDW_DPN_SAMPLECTRL_HIGH: u32 = 0xff00;
pub const SDW_DPN_HCTRL_HSTART: u32 = 0xf0;
pub const SDW_DPN_HCTRL_HSTOP: u32 = 0xf;

pub const SDW_NUM_CASC_PORT_INTSTAT1: u32 = 4;
pub const SDW_CASC_PORT_START_INTSTAT1: u32 = 0;
pub const SDW_CASC_PORT_MASK_INTSTAT1: u32 = 8;
pub const SDW_CASC_PORT_REG_OFFSET_INTSTAT1: u32 = 0;
pub const SDW_NUM_CASC_PORT_INTSTAT2: u32 = 7;
pub const SDW_CASC_PORT_START_INTSTAT2: u32 = 4;
pub const SDW_CASC_PORT_MASK_INTSTAT2: u32 = 1;
pub const SDW_CASC_PORT_REG_OFFSET_INTSTAT2: u32 = 1;
pub const SDW_NUM_CASC_PORT_INTSTAT3: u32 = 4;
pub const SDW_CASC_PORT_START_INTSTAT3: u32 = 11;
pub const SDW_CASC_PORT_MASK_INTSTAT3: u32 = 1;
pub const SDW_CASC_PORT_REG_OFFSET_INTSTAT3: u32 = 2;

#[inline] pub const fn SDW_SDCA_CTL(fun: u32, ent: u32, ctl: u32, ch: u32) -> u32 {
    (1 << 30) | ((fun & 7) << 22) | ((ent & 0x40) << 15) | ((ent & 0x3f) << 7) |
    ((ctl & 0x30) << 15) | ((ctl & 0xf) << 3) | ((ch & 0x38) << 12) | (ch & 7)
}
#[inline] pub const fn SDW_SDCA_CTL_FUNC(reg: u32) -> u32 { (reg >> 22) & 7 }
#[inline] pub const fn SDW_SDCA_CTL_ENT(reg: u32) -> u32 { (((reg >> 21) & 1) << 6) | ((reg >> 7) & 0x3f) }
#[inline] pub const fn SDW_SDCA_CTL_CSEL(reg: u32) -> u32 { (((reg >> 19) & 3) << 4) | ((reg >> 3) & 0xf) }
#[inline] pub const fn SDW_SDCA_CTL_CNUM(reg: u32) -> u32 { (((reg >> 15) & 7) << 3) | (reg & 7) }
#[inline] pub const fn SDW_SDCA_MBQ_CTL(reg: u32) -> u32 { reg | (1 << 13) }
#[inline] pub const fn SDW_SDCA_NEXT_CTL(reg: u32) -> u32 { reg | (1 << 14) }
#[inline] pub const fn SDW_SDCA_VALID_CTL(reg: u32) -> bool { (reg & (0xfe000000 | (1 << 18) | (1 << 13))) == (1 << 30) }
pub const SDW_SDCA_MAX_REGISTER: u32 = 0x47ffffff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
