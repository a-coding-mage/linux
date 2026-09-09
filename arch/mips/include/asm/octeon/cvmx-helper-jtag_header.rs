/***********************license start***************
 * Author: Cavium Networks
 *
 * Contact: support@caviumnetworks.com
 * This file is part of the OCTEON SDK
 *
 * Copyright (c) 2003-2008 Cavium Networks
 *
 * This file is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License, version 2, as
 * published by the Free Software Foundation.
 *
 * This file is distributed in the hope that it will be useful, but
 * AS-IS and WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, TITLE, or
 * NONINFRINGEMENT.  See the GNU General Public License for more
 * details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this file; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA
 * or visit http://www.gnu.org/licenses/.
 *
 * This file may also be available under a different license from Cavium.
 * Contact Cavium Networks for more information
 ***********************license end**************************************/

/**
 * @file
 *
 *  Helper utilities for qlm_jtag.
 *
 */

// Original C header guard: __CVMX_HELPER_JTAG_H__

extern "C" {
    pub fn cvmx_helper_qlm_jtag_init();
    pub fn cvmx_helper_qlm_jtag_shift(qlm: core::ffi::c_int, bits: core::ffi::c_int, data: u32) -> u32;
    pub fn cvmx_helper_qlm_jtag_shift_zeros(qlm: core::ffi::c_int, bits: core::ffi::c_int);
    pub fn cvmx_helper_qlm_jtag_update(qlm: core::ffi::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
