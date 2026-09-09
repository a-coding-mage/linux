/* SPDX-License-Identifier: LGPL-2.0+ WITH Linux-syscall-note */
/*  Generic MTRR (Memory Type Range Register) ioctls.

    Copyright (C) 1997-1999  Richard Gooch

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Library General Public
    License as published by the Free Software Foundation; either
    version 2 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Library General Public License for more details.

    You should have received a copy of the GNU Library General Public
    License along with this library; if not, write to the Free
    Software Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.

    Richard Gooch may be reached by email at  rgooch@atnf.csiro.au
    The postal address is:
      Richard Gooch, c/o ATNF, P. O. Box 76, Epping, N.S.W., 2121, Australia.
*/

// Dependencies supplied by the surrounding translation unit:
// linux/types.h, linux/ioctl.h, and linux/errno.h.

pub const MTRR_IOCTL_BASE: u8 = b'M';

/* Warning: this structure has a different order from i386
   on x86-64. The 32bit emulation code takes care of that.
   But you need to use this for 64bit, otherwise your X server
   will break. */

#[cfg(target_arch = "x86")]
#[repr(C)]
pub struct mtrr_sentry {
    pub base: ::core::ffi::c_ulong, /*  Base address     */
    pub size: ::core::ffi::c_uint,  /*  Size of region   */
    pub type_: ::core::ffi::c_uint, /*  Type of region   */
}

#[cfg(target_arch = "x86")]
#[repr(C)]
pub struct mtrr_gentry {
    pub regnum: ::core::ffi::c_uint, /*  Register number  */
    pub base: ::core::ffi::c_ulong,  /*  Base address     */
    pub size: ::core::ffi::c_uint,   /*  Size of region   */
    pub type_: ::core::ffi::c_uint,  /*  Type of region   */
}

#[cfg(not(target_arch = "x86"))]
#[repr(C)]
pub struct mtrr_sentry {
    pub base: u64, /*  Base address     */
    pub size: u32, /*  Size of region   */
    pub type_: u32, /*  Type of region   */
}

#[cfg(not(target_arch = "x86"))]
#[repr(C)]
pub struct mtrr_gentry {
    pub base: u64, /*  Base address     */
    pub size: u32, /*  Size of region   */
    pub regnum: u32, /*  Register number  */
    pub type_: u32, /*  Type of region   */
    pub _pad: u32, /*  Unused           */
}

#[repr(C)]
pub struct mtrr_var_range {
    pub base_lo: u32,
    pub base_hi: u32,
    pub mask_lo: u32,
    pub mask_hi: u32,
}

/* In the Intel processor's MTRR interface, the MTRR type is always held in
   an 8 bit field: */
pub type mtrr_type = u8;

pub const MTRR_NUM_FIXED_RANGES: u32 = 88;
pub const MTRR_MAX_VAR_RANGES: u32 = 256;

#[inline]
pub const fn MTRRphysBase_MSR(reg: u32) -> u32 { 0x200 + 2 * reg }
#[inline]
pub const fn MTRRphysMask_MSR(reg: u32) -> u32 { 0x200 + 2 * reg + 1 }

/*  These are the various ioctls  */
pub const MTRRIOC_ADD_ENTRY: _ = _IOW(MTRR_IOCTL_BASE, 0, mtrr_sentry);
pub const MTRRIOC_SET_ENTRY: _ = _IOW(MTRR_IOCTL_BASE, 1, mtrr_sentry);
pub const MTRRIOC_DEL_ENTRY: _ = _IOW(MTRR_IOCTL_BASE, 2, mtrr_sentry);
pub const MTRRIOC_GET_ENTRY: _ = _IOWR(MTRR_IOCTL_BASE, 3, mtrr_gentry);
pub const MTRRIOC_KILL_ENTRY: _ = _IOW(MTRR_IOCTL_BASE, 4, mtrr_sentry);
pub const MTRRIOC_ADD_PAGE_ENTRY: _ = _IOW(MTRR_IOCTL_BASE, 5, mtrr_sentry);
pub const MTRRIOC_SET_PAGE_ENTRY: _ = _IOW(MTRR_IOCTL_BASE, 6, mtrr_sentry);
pub const MTRRIOC_DEL_PAGE_ENTRY: _ = _IOW(MTRR_IOCTL_BASE, 7, mtrr_sentry);
pub const MTRRIOC_GET_PAGE_ENTRY: _ = _IOWR(MTRR_IOCTL_BASE, 8, mtrr_gentry);
pub const MTRRIOC_KILL_PAGE_ENTRY: _ = _IOW(MTRR_IOCTL_BASE, 9, mtrr_sentry);

/* MTRR memory types, which are defined in SDM */
pub const MTRR_TYPE_UNCACHABLE: u32 = 0;
pub const MTRR_TYPE_WRCOMB: u32 = 1;
/* #define MTRR_TYPE_ 2 */
/* #define MTRR_TYPE_ 3 */
pub const MTRR_TYPE_WRTHROUGH: u32 = 4;
pub const MTRR_TYPE_WRPROT: u32 = 5;
pub const MTRR_TYPE_WRBACK: u32 = 6;
pub const MTRR_NUM_TYPES: u32 = 7;

/*
 * Invalid MTRR memory type.  No longer used outside of MTRR code.
 * Note, this value is allocated from the reserved values (0x7-0xff) of
 * the MTRR memory types.
 */
pub const MTRR_TYPE_INVALID: u8 = 0xff;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
