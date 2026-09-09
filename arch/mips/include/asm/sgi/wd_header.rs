/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2007 by Ralf Baechle
 */

// Dependency supplied by the corresponding asm/sgi/hpc3 header.

#[repr(C)]
pub struct sgiwd93_platform_data {
    pub unit: ::core::ffi::c_uint,
    pub irq: ::core::ffi::c_uint,
    pub hregs: *mut hpc3_scsiregs,
    pub wdregs: *mut ::core::ffi::c_uchar,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
