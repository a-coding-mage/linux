/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Header file for Intel Broxton Whiskey Cove PMIC
 *
 * Copyright (C) 2015 Intel Corporation. All rights reserved.
 */

/* BXT WC devices */
pub const BXTWC_DEVICE1_ADDR: u16 = 0x4E;
pub const BXTWC_DEVICE2_ADDR: u16 = 0x4F;
pub const BXTWC_DEVICE3_ADDR: u16 = 0x5E;

/* device1 Registers */
pub const BXTWC_CHIPID: u16 = 0x4E00;
pub const BXTWC_CHIPVER: u16 = 0x4E01;

pub const BXTWC_SCHGRIRQ0_ADDR: u16 = 0x5E1A;
pub const BXTWC_CHGRCTRL0_ADDR: u16 = 0x5E16;
pub const BXTWC_CHGRCTRL1_ADDR: u16 = 0x5E17;
pub const BXTWC_CHGRCTRL2_ADDR: u16 = 0x5E18;
pub const BXTWC_CHGRSTATUS_ADDR: u16 = 0x5E19;
pub const BXTWC_THRMBATZONE_ADDR: u16 = 0x4F22;

pub const BXTWC_USBPATH_ADDR: u16 = 0x5E19;
pub const BXTWC_USBPHYCTRL_ADDR: u16 = 0x5E07;
pub const BXTWC_USBIDCTRL_ADDR: u16 = 0x5E05;
pub const BXTWC_USBIDEN_MASK: u16 = 0x01;
pub const BXTWC_USBIDSTAT_ADDR: u16 = 0x00FF;
pub const BXTWC_USBSRCDETSTATUS_ADDR: u16 = 0x5E29;

pub const BXTWC_DBGUSBBC1_ADDR: u16 = 0x5FE0;
pub const BXTWC_DBGUSBBC2_ADDR: u16 = 0x5FE1;
pub const BXTWC_DBGUSBBCSTAT_ADDR: u16 = 0x5FE2;

pub const BXTWC_WAKESRC_ADDR: u16 = 0x4E22;
pub const BXTWC_WAKESRC2_ADDR: u16 = 0x4EE5;
pub const BXTWC_CHRTTADDR_ADDR: u16 = 0x5E22;
pub const BXTWC_CHRTTDATA_ADDR: u16 = 0x5E23;

pub const BXTWC_STHRMIRQ0_ADDR: u16 = 0x4F19;
pub const WC_MTHRMIRQ1_ADDR: u16 = 0x4E12;
pub const WC_STHRMIRQ1_ADDR: u16 = 0x4F1A;
pub const WC_STHRMIRQ2_ADDR: u16 = 0x4F1B;

pub const BXTWC_THRMZN0H_ADDR: u16 = 0x4F44;
pub const BXTWC_THRMZN0L_ADDR: u16 = 0x4F45;
pub const BXTWC_THRMZN1H_ADDR: u16 = 0x4F46;
pub const BXTWC_THRMZN1L_ADDR: u16 = 0x4F47;
pub const BXTWC_THRMZN2H_ADDR: u16 = 0x4F48;
pub const BXTWC_THRMZN2L_ADDR: u16 = 0x4F49;
pub const BXTWC_THRMZN3H_ADDR: u16 = 0x4F4A;
pub const BXTWC_THRMZN3L_ADDR: u16 = 0x4F4B;
pub const BXTWC_THRMZN4H_ADDR: u16 = 0x4F4C;
pub const BXTWC_THRMZN4L_ADDR: u16 = 0x4F4D;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
