// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/io.c - Machine independent I/O functions.
 *
 * Copyright (C) 2000 - 2009  Stuart Menefy
 * Copyright (C) 2005  Paul Mundt
 */

/* External kernel primitives supplied by the surrounding platform. */
extern "C" {
    fn mb();
    fn writeb(value: i32, address: *mut core::ffi::c_void);
}

/*
 * Copy data from IO memory space to "real" memory space.
 */
pub unsafe fn memcpy_fromio(
    mut to: *mut core::ffi::c_void,
    mut from: *const core::ffi::c_void,
    mut count: usize,
) {
    /*
     * Would it be worthwhile doing byte and long transfers first
     * to try and get aligned?
     */
    // CONFIG_CPU_SH4 contains an architecture-specific SH assembly fast path.
    // Its inline assembly is intentionally left as a conditional dependency;
    // the portable word/byte path below preserves the operation and ordering.

    if ((((to as usize) | (from as usize)) & 0x3) == 0) {
        while count > 3 {
            core::ptr::write(to as *mut u32, core::ptr::read_volatile(from as *const u32));
            to = (to as *mut u8).add(4) as *mut core::ffi::c_void;
            from = (from as *const u8).add(4) as *const core::ffi::c_void;
            count -= 4;
        }
    }

    while count > 0 {
        core::ptr::write(to as *mut u8, core::ptr::read_volatile(from as *const u8));
        to = (to as *mut u8).add(1) as *mut core::ffi::c_void;
        from = (from as *const u8).add(1) as *const core::ffi::c_void;
        count -= 1;
    }

    mb();
}

/*
 * Copy data from "real" memory space to IO memory space.
 */
pub unsafe fn memcpy_toio(
    mut to: *mut core::ffi::c_void,
    mut from: *const core::ffi::c_void,
    mut count: usize,
) {
    if ((((to as usize) | (from as usize)) & 0x3) == 0) {
        while count > 3 {
            core::ptr::write_volatile(to as *mut u32, core::ptr::read(from as *const u32));
            to = (to as *mut u8).add(4) as *mut core::ffi::c_void;
            from = (from as *const u8).add(4) as *const core::ffi::c_void;
            count -= 4;
        }
    }

    while count > 0 {
        core::ptr::write_volatile(to as *mut u8, core::ptr::read(from as *const u8));
        to = (to as *mut u8).add(1) as *mut core::ffi::c_void;
        from = (from as *const u8).add(1) as *const core::ffi::c_void;
        count -= 1;
    }

    mb();
}

/*
 * "memset" on IO memory space.
 * This needs to be optimized.
 */
pub unsafe fn memset_io(
    mut dst: *mut core::ffi::c_void,
    c: i32,
    mut count: usize,
) {
    while count != 0 {
        count -= 1;
        writeb(c, dst);
        dst = (dst as *mut u8).add(1) as *mut core::ffi::c_void;
    }
}

// EXPORT_SYMBOL(memcpy_fromio);
// EXPORT_SYMBOL(memcpy_toio);
// EXPORT_SYMBOL(memset_io);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
