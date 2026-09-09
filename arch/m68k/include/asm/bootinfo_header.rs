/*
** asm/bootinfo.h -- Definition of the Linux/m68k boot information structure
**
** Copyright 1992 by Greg Harp
**
** This file is subject to the terms and conditions of the GNU General Public
** License.  See the file COPYING in the main directory of this archive
** for more details.
*/

// Dependency supplied by <uapi/asm/bootinfo.h>.
use crate::bi_record;

// CONFIG_BOOTINFO_PROC
#[cfg(feature = "CONFIG_BOOTINFO_PROC")]
extern "C" {
    pub fn save_bootinfo(bi: *const bi_record);
}

// CONFIG_BOOTINFO_PROC is not enabled.
#[cfg(not(feature = "CONFIG_BOOTINFO_PROC"))]
#[inline]
pub unsafe fn save_bootinfo(_bi: *const bi_record) {}

// CONFIG_UBOOT
#[cfg(feature = "CONFIG_UBOOT")]
extern "C" {
    pub fn process_uboot_commandline(commandp: *mut core::ffi::c_char, size: core::ffi::c_int);
}

// CONFIG_UBOOT is not enabled.
#[cfg(not(feature = "CONFIG_UBOOT"))]
#[inline]
pub unsafe fn process_uboot_commandline(
    _commandp: *mut core::ffi::c_char,
    _size: core::ffi::c_int,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
