/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/* Copyright (c) 2018 Mellanox Technologies. All rights reserved */

//! Rust translation of the objagg trace-event header.
//! The original tracepoint registration and formatting are supplied by the
//! kernel tracing infrastructure; the record layouts and assignment behavior
//! are preserved here.

use core::ffi::c_void;

#[repr(C)]
pub struct objagg;
#[repr(C)]
pub struct objagg_obj;

#[repr(C)]
pub struct ObjaggCreateEntry {
    pub objagg: *const c_void,
}

/// Trace format: `objagg %p`.
#[inline]
pub unsafe fn objagg_create(objagg: *const objagg) -> ObjaggCreateEntry {
    ObjaggCreateEntry { objagg: objagg.cast() }
}

#[repr(C)]
pub struct ObjaggDestroyEntry {
    pub objagg: *const c_void,
}

/// Trace format: `objagg %p`.
#[inline]
pub unsafe fn objagg_destroy(objagg: *const objagg) -> ObjaggDestroyEntry {
    ObjaggDestroyEntry { objagg: objagg.cast() }
}

#[repr(C)]
pub struct ObjaggObjCreateEntry {
    pub objagg: *const c_void,
    pub obj: *const c_void,
}

/// Trace format: `objagg %p, obj %p`.
#[inline]
pub unsafe fn objagg_obj_create(
    objagg: *const objagg,
    obj: *const objagg_obj,
) -> ObjaggObjCreateEntry {
    ObjaggObjCreateEntry { objagg: objagg.cast(), obj: obj.cast() }
}

#[repr(C)]
pub struct ObjaggObjDestroyEntry {
    pub objagg: *const c_void,
    pub obj: *const c_void,
}

/// Trace format: `objagg %p, obj %p`.
#[inline]
pub unsafe fn objagg_obj_destroy(
    objagg: *const objagg,
    obj: *const objagg_obj,
) -> ObjaggObjDestroyEntry {
    ObjaggObjDestroyEntry { objagg: objagg.cast(), obj: obj.cast() }
}

#[repr(C)]
pub struct ObjaggObjGetEntry {
    pub objagg: *const c_void,
    pub obj: *const c_void,
    pub refcount: u32,
}

/// Trace format: `objagg %p, obj %p, refcount %u`.
#[inline]
pub unsafe fn objagg_obj_get(
    objagg: *const objagg,
    obj: *const objagg_obj,
    refcount: u32,
) -> ObjaggObjGetEntry {
    ObjaggObjGetEntry { objagg: objagg.cast(), obj: obj.cast(), refcount }
}

#[repr(C)]
pub struct ObjaggObjPutEntry {
    pub objagg: *const c_void,
    pub obj: *const c_void,
    pub refcount: u32,
}

/// Trace format: `objagg %p, obj %p, refcount %u`.
#[inline]
pub unsafe fn objagg_obj_put(
    objagg: *const objagg,
    obj: *const objagg_obj,
    refcount: u32,
) -> ObjaggObjPutEntry {
    ObjaggObjPutEntry { objagg: objagg.cast(), obj: obj.cast(), refcount }
}

#[repr(C)]
pub struct ObjaggObjParentAssignEntry {
    pub objagg: *const c_void,
    pub obj: *const c_void,
    pub parent: *const c_void,
    pub parent_refcount: u32,
}

/// Trace format: `objagg %p, obj %p, parent %p, parent_refcount %u`.
#[inline]
pub unsafe fn objagg_obj_parent_assign(
    objagg: *const objagg,
    obj: *const objagg_obj,
    parent: *const objagg_obj,
    parent_refcount: u32,
) -> ObjaggObjParentAssignEntry {
    ObjaggObjParentAssignEntry {
        objagg: objagg.cast(), obj: obj.cast(), parent: parent.cast(), parent_refcount,
    }
}

#[repr(C)]
pub struct ObjaggObjParentUnassignEntry {
    pub objagg: *const c_void,
    pub obj: *const c_void,
    pub parent: *const c_void,
    pub parent_refcount: u32,
}

/// Trace format: `objagg %p, obj %p, parent %p, parent_refcount %u`.
#[inline]
pub unsafe fn objagg_obj_parent_unassign(
    objagg: *const objagg,
    obj: *const objagg_obj,
    parent: *const objagg_obj,
    parent_refcount: u32,
) -> ObjaggObjParentUnassignEntry {
    ObjaggObjParentUnassignEntry {
        objagg: objagg.cast(), obj: obj.cast(), parent: parent.cast(), parent_refcount,
    }
}

#[repr(C)]
pub struct ObjaggObjRootCreateEntry {
    pub objagg: *const c_void,
    pub obj: *const c_void,
}

/// Trace format: `objagg %p, obj %p`.
#[inline]
pub unsafe fn objagg_obj_root_create(
    objagg: *const objagg,
    obj: *const objagg_obj,
) -> ObjaggObjRootCreateEntry {
    ObjaggObjRootCreateEntry { objagg: objagg.cast(), obj: obj.cast() }
}

#[repr(C)]
pub struct ObjaggObjRootDestroyEntry {
    pub objagg: *const c_void,
    pub obj: *const c_void,
}

/// Trace format: `objagg %p, obj %p`.
#[inline]
pub unsafe fn objagg_obj_root_destroy(
    objagg: *const objagg,
    obj: *const objagg_obj,
) -> ObjaggObjRootDestroyEntry {
    ObjaggObjRootDestroyEntry { objagg: objagg.cast(), obj: obj.cast() }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
