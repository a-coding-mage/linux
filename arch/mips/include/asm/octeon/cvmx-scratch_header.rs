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

/**
 *
 * This file provides support for the processor local scratch memory.
 * Scratch memory is byte addressable - all addresses are byte addresses.
 *
 */

// The C header guard is omitted in Rust.

/*
 * Note: This define must be a long, not a long long in order to
 * compile without warnings for both 32bit and 64bit.
 */
pub const CVMX_SCRATCH_BASE: i64 = -32768; /* 0xffffffffffff8000 */

/**
 * Reads an 8 bit value from the processor local scratchpad memory.
 *
 * @address: byte address to read from
 *
 * Returns value read
 */
#[inline]
pub unsafe fn cvmx_scratch_read8(address: u64) -> u8 {
    core::ptr::read_volatile((CVMX_SCRATCH_BASE as u64).wrapping_add(address) as *const u8)
}

/**
 * Reads a 16 bit value from the processor local scratchpad memory.
 *
 * @address: byte address to read from
 *
 * Returns value read
 */
#[inline]
pub unsafe fn cvmx_scratch_read16(address: u64) -> u16 {
    core::ptr::read_volatile((CVMX_SCRATCH_BASE as u64).wrapping_add(address) as *const u16)
}

/**
 * Reads a 32 bit value from the processor local scratchpad memory.
 *
 * @address: byte address to read from
 *
 * Returns value read
 */
#[inline]
pub unsafe fn cvmx_scratch_read32(address: u64) -> u32 {
    core::ptr::read_volatile((CVMX_SCRATCH_BASE as u64).wrapping_add(address) as *const u32)
}

/**
 * Reads a 64 bit value from the processor local scratchpad memory.
 *
 * @address: byte address to read from
 *
 * Returns value read
 */
#[inline]
pub unsafe fn cvmx_scratch_read64(address: u64) -> u64 {
    core::ptr::read_volatile((CVMX_SCRATCH_BASE as u64).wrapping_add(address) as *const u64)
}

/**
 * Writes an 8 bit value to the processor local scratchpad memory.
 *
 * @address: byte address to write to
 * @value:   value to write
 */
#[inline]
pub unsafe fn cvmx_scratch_write8(address: u64, value: u64) {
    core::ptr::write_volatile(
        (CVMX_SCRATCH_BASE as u64).wrapping_add(address) as *mut u8,
        value as u8,
    );
}

/**
 * Writes a 32 bit value to the processor local scratchpad memory.
 *
 * @address: byte address to write to
 * @value:   value to write
 */
#[inline]
pub unsafe fn cvmx_scratch_write16(address: u64, value: u64) {
    core::ptr::write_volatile(
        (CVMX_SCRATCH_BASE as u64).wrapping_add(address) as *mut u16,
        value as u16,
    );
}

/**
 * Writes a 16 bit value to the processor local scratchpad memory.
 *
 * @address: byte address to write to
 * @value:   value to write
 */
#[inline]
pub unsafe fn cvmx_scratch_write32(address: u64, value: u64) {
    core::ptr::write_volatile(
        (CVMX_SCRATCH_BASE as u64).wrapping_add(address) as *mut u32,
        value as u32,
    );
}

/**
 * Writes a 64 bit value to the processor local scratchpad memory.
 *
 * @address: byte address to write to
 * @value:   value to write
 */
#[inline]
pub unsafe fn cvmx_scratch_write64(address: u64, value: u64) {
    core::ptr::write_volatile(
        (CVMX_SCRATCH_BASE as u64).wrapping_add(address) as *mut u64,
        value,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
