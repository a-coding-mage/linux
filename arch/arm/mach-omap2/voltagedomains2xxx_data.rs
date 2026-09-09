// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP3 voltage domain data
 *
 * Copyright (C) 2011 Texas Instruments, Inc.
 */

// C dependencies: <linux/kernel.h>, <linux/init.h>, and "voltage.h".

extern "C" {
    fn voltdm_init(domains: *mut *mut voltagedomain);
}

static mut omap2_voltdm_core: voltagedomain = voltagedomain {
    name: b"core\0".as_ptr() as *const i8,
};

static mut omap2_voltdm_wkup: voltagedomain = voltagedomain {
    name: b"wakeup\0".as_ptr() as *const i8,
};

static mut voltagedomains_omap2: [*mut voltagedomain; 3] = [
    unsafe { &mut omap2_voltdm_core as *mut voltagedomain },
    unsafe { &mut omap2_voltdm_wkup as *mut voltagedomain },
    core::ptr::null_mut(),
];

pub unsafe extern "C" fn omap2xxx_voltagedomains_init() {
    voltdm_init(voltagedomains_omap2.as_mut_ptr());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
