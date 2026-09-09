// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2000 Philipp Rumpf <prumpf@tux.org>
 * Copyright (C) 2001-2020 Helge Deller <deller@gmx.de>
 * Copyright (C) 2001-2002 Thomas Bogendoerfer <tsbogend@alpha.franken.de>
 */

// Declarations supplied by the Linux video and architecture dependencies.
use core::ffi::c_void;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sti_struct {
    pub dev: *mut device,
}

extern "C" {
    pub fn sti_get_rom(index: i32) -> *mut sti_struct;
}

/// Returns whether `dev` is the primary video device.
#[no_mangle]
pub unsafe extern "C" fn video_is_primary_device(dev: *mut device) -> bool {
    let sti: *mut sti_struct;

    sti = sti_get_rom(0);

    /* if no built-in graphics card found, allow any fb driver as default */
    if sti.is_null() {
        return true;
    }

    /* return true if it's the default built-in framebuffer driver */
    (*sti).dev == dev
}

// EXPORT_SYMBOL(video_is_primary_device);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
