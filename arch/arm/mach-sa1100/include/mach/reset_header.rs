/* SPDX-License-Identifier: GPL-2.0 */

// `hardware.h` supplies the reset control/status register `RCSR`.

pub const RESET_STATUS_HARDWARE: u32 = 1 << 0; // Hardware Reset
pub const RESET_STATUS_WATCHDOG: u32 = 1 << 1; // Watchdog Reset
pub const RESET_STATUS_LOWPOWER: u32 = 1 << 2; // Exit from Low Power/Sleep
pub const RESET_STATUS_GPIO: u32 = 1 << 3; // GPIO Reset
pub const RESET_STATUS_ALL: u32 = 0xf;

unsafe extern "C" {
    static mut RCSR: u32;
}

#[inline]
pub unsafe fn clear_reset_status(mask: u32) {
    RCSR = mask;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
