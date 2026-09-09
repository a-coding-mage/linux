/* SPDX-License-Identifier: GPL-2.0  */
/*
 * Copyright (C) STMicroelectronics 2022 - All Rights Reserved
 * Author: Gabriel Fernandez <gabriel.fernandez@foss.st.com> for STMicroelectronics.
 */

#[repr(C)]
pub struct stm32_reset_cfg {
    pub offset: u16,
    pub bit_idx: u8,
    pub set_clr: bool,
}

#[repr(C)]
pub struct clk_stm32_reset_data {
    pub ops: *const reset_control_ops,
    pub reset_lines: *mut *const stm32_reset_cfg,
    pub nr_lines: core::ffi::c_uint,
    pub clear_offset: u32,
}

extern "C" {
    pub fn stm32_rcc_reset_init(
        dev: *mut device,
        data: *mut clk_stm32_reset_data,
        base: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
