// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2009 Lemote Inc.
 * Author: Wu Zhangjin, wuzhangjin@gmail.com
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/err.h, linux/smp.h, and linux/platform_device.h

#[repr(C)]
pub struct platform_device {
    pub name: *const core::ffi::c_char,
    pub id: i32,
}

#[repr(C)]
pub struct cpuinfo_mips {
    pub processor_id: u32,
}

extern "C" {
    pub static mut current_cpu_data: cpuinfo_mips;
    pub fn platform_device_register(dev: *mut platform_device) -> i32;
}

pub const PRID_REV_MASK: u32 = 0xff;
pub const PRID_REV_LOONGSON2F: u32 = 0x2;
pub const ENODEV: i32 = 19;

static mut loongson2_cpufreq_device: platform_device = platform_device {
    name: b"loongson2_cpufreq\0".as_ptr() as *const core::ffi::c_char,
    id: -1,
};

pub unsafe fn loongson2_cpufreq_init() -> i32 {
    let c: *mut cpuinfo_mips = &raw mut current_cpu_data;

    /* Only 2F revision and its successors support CPUFreq */
    if ((*c).processor_id & PRID_REV_MASK) >= PRID_REV_LOONGSON2F {
        return platform_device_register(&raw mut loongson2_cpufreq_device);
    }

    -ENODEV
}

// arch_initcall(loongson2_cpufreq_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
