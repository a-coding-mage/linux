/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1999 Silicon Graphics, Inc.
 * Copyright (C) 1999 by Ralf Baechle
 */

// Dependency supplied by the translated Linux types definitions.

#[cfg(not(asm))]
pub type cpuid_t = ::core::ffi::c_ulong;

#[cfg(not(asm))]
pub type nasid_t = i16; // node id in numa-as-id space

#[cfg(not(asm))]
pub type partid_t = i8; // partition ID type

#[cfg(not(asm))]
pub type moduleid_t = i16; // user-visible module number type

#[cfg(not(asm))]
pub type vertex_hdl_t = dev_t; // hardware graph vertex handle

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
