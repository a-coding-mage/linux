/* SPDX-License-Identifier: GPL-2.0+ */
/*
 *  comedi/drivers/ni_routing/ni_route_values.h
 *  Route information for NI boards.
 *
 *  COMEDI - Linux Control and Measurement Device Interface
 *  Copyright (C) 2016 Spencer E. Olson <olsonse@umich.edu>
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 */

// Dependencies supplied by the surrounding translation unit:
// linux/comedi.h and linux/types.h

/*
 * This file includes the tables that are a list of all the values of various
 * signals routes available on NI hardware.  In many cases, one does not
 * explicitly make these routes, rather one might indicate that something is
 * used as the source of one particular trigger or another (using
 * *_src=TRIG_EXT).
 *
 * This file is meant to be included by comedi/drivers/ni_routes.c
 */

#[inline]
pub const fn b(x: i32) -> i32 {
    x - NI_NAMES_BASE
}

/** Marks a register value as valid, implemented, and tested. */
#[inline]
pub const fn v(x: i32) -> i32 {
    (x & 0x7f) | 0x80
}

/*
 * The C header selects this branch unless NI_ROUTE_VALUE_EXTERNAL_CONVERSION
 * is defined by the build.
 */
#[cfg(not(feature = "NI_ROUTE_VALUE_EXTERNAL_CONVERSION"))]
pub type register_type = u8;

#[cfg(not(feature = "NI_ROUTE_VALUE_EXTERNAL_CONVERSION"))]
#[inline]
pub const fn i(x: i32) -> i32 {
    v(x)
}

#[cfg(not(feature = "NI_ROUTE_VALUE_EXTERNAL_CONVERSION"))]
#[inline]
pub const fn u(_x: i32) -> i32 {
    0x0
}

#[cfg(feature = "NI_ROUTE_VALUE_EXTERNAL_CONVERSION")]
pub type register_type = u16;

#[cfg(feature = "NI_ROUTE_VALUE_EXTERNAL_CONVERSION")]
#[inline]
pub const fn i(x: i32) -> i32 {
    (x & 0x7f) | 0x100
}

#[cfg(feature = "NI_ROUTE_VALUE_EXTERNAL_CONVERSION")]
#[inline]
pub const fn u(x: i32) -> i32 {
    (x & 0x7f) | 0x200
}

#[cfg(feature = "NI_ROUTE_VALUE_EXTERNAL_CONVERSION")]
#[inline]
pub const fn marked_v(x: i32) -> bool {
    (x & 0x80) != 0
}

#[cfg(feature = "NI_ROUTE_VALUE_EXTERNAL_CONVERSION")]
#[inline]
pub const fn marked_i(x: i32) -> bool {
    (x & 0x100) != 0
}

#[cfg(feature = "NI_ROUTE_VALUE_EXTERNAL_CONVERSION")]
#[inline]
pub const fn marked_u(x: i32) -> bool {
    (x & 0x200) != 0
}

/* Mask out the marking bit(s). */
#[inline]
pub const fn unmark(x: i32) -> i32 {
    x & 0x7f
}

/*
 * Gi_SRC(x,1) implements Gi_Src_SubSelect = 1
 *
 * This appears to only really be a valid MUX for m-series devices.
 */
#[inline]
pub const fn gi_src(val: i32, subsel: i32) -> i32 {
    val | (subsel << 6)
}

/**
 * struct family_route_values - Register values for all routes for a particular
 *                              family.
 * @family: lower-case string representation of a specific series or family of
 *          devices from National Instruments where each member of this family
 *          shares the same register values for the various signal MUXes.  It
 *          should be noted that not all devices of any family have access to
 *          all routes defined.
 * @register_values: Table of all register values for various signal MUXes on
 *          National Instruments devices.  The first index of this table is the
 *          signal destination (i.e. identification of the signal MUX).  The
 *          second index of this table is the signal source (i.e. input of the
 *          signal MUX).
 */
#[repr(C)]
pub struct family_route_values {
    pub family: *const core::ffi::c_char,
    pub register_values: [[register_type; NI_NUM_NAMES]; NI_NUM_NAMES],
}

unsafe extern "C" {
    pub static ni_all_route_values: [*const family_route_values; NI_NUM_NAMES];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
