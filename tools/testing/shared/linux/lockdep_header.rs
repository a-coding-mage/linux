// Translated from testing/shared/linux/lockdep.h
// Original dependency: #include <linux/spinlock.h>

#[repr(C)]
pub struct lock_class_key {
    pub a: ::std::os::raw::c_uint,
}

#[inline]
pub unsafe fn lockdep_set_class(
    lock: *mut spinlock_t,
    key: *mut lock_class_key,
) {
    let _ = lock;
    let _ = key;
}

extern "C" {
    pub fn lockdep_is_held(arg1: *const ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
