/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Dependency: linux/of.h determines include ordering. */

/*
 * Definitions for talking to the Open Firmware PROM on
 * Power Macintosh computers.
 *
 * Copyright (C) 1996-2005 Paul Mackerras.
 *
 * Updates for PPC64 by Peter Bergner & David Engebretsen, IBM Corp.
 * Updates for SPARC by David S. Miller
 */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/* Kernel-only declarations from the original header. */

pub unsafe fn of_compat_cmp(s1: *const c_char, s2: *const c_char, l: usize) -> c_int {
    strncmp(s1, s2, l)
}

pub unsafe fn of_prop_cmp(s1: *const c_char, s2: *const c_char) -> c_int {
    strcasecmp(s1, s2)
}

pub unsafe fn of_node_cmp(s1: *const c_char, s2: *const c_char) -> c_int {
    strcmp(s1, s2)
}

pub static mut devtree_lock: raw_spinlock_t;

#[repr(C)]
pub struct of_irq_controller {
    pub irq_build: Option<unsafe extern "C" fn(*mut device_node, c_uint, *mut c_void) -> c_uint>,
    pub data: *mut c_void,
}

unsafe extern "C" {
    pub fn of_find_node_by_cpuid(cpuid: c_int) -> *mut device_node;
    pub fn of_set_property(
        node: *mut device_node,
        name: *const c_char,
        val: *mut c_void,
        len: c_int,
    ) -> c_int;
    pub static mut of_set_property_mutex: mutex;
    pub fn of_getintprop_default(
        np: *mut device_node,
        name: *const c_char,
        def: c_int,
    ) -> c_int;
    pub fn of_find_in_proplist(list: *const c_char, r#match: *const c_char, len: c_int) -> c_int;

    pub fn prom_build_devicetree();
    pub fn of_populate_present_mask();
    pub fn of_fill_in_cpu_data();

    pub fn of_ioremap(
        res: *mut resource,
        offset: c_ulong,
        size: c_ulong,
        name: *mut c_char,
    ) -> *mut c_void;
    pub fn of_iounmap(res: *mut resource, base: *mut c_void, size: c_ulong);

    pub static mut of_console_device: *mut device_node;
    pub static mut of_console_path: *mut c_char;
    pub static mut of_console_options: *mut c_char;

    pub fn irq_trans_init(dp: *mut device_node);
    pub fn build_path_component(dp: *mut device_node) -> *mut c_char;
}

/* External kernel types and C string functions are supplied by dependencies. */
extern "C" {
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
