/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_void;

#[repr(C)]
pub struct pstack {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn pstack__new(max_nr_entries: c_ushort) -> *mut pstack;
    pub fn pstack__delete(pstack: *mut pstack);
    pub fn pstack__empty(pstack: *const pstack) -> bool;
    pub fn pstack__remove(pstack: *mut pstack, key: *mut c_void);
    pub fn pstack__push(pstack: *mut pstack, key: *mut c_void);
    pub fn pstack__peek(pstack: *mut pstack) -> *mut c_void;
}

type c_ushort = u16;
