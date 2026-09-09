// SPDX-License-Identifier: GPL-2.0-only
/*
 * Virtual EISA root driver.
 * Acts as a placeholder if we don't have a proper EISA bridge.
 *
 * (C) 2003 Marc Zyngier <maz@wild-wind.fr.eu.org>
 */

// Linux kernel headers supplied by the surrounding translation unit.

#[cfg(CONFIG_EISA_VLB_PRIMING)]
const EISA_FORCE_PROBE_DEFAULT: i32 = 1;
#[cfg(not(CONFIG_EISA_VLB_PRIMING))]
const EISA_FORCE_PROBE_DEFAULT: i32 = 0;

const EISA_MAX_SLOTS: u32 = 8;

#[repr(C)]
pub struct device {
    pub release: Option<unsafe extern "C" fn(*mut device)>,
    pub driver_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct platform_device {
    pub name: *const core::ffi::c_char,
    pub id: i32,
    pub dev: device,
}

#[repr(C)]
pub struct eisa_root_device {
    pub dev: *mut device,
    pub bus_base_addr: u32,
    pub res: *mut resource,
    pub slots: u32,
    pub dma_mask: u32,
    pub force_probe: i32,
}

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

extern "C" {
    static mut ioport_resource: resource;

    fn platform_device_register(dev: *mut platform_device) -> i32;
    fn platform_device_unregister(dev: *mut platform_device);
    fn dev_set_drvdata(dev: *mut device, data: *mut core::ffi::c_void);
    fn eisa_root_register(root: *mut eisa_root_device) -> i32;
    fn module_param(param: *mut i32, ty: *const core::ffi::c_char, mode: u32);
    fn device_initcall(init: unsafe extern "C" fn() -> i32);
}

static mut force_probe: i32 = EISA_FORCE_PROBE_DEFAULT;

/* The default EISA device parent (virtual root device).
 * Now use a platform device, since that's the obvious choice. */
static mut eisa_root_dev: platform_device = platform_device {
    name: b"eisa\0".as_ptr() as *const core::ffi::c_char,
    id: 0,
    dev: device {
        release: Some(virtual_eisa_release),
        driver_data: core::ptr::null_mut(),
    },
};

static mut eisa_bus_root: eisa_root_device = eisa_root_device {
    dev: unsafe { &raw mut eisa_root_dev.dev },
    bus_base_addr: 0,
    res: unsafe { &raw mut ioport_resource },
    slots: EISA_MAX_SLOTS,
    dma_mask: 0xffff_ffff,
    force_probe: 0,
};

unsafe extern "C" fn virtual_eisa_release(_dev: *mut device) {
    /* nothing really to do here */
}

unsafe extern "C" fn virtual_eisa_root_init() -> i32 {
    let r: i32;

    r = platform_device_register(&raw mut eisa_root_dev);
    if r != 0 {
        return r;
    }

    eisa_bus_root.force_probe = force_probe;

    dev_set_drvdata(
        &raw mut eisa_root_dev.dev,
        &raw mut eisa_bus_root as *mut eisa_root_device as *mut core::ffi::c_void,
    );

    if eisa_root_register(&raw mut eisa_bus_root) != 0 {
        /* A real bridge may have been registered before
         * us. So quietly unregister. */
        platform_device_unregister(&raw mut eisa_root_dev);
        return -1;
    }

    0
}

// module_param(force_probe, int, 0444);
// device_initcall(virtual_eisa_root_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
