/* SPDX-License-Identifier: GPL-2.0-or-later */

use std::ffi::{c_char, c_int, c_uchar, c_void};

pub const EINVAL: c_int = 22;
pub const ENOTSUP: c_int = 95;

/* Original C header depends on <linux/compiler.h> for __maybe_unused. */

/* Original C condition: #ifdef HAVE_LIBDW_SUPPORT */
#[cfg(HAVE_LIBDW_SUPPORT)]
/* Original C include dependency: "dwarf-aux.h" */

/* debug information structure */
#[cfg(HAVE_LIBDW_SUPPORT)]
#[repr(C)]
pub struct debuginfo {
    pub dbg: *mut Dwarf,
    pub mod_: *mut Dwfl_Module,
    pub dwfl: *mut Dwfl,
    pub bias: Dwarf_Addr,
    pub build_id: *const c_uchar,
}

#[cfg(HAVE_LIBDW_SUPPORT)]
unsafe extern "C" {
    /* This also tries to open distro debuginfo */
    pub fn debuginfo__new(path: *const c_char) -> *mut debuginfo;
    pub fn debuginfo__delete(dbg: *mut debuginfo);

    pub fn debuginfo__get_text_offset(
        dbg: *mut debuginfo,
        offs: *mut Dwarf_Addr,
        adjust_offset: bool,
    ) -> c_int;
}

/* Original C condition: #else HAVE_LIBDW_SUPPORT */

/* dummy debug information structure */
#[cfg(not(HAVE_LIBDW_SUPPORT))]
#[repr(C)]
pub struct debuginfo {}

#[cfg(not(HAVE_LIBDW_SUPPORT))]
#[inline]
pub unsafe fn debuginfo__new(_path: *const c_char) -> *mut debuginfo {
    std::ptr::null_mut()
}

#[cfg(not(HAVE_LIBDW_SUPPORT))]
#[inline]
pub unsafe fn debuginfo__delete(_dbg: *mut debuginfo) {}

#[cfg(not(HAVE_LIBDW_SUPPORT))]
pub type Dwarf_Addr = c_void;

#[cfg(not(HAVE_LIBDW_SUPPORT))]
#[inline]
pub unsafe fn debuginfo__get_text_offset(
    _dbg: *mut debuginfo,
    _offs: *mut Dwarf_Addr,
    _adjust_offset: bool,
) -> c_int {
    -EINVAL
}

/* Original C condition: #ifdef HAVE_DEBUGINFOD_SUPPORT */
#[cfg(HAVE_DEBUGINFOD_SUPPORT)]
unsafe extern "C" {
    pub fn get_source_from_debuginfod(
        raw_path: *const c_char,
        sbuild_id: *const c_char,
        new_path: *mut *mut c_char,
    ) -> c_int;
}

/* Original C condition: #else HAVE_DEBUGINFOD_SUPPORT */
#[cfg(not(HAVE_DEBUGINFOD_SUPPORT))]
#[inline]
pub unsafe fn get_source_from_debuginfod(
    _raw_path: *const c_char,
    _sbuild_id: *const c_char,
    _new_path: *mut *mut c_char,
) -> c_int {
    -ENOTSUP
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
