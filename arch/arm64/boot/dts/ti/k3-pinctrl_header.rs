/* SPDX-License-Identifier: GPL-2.0-only OR MIT */
/*
 * This header provides constants for pinctrl bindings for TI's K3 SoC
 * family.
 *
 * Copyright (C) 2018-2025 Texas Instruments Incorporated - https://www.ti.com/
 */

pub const WKUP_LVL_EN_SHIFT: u32 = 7;
pub const WKUP_LVL_POL_SHIFT: u32 = 8;
pub const DEBOUNCE_SHIFT: u32 = 11;
pub const ST_EN_SHIFT: u32 = 14;
pub const FORCE_DS_EN_SHIFT: u32 = 15;
pub const PULLUDEN_SHIFT: u32 = 16;
pub const PULLTYPESEL_SHIFT: u32 = 17;
pub const RXACTIVE_SHIFT: u32 = 18;
pub const DRV_STR_SHIFT: u32 = 19;
pub const ISO_OVERRIDE_EN_SHIFT: u32 = 22;
pub const ISO_BYPASS_EN_SHIFT: u32 = 23;
pub const DS_EN_SHIFT: u32 = 24;
pub const DS_OUT_DIS_SHIFT: u32 = 25;
pub const DS_OUT_VAL_SHIFT: u32 = 26;
pub const DS_PULLUD_EN_SHIFT: u32 = 27;
pub const DS_PULLTYPE_SEL_SHIFT: u32 = 28;
pub const WKUP_EN_SHIFT: u32 = 29;

/* Schmitt trigger configuration */
pub const ST_DISABLE: u32 = 0 << ST_EN_SHIFT;
pub const ST_ENABLE: u32 = 1 << ST_EN_SHIFT;

pub const PULL_DISABLE: u32 = 1 << PULLUDEN_SHIFT;
pub const PULL_ENABLE: u32 = 0 << PULLUDEN_SHIFT;

pub const PULL_UP: u32 = (1 << PULLTYPESEL_SHIFT) | PULL_ENABLE;
pub const PULL_DOWN: u32 = (0 << PULLTYPESEL_SHIFT) | PULL_ENABLE;

pub const INPUT_EN: u32 = 1 << RXACTIVE_SHIFT;
pub const INPUT_DISABLE: u32 = 0 << RXACTIVE_SHIFT;

pub const DS_PULL_DISABLE: u32 = 1 << DS_PULLUD_EN_SHIFT;
pub const DS_PULL_ENABLE: u32 = 0 << DS_PULLUD_EN_SHIFT;

pub const DS_PULL_UP: u32 = (1 << DS_PULLTYPE_SEL_SHIFT) | DS_PULL_ENABLE;
pub const DS_PULL_DOWN: u32 = (0 << DS_PULLTYPE_SEL_SHIFT) | DS_PULL_ENABLE;

pub const DS_STATE_EN: u32 = 1 << DS_EN_SHIFT;
pub const DS_STATE_DISABLE: u32 = 0 << DS_EN_SHIFT;

pub const DS_INPUT_EN: u32 = (1 << DS_OUT_DIS_SHIFT) | DS_STATE_EN;
pub const DS_INPUT_DISABLE: u32 = (0 << DS_OUT_DIS_SHIFT) | DS_STATE_EN;

pub const DS_OUT_VALUE_ZERO: u32 = 0 << DS_OUT_VAL_SHIFT;
pub const DS_OUT_VALUE_ONE: u32 = 1 << DS_OUT_VAL_SHIFT;

/* Configuration to enable wake-up on pin activity */
pub const WKUP_ENABLE: u32 = 1 << WKUP_EN_SHIFT;
pub const WKUP_DISABLE: u32 = 0 << WKUP_EN_SHIFT;
pub const WKUP_ON_LEVEL: u32 = 1 << WKUP_LVL_EN_SHIFT;
pub const WKUP_ON_EDGE: u32 = 0 << WKUP_LVL_EN_SHIFT;
pub const WKUP_LEVEL_LOW: u32 = 0 << WKUP_LVL_POL_SHIFT;
pub const WKUP_LEVEL_HIGH: u32 = 1 << WKUP_LVL_POL_SHIFT;

/* Only these macros are expected be used directly in device tree files */
pub const PIN_OUTPUT: u32 = INPUT_DISABLE | PULL_DISABLE;
pub const PIN_OUTPUT_PULLUP: u32 = INPUT_DISABLE | PULL_UP;
pub const PIN_OUTPUT_PULLDOWN: u32 = INPUT_DISABLE | PULL_DOWN;
pub const PIN_INPUT: u32 = INPUT_EN | ST_ENABLE | PULL_DISABLE;
pub const PIN_INPUT_PULLUP: u32 = INPUT_EN | ST_ENABLE | PULL_UP;
pub const PIN_INPUT_PULLDOWN: u32 = INPUT_EN | ST_ENABLE | PULL_DOWN;
/* Input configurations with Schmitt Trigger disabled */
pub const PIN_INPUT_NOST: u32 = INPUT_EN | PULL_DISABLE;
pub const PIN_INPUT_PULLUP_NOST: u32 = INPUT_EN | PULL_UP;
pub const PIN_INPUT_PULLDOWN_NOST: u32 = INPUT_EN | PULL_DOWN;

pub const PIN_DEBOUNCE_DISABLE: u32 = 0 << DEBOUNCE_SHIFT;
pub const PIN_DEBOUNCE_CONF1: u32 = 1 << DEBOUNCE_SHIFT;
pub const PIN_DEBOUNCE_CONF2: u32 = 2 << DEBOUNCE_SHIFT;
pub const PIN_DEBOUNCE_CONF3: u32 = 3 << DEBOUNCE_SHIFT;
pub const PIN_DEBOUNCE_CONF4: u32 = 4 << DEBOUNCE_SHIFT;
pub const PIN_DEBOUNCE_CONF5: u32 = 5 << DEBOUNCE_SHIFT;
pub const PIN_DEBOUNCE_CONF6: u32 = 6 << DEBOUNCE_SHIFT;

pub const PIN_DRIVE_STRENGTH_NOMINAL: u32 = 0 << DRV_STR_SHIFT;
pub const PIN_DRIVE_STRENGTH_SLOW: u32 = 1 << DRV_STR_SHIFT;
pub const PIN_DRIVE_STRENGTH_FAST: u32 = 2 << DRV_STR_SHIFT;

pub const PIN_DS_FORCE_DISABLE: u32 = 0 << FORCE_DS_EN_SHIFT;
pub const PIN_DS_FORCE_ENABLE: u32 = 1 << FORCE_DS_EN_SHIFT;
pub const PIN_DS_ISO_OVERRIDE_DISABLE: u32 = 0 << ISO_OVERRIDE_EN_SHIFT;
pub const PIN_DS_ISO_OVERRIDE_ENABLE: u32 = 1 << ISO_OVERRIDE_EN_SHIFT;
pub const PIN_DS_OUT_ENABLE: u32 = 0 << DS_OUT_DIS_SHIFT;
pub const PIN_DS_OUT_DISABLE: u32 = 1 << DS_OUT_DIS_SHIFT;
pub const PIN_DS_OUT_VALUE_ZERO: u32 = 0 << DS_OUT_VAL_SHIFT;
pub const PIN_DS_OUT_VALUE_ONE: u32 = 1 << DS_OUT_VAL_SHIFT;
pub const PIN_DS_PULLUD_ENABLE: u32 = 0 << DS_PULLUD_EN_SHIFT;
pub const PIN_DS_PULLUD_DISABLE: u32 = 1 << DS_PULLUD_EN_SHIFT;
pub const PIN_DS_PULL_DOWN: u32 = 0 << DS_PULLTYPE_SEL_SHIFT;
pub const PIN_DS_PULL_UP: u32 = 1 << DS_PULLTYPE_SEL_SHIFT;
pub const PIN_DS_ISO_BYPASS: u32 = 1 << ISO_BYPASS_EN_SHIFT;
pub const PIN_DS_ISO_BYPASS_DISABLE: u32 = 0 << ISO_BYPASS_EN_SHIFT;

pub const PIN_DS_OUTPUT_LOW: u32 = DS_INPUT_DISABLE | DS_OUT_VALUE_ZERO;
pub const PIN_DS_OUTPUT_HIGH: u32 = DS_INPUT_DISABLE | DS_OUT_VALUE_ONE;
pub const PIN_DS_INPUT: u32 = DS_INPUT_EN | PULL_DISABLE;
pub const PIN_DS_INPUT_PULLUP: u32 = DS_INPUT_EN | DS_PULL_UP;
pub const PIN_DS_INPUT_PULLDOWN: u32 = DS_INPUT_EN | DS_PULL_DOWN;

pub const PIN_WKUP_EN_LEVEL_LOW: u32 = WKUP_ENABLE | WKUP_ON_LEVEL | WKUP_LEVEL_LOW;
pub const PIN_WKUP_EN_LEVEL_HIGH: u32 = WKUP_ENABLE | WKUP_ON_LEVEL | WKUP_LEVEL_HIGH;
pub const PIN_WKUP_EN: u32 = WKUP_ENABLE | WKUP_ON_EDGE;

/* Default mux configuration for gpio-ranges to use with pinctrl */
pub const PIN_GPIO_RANGE_IOPAD: u32 = PIN_INPUT | 7;

/* These C macros expand to two device-tree cells: the offset and configuration. */
macro_rules! iopad {
    ($pa:expr, $val:expr, $muxmode:expr) => {
        (($pa) & 0x1fff, ($val) | ($muxmode))
    };
}

pub use iopad as AM62AX_IOPAD;
pub use iopad as AM62AX_MCU_IOPAD;
pub use iopad as AM62DX_IOPAD;
pub use iopad as AM62DX_MCU_IOPAD;
pub use iopad as AM62PX_IOPAD;
pub use iopad as AM62PX_MCU_IOPAD;
pub use iopad as AM62LX_IOPAD;
pub use iopad as AM62X_IOPAD;
pub use iopad as AM62X_MCU_IOPAD;
pub use iopad as AM64X_IOPAD;
pub use iopad as AM64X_MCU_IOPAD;
pub use iopad as AM65X_IOPAD;
pub use iopad as AM65X_WKUP_IOPAD;
pub use iopad as J721E_IOPAD;
pub use iopad as J721E_WKUP_IOPAD;
pub use iopad as J721S2_IOPAD;
pub use iopad as J721S2_WKUP_IOPAD;
pub use iopad as J722S_IOPAD;
pub use iopad as J722S_MCU_IOPAD;
pub use iopad as J784S4_IOPAD;
pub use iopad as J784S4_WKUP_IOPAD;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
