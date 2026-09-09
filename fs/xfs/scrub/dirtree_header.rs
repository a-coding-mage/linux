/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (c) 2023-2024 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

/* Translated from xfs/scrub/dirtree.h. */

#[repr(C)]
pub struct xchk_dirpath_step {
    /* Directory entry name associated with this parent link. */
    pub name_cookie: xfblob_cookie,
    pub name_len: ::core::ffi::c_uint,

    /* Handle of the parent directory. */
    pub pptr_rec: xfs_parent_rec,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xchk_dirpath_outcome {
    XCHK_DIRPATH_SCANNING = 0, /* still being put together */
    XCHK_DIRPATH_DELETE,       /* delete this path */
    XCHK_DIRPATH_CORRUPT,      /* corruption detected in path */
    XCHK_DIRPATH_LOOP,         /* cycle detected further up */
    XCHK_DIRPATH_STALE,        /* path is stale */
    XCHK_DIRPATH_OK,           /* path reaches the root */

    XREP_DIRPATH_DELETING,     /* path is being deleted */
    XREP_DIRPATH_DELETED,      /* path has been deleted */
    XREP_DIRPATH_ADOPTING,     /* path is being adopted */
    XREP_DIRPATH_ADOPTED,      /* path has been adopted */
}

#[repr(C)]
pub struct xchk_dirpath {
    pub list: list_head,

    /* Index of the first step in this path. */
    pub first_step: xfarray_idx_t,

    /* Index of the second step in this path. */
    pub second_step: xfarray_idx_t,

    /* Inodes seen while walking this path. */
    pub seen_inodes: xino_bitmap,

    /* Number of steps in this path. */
    pub nr_steps: ::core::ffi::c_uint,

    /* Which path is this? */
    pub path_nr: ::core::ffi::c_uint,

    /* What did we conclude from following this path? */
    pub outcome: xchk_dirpath_outcome,
}

#[repr(C)]
pub struct xchk_dirtree_outcomes {
    /* Number of XCHK_DIRPATH_DELETE */
    pub bad: ::core::ffi::c_uint,

    /* Number of XCHK_DIRPATH_CORRUPT or XCHK_DIRPATH_LOOP */
    pub suspect: ::core::ffi::c_uint,

    /* Number of XCHK_DIRPATH_OK */
    pub good: ::core::ffi::c_uint,

    /* Directory needs to be added to lost+found */
    pub needs_adoption: bool,
}

#[repr(C)]
pub struct xchk_dirtree {
    pub sc: *mut xfs_scrub,

    /* Root inode that we're looking for. */
    pub root_ino: xfs_ino_t,

    /* This is the inode that we're scanning. */
    pub scan_ino: xfs_ino_t,

    /* Inode number of the surviving parent, or NULLFSINO. */
    pub parent_ino: xfs_ino_t,

    /* Scratch buffer for scanning pptr xattrs */
    pub pptr_rec: xfs_parent_rec,
    pub pptr_args: xfs_da_args,

    /* Name buffer */
    pub xname: xfs_name,
    pub namebuf: [::core::ffi::c_char; MAXNAMELEN],

    /* Information for reparenting this directory. */
    pub adoption: xrep_adoption,

    /* Hook into directory updates. */
    pub dhook: xfs_dir_hook,

    /* Parent pointer update arguments. */
    pub ppargs: xfs_parent_args,

    /* lock for everything below here */
    pub lock: mutex,

    /* buffer for the live update functions to use for dirent names */
    pub hook_xname: xfs_name,
    pub hook_namebuf: [u8; MAXNAMELEN],

    /* All path steps observed during this scan. */
    pub path_steps: *mut xfarray,

    /* All names observed during this scan. */
    pub path_names: *mut xfblob,

    /* All paths being tracked by this scanner. */
    pub path_list: list_head,

    /* Number of paths in path_list. */
    pub nr_paths: ::core::ffi::c_uint,

    /* Number of parents found by a pptr scan. */
    pub parents_found: ::core::ffi::c_uint,

    /* Have the path data been invalidated by a concurrent update? */
    pub stale: bool,

    /* Has the scan been aborted? */
    pub aborted: bool,
}

#[macro_export]
macro_rules! xchk_dirtree_for_each_path_safe {
    ($dl:expr, $path:expr, $n:expr) => {
        list_for_each_entry_safe!($path, $n, &mut $dl.path_list, list)
    };
}

#[macro_export]
macro_rules! xchk_dirtree_for_each_path {
    ($dl:expr, $path:expr) => {
        list_for_each_entry!($path, &mut $dl.path_list, list)
    };
}

extern "C" {
    pub fn xchk_dirtree_parentless(dl: *const xchk_dirtree) -> bool;
    pub fn xchk_dirtree_find_paths_to_root(dl: *mut xchk_dirtree) -> ::core::ffi::c_int;
    pub fn xchk_dirpath_append(
        dl: *mut xchk_dirtree,
        ip: *mut xfs_inode,
        path: *mut xchk_dirpath,
        name: *const xfs_name,
        pptr: *const xfs_parent_rec,
    ) -> ::core::ffi::c_int;
    pub fn xchk_dirtree_evaluate(
        dl: *mut xchk_dirtree,
        oc: *mut xchk_dirtree_outcomes,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
