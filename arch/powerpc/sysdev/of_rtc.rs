// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Instantiate mmio-mapped RTC chips based on device tree information
 *
 * Copyright 2007 David Gibson <dwg@au1.ibm.com>, IBM Corporation.
 */

// Linux kernel headers supplying these declarations are external dependencies.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: u64,
    pub end: u64,
    _private: [u8; 0],
}

#[repr(C)]
struct OfRtcTableEntry {
    compatible: *const c_char,
    plat_name: *mut c_char,
}

static mut OF_RTC_TABLE: [OfRtcTableEntry; 1] = [OfRtcTableEntry {
    compatible: b"ds1743-nvram\0".as_ptr() as *const c_char,
    plat_name: b"rtc-ds1742\0".as_ptr() as *mut c_char,
}];

extern "C" {
    fn kmalloc_resource() -> *mut resource;
    fn of_address_to_resource(node: *mut device_node, index: c_int, res: *mut resource) -> c_int;
    fn printk(format: *const c_char, ...) -> c_int;
    fn platform_device_register_simple(
        name: *mut c_char,
        id: c_int,
        res: *const resource,
        num_resources: c_int,
    ) -> c_int;
    // Equivalent to the kernel's for_each_compatible_node iteration macro.
    fn of_find_compatible_node(
        from: *mut device_node,
        type_: *const c_char,
        compatible: *const c_char,
    ) -> *mut device_node;
}

const KERN_ERR: &[u8] = b"<3>\0";
const KERN_INFO: &[u8] = b"<6>\0";

pub unsafe extern "C" fn of_instantiate_rtc() {
    let mut i: usize = 0;

    while i < OF_RTC_TABLE.len() {
        let plat_name = OF_RTC_TABLE[i].plat_name;
        let mut node: *mut device_node = core::ptr::null_mut();

        loop {
            node = of_find_compatible_node(
                node,
                core::ptr::null(),
                OF_RTC_TABLE[i].compatible,
            );
            if node.is_null() {
                break;
            }

            let res = kmalloc_resource();
            if res.is_null() {
                let fmt = b"<3>OF RTC: Out of memory allocating resource structure for %pOF\n\0";
                printk(fmt.as_ptr() as *const c_char, node);
                continue;
            }

            let err = of_address_to_resource(node, 0, res);
            if err != 0 {
                let fmt = b"<3>OF RTC: Error translating resources for %pOF\n\0";
                printk(fmt.as_ptr() as *const c_char, node);
                continue;
            }

            let fmt = b"<6>OF_RTC: %pOF is a %s @ 0x%llx-0x%llx\n\0";
            printk(
                fmt.as_ptr() as *const c_char,
                node,
                plat_name,
                (*res).start,
                (*res).end,
            );
            platform_device_register_simple(plat_name, -1, res, 1);
        }

        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
