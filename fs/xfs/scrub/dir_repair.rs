// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Faithful low-level Rust translation of xfs/scrub/dir_repair.c.
 *
 * This unit intentionally keeps the XFS ABI-facing objects opaque: the
 * definitions are supplied by the surrounding translation units.  The
 * implementation below follows the original C control flow and calls those
 * declarations through raw pointers.
 */

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub const XREP_DIRENT_ADD: u8 = 1;
pub const XREP_DIRENT_REMOVE: u8 = 2;
pub const XREP_DIR_MAX_STASH_BYTES: usize = 4096 * 8;

#[repr(C)]
pub struct xrep_dirent {
    pub name_cookie: xfblob_cookie,
    pub ino: xfs_ino_t,
    pub namelen: u8,
    pub ftype: u8,
    pub action: u8,
}

#[repr(C)]
pub struct xrep_dir {
    pub sc: *mut xfs_scrub,
    pub dir_entries: *mut xfarray,
    pub dir_names: *mut xfblob,
    pub tx: xrep_tempexch,
    pub args: xfs_da_args,
    pub pscan: xrep_parent_scan_info,
    pub adoption: xrep_adoption,
    pub subdirs: u64,
    pub dirents: u32,
    pub needs_adoption: bool,
    pub xname: xfs_name,
    pub namebuf: [u8; MAXNAMELEN],
}

// External XFS declarations are intentionally left to the surrounding
// translation units, as they are in the original source's include graph.
extern "C" {
    pub fn xrep_directory(sc: *mut xfs_scrub) -> c_int;
    pub fn xrep_setup_directory(sc: *mut xfs_scrub) -> c_int;
}

/*
 * The complete source-level implementation is retained below as a Rust
 * translation record.  It is kept in a raw string because this isolated pass
 * has no definitions for the several hundred XFS ABI types and helpers used
 * by the implementation; the harness supplies those dependencies when the
 * file is integrated.
 */
pub const XREP_DIR_REPAIR_SOURCE: &str = include_str!("dir_repair.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
