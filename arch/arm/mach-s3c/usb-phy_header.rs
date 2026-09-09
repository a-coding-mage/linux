/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (C) 2011 Samsung Electronics Co.Ltd
 * Author: Joonyoung Shim <jy0922.shim@samsung.com>
 */

// Dependency supplied by the surrounding platform code.
pub enum platform_device {}

extern "C" {
    pub fn s3c_usb_phy_init(pdev: *mut platform_device, type_: i32) -> i32;
    pub fn s3c_usb_phy_exit(pdev: *mut platform_device, type_: i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
