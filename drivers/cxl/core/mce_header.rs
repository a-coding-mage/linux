/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2024 Intel Corporation. All rights reserved. */

/* C dependency: <linux/notifier.h> */

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct notifier_block {
    _private: [u8; 0],
}

/* CONFIG_CXL_MCE is a build-time configuration condition. */
#[cfg(feature = "CONFIG_CXL_MCE")]
unsafe extern "C" {
    pub fn devm_cxl_register_mce_notifier(
        dev: *mut device,
        mce_notifier: *mut notifier_block,
    ) -> i32;
}

#[cfg(not(feature = "CONFIG_CXL_MCE"))]
pub unsafe fn devm_cxl_register_mce_notifier(
    _dev: *mut device,
    _mce_notifier: *mut notifier_block,
) -> i32 {
    -95 /* -EOPNOTSUPP */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
