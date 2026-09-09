// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Eurobraille/Iris power off support.
 *
 * Eurobraille's Iris machine is a PC with no APM or ACPI support.
 * It is shutdown by a special I/O sequence which this module provides.
 *
 *  Copyright (C) Shérab <Sebastien.Hinderer@ens-lyon.org>
 */

// Linux kernel dependencies supplied by other translation units.
use core::ffi::{c_char, c_int, c_void};

const IRIS_GIO_BASE: u16 = 0x340;
const IRIS_GIO_INPUT: u16 = IRIS_GIO_BASE;
const IRIS_GIO_OUTPUT: u16 = IRIS_GIO_BASE + 1;
const IRIS_GIO_PULSE: u8 = 0x80; // First byte to send
const IRIS_GIO_REST: u8 = 0x00; // Second byte to send
const IRIS_GIO_NODEV: u8 = 0xff; // Likely not an Iris

extern "C" {
    fn outb(value: u8, port: u16);
    fn inb(port: u16) -> u8;
    fn msleep(milliseconds: u32);
    fn printk(format: *const c_char, ...);
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
    fn platform_device_register_simple(
        name: *const c_char,
        id: c_int,
        resources: *mut resource,
        num_resources: usize,
    ) -> *mut platform_device;
    fn platform_device_unregister(device: *mut platform_device);
    fn ptr_err(pointer: *mut c_void) -> c_int;
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: u16,
    pub end: u16,
    pub flags: u64,
    pub name: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

const IORESOURCE_IO: u64 = 0x0000_0100;

static mut force: bool = false;
static mut old_pm_power_off: Option<unsafe extern "C" fn()> = None;

extern "C" {
    static mut pm_power_off: Option<unsafe extern "C" fn()>;
}

unsafe extern "C" fn iris_power_off() {
    outb(IRIS_GIO_PULSE, IRIS_GIO_OUTPUT);
    msleep(850);
    outb(IRIS_GIO_REST, IRIS_GIO_OUTPUT);
}

/*
 * Before installing the power_off handler, try to make sure the OS is
 * running on an Iris.  Since Iris does not support DMI, this is done
 * by reading its input port and seeing whether the read value is
 * meaningful.
 */
unsafe extern "C" fn iris_probe(_pdev: *mut platform_device) -> c_int {
    let status: u8 = inb(IRIS_GIO_INPUT);
    if status == IRIS_GIO_NODEV {
        // printk(KERN_ERR "This machine does not seem to be an Iris. "
        //     "Power off handler not installed.\n");
        return -19; // -ENODEV
    }
    old_pm_power_off = pm_power_off;
    pm_power_off = Some(iris_power_off);
    // printk(KERN_INFO "Iris power_off handler installed.\n");
    0
}

unsafe extern "C" fn iris_remove(_pdev: *mut platform_device) {
    pm_power_off = old_pm_power_off;
    // printk(KERN_INFO "Iris power_off handler uninstalled.\n");
}

static mut iris_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"iris\0".as_ptr() as *const c_char,
    },
    probe: Some(iris_probe),
    remove: Some(iris_remove),
};

static mut iris_resources: [resource; 1] = [resource {
    start: IRIS_GIO_BASE,
    end: IRIS_GIO_OUTPUT,
    flags: IORESOURCE_IO,
    name: b"address\0".as_ptr() as *const c_char,
}];

static mut iris_device: *mut platform_device = core::ptr::null_mut();

unsafe extern "C" fn iris_init() -> c_int {
    let ret: c_int;
    if force != true {
        // printk(KERN_ERR "The force parameter has not been set to 1."
        //     " The Iris poweroff handler will not be installed.\n");
        return -19; // -ENODEV
    }
    ret = platform_driver_register(&raw mut iris_driver);
    if ret < 0 {
        // printk(KERN_ERR "Failed to register iris platform driver: %d\n", ret);
        return ret;
    }
    iris_device = platform_device_register_simple(
        b"iris\0".as_ptr() as *const c_char,
        -1,
        &raw mut iris_resources[0],
        iris_resources.len(),
    );
    if iris_device.is_null() {
        // printk(KERN_ERR "Failed to register iris platform device\n");
        platform_driver_unregister(&raw mut iris_driver);
        return -1;
    }
    0
}

unsafe extern "C" fn iris_exit() {
    platform_device_unregister(iris_device);
    platform_driver_unregister(&raw mut iris_driver);
}

// module_init(iris_init);
// module_exit(iris_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
