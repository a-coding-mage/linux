/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * This header provides constants for DRA pinctrl bindings.
 *
 * Copyright (C) 2013 Texas Instruments Incorporated - http://www.ti.com/
 * Author: Rajendra Nayak <rnayak@ti.com>
 */

/* DRA7 mux mode options for each pin. See TRM for options */
pub const MUX_MODE0: u32 = 0x0;
pub const MUX_MODE1: u32 = 0x1;
pub const MUX_MODE2: u32 = 0x2;
pub const MUX_MODE3: u32 = 0x3;
pub const MUX_MODE4: u32 = 0x4;
pub const MUX_MODE5: u32 = 0x5;
pub const MUX_MODE6: u32 = 0x6;
pub const MUX_MODE7: u32 = 0x7;
pub const MUX_MODE8: u32 = 0x8;
pub const MUX_MODE9: u32 = 0x9;
pub const MUX_MODE10: u32 = 0xa;
pub const MUX_MODE11: u32 = 0xb;
pub const MUX_MODE12: u32 = 0xc;
pub const MUX_MODE13: u32 = 0xd;
pub const MUX_MODE14: u32 = 0xe;
pub const MUX_MODE15: u32 = 0xf;

/* Certain pins need virtual mode, but note: they may glitch */
pub const MUX_VIRTUAL_MODE0: u32 = MODE_SELECT | (0x0 << 4);
pub const MUX_VIRTUAL_MODE1: u32 = MODE_SELECT | (0x1 << 4);
pub const MUX_VIRTUAL_MODE2: u32 = MODE_SELECT | (0x2 << 4);
pub const MUX_VIRTUAL_MODE3: u32 = MODE_SELECT | (0x3 << 4);
pub const MUX_VIRTUAL_MODE4: u32 = MODE_SELECT | (0x4 << 4);
pub const MUX_VIRTUAL_MODE5: u32 = MODE_SELECT | (0x5 << 4);
pub const MUX_VIRTUAL_MODE6: u32 = MODE_SELECT | (0x6 << 4);
pub const MUX_VIRTUAL_MODE7: u32 = MODE_SELECT | (0x7 << 4);
pub const MUX_VIRTUAL_MODE8: u32 = MODE_SELECT | (0x8 << 4);
pub const MUX_VIRTUAL_MODE9: u32 = MODE_SELECT | (0x9 << 4);
pub const MUX_VIRTUAL_MODE10: u32 = MODE_SELECT | (0xa << 4);
pub const MUX_VIRTUAL_MODE11: u32 = MODE_SELECT | (0xb << 4);
pub const MUX_VIRTUAL_MODE12: u32 = MODE_SELECT | (0xc << 4);
pub const MUX_VIRTUAL_MODE13: u32 = MODE_SELECT | (0xd << 4);
pub const MUX_VIRTUAL_MODE14: u32 = MODE_SELECT | (0xe << 4);
pub const MUX_VIRTUAL_MODE15: u32 = MODE_SELECT | (0xf << 4);

pub const MODE_SELECT: u32 = 1 << 8;

pub const PULL_ENA: u32 = 0 << 16;
pub const PULL_DIS: u32 = 1 << 16;
pub const PULL_UP: u32 = 1 << 17;
pub const INPUT_EN: u32 = 1 << 18;
pub const SLEWCONTROL: u32 = 1 << 19;
pub const WAKEUP_EN: u32 = 1 << 24;
pub const WAKEUP_EVENT: u32 = 1 << 25;

/* Active pin states */
pub const PIN_OUTPUT: u32 = 0 | PULL_DIS;
pub const PIN_OUTPUT_PULLUP: u32 = PULL_UP;
pub const PIN_OUTPUT_PULLDOWN: u32 = 0;
pub const PIN_INPUT: u32 = INPUT_EN | PULL_DIS;
pub const PIN_INPUT_SLEW: u32 = INPUT_EN | SLEWCONTROL;
pub const PIN_INPUT_PULLUP: u32 = PULL_ENA | INPUT_EN | PULL_UP;
pub const PIN_INPUT_PULLDOWN: u32 = PULL_ENA | INPUT_EN;

/*
 * Macro to allow using the absolute physical address instead of the
 * padconf registers instead of the offset from padconf base.
 */
#[macro_export]
macro_rules! DRA7XX_CORE_IOPAD {
    ($pa:expr, $val:expr) => {
        ((($pa) & 0xffff) - 0x3400)($val)
    };
}

/* DRA7 IODELAY configuration parameters */
#[macro_export]
macro_rules! A_DELAY_PS {
    ($val:expr) => { (($val) & 0xffff) };
}

#[macro_export]
macro_rules! G_DELAY_PS {
    ($val:expr) => { (($val) & 0xffff) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
