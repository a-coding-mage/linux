// SPDX-License-Identifier: GPL-2.0-only
/*
 * MIPS support for CONFIG_OF device tree support
 *
 * Copyright (C) 2010 Cisco Systems Inc. <dediao@cisco.com>
 */

use core::ffi::{c_char, c_void};

// Kernel-provided declarations from the corresponding C headers.
#[repr(C)]
pub struct OfDeviceId {
    pub name: [c_char; 32],
    pub type_: [c_char; 32],
    pub compatible: [c_char; 128],
    pub data: *const c_void,
}

extern "C" {
    fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
    fn pr_info(fmt: *const c_char, ...);
    fn dump_stack_set_arch_desc(desc: *const c_char);
    fn early_init_dt_scan(bph: *mut c_void, mem: usize) -> bool;
    fn __pa(addr: *mut c_void) -> usize;
    fn of_flat_dt_get_machine_name() -> *const c_char;
    fn of_have_populated_dt() -> bool;
    fn of_platform_populate(
        root: *const c_void,
        matches: *const OfDeviceId,
        lookup: *const c_void,
        parent: *const c_void,
    ) -> i32;
    fn unflatten_and_copy_device_tree();
}

static mut MIPS_MACHINE_NAME: [c_char; 64] = {
    let mut value = [0 as c_char; 64];
    value[0] = b'U' as c_char;
    value[1] = b'n' as c_char;
    value[2] = b'k' as c_char;
    value[3] = b'n' as c_char;
    value[4] = b'o' as c_char;
    value[5] = b'w' as c_char;
    value[6] = b'n' as c_char;
    value
};

// __init
pub unsafe extern "C" fn mips_set_machine_name(name: *const c_char) {
    if name.is_null() {
        return;
    }

    strscpy(
        MIPS_MACHINE_NAME.as_mut_ptr(),
        name,
        core::mem::size_of::<[c_char; 64]>(),
    );
    pr_info(
        b"MIPS: machine is %s\n\0".as_ptr() as *const c_char,
        mips_get_machine_name(),
    );

    dump_stack_set_arch_desc(name);
}

pub unsafe extern "C" fn mips_get_machine_name() -> *mut c_char {
    MIPS_MACHINE_NAME.as_mut_ptr()
}

// CONFIG_USE_OF

// __init
pub unsafe extern "C" fn __dt_setup_arch(bph: *mut c_void) {
    if !early_init_dt_scan(bph, __pa(bph)) {
        return;
    }

    mips_set_machine_name(of_flat_dt_get_machine_name());
}

// __init
pub unsafe extern "C" fn __dt_register_buses(
    bus0: *const c_char,
    bus1: *const c_char,
) -> i32 {
    let mut of_ids: [OfDeviceId; 3] = core::mem::zeroed();

    if !of_have_populated_dt() {
        panic!("device tree not present");
    }

    strscpy(
        of_ids[0].compatible.as_mut_ptr(),
        bus0,
        core::mem::size_of_val(&of_ids[0].compatible),
    );
    if !bus1.is_null() {
        strscpy(
            of_ids[1].compatible.as_mut_ptr(),
            bus1,
            core::mem::size_of_val(&of_ids[1].compatible),
        );
    }

    if of_platform_populate(
        core::ptr::null(),
        of_ids.as_ptr(),
        core::ptr::null(),
        core::ptr::null(),
    ) != 0
    {
        panic!("failed to populate DT");
    }

    0
}

// __weak __init
pub unsafe extern "C" fn device_tree_init() {
    unflatten_and_copy_device_tree();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
