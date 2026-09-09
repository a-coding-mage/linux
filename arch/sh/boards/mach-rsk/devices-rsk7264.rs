// SPDX-License-Identifier: GPL-2.0
/*
 * RSK+SH7264 Support.
 *
 * Copyright (C) 2012 Renesas Electronics Europe
 */

// Linux kernel headers and architecture headers provide the following types,
// constants, macros, and functions in the containing translation unit.

static mut smsc911x_config: smsc911x_platform_config = smsc911x_platform_config {
    phy_interface: PHY_INTERFACE_MODE_MII,
    irq_polarity: SMSC911X_IRQ_POLARITY_ACTIVE_LOW,
    irq_type: SMSC911X_IRQ_TYPE_OPEN_DRAIN,
    flags: SMSC911X_USE_16BIT | SMSC911X_SWAP_FIFO,
};

static mut smsc911x_resources: [resource; 2] = [
    resource {
        start: 0x28000000,
        end: 0x280000ff,
        flags: IORESOURCE_MEM,
    },
    resource {
        start: 65,
        end: 65,
        flags: IORESOURCE_IRQ,
    },
];

static mut smsc911x_device: platform_device = platform_device {
    name: "smsc911x",
    id: -1,
    num_resources: smsc911x_resources.len(),
    resource: smsc911x_resources.as_ptr(),
    dev: device {
        platform_data: core::ptr::addr_of_mut!(smsc911x_config) as *mut _,
    },
};

// `__initdata` places this table in the kernel's initialization-data section.
static mut rsk7264_devices: [*mut platform_device; 1] = [
    core::ptr::addr_of_mut!(smsc911x_device),
];

unsafe extern "C" {
    fn platform_add_devices(devices: *mut *mut platform_device, count: usize) -> i32;
}

unsafe extern "C" fn rsk7264_devices_setup() -> i32 {
    platform_add_devices(rsk7264_devices.as_mut_ptr(), rsk7264_devices.len())
}

// `device_initcall(rsk7264_devices_setup)` registers the setup function as a
// device initialization callback.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
