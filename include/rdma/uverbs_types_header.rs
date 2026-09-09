/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/*
 * Copyright (c) 2017, Mellanox Technologies inc.  All rights reserved.
 */

// Dependencies supplied by the surrounding translation unit:
// <linux/kernel.h>, <rdma/ib_verbs.h>

use core::ffi::c_char;

#[repr(C)]
pub struct uverbs_api_object { _private: [u8; 0] }
#[repr(C)]
pub struct ib_uobject { pub ref_: kref }
#[repr(C)] pub struct uverbs_attr_bundle { _private: [u8; 0] }
#[repr(C)] pub struct ib_uverbs_file { _private: [u8; 0] }
#[repr(C)] pub struct file_operations { _private: [u8; 0] }
#[repr(C)] pub struct ib_uverbs_device { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct ib_ucontext { _private: [u8; 0] }
#[repr(C)] pub struct ib_uverbs_async_event_file { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct rw_semaphore { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct xarray { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct kref { _private: [u8; 0] }

pub type s64 = i64;
pub type size_t = usize;
pub type bool_ = bool;
pub type rdma_remove_reason = i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum rdma_lookup_mode {
    UVERBS_LOOKUP_READ,
    UVERBS_LOOKUP_WRITE,
    UVERBS_LOOKUP_DESTROY,
}

#[repr(C)]
pub struct uverbs_obj_type_class {
    pub alloc_begin: Option<unsafe extern "C" fn(*const uverbs_api_object, *mut uverbs_attr_bundle) -> *mut ib_uobject>,
    pub alloc_commit: Option<unsafe extern "C" fn(*mut ib_uobject)>,
    pub alloc_abort: Option<unsafe extern "C" fn(*mut ib_uobject)>,
    pub lookup_get: Option<unsafe extern "C" fn(*const uverbs_api_object, *mut ib_uverbs_file, s64, rdma_lookup_mode) -> *mut ib_uobject>,
    pub lookup_put: Option<unsafe extern "C" fn(*mut ib_uobject, rdma_lookup_mode)>,
    pub destroy_hw: Option<unsafe extern "C" fn(*mut ib_uobject, rdma_remove_reason, *mut uverbs_attr_bundle) -> i32>,
    pub remove_handle: Option<unsafe extern "C" fn(*mut ib_uobject)>,
    pub swap_uobjects: Option<unsafe extern "C" fn(*mut ib_uobject, *mut ib_uobject)>,
}

#[repr(C)]
pub struct uverbs_obj_type {
    pub type_class: *const uverbs_obj_type_class,
    pub obj_size: size_t,
}

#[repr(C)]
pub struct uverbs_obj_idr_type {
    pub type_: uverbs_obj_type,
    pub destroy_object: Option<unsafe extern "C" fn(*mut ib_uobject, rdma_remove_reason, *mut uverbs_attr_bundle) -> i32>,
}

extern "C" {
    pub fn rdma_lookup_get_uobject(obj: *const uverbs_api_object, ufile: *mut ib_uverbs_file, id: s64, mode: rdma_lookup_mode, attrs: *mut uverbs_attr_bundle) -> *mut ib_uobject;
    pub fn rdma_lookup_put_uobject(uobj: *mut ib_uobject, mode: rdma_lookup_mode);
    pub fn rdma_alloc_begin_uobject(obj: *const uverbs_api_object, attrs: *mut uverbs_attr_bundle) -> *mut ib_uobject;
    pub fn rdma_alloc_abort_uobject(uobj: *mut ib_uobject, attrs: *mut uverbs_attr_bundle, hw_obj_valid: bool);
    pub fn rdma_alloc_commit_uobject(uobj: *mut ib_uobject, attrs: *mut uverbs_attr_bundle);
    pub fn rdma_assign_uobject(to_uobj: *mut ib_uobject, new_uobj: *mut ib_uobject, attrs: *mut uverbs_attr_bundle);
    pub fn uverbs_uobject_put(uobject: *mut ib_uobject);
    pub fn uverbs_try_lock_object(uobj: *mut ib_uobject, mode: rdma_lookup_mode) -> i32;
    pub fn uverbs_uobject_fd_release(inode: *mut inode, filp: *mut file) -> i32;
    pub fn uverbs_uobject_release(uobj: *mut ib_uobject) -> i32;
    pub static uverbs_idr_class: uverbs_obj_type_class;
    pub static uverbs_fd_class: uverbs_obj_type_class;
    pub fn kref_get(r: *mut kref);
}

#[inline]
pub unsafe fn uverbs_uobject_get(uobject: *mut ib_uobject) {
    kref_get(&mut (*uobject).ref_);
}

#[repr(C)]
pub struct uverbs_obj_fd_type {
    pub type_: uverbs_obj_type,
    pub destroy_object: Option<unsafe extern "C" fn(*mut ib_uobject, rdma_remove_reason)>,
    pub release_cleanup: Option<unsafe extern "C" fn(*mut ib_uobject)>,
    pub fops: *const file_operations,
    pub name: *const c_char,
    pub flags: i32,
}

// C preprocessor constructors translated as Rust helper functions.
#[inline]
pub const fn uverbs_build_bug_on(cond: bool) -> usize { if cond { usize::MAX } else { 0 } }

#[inline]
pub unsafe fn uverbs_type_alloc_fd_release(obj_size: size_t, destroy_object: Option<unsafe extern "C" fn(*mut ib_uobject, rdma_remove_reason)>, release_cleanup: Option<unsafe extern "C" fn(*mut ib_uobject)>, fops: *const file_operations, name: *const c_char, flags: i32) -> uverbs_obj_type {
    uverbs_obj_type { type_class: &uverbs_fd_class, obj_size: obj_size + uverbs_build_bug_on(obj_size < core::mem::size_of::<ib_uobject>()) }
}

#[inline]
pub unsafe fn uverbs_type_alloc_fd(obj_size: size_t, destroy_object: Option<unsafe extern "C" fn(*mut ib_uobject, rdma_remove_reason)>, fops: *const file_operations, name: *const c_char, flags: i32) -> uverbs_obj_type {
    uverbs_type_alloc_fd_release(obj_size, destroy_object, None, fops, name, flags)
}

#[inline]
pub unsafe fn uverbs_type_alloc_idr_sz(size: size_t, destroy_object: Option<unsafe extern "C" fn(*mut ib_uobject, rdma_remove_reason, *mut uverbs_attr_bundle) -> i32>) -> uverbs_obj_type {
    uverbs_obj_type { type_class: &uverbs_idr_class, obj_size: size + uverbs_build_bug_on(size < core::mem::size_of::<ib_uobject>()) }
}

#[inline]
pub unsafe fn uverbs_type_alloc_idr(destroy_object: Option<unsafe extern "C" fn(*mut ib_uobject, rdma_remove_reason, *mut uverbs_attr_bundle) -> i32>) -> uverbs_obj_type {
    uverbs_type_alloc_idr_sz(core::mem::size_of::<ib_uobject>(), destroy_object)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
