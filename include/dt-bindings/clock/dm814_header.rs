/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2017 Texas Instruments, Inc.
 */

pub const DM814_CLKCTRL_OFFSET: u32 = 0x0;
pub const fn dm814_clkctrl_index(offset: u32) -> u32 {
    offset - DM814_CLKCTRL_OFFSET
}

/* default clocks */
pub const DM814_USB_OTG_HS_CLKCTRL: u32 = dm814_clkctrl_index(0x58);

/* alwon clocks */
pub const DM814_UART1_CLKCTRL: u32 = dm814_clkctrl_index(0x150);
pub const DM814_UART2_CLKCTRL: u32 = dm814_clkctrl_index(0x154);
pub const DM814_UART3_CLKCTRL: u32 = dm814_clkctrl_index(0x158);
pub const DM814_GPIO1_CLKCTRL: u32 = dm814_clkctrl_index(0x15c);
pub const DM814_GPIO2_CLKCTRL: u32 = dm814_clkctrl_index(0x160);
pub const DM814_I2C1_CLKCTRL: u32 = dm814_clkctrl_index(0x164);
pub const DM814_I2C2_CLKCTRL: u32 = dm814_clkctrl_index(0x168);
pub const DM814_WD_TIMER_CLKCTRL: u32 = dm814_clkctrl_index(0x18c);
pub const DM814_MCSPI1_CLKCTRL: u32 = dm814_clkctrl_index(0x190);
pub const DM814_GPMC_CLKCTRL: u32 = dm814_clkctrl_index(0x1d0);
pub const DM814_CPGMAC0_CLKCTRL: u32 = dm814_clkctrl_index(0x1d4);
pub const DM814_MPU_CLKCTRL: u32 = dm814_clkctrl_index(0x1dc);
pub const DM814_RTC_CLKCTRL: u32 = dm814_clkctrl_index(0x1f0);
pub const DM814_TPCC_CLKCTRL: u32 = dm814_clkctrl_index(0x1f4);
pub const DM814_TPTC0_CLKCTRL: u32 = dm814_clkctrl_index(0x1f8);
pub const DM814_TPTC1_CLKCTRL: u32 = dm814_clkctrl_index(0x1fc);
pub const DM814_TPTC2_CLKCTRL: u32 = dm814_clkctrl_index(0x200);
pub const DM814_TPTC3_CLKCTRL: u32 = dm814_clkctrl_index(0x204);
pub const DM814_MMC1_CLKCTRL: u32 = dm814_clkctrl_index(0x21c);
pub const DM814_MMC2_CLKCTRL: u32 = dm814_clkctrl_index(0x220);
pub const DM814_MMC3_CLKCTRL: u32 = dm814_clkctrl_index(0x224);

/* alwon_ethernet clocks */
pub const DM814_ETHERNET_CLKCTRL_OFFSET: u32 = 0x1d4;
pub const fn dm814_ethernet_clkctrl_index(offset: u32) -> u32 {
    offset - DM814_ETHERNET_CLKCTRL_OFFSET
}
pub const DM814_ETHERNET_CPGMAC0_CLKCTRL: u32 =
    dm814_ethernet_clkctrl_index(0x1d4);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
