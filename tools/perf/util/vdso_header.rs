/* SPDX-License-Identifier: GPL-2.0 */

use std::os::raw::{c_char, c_int};

pub const VDSO__MAP_NAME: &[u8; 7] = b"[vdso]\0";

pub const DSO__NAME_VDSO: &[u8; 7] = b"[vdso]\0";
pub const DSO__NAME_VDSO32: &[u8; 9] = b"[vdso32]\0";
pub const DSO__NAME_VDSOX32: &[u8; 10] = b"[vdsox32]\0";

unsafe extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

#[inline]
pub unsafe fn is_vdso_map(filename: *const c_char) -> bool {
    unsafe { strcmp(filename, VDSO__MAP_NAME.as_ptr() as *const c_char) == 0 }
}

#[repr(C)]
pub struct dso {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn dso__is_vdso(dso: *mut dso) -> bool;
}

#[repr(C)]
pub struct machine {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn machine__findnew_vdso(machine: *mut machine, thread: *mut thread) -> *mut dso;
    pub fn machine__exit_vdso(machine: *mut machine);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
