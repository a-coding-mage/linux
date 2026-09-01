#[repr(C)]
pub struct local_lock_t {}

#[inline]
pub unsafe fn local_lock(lock: *mut local_lock_t) {}

#[inline]
pub unsafe fn local_unlock(lock: *mut local_lock_t) {}

pub const INIT_LOCAL_LOCK: local_lock_t = local_lock_t {};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
