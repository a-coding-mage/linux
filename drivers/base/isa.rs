// SPDX-License-Identifier: GPL-2.0
/*
 * ISA bus.
 */

use core::ptr;

// Linux kernel dependencies supplied by other translation units.
use crate::{
    bus_type, device, device_driver, isa_driver, pm_message_t,
    bus_register, bus_unregister, device_register, device_unregister,
    driver_register, driver_unregister, dev_set_name, put_device,
    root_device_register, PTR_ERR, IS_ERR, DMA_BIT_MASK, ENODEV, ENOMEM,
};

static mut isa_bus: *mut device = ptr::null_mut();

#[repr(C)]
struct isa_dev {
    dev: device,
    next: *mut device,
    id: u32,
}

#[inline]
unsafe fn to_isa_dev(x: *mut device) -> *mut isa_dev {
    (x as *mut u8).sub(core::mem::offset_of!(isa_dev, dev)) as *mut isa_dev
}

unsafe fn isa_bus_match(dev: *mut device, driver: *const device_driver) -> i32 {
    let isa_driver = crate::to_isa_driver(driver);

    if (*dev).platform_data == isa_driver as *mut _ {
        if (*isa_driver).match_.is_none()
            || ((*isa_driver).match_.unwrap())(dev, (*to_isa_dev(dev)).id) != 0
        {
            return 1;
        }
        (*dev).platform_data = ptr::null_mut();
    }
    0
}

unsafe fn isa_bus_probe(dev: *mut device) -> i32 {
    let isa_driver = (*dev).platform_data as *mut isa_driver;
    if !isa_driver.is_null() {
        if let Some(probe) = (*isa_driver).probe {
            return probe(dev, (*to_isa_dev(dev)).id);
        }
    }
    0
}

unsafe fn isa_bus_remove(dev: *mut device) {
    let isa_driver = (*dev).platform_data as *mut isa_driver;
    if !isa_driver.is_null() {
        if let Some(remove) = (*isa_driver).remove {
            remove(dev, (*to_isa_dev(dev)).id);
        }
    }
}

unsafe fn isa_bus_shutdown(dev: *mut device) {
    let isa_driver = (*dev).platform_data as *mut isa_driver;
    if !isa_driver.is_null() {
        if let Some(shutdown) = (*isa_driver).shutdown {
            shutdown(dev, (*to_isa_dev(dev)).id);
        }
    }
}

unsafe fn isa_bus_suspend(dev: *mut device, state: pm_message_t) -> i32 {
    let isa_driver = (*dev).platform_data as *mut isa_driver;
    if !isa_driver.is_null() {
        if let Some(suspend) = (*isa_driver).suspend {
            return suspend(dev, (*to_isa_dev(dev)).id, state);
        }
    }
    0
}

unsafe fn isa_bus_resume(dev: *mut device) -> i32 {
    let isa_driver = (*dev).platform_data as *mut isa_driver;
    if !isa_driver.is_null() {
        if let Some(resume) = (*isa_driver).resume {
            return resume(dev, (*to_isa_dev(dev)).id);
        }
    }
    0
}

static isa_bus_type: bus_type = bus_type {
    name: "isa",
    match_: Some(isa_bus_match),
    probe: Some(isa_bus_probe),
    remove: Some(isa_bus_remove),
    shutdown: Some(isa_bus_shutdown),
    suspend: Some(isa_bus_suspend),
    resume: Some(isa_bus_resume),
};

unsafe fn isa_dev_release(dev: *mut device) {
    crate::kfree(to_isa_dev(dev));
}

pub unsafe fn isa_unregister_driver(isa_driver: *mut isa_driver) {
    let mut dev = (*isa_driver).devices;
    while !dev.is_null() {
        let tmp = (*to_isa_dev(dev)).next;
        device_unregister(dev);
        dev = tmp;
    }
    driver_unregister(&mut (*isa_driver).driver);
}

pub unsafe fn isa_register_driver(isa_driver: *mut isa_driver, ndev: u32) -> i32 {
    let mut error: i32;
    (*isa_driver).driver.bus = &isa_bus_type;
    (*isa_driver).devices = ptr::null_mut();

    error = driver_register(&mut (*isa_driver).driver);
    if error != 0 { return error; }

    let mut id = 0u32;
    while id < ndev {
        let isa_dev = crate::kzalloc_obj::<isa_dev>();
        if isa_dev.is_null() {
            error = -ENOMEM;
            break;
        }
        (*isa_dev).dev.parent = isa_bus;
        (*isa_dev).dev.bus = &isa_bus_type;
        dev_set_name(&mut (*isa_dev).dev, (*isa_driver).driver.name, id);
        (*isa_dev).dev.platform_data = isa_driver as *mut _;
        (*isa_dev).dev.release = Some(isa_dev_release);
        (*isa_dev).id = id;
        (*isa_dev).dev.coherent_dma_mask = DMA_BIT_MASK(24);
        (*isa_dev).dev.dma_mask = &mut (*isa_dev).dev.coherent_dma_mask;
        error = device_register(&mut (*isa_dev).dev);
        if error != 0 {
            put_device(&mut (*isa_dev).dev);
            break;
        }
        (*isa_dev).next = (*isa_driver).devices;
        (*isa_driver).devices = &mut (*isa_dev).dev;
        id += 1;
    }
    if error == 0 && (*isa_driver).devices.is_null() { error = -ENODEV; }
    if error != 0 { isa_unregister_driver(isa_driver); }
    error
}

unsafe fn isa_bus_init() -> i32 {
    let error = bus_register(&isa_bus_type);
    if error != 0 { return error; }
    isa_bus = root_device_register("isa");
    if IS_ERR(isa_bus) {
        bus_unregister(&isa_bus_type);
        return PTR_ERR(isa_bus);
    }
    0
}

// postcore_initcall(isa_bus_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
