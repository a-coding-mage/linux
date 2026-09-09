/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * HiSilicon Clock and Reset Driver Header
 *
 * Copyright (c) 2016 HiSilicon Limited.
 */

// C forward declarations.
#[repr(C)]
pub struct hisi_clock_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hisi_reset_controller {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hisi_crg_funcs {
    pub register_clks:
        Option<unsafe extern "C" fn(pdev: *mut platform_device) -> *mut hisi_clock_data>,
    pub unregister_clks: Option<unsafe extern "C" fn(pdev: *mut platform_device)>,
}

#[repr(C)]
pub struct hisi_crg_dev {
    pub clk_data: *mut hisi_clock_data,
    pub rstc: *mut hisi_reset_controller,
    pub funcs: *const hisi_crg_funcs,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
