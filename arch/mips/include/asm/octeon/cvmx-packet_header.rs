/***********************license start***************
 * Author: Cavium Networks
 *
 * Contact: support@caviumnetworks.com
 * This file is part of the OCTEON SDK
 *
 * Copyright (c) 2003-2008 Cavium Networks
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

/* Packet buffer defines. */

use core::ffi::c_void;

/// This structure defines a buffer pointer on Octeon.
#[repr(C)]
pub union cvmx_buf_ptr {
    pub ptr: *mut c_void,
    pub u64: u64,
    pub s: cvmx_buf_ptr_s,
}

/// Bit-field representation of `cvmx_buf_ptr.s`.
///
/// C bit-fields are represented by their packed 64-bit storage word.  The
/// masks and shifts preserve the source layout for both bit-field orders.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvmx_buf_ptr_s {
    pub bits: u64,
}

impl cvmx_buf_ptr_s {
    #[cfg(target_endian = "big")]
    pub const I_SHIFT: u32 = 63;
    #[cfg(target_endian = "big")]
    pub const BACK_SHIFT: u32 = 59;
    #[cfg(target_endian = "big")]
    pub const POOL_SHIFT: u32 = 56;
    #[cfg(target_endian = "big")]
    pub const SIZE_SHIFT: u32 = 40;
    #[cfg(target_endian = "big")]
    pub const ADDR_SHIFT: u32 = 0;

    #[cfg(target_endian = "little")]
    pub const ADDR_SHIFT: u32 = 0;
    #[cfg(target_endian = "little")]
    pub const SIZE_SHIFT: u32 = 40;
    #[cfg(target_endian = "little")]
    pub const POOL_SHIFT: u32 = 56;
    #[cfg(target_endian = "little")]
    pub const BACK_SHIFT: u32 = 59;
    #[cfg(target_endian = "little")]
    pub const I_SHIFT: u32 = 63;

    pub const I_MASK: u64 = 0x1;
    pub const BACK_MASK: u64 = 0xf;
    pub const POOL_MASK: u64 = 0x7;
    pub const SIZE_MASK: u64 = 0xffff;
    pub const ADDR_MASK: u64 = 0xffffffffff;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
