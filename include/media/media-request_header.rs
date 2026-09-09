// SPDX-License-Identifier: GPL-2.0
/* Media device request objects. Translated from media-request.h. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Dependencies supplied by the surrounding kernel translation.
#[repr(C)]
pub struct media_device { _private: [u8; 0] }
#[repr(C)]
pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)]
pub struct kref { pub refcount: c_uint }
#[repr(C)]
pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)]
pub struct spinlock_t { _private: [u8; 0] }

pub const TASK_COMM_LEN: usize = 16;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum media_request_state {
    MEDIA_REQUEST_STATE_IDLE,
    MEDIA_REQUEST_STATE_VALIDATING,
    MEDIA_REQUEST_STATE_QUEUED,
    MEDIA_REQUEST_STATE_COMPLETE,
    MEDIA_REQUEST_STATE_CLEANING,
    MEDIA_REQUEST_STATE_UPDATING,
    NR_OF_MEDIA_REQUEST_STATE,
}

#[repr(C)]
pub struct media_request {
    pub mdev: *mut media_device,
    pub kref: kref,
    pub debug_str: [c_char; TASK_COMM_LEN + 11],
    pub state: media_request_state,
    pub updating_count: c_uint,
    pub access_count: c_uint,
    pub objects: list_head,
    pub num_incomplete_objects: c_uint,
    pub manual_completion: bool,
    pub poll_wait: wait_queue_head_t,
    pub lock: spinlock_t,
}

#[repr(C)]
pub struct media_request_object_ops {
    pub prepare: Option<unsafe extern "C" fn(*mut media_request_object) -> c_int>,
    pub unprepare: Option<unsafe extern "C" fn(*mut media_request_object)>,
    pub queue: Option<unsafe extern "C" fn(*mut media_request_object)>,
    pub unbind: Option<unsafe extern "C" fn(*mut media_request_object)>,
    pub release: Option<unsafe extern "C" fn(*mut media_request_object)>,
}

#[repr(C)]
pub struct media_request_object {
    pub mdev: *mut media_device,
    pub ops: *const media_request_object_ops,
    pub priv_: *mut c_void,
    pub req: *mut media_request,
    pub list: list_head,
    pub kref: kref,
    pub completed: bool,
}

#[cfg(feature = "CONFIG_MEDIA_CONTROLLER")]
extern "C" {
    pub fn media_request_put(req: *mut media_request);
    pub fn media_request_get_by_fd(mdev: *mut media_device, request_fd: c_int) -> *mut media_request;
    pub fn media_request_alloc(mdev: *mut media_device, alloc_fd: *mut c_int) -> c_int;
    pub fn media_request_manual_complete(req: *mut media_request);
    pub fn media_request_object_put(obj: *mut media_request_object);
    pub fn media_request_object_find(req: *mut media_request, ops: *const media_request_object_ops, priv_: *mut c_void) -> *mut media_request_object;
    pub fn media_request_object_init(obj: *mut media_request_object);
    pub fn media_request_object_bind(req: *mut media_request, ops: *const media_request_object_ops, priv_: *mut c_void, is_buffer: bool, obj: *mut media_request_object) -> c_int;
    pub fn media_request_object_unbind(obj: *mut media_request_object);
    pub fn media_request_object_complete(obj: *mut media_request_object);
}

#[cfg(feature = "CONFIG_MEDIA_CONTROLLER")]
pub unsafe fn media_request_get(req: *mut media_request) { kref_get(&mut (*req).kref); }

#[cfg(feature = "CONFIG_MEDIA_CONTROLLER")]
pub unsafe fn media_request_lock_for_access(req: *mut media_request) -> c_int {
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*req).lock, &mut flags);
    let mut ret = -16;
    if (*req).state == media_request_state::MEDIA_REQUEST_STATE_COMPLETE { (*req).access_count += 1; ret = 0; }
    spin_unlock_irqrestore(&mut (*req).lock, flags);
    ret
}

#[cfg(feature = "CONFIG_MEDIA_CONTROLLER")]
pub unsafe fn media_request_unlock_for_access(req: *mut media_request) {
    let mut flags: c_ulong = 0; spin_lock_irqsave(&mut (*req).lock, &mut flags);
    if (*req).access_count != 0 { (*req).access_count -= 1; }
    spin_unlock_irqrestore(&mut (*req).lock, flags);
}

#[cfg(feature = "CONFIG_MEDIA_CONTROLLER")]
pub unsafe fn media_request_lock_for_update(req: *mut media_request) -> c_int {
    let mut flags: c_ulong = 0; spin_lock_irqsave(&mut (*req).lock, &mut flags); let mut ret = 0;
    if (*req).state == media_request_state::MEDIA_REQUEST_STATE_IDLE || (*req).state == media_request_state::MEDIA_REQUEST_STATE_UPDATING { (*req).state = media_request_state::MEDIA_REQUEST_STATE_UPDATING; (*req).updating_count += 1; } else { ret = -16; }
    spin_unlock_irqrestore(&mut (*req).lock, flags); ret
}

#[cfg(feature = "CONFIG_MEDIA_CONTROLLER")]
pub unsafe fn media_request_unlock_for_update(req: *mut media_request) {
    let mut flags: c_ulong = 0; spin_lock_irqsave(&mut (*req).lock, &mut flags);
    if (*req).updating_count > 0 { (*req).updating_count -= 1; if (*req).updating_count == 0 { (*req).state = media_request_state::MEDIA_REQUEST_STATE_IDLE; } }
    spin_unlock_irqrestore(&mut (*req).lock, flags);
}

#[cfg(feature = "CONFIG_MEDIA_CONTROLLER")]
pub unsafe fn media_request_mark_manual_completion(req: *mut media_request) { (*req).manual_completion = true; }

#[cfg(feature = "CONFIG_MEDIA_CONTROLLER")]
pub unsafe fn media_request_object_get(obj: *mut media_request_object) { kref_get(&mut (*obj).kref); }

#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn media_request_get(_req: *mut media_request) {}
#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn media_request_put(_req: *mut media_request) {}
#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn media_request_get_by_fd(_mdev: *mut media_device, _request_fd: c_int) -> *mut media_request { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn media_request_lock_for_access(_req: *mut media_request) -> c_int { -22 }
#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn media_request_unlock_for_access(_req: *mut media_request) {}
#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn media_request_lock_for_update(_req: *mut media_request) -> c_int { -22 }
#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn media_request_unlock_for_update(_req: *mut media_request) {}
#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn media_request_object_get(_obj: *mut media_request_object) {}
#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn media_request_object_put(_obj: *mut media_request_object) {}
#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn media_request_object_find(_req: *mut media_request, _ops: *const media_request_object_ops, _priv_: *mut c_void) -> *mut media_request_object { core::ptr::null_mut() }
#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn media_request_object_init(obj: *mut media_request_object) { (*obj).ops = core::ptr::null(); (*obj).req = core::ptr::null_mut(); }
#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn media_request_object_bind(_req: *mut media_request, _ops: *const media_request_object_ops, _priv_: *mut c_void, _is_buffer: bool, _obj: *mut media_request_object) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn media_request_object_unbind(_obj: *mut media_request_object) {}
#[cfg(not(feature = "CONFIG_MEDIA_CONTROLLER"))]
pub unsafe fn media_request_object_complete(_obj: *mut media_request_object) {}

extern "C" {
    fn kref_get(kref: *mut kref);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
