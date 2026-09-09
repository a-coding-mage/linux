/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Linux host-side vring helpers; for when the kernel needs to access
 * someone else's vring.
 *
 * Copyright IBM Corporation, 2013.
 * Parts taken from drivers/vhost/vhost.c Copyright 2009 Red Hat, Inc.
 *
 * Written by: Rusty Russell <rusty@rustcorp.com.au>
 */

use core::ffi::c_void;

/* Dependencies supplied by the surrounding kernel/virtio translation. */

#[repr(C)]
pub struct vringh {
    /* Everything is little endian */
    pub little_endian: bool,
    /* Guest publishes used event idx (note: we always do). */
    pub event_indices: bool,
    /* Can we get away with weak barriers? */
    pub weak_barriers: bool,
    /* Use user's VA */
    pub use_va: bool,
    /* Last available index we saw (ie. where we're up to). */
    pub last_avail_idx: u16,
    /* Last index we used. */
    pub last_used_idx: u16,
    /* How many descriptors we've completed since last need_notify(). */
    pub completed: u32,
    /* The vring (note: it may contain user pointers!) */
    pub vring: vring,
    /* IOTLB for this vring */
    pub iotlb: *mut vhost_iotlb,
    /* spinlock to synchronize IOTLB accesses */
    pub iotlb_lock: *mut spinlock_t,
    /* The function to call to notify the guest about added buffers */
    pub notify: Option<unsafe extern "C" fn(*mut vringh)>,
}

pub type vrh_callback_t = unsafe extern "C" fn(*mut virtio_device, *mut vringh);

#[repr(C)]
pub struct vringh_config_ops {
    pub find_vrhs: Option<unsafe extern "C" fn(
        *mut virtio_device,
        c_uint,
        *mut *mut vringh,
        *mut *mut vrh_callback_t,
    ) -> c_int>,
    pub del_vrhs: Option<unsafe extern "C" fn(*mut virtio_device)>,
}

#[repr(C)]
pub struct vringh_range {
    pub start: u64,
    pub end_incl: u64,
    pub offset: u64,
}

#[repr(C)]
pub struct vringh_iov {
    pub iov: *mut iovec,
    pub consumed: usize,
    pub i: c_uint,
    pub used: c_uint,
    pub max_num: c_uint,
}

#[repr(C)]
pub struct vringh_kiov {
    pub iov: *mut kvec,
    pub consumed: usize,
    pub i: c_uint,
    pub used: c_uint,
    pub max_num: c_uint,
}

/* Flag on max_num to indicate we're kmalloced. */
pub const VRINGH_IOV_ALLOCATED: c_uint = 0x8000000;

extern "C" {
    pub fn vringh_init_user(
        vrh: *mut vringh, features: u64, num: c_uint, weak_barriers: bool,
        desc: *mut vring_desc_t, avail: *mut vring_avail_t, used: *mut vring_used_t,
    ) -> c_int;

    pub fn vringh_getdesc_user(
        vrh: *mut vringh, riov: *mut vringh_iov, wiov: *mut vringh_iov,
        getrange: Option<unsafe extern "C" fn(*mut vringh, u64, *mut vringh_range) -> bool>,
        head: *mut u16,
    ) -> c_int;
    pub fn vringh_iov_pull_user(riov: *mut vringh_iov, dst: *mut c_void, len: usize) -> isize;
    pub fn vringh_iov_push_user(wiov: *mut vringh_iov, src: *const c_void, len: usize) -> isize;
    pub fn vringh_complete_user(vrh: *mut vringh, head: u16, len: u32) -> c_int;
    pub fn vringh_complete_multi_user(vrh: *mut vringh, used: *const vring_used_elem, num_used: c_uint) -> c_int;
    pub fn vringh_need_notify_user(vrh: *mut vringh) -> c_int;
    pub fn vringh_notify_enable_user(vrh: *mut vringh) -> bool;
    pub fn vringh_notify_disable_user(vrh: *mut vringh);

    pub fn vringh_init_kern(
        vrh: *mut vringh, features: u64, num: c_uint, weak_barriers: bool,
        desc: *mut vring_desc, avail: *mut vring_avail, used: *mut vring_used,
    ) -> c_int;
    pub fn vringh_kiov_advance(kiov: *mut vringh_kiov, len: usize);
    pub fn vringh_getdesc_kern(vrh: *mut vringh, riov: *mut vringh_kiov, wiov: *mut vringh_kiov, head: *mut u16, gfp: gfp_t) -> c_int;
    pub fn vringh_complete_kern(vrh: *mut vringh, head: u16, len: u32) -> c_int;
    pub fn vringh_notify_enable_kern(vrh: *mut vringh) -> bool;
    pub fn vringh_notify_disable_kern(vrh: *mut vringh);
    pub fn vringh_need_notify_kern(vrh: *mut vringh) -> c_int;
}

pub unsafe fn vringh_iov_init(iov: *mut vringh_iov, iovec: *mut iovec, num: c_uint) {
    (*iov).used = 0; (*iov).i = 0; (*iov).consumed = 0; (*iov).max_num = num; (*iov).iov = iovec;
}
pub unsafe fn vringh_iov_reset(iov: *mut vringh_iov) {
    (*iov).iov.add((*iov).i as usize).as_mut().unwrap().iov_len += (*iov).consumed;
    (*iov).iov.add((*iov).i as usize).as_mut().unwrap().iov_base = (*iov).iov.add((*iov).i as usize).as_mut().unwrap().iov_base.sub((*iov).consumed);
    (*iov).consumed = 0; (*iov).i = 0;
}
pub unsafe fn vringh_iov_cleanup(iov: *mut vringh_iov) {
    if (*iov).max_num & VRINGH_IOV_ALLOCATED != 0 { kfree((*iov).iov as *mut c_void); }
    (*iov).max_num = 0; (*iov).used = 0; (*iov).i = 0; (*iov).consumed = 0; (*iov).iov = core::ptr::null_mut();
}

pub unsafe fn vringh_kiov_init(kiov: *mut vringh_kiov, kvec_: *mut kvec, num: c_uint) {
    (*kiov).used = 0; (*kiov).i = 0; (*kiov).consumed = 0; (*kiov).max_num = num; (*kiov).iov = kvec_;
}
pub unsafe fn vringh_kiov_reset(kiov: *mut vringh_kiov) {
    (*kiov).iov.add((*kiov).i as usize).as_mut().unwrap().iov_len += (*kiov).consumed;
    (*kiov).iov.add((*kiov).i as usize).as_mut().unwrap().iov_base = (*kiov).iov.add((*kiov).i as usize).as_mut().unwrap().iov_base.sub((*kiov).consumed);
    (*kiov).consumed = 0; (*kiov).i = 0;
}
pub unsafe fn vringh_kiov_cleanup(kiov: *mut vringh_kiov) {
    if (*kiov).max_num & VRINGH_IOV_ALLOCATED != 0 { kfree((*kiov).iov as *mut c_void); }
    (*kiov).max_num = 0; (*kiov).used = 0; (*kiov).i = 0; (*kiov).consumed = 0; (*kiov).iov = core::ptr::null_mut();
}
pub unsafe fn vringh_kiov_length(kiov: *mut vringh_kiov) -> usize {
    let mut len = 0; let mut i = (*kiov).i;
    while i < (*kiov).used { len += (*kiov).iov.add(i as usize).read().iov_len; i += 1; }
    len
}

extern "C" {
    fn kfree(ptr: *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
