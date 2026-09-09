// SPDX-License-Identifier: GPL-2.0
/*
 * RSK+SH7269 Support
 *
 * Copyright (C) 2012  Renesas Electronics Europe Ltd
 * Copyright (C) 2012  Phil Edworthy
 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux/init.h, linux/types.h, linux/platform_device.h, linux/interrupt.h,
// linux/input.h, linux/smsc911x.h, asm/machvec.h, and asm/io.h.

static mut smsc911x_config: smsc911x_platform_config = smsc911x_platform_config {
    phy_interface: PHY_INTERFACE_MODE_MII,
    irq_polarity: SMSC911X_IRQ_POLARITY_ACTIVE_LOW,
    irq_type: SMSC911X_IRQ_TYPE_PUSH_PULL,
    flags: SMSC911X_USE_16BIT | SMSC911X_SWAP_FIFO,
};

static mut smsc911x_resources: [resource; 2] = [
    resource {
        start: 0x24000000,
        end: 0x240000ff,
        flags: IORESOURCE_MEM,
    },
    resource {
        start: 85,
        end: 85,
        flags: IORESOURCE_IRQ,
    },
];

static mut smsc911x_device: platform_device = platform_device {
    name: "smsc911x\\0".as_ptr() as *const i8,
    id: -1,
    num_resources: smsc911x_resources.len(),
    resource: smsc911x_resources.as_mut_ptr(),
    dev: device {
        platform_data: unsafe { &raw mut smsc911x_config as *mut _ as *mut core::ffi::c_void },
    },
};

#[link_section = ".data.init"]
static mut rsk7269_devices: [*mut platform_device; 1] = [
    unsafe { &raw mut smsc911x_device },
];

unsafe extern "C" {
    fn platform_add_devices(devices: *mut *mut platform_device, num: usize) -> i32;
}

#[allow(non_snake_case)]
unsafe fn rsk7269_devices_setup() -> i32 {
    platform_add_devices(rsk7269_devices.as_mut_ptr(), rsk7269_devices.len())
}

// C: device_initcall(rsk7269_devices_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
