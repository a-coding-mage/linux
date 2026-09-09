/* SPDX-License-Identifier: GPL-2.0 */

pub const RESET_STATUS_HARDWARE: u32 = 1 << 0; // Hardware Reset
pub const RESET_STATUS_WATCHDOG: u32 = 1 << 1; // Watchdog Reset
pub const RESET_STATUS_LOWPOWER: u32 = 1 << 2; // Low Power/Sleep Exit
pub const RESET_STATUS_GPIO: u32 = 1 << 3; // GPIO Reset
pub const RESET_STATUS_ALL: u32 = 0xf;

unsafe extern "C" {
    pub fn clear_reset_status(mask: u32);
    pub fn pxa_register_wdt(reset_status: u32);

    /**
     * init_gpio_reset() - register GPIO as reset generator
     * @gpio: gpio nr
     * @output: set gpio as output instead of input during normal work
     * @level: output level
     */
    pub fn init_gpio_reset(gpio: i32, output: i32, level: i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
