/* SPDX-License-Identifier: GPL-2.0 */
// Translation of BCM63XX_BOARD_H_ declarations.

use core::ffi::c_char;

extern "C" {
    pub fn board_get_name() -> *const c_char;

    pub fn board_prom_init();

    pub fn board_setup();

    pub fn board_register_devices() -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
