/* SPDX-License-Identifier: GPL-2.0-or-later
 *
 * Copyright (C) 2022 Oracle.  All Rights Reserved.
 * Author: Allison Henderson <allison.henderson@oracle.com>
 */

/* kernel only ATTRI/ATTRD definitions */

/* External declarations supplied by other translation units. */
pub struct xfs_mount;
pub struct kmem_zone;

#[repr(C)]
pub struct xfs_attri_log_nameval {
    pub name: kvec,
    pub new_name: kvec, /* PPTR_REPLACE only */
    pub value: kvec,
    pub new_value: kvec, /* PPTR_REPLACE only */
    pub refcount: refcount_t,

    /* name and value follow the end of this struct */
}

/*
 * This is the "attr intention" log item.  It is used to log the fact that some
 * extended attribute operations need to be processed.  An operation is
 * currently either a set or remove.  Set or remove operations are described by
 * the xfs_attr_intent which may be logged to this intent.
 *
 * During a normal attr operation, name and value point to the name and value
 * fields of the caller's xfs_da_args structure.  During a recovery, the name
 * and value buffers are copied from the log, and stored in a trailing buffer
 * attached to the xfs_attr_intent until they are committed.  They are freed
 * when the xfs_attr_intent itself is freed when the work is done.
 */
#[repr(C)]
pub struct xfs_attri_log_item {
    pub attri_item: xfs_log_item,
    pub attri_refcount: atomic_t,
    pub attri_nameval: *mut xfs_attri_log_nameval,
    pub attri_format: xfs_attri_log_format,
}

/*
 * This is the "attr done" log item.  It is used to log the fact that some attrs
 * earlier mentioned in an attri item have been freed.
 */
#[repr(C)]
pub struct xfs_attrd_log_item {
    pub attrd_item: xfs_log_item,
    pub attrd_attrip: *mut xfs_attri_log_item,
    pub attrd_format: xfs_attrd_log_format,
}

extern "C" {
    pub static mut xfs_attri_cache: *mut kmem_cache;
    pub static mut xfs_attrd_cache: *mut kmem_cache;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum xfs_attr_defer_op {
    XFS_ATTR_DEFER_SET,
    XFS_ATTR_DEFER_REMOVE,
    XFS_ATTR_DEFER_REPLACE,
}

extern "C" {
    pub fn xfs_attr_defer_add(args: *mut xfs_da_args, op: xfs_attr_defer_op);
}

/* Types declared by included headers and supplied by other translation units. */
// kvec, refcount_t, xfs_log_item, atomic_t, xfs_attri_log_format,
// xfs_attrd_log_format, kmem_cache, and xfs_da_args

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
