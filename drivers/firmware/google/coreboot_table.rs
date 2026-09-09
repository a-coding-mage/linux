// SPDX-License-Identifier: GPL-2.0-only
/*
 * coreboot_table.c
 *
 * Module providing coreboot table access.
 *
 * Copyright 2017 Google Inc.
 * Copyright 2017 Samuel Holland <samuel@sholland.org>
 */

// Kernel and coreboot-table declarations are supplied by the surrounding
// translation unit/dependencies.

#[repr(C)]
struct coreboot_table_header {
    signature: [core::ffi::c_char; 4],
    header_bytes: u32,
    header_checksum: u32,
    table_bytes: u32,
    table_checksum: u32,
    table_entries: u32,
}

// These types and constants are declared by the kernel/coreboot headers.
use crate::*;

unsafe fn cb_dev(d: *mut device) -> *mut coreboot_device {
    // Equivalent to container_of(d, struct coreboot_device, dev).
    d as *mut coreboot_device
}

unsafe fn cb_drv(d: *const device_driver) -> *const coreboot_driver {
    // Equivalent to container_of_const(d, struct coreboot_driver, drv).
    d as *const coreboot_driver
}

unsafe extern "C" fn coreboot_bus_match(
    dev: *mut device,
    drv: *const device_driver,
) -> i32 {
    let device = &*cb_dev(dev);
    let driver = &*cb_drv(drv);
    let mut id = driver.id_table;

    if id.is_null() {
        return 0;
    }
    while (*id).tag != 0 {
        if device.entry.tag == (*id).tag {
            return 1;
        }
        id = id.add(1);
    }
    0
}

unsafe extern "C" fn coreboot_bus_probe(dev: *mut device) -> i32 {
    let mut ret = -ENODEV;
    let device = cb_dev(dev);
    let driver = cb_drv((*dev).driver as *const device_driver) as *mut coreboot_driver;
    if let Some(probe) = (*driver).probe {
        ret = probe(device);
    }
    ret
}

unsafe extern "C" fn coreboot_bus_remove(dev: *mut device) {
    let device = cb_dev(dev);
    let driver = cb_drv((*dev).driver as *const device_driver) as *mut coreboot_driver;
    if let Some(remove) = (*driver).remove {
        remove(device);
    }
}

unsafe extern "C" fn coreboot_bus_uevent(
    dev: *const device,
    env: *mut kobj_uevent_env,
) -> i32 {
    let device = &*cb_dev(dev as *mut device);
    add_uevent_var(env, b"MODALIAS=coreboot:t%08X\0".as_ptr() as *const _, device.entry.tag)
}

static mut coreboot_bus_type: bus_type = bus_type {
    name: b"coreboot\0".as_ptr() as *const _,
    match_: Some(coreboot_bus_match),
    probe: Some(coreboot_bus_probe),
    remove: Some(coreboot_bus_remove),
    uevent: Some(coreboot_bus_uevent),
};

unsafe extern "C" fn coreboot_device_release(dev: *mut device) {
    kfree(cb_dev(dev));
}

#[no_mangle]
pub unsafe extern "C" fn __coreboot_driver_register(
    driver: *mut coreboot_driver,
    owner: *mut module,
) -> i32 {
    (*driver).drv.bus = &mut coreboot_bus_type;
    (*driver).drv.owner = owner;
    driver_register(&mut (*driver).drv)
}

#[no_mangle]
pub unsafe extern "C" fn coreboot_driver_unregister(driver: *mut coreboot_driver) {
    driver_unregister(&mut (*driver).drv);
}

unsafe fn coreboot_table_populate(dev: *mut device, ptr: *mut u8, len: usize) -> i32 {
    let header = ptr as *mut coreboot_table_header;
    let ptr_end = ptr.add(len);
    let mut ptr_entry = ptr.add((*header).header_bytes as usize);
    let mut entry: *mut coreboot_table_entry;

    for i in 0..(*header).table_entries {
        if ptr_entry.add(core::mem::size_of::<coreboot_table_entry>()) > ptr_end {
            return -EINVAL;
        }
        entry = ptr_entry as *mut coreboot_table_entry;
        if (*entry).size < core::mem::size_of::<coreboot_table_entry>() {
            dev_warn(dev, b"coreboot table entry too small!\n\0".as_ptr() as *const _);
            return -EINVAL;
        }
        if ptr_entry.add((*entry).size as usize) > ptr_end {
            return -EINVAL;
        }

        let device = kzalloc(core::mem::size_of::<device>() + (*entry).size as usize, GFP_KERNEL)
            as *mut coreboot_device;
        if device.is_null() {
            return -ENOMEM;
        }
        (*device).dev.parent = dev;
        (*device).dev.bus = &mut coreboot_bus_type;
        (*device).dev.release = Some(coreboot_device_release);
        memcpy((*device).raw.as_mut_ptr() as *mut _, ptr_entry as *const _, (*entry).size as usize);

        match (*device).entry.tag {
            LB_TAG_CBMEM_ENTRY => {
                if region_intersects((*device).cbmem_entry.address,
                    (*device).cbmem_entry.entry_size, IORESOURCE_SYSTEM_RAM, IORES_DESC_NONE)
                    != REGION_INTERSECTS
                    && region_intersects((*device).cbmem_entry.address,
                        (*device).cbmem_entry.entry_size, IORESOURCE_MEM, IORES_DESC_RESERVED)
                        != REGION_INTERSECTS {
                    kfree(device);
                    ptr_entry = ptr_entry.add((*entry).size as usize);
                    continue;
                }
                dev_set_name(&mut (*device).dev, b"cbmem-%08x\0".as_ptr() as *const _,
                    (*device).cbmem_entry.id);
            }
            _ => dev_set_name(&mut (*device).dev, b"coreboot%d\0".as_ptr() as *const _, i),
        }

        let ret = device_register(&mut (*device).dev);
        if ret != 0 {
            dev_warn(dev, b"failed to register coreboot device: %d\n\0".as_ptr() as *const _, ret);
            put_device(&mut (*device).dev);
        }
        ptr_entry = ptr_entry.add((*entry).size as usize);
    }
    0
}

unsafe extern "C" fn coreboot_table_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if res.is_null() { return -EINVAL; }
    let len = resource_size(res);
    if (*res).start == 0 || len < core::mem::size_of::<coreboot_table_header>() { return -EINVAL; }
    let header = memremap((*res).start, core::mem::size_of::<coreboot_table_header>(), MEMREMAP_WB)
        as *mut coreboot_table_header;
    if header.is_null() { return -ENOMEM; }
    let mut ret = strncmp((*header).signature.as_ptr(), b"LBIO\0".as_ptr() as *const _, 4);
    let mut table_span = 0usize;
    if ret == 0 && ((*header).header_bytes as usize < core::mem::size_of::<coreboot_table_header>()
        || check_add_overflow((*header).header_bytes as usize, (*header).table_bytes as usize, &mut table_span)
        || table_span > len) { ret = -EINVAL; }
    memunmap(header as *mut _);
    if ret != 0 { dev_warn(dev, b"coreboot table missing or corrupt!\n\0".as_ptr() as *const _); return -ENODEV; }
    let ptr = memremap((*res).start, table_span, MEMREMAP_WB);
    if ptr.is_null() { return -ENOMEM; }
    ret = coreboot_table_populate(dev, ptr as *mut u8, table_span);
    memunmap(ptr);
    ret
}

unsafe extern "C" fn __cb_dev_unregister(dev: *mut device, _dummy: *mut core::ffi::c_void) -> i32 {
    device_unregister(dev);
    0
}

unsafe extern "C" fn coreboot_table_remove(_pdev: *mut platform_device) {
    bus_for_each_dev(&mut coreboot_bus_type, core::ptr::null_mut(), core::ptr::null_mut(), Some(__cb_dev_unregister));
}

#[cfg(CONFIG_ACPI)]
static cros_coreboot_acpi_match: [acpi_device_id; 3] = [
    acpi_device_id { id: *b"GOOGCB00", driver_data: 0 },
    acpi_device_id { id: *b"BOOT0000", driver_data: 0 },
    acpi_device_id { id: [0; 8], driver_data: 0 },
];

#[cfg(CONFIG_OF)]
static coreboot_of_match: [of_device_id; 2] = [
    of_device_id { compatible: b"coreboot\0".as_ptr() as *const _ },
    of_device_id { compatible: core::ptr::null() },
];

static mut coreboot_table_driver: platform_driver = platform_driver {
    probe: Some(coreboot_table_probe),
    remove: Some(coreboot_table_remove),
    driver: device_driver {
        name: b"coreboot_table\0".as_ptr() as *const _,
        acpi_match_table: core::ptr::null(),
        of_match_table: core::ptr::null(),
        ..device_driver::default()
    },
};

unsafe extern "C" fn coreboot_table_driver_init() -> i32 {
    let mut ret = bus_register(&mut coreboot_bus_type);
    if ret != 0 { return ret; }
    ret = platform_driver_register(&mut coreboot_table_driver);
    if ret != 0 { bus_unregister(&mut coreboot_bus_type); return ret; }
    0
}

unsafe extern "C" fn coreboot_table_driver_exit() {
    platform_driver_unregister(&mut coreboot_table_driver);
    bus_unregister(&mut coreboot_bus_type);
}

// Equivalent registration annotations: subsys_initcall(coreboot_table_driver_init);
// module_exit(coreboot_table_driver_exit);
// MODULE_AUTHOR("Google, Inc.");
// MODULE_DESCRIPTION("Module providing coreboot table access");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
