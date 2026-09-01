/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (C) 2018 Vasily Khoruzhick <anarsoul@gmail.com>
 */

unsafe extern "C" {
    pub fn sun8i_adda_pr_regmap_init(dev: *mut device, base: *mut core::ffi::c_void) -> *mut regmap;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
