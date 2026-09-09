/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * SGI ARCS firmware interface library for the Linux kernel.
 *
 * Copyright (C) 1996 David S. Miller (davem@davemloft.net)
 * Copyright (C) 2001, 2002 Ralf Baechle (ralf@gnu.org)
 */

// Types supplied by the corresponding ARCS interface dependency.
#[repr(C)]
pub struct linux_romvec {
    _private: [u8; 0],
}

#[repr(C)]
pub struct linux_mdesc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pcomponent {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DISPLAY_STATUS {
    _private: [u8; 0],
}

pub type PCHAR = *mut ::core::ffi::c_char;
pub type LONG = ::core::ffi::c_long;
pub type ULONG = ::core::ffi::c_ulong;
pub type PULONG = *mut ULONG;
pub type PVOID = *mut ::core::ffi::c_void;
pub type VOID = ();

extern "C" {
    pub static mut romvec: *mut linux_romvec;

    pub static mut prom_flags: ::core::ffi::c_int;

    /* Simple char-by-char console I/O. */
    pub fn prom_getchar() -> ::core::ffi::c_char;

    /* Get next memory descriptor after CURR, returns first descriptor
     * in chain if CURR is NULL.
     */
    pub fn prom_getmdesc(curr: *mut linux_mdesc) -> *mut linux_mdesc;

    /* Called by prom_init to setup the physical memory pmemblock
     * array.
     */
    pub fn prom_meminit();

    /* This is called at prom_init time to identify the
     * ARC architecture we are running on
     */
    pub fn prom_identify_arch();

    /* Environment variable routines. */
    pub fn ArcGetEnvironmentVariable(name: PCHAR) -> PCHAR;

    /* ARCS command line parsing. */
    pub fn prom_init_cmdline(argc: ::core::ffi::c_int, argv: *mut LONG);

    /* File operations. */
    pub fn ArcRead(fd: ULONG, buf: PVOID, num: ULONG, cnt: PULONG) -> LONG;
    pub fn ArcWrite(fd: ULONG, buf: PVOID, num: ULONG, cnt: PULONG) -> LONG;

    /* Misc. routines. */
    pub fn ArcEnterInteractiveMode() -> !;
    pub fn ArcGetDisplayStatus(FileID: ULONG) -> *mut DISPLAY_STATUS;
}

pub const PROM_FLAG_ARCS: ::core::ffi::c_int = 1;
pub const PROM_FLAG_USE_AS_CONSOLE: ::core::ffi::c_int = 2;
pub const PROM_FLAG_DONT_FREE_TEMP: ::core::ffi::c_int = 4;

pub const PROM_NULL_MDESC: *mut linux_mdesc = ::core::ptr::null_mut();
pub const PROM_NULL_COMPONENT: *mut pcomponent = ::core::ptr::null_mut();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
