// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Registration of Cobalt MTD device.
 *
 *  Copyright (C) 2006  Yoichi Yuasa <yuasa@linux-mips.org>
 */

// Types, constants, registration functions, and the device-initcall macro are
// supplied by the corresponding Linux kernel Rust bindings.

static mut COBALT_MTD_PARTITIONS: [mtd_partition; 1] = [mtd_partition {
    name: "firmware",
    offset: 0x0,
    size: 0x80000,
}];

static mut COBALT_FLASH_DATA: physmap_flash_data = physmap_flash_data {
    width: 1,
    nr_parts: 1,
    parts: unsafe { core::ptr::addr_of_mut!(COBALT_MTD_PARTITIONS[0]) },
};

static mut COBALT_MTD_RESOURCE: resource = resource {
    start: 0x1fc00000,
    end: 0x1fc7ffff,
    flags: IORESOURCE_MEM,
};

static mut COBALT_MTD: platform_device = platform_device {
    name: "physmap-flash",
    dev: device {
        platform_data: unsafe { core::ptr::addr_of_mut!(COBALT_FLASH_DATA) },
    },
    num_resources: 1,
    resource: unsafe { core::ptr::addr_of_mut!(COBALT_MTD_RESOURCE) },
};

unsafe extern "C" {
    fn platform_device_register(device: *mut platform_device) -> i32;
}

unsafe fn cobalt_mtd_init() -> i32 {
    platform_device_register(core::ptr::addr_of_mut!(COBALT_MTD));

    0
}

device_initcall!(cobalt_mtd_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
