// SPDX-License-Identifier: GPL-2.0-only
/*
 * OF-platform PATA driver
 *
 * Copyright (c) 2007  MontaVista Software, Inc.
 *                     Anton Vorontsov <avorontsov@ru.mvista.com>
 */

// Dependencies supplied by the Linux kernel and other translation units.

use core::ffi::c_void;

const DRV_NAME: &[u8] = b"pata_of_platform\0";

#[repr(C)]
pub struct ScsiHostTemplate {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PlatformDevice {
    pub dev: Device,
}

#[repr(C)]
pub struct Device {
    pub of_node: *mut DeviceNode,
}

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Resource {
    pub start: u64,
    pub end: u64,
    _private: [u8; 16],
}

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const u8,
}

#[repr(C)]
pub struct DeviceDriver {
    pub name: *const u8,
    pub of_match_table: *const OfDeviceId,
}

#[repr(C)]
pub struct PlatformDriver {
    pub driver: DeviceDriver,
    pub probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>,
}

extern "C" {
    fn of_address_to_resource(
        node: *mut DeviceNode,
        index: u32,
        resource: *mut Resource,
    ) -> i32;
    fn platform_get_irq_optional(device: *mut PlatformDevice, index: u32) -> i32;
    fn of_property_read_u32(node: *mut DeviceNode, property: *const u8, value: *mut u32) -> i32;
    fn of_property_read_bool(node: *mut DeviceNode, property: *const u8) -> bool;
    fn __pata_platform_probe(
        dev: *mut Device,
        io_res: *mut Resource,
        ctl_res: *mut Resource,
        irq_res: *mut Resource,
        reg_shift: u32,
        pio_mask: i32,
        sht: *const ScsiHostTemplate,
        use16bit: bool,
    ) -> i32;
    fn ata_platform_remove_one(device: *mut PlatformDevice) -> i32;
    fn dev_err(dev: *mut Device, format: *const u8, ...);
    fn dev_info(dev: *mut Device, format: *const u8, ...);
}

// ATA_PIO_SHT(DRV_NAME)
static pata_platform_sht: ScsiHostTemplate = ScsiHostTemplate { _private: [] };

unsafe extern "C" fn pata_of_platform_probe(ofdev: *mut PlatformDevice) -> i32 {
    let mut ret: i32;
    let dn: *mut DeviceNode = (*ofdev).dev.of_node;
    let mut io_res: Resource = core::mem::zeroed();
    let mut ctl_res: Resource = core::mem::zeroed();
    let mut irq_res: Resource = core::mem::zeroed();
    let mut reg_shift: u32 = 0;
    let mut pio_mode: i32 = 0;
    let pio_mask: i32;
    let use16bit: bool;
    let irq: i32;

    ret = of_address_to_resource(dn, 0, &mut io_res);
    if ret != 0 {
        dev_err(&mut (*ofdev).dev, b"can't get IO address from device tree\n\0".as_ptr());
        return -22; // -EINVAL
    }

    ret = of_address_to_resource(dn, 1, &mut ctl_res);
    if ret != 0 {
        dev_err(&mut (*ofdev).dev, b"can't get CTL address from device tree\n\0".as_ptr());
        return -22; // -EINVAL
    }

    irq_res = core::mem::zeroed();

    irq = platform_get_irq_optional(ofdev, 0);
    if irq < 0 && irq != -6 { // -ENXIO
        return irq;
    }
    if irq > 0 {
        irq_res.start = irq as u64;
        irq_res.end = irq as u64;
    }

    of_property_read_u32(dn, b"reg-shift\0".as_ptr(), &mut reg_shift);

    if of_property_read_u32(dn, b"pio-mode\0".as_ptr(), &mut pio_mode) == 0 {
        if pio_mode > 6 {
            dev_err(&mut (*ofdev).dev, b"invalid pio-mode\n\0".as_ptr());
            return -22; // -EINVAL
        }
    } else {
        dev_info(&mut (*ofdev).dev, b"pio-mode unspecified, assuming PIO0\n\0".as_ptr());
    }

    use16bit = of_property_read_bool(dn, b"ata-generic,use16bit\0".as_ptr());

    pio_mask = (1i32 << pio_mode) | ((1i32 << pio_mode) - 1);

    __pata_platform_probe(
        &mut (*ofdev).dev,
        &mut io_res,
        &mut ctl_res,
        if irq > 0 { &mut irq_res } else { core::ptr::null_mut() },
        reg_shift,
        pio_mask,
        &pata_platform_sht,
        use16bit,
    )
}

static pata_of_platform_match: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"ata-generic\0".as_ptr() },
    OfDeviceId { compatible: core::ptr::null() }, // sentinel
];

static mut pata_of_platform_driver: PlatformDriver = PlatformDriver {
    driver: DeviceDriver {
        name: DRV_NAME.as_ptr(),
        of_match_table: pata_of_platform_match.as_ptr(),
    },
    probe: Some(pata_of_platform_probe),
    remove: Some(ata_platform_remove_one),
};

// module_platform_driver(pata_of_platform_driver);
// MODULE_DEVICE_TABLE(of, pata_of_platform_match);
// MODULE_DESCRIPTION("OF-platform PATA driver");
// MODULE_AUTHOR("Anton Vorontsov <avorontsov@ru.mvista.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
