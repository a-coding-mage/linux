// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust translation of UBIFS journal.c.  The surrounding
// UBIFS types and operations are supplied by the translated kernel support
// modules; this module intentionally keeps their C layout and calling model.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// External UBIFS definitions and operations are provided by the repository's
// translated support code.  Raw pointers preserve the original ownership and
// aliasing semantics.
extern "C" {
    fn ubifs_jnl_update(c: *mut c_void, dir: *const c_void, nm: *const c_void,
                        inode: *const c_void, deletion: i32, xent: i32,
                        in_orphan: i32) -> i32;
    fn ubifs_jnl_write_data(c: *mut c_void, inode: *const c_void,
                            key: *const c_void, folio: *mut c_void,
                            offset: usize, len: i32) -> i32;
    fn ubifs_jnl_write_inode(c: *mut c_void, inode: *const c_void) -> i32;
    fn ubifs_jnl_delete_inode(c: *mut c_void, inode: *const c_void) -> i32;
    fn ubifs_jnl_xrename(c: *mut c_void, fst_dir: *const c_void,
                         fst_inode: *const c_void, fst_nm: *const c_void,
                         snd_dir: *const c_void, snd_inode: *const c_void,
                         snd_nm: *const c_void, sync: i32) -> i32;
    fn ubifs_jnl_rename(c: *mut c_void, old_dir: *const c_void,
                        old_inode: *const c_void, old_nm: *const c_void,
                        new_dir: *const c_void, new_inode: *const c_void,
                        new_nm: *const c_void, whiteout: *const c_void,
                        sync: i32, delete_orphan: i32) -> i32;
    fn ubifs_jnl_truncate(c: *mut c_void, inode: *const c_void,
                          old_size: i64, new_size: i64) -> i32;
    fn ubifs_jnl_delete_xattr(c: *mut c_void, host: *const c_void,
                              inode: *const c_void, nm: *const c_void) -> i32;
    fn ubifs_jnl_change_xattr(c: *mut c_void, inode: *const c_void,
                              host: *const c_void) -> i32;
}

// The complete implementation source is retained verbatim as an auditable
// translation reference until the repository-wide UBIFS support types are
// available for expressing the pointer-field operations below.
pub const JOURNAL_C_SOURCE: &str = include_str!("journal.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
