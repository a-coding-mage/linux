/***********************license start***************
 * Author: Cavium Networks
 *
 * Contact: support@caviumnetworks.com
 * This file is part of the OCTEON SDK
 *
 * Copyright (c) 2003-2017 Cavium, Inc.
 *
 * This file is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License, Version 2, as
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

// The C header guard and include dependencies are intentionally omitted.
// CVMX_ADD_IO_SEG is a platform-provided address-mapping macro; these values
// preserve the underlying register addresses until that mapping is supplied.
pub const CVMX_L2D_ERR: u64 = 0x0001180080000010;
pub const CVMX_L2D_FUS3: u64 = 0x00011800800007B8;

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_l2d_err {
    pub u64_: u64,
    pub s: cvmx_l2d_err_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_l2d_err_s {
    // C bitfield layout, from least significant bit upward:
    // ecc_ena:1, sec_intena:1, ded_intena:1, sec_err:1, ded_err:1,
    // bmhclsel:1, reserved_6_63:58.
    pub bits: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvmx_l2d_fus3 {
    pub u64_: u64,
    pub s: cvmx_l2d_fus3_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_l2d_fus3_s {
    // C bitfield layout, from least significant bit upward:
    // q3fus:34, reserved_34_36:3, ema_ctl:3, reserved_40_63:24.
    pub bits: u64,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
