/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides constants for OMAP pinctrl bindings.
 *
 * Copyright (C) 2009 Nokia
 * Copyright (C) 2009-2010 Texas Instruments
 */

/* 34xx mux mode options for each pin. See TRM for options */
pub const MUX_MODE0: u32 = 0;
pub const MUX_MODE1: u32 = 1;
pub const MUX_MODE2: u32 = 2;
pub const MUX_MODE3: u32 = 3;
pub const MUX_MODE4: u32 = 4;
pub const MUX_MODE5: u32 = 5;
pub const MUX_MODE6: u32 = 6;
pub const MUX_MODE7: u32 = 7;

/* 24xx/34xx mux bit defines */
pub const PULL_ENA: u32 = 1 << 3;
pub const PULL_UP: u32 = 1 << 4;
pub const ALTELECTRICALSEL: u32 = 1 << 5;

/* omap3/4/5 specific mux bit defines */
pub const INPUT_EN: u32 = 1 << 8;
pub const OFF_EN: u32 = 1 << 9;
pub const OFFOUT_EN: u32 = 1 << 10;
pub const OFFOUT_VAL: u32 = 1 << 11;
pub const OFF_PULL_EN: u32 = 1 << 12;
pub const OFF_PULL_UP: u32 = 1 << 13;
pub const WAKEUP_EN: u32 = 1 << 14;
pub const WAKEUP_EVENT: u32 = 1 << 15;

/* Active pin states */
pub const PIN_OUTPUT: u32 = 0;
pub const PIN_OUTPUT_PULLUP: u32 = PIN_OUTPUT | PULL_ENA | PULL_UP;
pub const PIN_OUTPUT_PULLDOWN: u32 = PIN_OUTPUT | PULL_ENA;
pub const PIN_INPUT: u32 = INPUT_EN;
pub const PIN_INPUT_PULLUP: u32 = PULL_ENA | INPUT_EN | PULL_UP;
pub const PIN_INPUT_PULLDOWN: u32 = PULL_ENA | INPUT_EN;

/* Off mode states */
pub const PIN_OFF_NONE: u32 = 0;
pub const PIN_OFF_OUTPUT_HIGH: u32 = OFF_EN | OFFOUT_EN | OFFOUT_VAL;
pub const PIN_OFF_OUTPUT_LOW: u32 = OFF_EN | OFFOUT_EN;
pub const PIN_OFF_INPUT_PULLUP: u32 = OFF_EN | OFFOUT_EN | OFF_PULL_EN | OFF_PULL_UP;
pub const PIN_OFF_INPUT_PULLDOWN: u32 = OFF_EN | OFFOUT_EN | OFF_PULL_EN;
pub const PIN_OFF_WAKEUPENABLE: u32 = WAKEUP_EN;

/*
 * Macros to allow using the absolute physical address instead of the
 * padconf registers instead of the offset from padconf base.
 */
pub const fn omap_iopad_offset(pa: u32, offset: u32) -> u32 {
    (pa & 0xffff).wrapping_sub(offset)
}

pub const fn OMAP2420_CORE_IOPAD(pa: u32, val: u32) -> (u32, u32) { (omap_iopad_offset(pa, 0x0030), val) }
pub const fn OMAP2430_CORE_IOPAD(pa: u32, val: u32) -> (u32, u32) { (omap_iopad_offset(pa, 0x2030), val) }
pub const fn OMAP3_CORE1_IOPAD(pa: u32, val: u32) -> (u32, u32) { (omap_iopad_offset(pa, 0x2030), val) }
pub const fn OMAP3430_CORE2_IOPAD(pa: u32, val: u32) -> (u32, u32) { (omap_iopad_offset(pa, 0x25d8), val) }
pub const fn OMAP3630_CORE2_IOPAD(pa: u32, val: u32) -> (u32, u32) { (omap_iopad_offset(pa, 0x25a0), val) }
pub const fn OMAP3_WKUP_IOPAD(pa: u32, val: u32) -> (u32, u32) { (omap_iopad_offset(pa, 0x2a00), val) }
pub const fn DM814X_IOPAD(pa: u32, val: u32) -> (u32, u32) { (omap_iopad_offset(pa, 0x0800), val) }
pub const fn DM816X_IOPAD(pa: u32, val: u32) -> (u32, u32) { (omap_iopad_offset(pa, 0x0800), val) }
pub const fn AM33XX_IOPAD(pa: u32, val: u32) -> (u32, u32, u32) { (omap_iopad_offset(pa, 0x0800), val, 0) }
pub const fn AM33XX_PADCONF(pa: u32, conf: u32, mux: u32) -> (u32, u32, u32) {
    (omap_iopad_offset(pa, 0x0800), conf, mux)
}

/*
 * Macros to allow using the offset from the padconf physical address
 * instead  of the offset from padconf base.
 */
pub const fn omap_padconf_offset(offset: u32, base_offset: u32) -> u32 {
    offset.wrapping_sub(base_offset)
}

pub const fn OMAP4_IOPAD(offset: u32, val: u32) -> (u32, u32) { (omap_padconf_offset(offset, 0x0040), val) }
pub const fn OMAP5_IOPAD(offset: u32, val: u32) -> (u32, u32) { (omap_padconf_offset(offset, 0x0040), val) }

/* Define some commonly used pins configured by the boards. */
pub const OMAP3_UART1_RX: u32 = 0x152;
pub const OMAP3_UART2_RX: u32 = 0x14a;
pub const OMAP3_UART3_RX: u32 = 0x16e;
pub const OMAP4_UART2_RX: u32 = 0xdc;
pub const OMAP4_UART3_RX: u32 = 0x104;
pub const OMAP4_UART4_RX: u32 = 0x11c;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
