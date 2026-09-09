/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides constants specific to AM43XX pinctrl bindings.
 */

pub const MUX_MODE0: i32 = 0;
pub const MUX_MODE1: i32 = 1;
pub const MUX_MODE2: i32 = 2;
pub const MUX_MODE3: i32 = 3;
pub const MUX_MODE4: i32 = 4;
pub const MUX_MODE5: i32 = 5;
pub const MUX_MODE6: i32 = 6;
pub const MUX_MODE7: i32 = 7;
pub const MUX_MODE8: i32 = 8;
pub const MUX_MODE9: i32 = 9;

pub const PULL_DISABLE: i32 = 1 << 16;
pub const PULL_UP: i32 = 1 << 17;
pub const INPUT_EN: i32 = 1 << 18;
pub const SLEWCTRL_SLOW: i32 = 1 << 19;
pub const SLEWCTRL_FAST: i32 = 0;
pub const DS0_FORCE_OFF_MODE: i32 = 1 << 24;
pub const DS0_INPUT: i32 = 1 << 25;
pub const DS0_FORCE_OUT_HIGH: i32 = 1 << 26;
pub const DS0_PULL_UP_DOWN_EN: i32 = 0 << 27;
pub const DS0_PULL_UP_DOWN_DIS: i32 = 1 << 27;
pub const DS0_PULL_UP_SEL: i32 = 1 << 28;
pub const WAKEUP_ENABLE: i32 = 1 << 29;

pub const DS0_PIN_OUTPUT: i32 = DS0_FORCE_OFF_MODE;
pub const DS0_PIN_OUTPUT_HIGH: i32 = DS0_FORCE_OFF_MODE | DS0_FORCE_OUT_HIGH;
pub const DS0_PIN_OUTPUT_PULLUP: i32 =
    DS0_FORCE_OFF_MODE | DS0_PULL_UP_DOWN_EN | DS0_PULL_UP_SEL;
pub const DS0_PIN_OUTPUT_PULLDOWN: i32 = DS0_FORCE_OFF_MODE | DS0_PULL_UP_DOWN_EN;
pub const DS0_PIN_INPUT: i32 = DS0_FORCE_OFF_MODE | DS0_INPUT;
pub const DS0_PIN_INPUT_PULLUP: i32 =
    DS0_FORCE_OFF_MODE | DS0_INPUT | DS0_PULL_UP_DOWN_EN | DS0_PULL_UP_SEL;
pub const DS0_PIN_INPUT_PULLDOWN: i32 = DS0_FORCE_OFF_MODE | DS0_INPUT | DS0_PULL_UP_DOWN_EN;

pub const PIN_OUTPUT: i32 = PULL_DISABLE;
pub const PIN_OUTPUT_PULLUP: i32 = PULL_UP;
pub const PIN_OUTPUT_PULLDOWN: i32 = 0;
pub const PIN_INPUT: i32 = INPUT_EN | PULL_DISABLE;
pub const PIN_INPUT_PULLUP: i32 = INPUT_EN | PULL_UP;
pub const PIN_INPUT_PULLDOWN: i32 = INPUT_EN;

/*
 * Macro to allow using the absolute physical address instead of the
 * padconf registers instead of the offset from padconf base.
 *
 * The original macro expands to (((pa) & 0xffff) - 0x0800) (val), retaining
 * its call-like expression form here for source-level fidelity.
 */
#[allow(unused_macros)]
macro_rules! AM4372_IOPAD {
    ($pa:expr, $val:expr) => {
        ((($pa) & 0xffff) - 0x0800) ($val)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
