#[repr(C)]
pub struct local_lock_t {}

#[inline]
pub unsafe fn local_lock(lock: *mut local_lock_t) {}

#[inline]
pub unsafe fn local_unlock(lock: *mut local_lock_t) {}

pub const INIT_LOCAL_LOCK: local_lock_t = local_lock_t {};
