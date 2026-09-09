/* SPDX-License-Identifier: GPL-2.0 */
/* Coda filesystem -- Linux Minicache
 *
 * Copyright (C) 1989 - 1997 Carnegie Mellon University
 *
 * Carnegie Mellon University encourages users of this software to
 * contribute improvements to the Coda project. Contact Peter Braam
 * <coda@cs.cmu.edu>
 */

// C header guard: _CFSNC_HEADER_

// Opaque types supplied by the surrounding kernel interfaces.
#[repr(C)]
pub struct inode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct super_block {
    _private: [u8; 0],
}

/* credential cache */
extern "C" {
    pub fn coda_cache_enter(inode: *mut inode, mask: ::core::ffi::c_int);
    pub fn coda_cache_clear_inode(inode: *mut inode);
    pub fn coda_cache_clear_all(sb: *mut super_block);
    pub fn coda_cache_check(inode: *mut inode, mask: ::core::ffi::c_int) -> ::core::ffi::c_int;

    /* for downcalls and attributes and lookups */
    pub fn coda_flag_inode_children(inode: *mut inode, flag: ::core::ffi::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
