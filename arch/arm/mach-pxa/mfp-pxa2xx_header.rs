/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: definitions from <linux/soc/pxa/mfp.h> are supplied externally.

/*
 * The following MFP_xxx bit definitions in mfp.h are re-used for pxa2xx:
 * MFP_PIN(x), MFP_AFx, MFP_LPM_DRIVE_{LOW, HIGH}, and MFP_LPM_EDGE_x.
 * Other MFP_x bit definitions are ignored.  PXA2xx adds bits 23, 24, and 25.
 */

pub const MFP_DIR_IN: u32 = 0x0 << 23;
pub const MFP_DIR_OUT: u32 = 0x1 << 23;
pub const MFP_DIR_MASK: u32 = 0x1 << 23;

#[inline]
pub const fn MFP_DIR(x: u32) -> u32 {
    (x >> 23) & 0x1
}

pub const MFP_LPM_CAN_WAKEUP: u32 = 0x1 << 24;

/* MFP_LPM_KEEP_OUTPUT retains the last output level; it has no effect on inputs. */
pub const MFP_LPM_KEEP_OUTPUT: u32 = 0x1 << 25;

// MFP_LPM_EDGE_* and MFP_CFG_DEFAULT are supplied by mfp.h.
#[macro_export]
macro_rules! WAKEUP_ON_EDGE_RISE { () => { MFP_LPM_CAN_WAKEUP | MFP_LPM_EDGE_RISE }; }
#[macro_export]
macro_rules! WAKEUP_ON_EDGE_FALL { () => { MFP_LPM_CAN_WAKEUP | MFP_LPM_EDGE_FALL }; }
#[macro_export]
macro_rules! WAKEUP_ON_EDGE_BOTH { () => { MFP_LPM_CAN_WAKEUP | MFP_LPM_EDGE_BOTH }; }

/* Specifically for enabling wakeup on keypad GPIOs. */
#[macro_export]
macro_rules! WAKEUP_ON_LEVEL_HIGH { () => { MFP_LPM_CAN_WAKEUP }; }

#[macro_export]
macro_rules! MFP_CFG_IN {
    ($pin:expr, $af:expr) => {
        (MFP_CFG_DEFAULT & !(MFP_AF_MASK | MFP_DIR_MASK)) |
        (MFP_PIN($pin) | $af | MFP_DIR_IN)
    };
}

/* Pins configured as output must provide a low-power state. */
#[macro_export]
macro_rules! MFP_CFG_OUT {
    ($pin:expr, $af:expr, $state:expr) => {
        (MFP_CFG_DEFAULT & !(MFP_AF_MASK | MFP_DIR_MASK | MFP_LPM_STATE_MASK)) |
        (MFP_PIN($pin) | $af | MFP_DIR_OUT | $state)
    };
}

/* Common configurations for pxa25x and pxa27x. GPIOs initialize as inputs. */

#[macro_export] macro_rules! GPIO0_GPIO { () => { MFP_CFG_IN!(MFP_PIN_GPIO0, MFP_AF0) }; }
#[macro_export] macro_rules! GPIO1_GPIO { () => { MFP_CFG_IN!(MFP_PIN_GPIO1, MFP_AF0) }; }

/* Generate the remaining literal GPIO configuration macros. */
#[macro_export]
macro_rules! pxa2xx_gpio_configs {
    ($($name:ident, $pin:ident),* $(,)?) => {
        $(#[macro_export] macro_rules! $name { () => { MFP_CFG_IN!($pin, MFP_AF0) }; })*
    };
}

pxa2xx_gpio_configs!(
    GPIO9_GPIO, MFP_PIN_GPIO9, GPIO10_GPIO, MFP_PIN_GPIO10,
    GPIO11_GPIO, MFP_PIN_GPIO11, GPIO12_GPIO, MFP_PIN_GPIO12,
    GPIO13_GPIO, MFP_PIN_GPIO13, GPIO14_GPIO, MFP_PIN_GPIO14,
    GPIO15_GPIO, MFP_PIN_GPIO15, GPIO16_GPIO, MFP_PIN_GPIO16,
    GPIO17_GPIO, MFP_PIN_GPIO17, GPIO18_GPIO, MFP_PIN_GPIO18,
    GPIO19_GPIO, MFP_PIN_GPIO19, GPIO20_GPIO, MFP_PIN_GPIO20,
    GPIO21_GPIO, MFP_PIN_GPIO21, GPIO22_GPIO, MFP_PIN_GPIO22,
    GPIO23_GPIO, MFP_PIN_GPIO23, GPIO24_GPIO, MFP_PIN_GPIO24,
    GPIO25_GPIO, MFP_PIN_GPIO25, GPIO26_GPIO, MFP_PIN_GPIO26,
    GPIO27_GPIO, MFP_PIN_GPIO27, GPIO28_GPIO, MFP_PIN_GPIO28,
    GPIO29_GPIO, MFP_PIN_GPIO29, GPIO30_GPIO, MFP_PIN_GPIO30,
    GPIO31_GPIO, MFP_PIN_GPIO31, GPIO32_GPIO, MFP_PIN_GPIO32,
    GPIO33_GPIO, MFP_PIN_GPIO33, GPIO34_GPIO, MFP_PIN_GPIO34,
    GPIO35_GPIO, MFP_PIN_GPIO35, GPIO36_GPIO, MFP_PIN_GPIO36,
    GPIO37_GPIO, MFP_PIN_GPIO37, GPIO38_GPIO, MFP_PIN_GPIO38,
    GPIO39_GPIO, MFP_PIN_GPIO39, GPIO40_GPIO, MFP_PIN_GPIO40,
    GPIO41_GPIO, MFP_PIN_GPIO41, GPIO42_GPIO, MFP_PIN_GPIO42,
    GPIO43_GPIO, MFP_PIN_GPIO43, GPIO44_GPIO, MFP_PIN_GPIO44,
    GPIO45_GPIO, MFP_PIN_GPIO45, GPIO46_GPIO, MFP_PIN_GPIO46,
    GPIO47_GPIO, MFP_PIN_GPIO47, GPIO48_GPIO, MFP_PIN_GPIO48,
    GPIO49_GPIO, MFP_PIN_GPIO49, GPIO50_GPIO, MFP_PIN_GPIO50,
    GPIO51_GPIO, MFP_PIN_GPIO51, GPIO52_GPIO, MFP_PIN_GPIO52,
    GPIO53_GPIO, MFP_PIN_GPIO53, GPIO54_GPIO, MFP_PIN_GPIO54,
    GPIO55_GPIO, MFP_PIN_GPIO55, GPIO56_GPIO, MFP_PIN_GPIO56,
    GPIO57_GPIO, MFP_PIN_GPIO57, GPIO58_GPIO, MFP_PIN_GPIO58,
    GPIO59_GPIO, MFP_PIN_GPIO59, GPIO60_GPIO, MFP_PIN_GPIO60,
    GPIO61_GPIO, MFP_PIN_GPIO61, GPIO62_GPIO, MFP_PIN_GPIO62,
    GPIO63_GPIO, MFP_PIN_GPIO63, GPIO64_GPIO, MFP_PIN_GPIO64,
    GPIO65_GPIO, MFP_PIN_GPIO65, GPIO66_GPIO, MFP_PIN_GPIO66,
    GPIO67_GPIO, MFP_PIN_GPIO67, GPIO68_GPIO, MFP_PIN_GPIO68,
    GPIO69_GPIO, MFP_PIN_GPIO69, GPIO70_GPIO, MFP_PIN_GPIO70,
    GPIO71_GPIO, MFP_PIN_GPIO71, GPIO72_GPIO, MFP_PIN_GPIO72,
    GPIO73_GPIO, MFP_PIN_GPIO73, GPIO74_GPIO, MFP_PIN_GPIO74,
    GPIO75_GPIO, MFP_PIN_GPIO75, GPIO76_GPIO, MFP_PIN_GPIO76,
    GPIO77_GPIO, MFP_PIN_GPIO77, GPIO78_GPIO, MFP_PIN_GPIO78,
    GPIO79_GPIO, MFP_PIN_GPIO79, GPIO80_GPIO, MFP_PIN_GPIO80,
    GPIO81_GPIO, MFP_PIN_GPIO81, GPIO82_GPIO, MFP_PIN_GPIO82,
    GPIO83_GPIO, MFP_PIN_GPIO83, GPIO84_GPIO, MFP_PIN_GPIO84,
);

unsafe extern "C" {
    pub fn pxa2xx_mfp_config(mfp_cfgs: *mut libc::c_ulong, num: libc::c_int);
    pub fn pxa2xx_mfp_set_lpm(mfp: libc::c_int, lpm: libc::c_ulong);
    pub fn gpio_set_wake(gpio: libc::c_uint, on: libc::c_uint) -> libc::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
