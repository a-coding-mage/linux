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

// The C header includes <uapi/asm/bitfield.h>; its bitfield declarations are
// represented below by their underlying 64-bit storage.

pub const CVMX_L2T_ERR: u64 = CVMX_ADD_IO_SEG(0x0001_1800_8000_0008u64);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CvmxL2tErrS {
    // reserved_29_63:35, fadru:1, lck_intena2:1, lckerr2:1,
    // lck_intena:1, lckerr:1, fset:3, fadr:10, fsyn:6,
    // ded_err:1, sec_err:1, ded_intena:1, sec_intena:1, ecc_ena:1
    pub bits: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CvmxL2tErrCn30xx {
    // reserved_28_63:36, lck_intena2:1, lckerr2:1, lck_intena:1,
    // lckerr:1, reserved_23_23:1, fset:2, reserved_19_20:2, fadr:8,
    // fsyn:6, ded_err:1, sec_err:1, ded_intena:1, sec_intena:1, ecc_ena:1
    pub bits: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CvmxL2tErrCn31xx {
    // reserved_28_63:36, lck_intena2:1, lckerr2:1, lck_intena:1,
    // lckerr:1, reserved_23_23:1, fset:2, reserved_20_20:1, fadr:9,
    // fsyn:6, ded_err:1, sec_err:1, ded_intena:1, sec_intena:1, ecc_ena:1
    pub bits: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CvmxL2tErrCn38xx {
    // reserved_28_63:36, lck_intena2:1, lckerr2:1, lck_intena:1,
    // lckerr:1, fset:3, fadr:10, fsyn:6, ded_err:1, sec_err:1,
    // ded_intena:1, sec_intena:1, ecc_ena:1
    pub bits: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CvmxL2tErrCn50xx {
    // reserved_28_63:36, lck_intena2:1, lckerr2:1, lck_intena:1,
    // lckerr:1, fset:3, reserved_18_20:3, fadr:7, fsyn:6, ded_err:1,
    // sec_err:1, ded_intena:1, sec_intena:1, ecc_ena:1
    pub bits: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CvmxL2tErrCn52xx {
    // reserved_28_63:36, lck_intena2:1, lckerr2:1, lck_intena:1,
    // lckerr:1, fset:3, reserved_20_20:1, fadr:9, fsyn:6, ded_err:1,
    // sec_err:1, ded_intena:1, sec_intena:1, ecc_ena:1
    pub bits: u64,
}

#[repr(C)]
pub union CvmxL2tErr {
    pub u64_: u64,
    pub s: CvmxL2tErrS,
    pub cn30xx: CvmxL2tErrCn30xx,
    pub cn31xx: CvmxL2tErrCn31xx,
    pub cn38xx: CvmxL2tErrCn38xx,
    pub cn50xx: CvmxL2tErrCn50xx,
    pub cn52xx: CvmxL2tErrCn52xx,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
