/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the Linux types layer.

pub const MATRIX_MAX_ROWS: u32 = 32;
pub const MATRIX_MAX_COLS: u32 = 32;

#[inline]
pub const fn key(row: u32, col: u32, val: u32) -> u32 {
    (((row & (MATRIX_MAX_ROWS - 1)) << 24)
        | ((col & (MATRIX_MAX_COLS - 1)) << 16)
        | (val & 0xffff))
}

#[inline]
pub const fn key_row(k: u32) -> u32 {
    (k >> 24) & 0xff
}

#[inline]
pub const fn key_col(k: u32) -> u32 {
    (k >> 16) & 0xff
}

#[inline]
pub const fn key_val(k: u32) -> u32 {
    k & 0xffff
}

#[inline]
pub const fn matrix_scan_code(row: u32, col: u32, row_shift: u32) -> u32 {
    (row << row_shift).wrapping_add(col)
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct input_dev {
    _private: [u8; 0],
}

/**
 * struct matrix_keymap_data - keymap for matrix keyboards
 * @keymap: pointer to array of uint32 values encoded with KEY() macro
 *	representing keymap
 * @keymap_size: number of entries (initialized) in this keymap
 *
 * This structure is supposed to be used by platform code to supply
 * keymaps to drivers that implement matrix-like keypads/keyboards.
 */
#[repr(C)]
pub struct matrix_keymap_data {
    pub keymap: *const u32,
    pub keymap_size: core::ffi::c_uint,
}

unsafe extern "C" {
    pub fn matrix_keypad_build_keymap(
        keymap_data: *const matrix_keymap_data,
        keymap_name: *const core::ffi::c_char,
        rows: core::ffi::c_uint,
        cols: core::ffi::c_uint,
        keymap: *mut core::ffi::c_ushort,
        input_dev: *mut input_dev,
    ) -> core::ffi::c_int;

    pub fn matrix_keypad_parse_properties(
        dev: *mut device,
        rows: *mut core::ffi::c_uint,
        cols: *mut core::ffi::c_uint,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
