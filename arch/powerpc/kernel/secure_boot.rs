// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019 IBM Corporation
 * Author: Nayna Jain
 */

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub name: *const c_char,
    pub type_: *const c_char,
    pub compatible: *const c_char,
    pub data: *const c_void,
}

unsafe extern "C" {
    fn of_find_matching_node(from: *mut device_node, matches: *const of_device_id)
        -> *mut device_node;
    fn of_property_read_bool(np: *const device_node, propname: *const c_char) -> bool;
    fn of_node_put(np: *mut device_node);
    fn of_find_node_by_path(path: *const c_char) -> *mut device_node;
    fn of_property_read_u32(
        np: *const device_node,
        propname: *const c_char,
        out_value: *mut u32,
    ) -> i32;
    fn str_enabled_disabled(enabled: bool) -> *const c_char;
    fn pr_info(fmt: *const c_char, ...);
}

unsafe fn get_ppc_fw_sb_node() -> *mut device_node {
    static IDS: [of_device_id; 4] = [
        of_device_id {
            name: core::ptr::null(),
            type_: core::ptr::null(),
            compatible: b"ibm,secureboot\0".as_ptr() as *const c_char,
            data: core::ptr::null(),
        },
        of_device_id {
            name: core::ptr::null(),
            type_: core::ptr::null(),
            compatible: b"ibm,secureboot-v1\0".as_ptr() as *const c_char,
            data: core::ptr::null(),
        },
        of_device_id {
            name: core::ptr::null(),
            type_: core::ptr::null(),
            compatible: b"ibm,secureboot-v2\0".as_ptr() as *const c_char,
            data: core::ptr::null(),
        },
        of_device_id {
            name: core::ptr::null(),
            type_: core::ptr::null(),
            compatible: core::ptr::null(),
            data: core::ptr::null(),
        },
    ];

    of_find_matching_node(core::ptr::null_mut(), IDS.as_ptr())
}

pub unsafe fn is_ppc_secureboot_enabled() -> bool {
    let mut node: *mut device_node;
    let mut enabled = false;
    let mut secureboot: u32 = 0;

    node = get_ppc_fw_sb_node();
    enabled = of_property_read_bool(node, b"os-secureboot-enforcing\0".as_ptr() as *const c_char);
    of_node_put(node);

    if enabled {
        return {
            pr_info(
                b"Secure boot mode %s\n\0".as_ptr() as *const c_char,
                str_enabled_disabled(enabled),
            );
            enabled
        };
    }

    node = of_find_node_by_path(b"/\0".as_ptr() as *const c_char);
    if of_property_read_u32(
        node,
        b"ibm,secure-boot\0".as_ptr() as *const c_char,
        &mut secureboot,
    ) == 0
    {
        enabled = secureboot > 1;
    }
    of_node_put(node);

    pr_info(
        b"Secure boot mode %s\n\0".as_ptr() as *const c_char,
        str_enabled_disabled(enabled),
    );

    enabled
}

pub unsafe fn arch_get_secureboot() -> bool {
    is_ppc_secureboot_enabled()
}

pub unsafe fn is_ppc_trustedboot_enabled() -> bool {
    let mut node: *mut device_node;
    let mut enabled = false;
    let mut trustedboot: u32 = 0;

    node = get_ppc_fw_sb_node();
    enabled = of_property_read_bool(node, b"trusted-enabled\0".as_ptr() as *const c_char);
    of_node_put(node);

    if enabled {
        return {
            pr_info(
                b"Trusted boot mode %s\n\0".as_ptr() as *const c_char,
                str_enabled_disabled(enabled),
            );
            enabled
        };
    }

    node = of_find_node_by_path(b"/\0".as_ptr() as *const c_char);
    if of_property_read_u32(
        node,
        b"ibm,trusted-boot\0".as_ptr() as *const c_char,
        &mut trustedboot,
    ) == 0
    {
        enabled = trustedboot > 0;
    }
    of_node_put(node);

    pr_info(
        b"Trusted boot mode %s\n\0".as_ptr() as *const c_char,
        str_enabled_disabled(enabled),
    );

    enabled
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
