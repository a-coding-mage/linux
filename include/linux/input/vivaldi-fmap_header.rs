/* SPDX-License-Identifier: GPL-2.0 */

// Header guard: _VIVALDI_FMAP_H
// Dependency: linux/types.h

pub const VIVALDI_MAX_FUNCTION_ROW_KEYS: usize = 24;

/**
 * struct vivaldi_data - Function row map data for ChromeOS Vivaldi keyboards
 * @function_row_physmap: An array of scancodes or their equivalent (HID usage
 *                        codes, encoded rows/columns, etc) for the top
 *                        row function keys, in an order from left to right
 * @num_function_row_keys: The number of top row keys in a custom keyboard
 *
 * This structure is supposed to be used by ChromeOS keyboards using
 * the Vivaldi keyboard function row design.
 */
#[repr(C)]
pub struct vivaldi_data {
    pub function_row_physmap: [u32; VIVALDI_MAX_FUNCTION_ROW_KEYS],
    pub num_function_row_keys: core::ffi::c_uint,
}

unsafe extern "C" {
    pub fn vivaldi_function_row_physmap_show(
        data: *const vivaldi_data,
        buf: *mut core::ffi::c_char,
    ) -> isize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
