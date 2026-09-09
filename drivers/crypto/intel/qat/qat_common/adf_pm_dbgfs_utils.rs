// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2025 Intel Corporation */

// Translated from adf_pm_dbgfs_utils.c.
// The declarations below are provided by the corresponding kernel headers and
// implementation units in the surrounding repository.

use core::ffi::{c_char, c_int, c_uint, c_void};

const PM_INFO_MAX_KEY_LEN: usize = 21;

#[repr(C)]
pub struct pm_status_row {
    pub key: *const c_char,
    pub field_mask: u32,
    pub reg_offset: usize,
}

extern "C" {
    fn string_lower(dst: *mut c_char, src: *const c_char);
    fn string_upper(dst: *mut c_char, src: *const c_char);
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
}

#[inline]
unsafe fn field_get(mask: u32, value: u32) -> u32 {
    (value & mask) >> mask.trailing_zeros()
}

unsafe fn pm_scnprint_table(
    buff: *mut c_char,
    table: *const pm_status_row,
    pm_info_regs: *mut u32,
    buff_size: usize,
    table_len: c_int,
    lowercase: bool,
) -> c_int {
    let mut key = [0 as c_char; PM_INFO_MAX_KEY_LEN];
    let mut wr: c_int = 0;
    let mut i: c_int = 0;

    while i < table_len {
        let row = &*table.add(i as usize);

        if lowercase {
            string_lower(key.as_mut_ptr(), row.key);
        } else {
            string_upper(key.as_mut_ptr(), row.key);
        }

        let value = field_get(row.field_mask, *pm_info_regs.add(row.reg_offset));
        let fmt = b"%s: %#x\n\0";
        wr += scnprintf(
            buff.add(wr as usize),
            buff_size - wr as usize,
            fmt.as_ptr() as *const c_char,
            key.as_ptr(),
            value,
        );

        i += 1;
    }

    wr
}

pub unsafe fn adf_pm_scnprint_table_upper_keys(
    buff: *mut c_char,
    table: *const pm_status_row,
    pm_info_regs: *mut u32,
    buff_size: usize,
    table_len: c_int,
) -> c_int {
    pm_scnprint_table(buff, table, pm_info_regs, buff_size, table_len, false)
}

pub unsafe fn adf_pm_scnprint_table_lower_keys(
    buff: *mut c_char,
    table: *const pm_status_row,
    pm_info_regs: *mut u32,
    buff_size: usize,
    table_len: c_int,
) -> c_int {
    pm_scnprint_table(buff, table, pm_info_regs, buff_size, table_len, true)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
