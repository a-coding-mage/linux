// SPDX-License-Identifier: GPL-2.0
/*
 * init.c:  Initialize internal variables used by the PROM
 *          library functions.
 *
 * Copyright (C) 1995 David S. Miller (davem@caip.rutgers.edu)
 */

// Dependencies supplied by the corresponding Linux and SPARC headers:
// linux_romvec, prom_major_version, and linux_nodeops.

pub static mut romvec: *mut linux_romvec = core::ptr::null_mut();
pub static mut prom_vers: prom_major_version = unsafe { core::mem::zeroed() };
pub static mut prom_rev: core::ffi::c_uint = 0;
pub static mut prom_prev: core::ffi::c_uint = 0;

/* The root node of the prom device tree. */
pub static mut prom_root_node: core::ffi::c_int = 0;

/* Pointer to the device tree operations structure. */
pub static mut prom_nodeops: *mut linux_nodeops = core::ptr::null_mut();

/* You must call prom_init() before you attempt to use any of the
 * routines in the prom library.
 * It gets passed the pointer to the PROM vector.
 */

#[no_mangle]
pub unsafe extern "C" fn prom_init(rp: *mut linux_romvec) {
    romvec = rp;

    /* Initialization successful. */
    return;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
