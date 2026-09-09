/*
 * MDIO bus multiplexer framwork.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2011, 2012 Cavium, Inc.
 */

// Dependencies supplied by the corresponding Linux device and PHY headers
// remain external to this translation.

use core::ffi::c_void;

/* mdio_mux_init() - Initialize a MDIO mux
 * @dev\t\tThe device owning the MDIO mux
 * @mux_node\tThe device node of the MDIO mux
 * @switch_fn\tThe function called for switching target MDIO child
 * mux_handle\tA pointer to a (void *) used internaly by mdio-mux
 * @data\tPrivate data used by switch_fn()
 * @mux_bus\tAn optional parent bus (Other case are to use parent_bus property)
 */
pub unsafe extern "C" fn mdio_mux_init(
    dev: *mut device,
    mux_node: *mut device_node,
    switch_fn: Option<unsafe extern "C" fn(cur: i32, desired: i32, data: *mut c_void) -> i32>,
    mux_handle: *mut *mut c_void,
    data: *mut c_void,
    mux_bus: *mut mii_bus,
) -> i32;

pub unsafe extern "C" fn mdio_mux_uninit(mux_handle: *mut c_void);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
