/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright © 2001      Red Hat UK Limited
 * Copyright © 2001-2010 David Woodhouse <dwmw2@infradead.org>
 */

// Dependencies supplied by the corresponding Linux MTD headers:
// linux/mtd/flashchip.h, linux/mtd/map.h, linux/mtd/cfi.h, linux/bitops.h

use core::ffi::{c_char, c_int, c_ulong};

// Opaque declarations corresponding to the externally supplied C types.
#[repr(C)]
pub struct map_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cfi_private {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtd_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct chip_probe {
    pub name: *mut c_char,
    pub probe_chip: Option<
        unsafe extern "C" fn(
            map: *mut map_info,
            base: u32,
            chip_map: *mut c_ulong,
            cfi: *mut cfi_private,
        ) -> c_int,
    >,
}

unsafe extern "C" {
    pub fn mtd_do_chip_probe(
        map: *mut map_info,
        cp: *mut chip_probe,
    ) -> *mut mtd_info;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
