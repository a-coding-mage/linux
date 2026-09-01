// Header guard SPINLOCK_H_STUB omitted in Rust.
// Dependency intent from C source: #include <pthread.h>

pub type PthreadSpinlockT = ::core::ffi::c_int;
pub type spinlock_t = PthreadSpinlockT;

unsafe extern "C" {
    pub fn pthread_spin_init(
        lock: *mut PthreadSpinlockT,
        pshared: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn pthread_spin_lock(lock: *mut PthreadSpinlockT) -> ::core::ffi::c_int;
    pub fn pthread_spin_unlock(lock: *mut PthreadSpinlockT) -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn spin_lock_init(lock: *mut spinlock_t) {
    let r: ::core::ffi::c_int = unsafe { pthread_spin_init(lock, 0) };
    assert!(!(r != 0));
}

#[inline]
pub unsafe fn spin_lock(lock: *mut spinlock_t) {
    let ret: ::core::ffi::c_int = unsafe { pthread_spin_lock(lock) };
    assert!(!(ret != 0));
}

#[inline]
pub unsafe fn spin_unlock(lock: *mut spinlock_t) {
    let ret: ::core::ffi::c_int = unsafe { pthread_spin_unlock(lock) };
    assert!(!(ret != 0));
}

#[inline]
pub unsafe fn spin_lock_bh(lock: *mut spinlock_t) {
    unsafe { spin_lock(lock) };
}

#[inline]
pub unsafe fn spin_unlock_bh(lock: *mut spinlock_t) {
    unsafe { spin_unlock(lock) };
}

#[inline]
pub unsafe fn spin_lock_irq(lock: *mut spinlock_t) {
    unsafe { spin_lock(lock) };
}

#[inline]
pub unsafe fn spin_unlock_irq(lock: *mut spinlock_t) {
    unsafe { spin_unlock(lock) };
}

#[inline]
pub unsafe fn spin_lock_irqsave(lock: *mut spinlock_t, f: ::core::ffi::c_ulong) {
    let _ = f;
    unsafe { spin_lock(lock) };
}

#[inline]
pub unsafe fn spin_unlock_irqrestore(lock: *mut spinlock_t, f: ::core::ffi::c_ulong) {
    let _ = f;
    unsafe { spin_unlock(lock) };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
