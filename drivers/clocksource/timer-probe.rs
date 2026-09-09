// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
 */

// Declarations supplied by the Linux kernel headers.
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub name: *const core::ffi::c_char,
    pub type_: *const core::ffi::c_char,
    pub compatible: *const core::ffi::c_char,
    pub data: Option<unsafe extern "C" fn(*mut device_node) -> i32>,
}

pub type of_init_fn_1_ret = unsafe extern "C" fn(*mut device_node) -> i32;

unsafe extern "C" {
    pub static mut __timer_of_table: of_device_id;
    fn of_device_is_available(np: *const device_node) -> bool;
    fn acpi_probe_device_table(timer: core::ffi::c_int) -> u32;
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn pr_crit(fmt: *const core::ffi::c_char, ...);
}

// The linker section and __used attribute are represented by the corresponding
// Rust/linker-facing attributes supplied by the build environment.
#[used]
#[link_section = "__timer_of_table_end"]
static __timer_of_table_sentinel: of_device_id = of_device_id {
    name: core::ptr::null(),
    type_: core::ptr::null(),
    compatible: core::ptr::null(),
    data: None,
};

// `for_each_matching_node_and_match` is a kernel macro that iterates the
// matching device-tree nodes and assigns the matching table entry.
unsafe extern "C" {
    fn next_matching_node_and_match(
        np: *mut *mut device_node,
        table: *mut of_device_id,
        match_: *mut *const of_device_id,
    ) -> bool;
}

// The `timer` token passed to the ACPI table macro is supplied by the kernel.
unsafe extern "C" {
    static timer: core::ffi::c_int;
}

#[inline]
pub unsafe fn timer_probe() {
    let mut np: *mut device_node;
    let mut match_: *const of_device_id;
    let mut init_func_ret: of_init_fn_1_ret;
    let mut timers: u32 = 0;
    let mut ret: i32;

    while next_matching_node_and_match(&mut np, &mut __timer_of_table, &mut match_) {
        if !of_device_is_available(np) {
            continue;
        }

        init_func_ret = (*match_).data.expect("of_device_id.data");

        ret = init_func_ret(np);
        if ret != 0 {
            if ret != -517 {
                pr_err(
                    b"Failed to initialize '%pOF': %d\0".as_ptr() as *const _,
                    np,
                    ret,
                );
            }
            continue;
        }

        timers = timers.wrapping_add(1);
    }

    timers = timers.wrapping_add(acpi_probe_device_table(timer));

    if timers == 0 {
        pr_crit(b"%s: no matching timers found\0".as_ptr() as *const _, b"timer_probe\0".as_ptr());
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
