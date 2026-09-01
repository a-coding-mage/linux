// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
// Copyright(c) 2024 Intel Corporation.

/*
 * SDCA Function Device management
 */

// C dependency intent:
// <linux/acpi.h>
// <linux/module.h>
// <linux/auxiliary_bus.h>
// <linux/soundwire/sdw.h>
// <sound/sdca.h>
// <sound/sdca_function.h>
// "sdca_function_device.h"

use core::ffi::{c_char, c_int, c_void};
use core::ptr::null_mut;

extern "C" {
    static GFP_KERNEL: gfp_t;

    fn ida_free(ida: *mut ida, id: c_int);
    fn ida_alloc(ida: *mut ida, gfp: gfp_t) -> c_int;
    fn kfree(ptr: *mut c_void);
    fn kzalloc(size: usize, flags: gfp_t) -> *mut c_void;

    fn auxiliary_device_init(auxdev: *mut auxiliary_device) -> c_int;
    fn auxiliary_device_add(auxdev: *mut auxiliary_device) -> c_int;
    fn auxiliary_device_delete(auxdev: *mut auxiliary_device);
    fn auxiliary_device_uninit(auxdev: *mut auxiliary_device);

    fn to_auxiliary_dev(dev: *mut device) -> *mut auxiliary_device;
    fn auxiliary_dev_to_sdca_dev(auxdev: *mut auxiliary_device) -> *mut sdca_dev;

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);

    fn ERR_PTR(error: isize) -> *mut sdca_dev;
    fn IS_ERR(ptr: *const sdca_dev) -> bool;
    fn PTR_ERR(ptr: *const sdca_dev) -> c_int;
}

/*
 * A SoundWire device can have multiple SDCA functions identified by
 * their type and ADR. there can be multiple SoundWire devices per
 * link, or multiple devices spread across multiple links. An IDA is
 * required to identify each instance.
 */
static mut sdca_function_ida: ida = IDA_INIT(sdca_function_ida);

unsafe extern "C" fn sdca_dev_release(dev: *mut device) {
    let auxdev: *mut auxiliary_device = to_auxiliary_dev(dev);
    let sdev: *mut sdca_dev = auxiliary_dev_to_sdca_dev(auxdev);

    ida_free(&raw mut sdca_function_ida, (*auxdev).id);
    kfree(sdev as *mut c_void);
}

/* alloc, init and add link devices */
unsafe fn sdca_dev_register(
    parent: *mut device,
    function_desc: *mut sdca_function_desc,
    swft: *mut acpi_table_swft,
) -> *mut sdca_dev {
    let sdev: *mut sdca_dev;
    let auxdev: *mut auxiliary_device;
    let mut ret: c_int;
    let rc: c_int;

    sdev = kzalloc(core::mem::size_of::<sdca_dev>(), GFP_KERNEL) as *mut sdca_dev;
    if sdev.is_null() {
        return ERR_PTR(-(ENOMEM as isize));
    }

    auxdev = &raw mut (*sdev).auxdev;
    (*auxdev).name = (*function_desc).name;
    (*auxdev).dev.parent = parent;
    (*auxdev).dev.fwnode = (*function_desc).node;
    (*auxdev).dev.release = Some(sdca_dev_release);

    (*sdev).function.desc = function_desc;
    (*sdev).function.fdl_data.swft = swft;

    rc = ida_alloc(&raw mut sdca_function_ida, GFP_KERNEL);
    if rc < 0 {
        kfree(sdev as *mut c_void);
        return ERR_PTR(rc as isize);
    }
    (*auxdev).id = rc;

    /* now follow the two-step init/add sequence */
    ret = auxiliary_device_init(auxdev);
    if ret < 0 {
        dev_err(
            parent,
            c"failed to initialize SDCA function dev %s\n".as_ptr(),
            (*function_desc).name,
        );
        ida_free(&raw mut sdca_function_ida, (*auxdev).id);
        kfree(sdev as *mut c_void);
        return ERR_PTR(ret as isize);
    }

    ret = auxiliary_device_add(auxdev);
    if ret < 0 {
        dev_err(
            parent,
            c"failed to add SDCA function dev %s\n".as_ptr(),
            (*sdev).auxdev.name,
        );
        /* sdev will be freed with the put_device() and .release sequence */
        auxiliary_device_uninit(&raw mut (*sdev).auxdev);
        return ERR_PTR(ret as isize);
    }

    sdev
}

unsafe fn sdca_dev_unregister(sdev: *mut sdca_dev) {
    if sdev.is_null() {
        return;
    }

    auxiliary_device_delete(&raw mut (*sdev).auxdev);
    auxiliary_device_uninit(&raw mut (*sdev).auxdev);
}

#[no_mangle]
pub unsafe extern "C" fn sdca_dev_register_functions(slave: *mut sdw_slave) -> c_int {
    let sdca_data: *mut sdca_device_data = &raw mut (*slave).sdca_data;
    let mut i: c_int;
    let ret: c_int;

    i = 0;
    while i < (*sdca_data).num_functions {
        let func_dev: *mut sdca_dev;

        func_dev = sdca_dev_register(
            &raw mut (*slave).dev,
            &raw mut *(*sdca_data).function.offset(i as isize),
            (*sdca_data).swft,
        );
        if IS_ERR(func_dev) {
            ret = PTR_ERR(func_dev);
            /*
             * Unregister functions that were successfully
             * registered before this failure. This also
             * sets func_dev to NULL so the caller will not
             * try to unregister them again.
             */
            sdca_dev_unregister_functions(slave);
            return ret;
        }

        (*(*sdca_data).function.offset(i as isize)).func_dev = func_dev;
        i += 1;
    }

    0
}
// EXPORT_SYMBOL_NS(sdca_dev_register_functions, "SND_SOC_SDCA");

#[no_mangle]
pub unsafe extern "C" fn sdca_dev_unregister_functions(slave: *mut sdw_slave) {
    let sdca_data: *mut sdca_device_data = &raw mut (*slave).sdca_data;
    let mut i: c_int;

    i = 0;
    while i < (*sdca_data).num_functions {
        if (*(*sdca_data).function.offset(i as isize)).func_dev.is_null() {
            i += 1;
            continue;
        }

        sdca_dev_unregister((*(*sdca_data).function.offset(i as isize)).func_dev);
        (*(*sdca_data).function.offset(i as isize)).func_dev = null_mut();
        i += 1;
    }
}
// EXPORT_SYMBOL_NS(sdca_dev_unregister_functions, "SND_SOC_SDCA");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
