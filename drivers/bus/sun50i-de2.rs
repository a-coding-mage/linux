// SPDX-License-Identifier: GPL-2.0
/*
 * Allwinner A64 Display Engine 2.0 Bus Driver
 *
 * Copyright (C) 2018 Icenowy Zheng <icenowy@aosc.io>
 */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct Device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PlatformDevice {
    pub dev: Device,
    pub of_node: *mut DeviceNode,
}

#[repr(C)]
pub struct OfDeviceId {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct Driver {
    pub name: *const c_char,
    pub of_match_table: *const OfDeviceId,
}

#[repr(C)]
pub struct PlatformDriver {
    pub probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut PlatformDevice)>,
    pub driver: Driver,
}

extern "C" {
    fn sunxi_sram_claim(dev: *mut Device) -> c_int;
    fn sunxi_sram_release(dev: *mut Device);
    fn dev_err_probe(dev: *mut Device, err: c_int, fmt: *const c_char) -> c_int;
    fn of_platform_populate(
        np: *mut DeviceNode,
        matches: *const c_void,
        lookup: *const c_void,
        parent: *mut Device,
    ) -> c_int;
    fn builtin_platform_driver(driver: *mut PlatformDriver);
}

unsafe extern "C" fn sun50i_de2_bus_probe(pdev: *mut PlatformDevice) -> c_int {
    let np = (*pdev).of_node;
    let ret: c_int;

    ret = sunxi_sram_claim(&mut (*pdev).dev);
    if ret != 0 {
        return dev_err_probe(
            &mut (*pdev).dev,
            ret,
            b"Couldn't map SRAM to device\0".as_ptr() as *const c_char,
        );
    }

    of_platform_populate(
        np,
        core::ptr::null(),
        core::ptr::null(),
        &mut (*pdev).dev,
    );

    0
}

unsafe extern "C" fn sun50i_de2_bus_remove(pdev: *mut PlatformDevice) {
    sunxi_sram_release(&mut (*pdev).dev);
}

static SUN50I_DE2_BUS_OF_MATCH: [OfDeviceId; 2] = [
    OfDeviceId {
        compatible: b"allwinner,sun50i-a64-de2\0".as_ptr() as *const c_char,
    },
    OfDeviceId {
        // sentinel
        compatible: core::ptr::null(),
    },
];

static mut SUN50I_DE2_BUS_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(sun50i_de2_bus_probe),
    remove: Some(sun50i_de2_bus_remove),
    driver: Driver {
        name: b"sun50i-de2-bus\0".as_ptr() as *const c_char,
        of_match_table: SUN50I_DE2_BUS_OF_MATCH.as_ptr(),
    },
};

// Equivalent to builtin_platform_driver(sun50i_de2_bus_driver).
#[used]
#[cfg_attr(target_os = "linux", link_section = ".initcall6.init")]
static SUN50I_DE2_BUS_DRIVER_INIT: unsafe extern "C" fn() = sun50i_de2_bus_register;

unsafe extern "C" fn sun50i_de2_bus_register() {
    builtin_platform_driver(&raw mut SUN50I_DE2_BUS_DRIVER);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
