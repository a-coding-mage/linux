// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  Registration of Cobalt LCD platform device.
 *
 *  Copyright (C) 2008  Yoichi Yuasa <yuasa@linux-mips.org>
 */

// C dependencies supplied by the surrounding kernel bindings.
use crate::bindings;

// `__initdata`: this resource is only needed during initialization.
static mut COBALT_LCD_RESOURCE: bindings::resource = bindings::resource {
    start: 0x1f000000,
    end: 0x1f00001f,
    flags: bindings::IORESOURCE_MEM as _,
    ..unsafe { core::mem::zeroed() }
};

unsafe fn cobalt_lcd_add() -> i32 {
    let pdev: *mut bindings::platform_device;
    let retval: i32;

    pdev = bindings::platform_device_alloc(b"cobalt-lcd\0".as_ptr() as *const _, -1);
    if pdev.is_null() {
        return -(bindings::ENOMEM as i32);
    }

    retval = bindings::platform_device_add_resources(
        pdev,
        &raw const COBALT_LCD_RESOURCE,
        1,
    );
    if retval != 0 {
        bindings::platform_device_put(pdev);
        return retval;
    }

    retval = bindings::platform_device_add(pdev);
    if retval != 0 {
        bindings::platform_device_put(pdev);
        return retval;
    }

    0
}

// Equivalent of `device_initcall(cobalt_lcd_add)`; registration is supplied
// by the surrounding kernel initialization mechanism.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
