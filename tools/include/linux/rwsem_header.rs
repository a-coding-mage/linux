/* SPDX-License-Identifier: GPL-2.0+ */

// C dependency intent: #include <pthread.h>

use core::ffi::{c_int, c_void};
use core::ptr;

extern "C" {
    pub fn pthread_rwlock_init(
        rwlock: *mut pthread_rwlock_t,
        attr: *const pthread_rwlockattr_t,
    ) -> c_int;
    pub fn pthread_rwlock_destroy(rwlock: *mut pthread_rwlock_t) -> c_int;
    pub fn pthread_rwlock_rdlock(rwlock: *mut pthread_rwlock_t) -> c_int;
    pub fn pthread_rwlock_wrlock(rwlock: *mut pthread_rwlock_t) -> c_int;
    pub fn pthread_rwlock_unlock(rwlock: *mut pthread_rwlock_t) -> c_int;
}

pub type pthread_rwlock_t = c_void;
pub type pthread_rwlockattr_t = c_void;

#[repr(C)]
pub struct rw_semaphore {
    pub lock: pthread_rwlock_t,
}

#[inline]
pub unsafe fn init_rwsem(sem: *mut rw_semaphore) -> c_int {
    unsafe { pthread_rwlock_init(core::ptr::addr_of_mut!((*sem).lock), ptr::null()) }
}

#[inline]
pub unsafe fn exit_rwsem(sem: *mut rw_semaphore) -> c_int {
    unsafe { pthread_rwlock_destroy(core::ptr::addr_of_mut!((*sem).lock)) }
}

#[inline]
pub unsafe fn down_read(sem: *mut rw_semaphore) -> c_int {
    unsafe { pthread_rwlock_rdlock(core::ptr::addr_of_mut!((*sem).lock)) }
}

#[inline]
pub unsafe fn up_read(sem: *mut rw_semaphore) -> c_int {
    unsafe { pthread_rwlock_unlock(core::ptr::addr_of_mut!((*sem).lock)) }
}

#[inline]
pub unsafe fn down_write(sem: *mut rw_semaphore) -> c_int {
    unsafe { pthread_rwlock_wrlock(core::ptr::addr_of_mut!((*sem).lock)) }
}

#[inline]
pub unsafe fn up_write(sem: *mut rw_semaphore) -> c_int {
    unsafe { pthread_rwlock_unlock(core::ptr::addr_of_mut!((*sem).lock)) }
}

#[inline]
pub unsafe fn down_read_nested(sem: *mut rw_semaphore, _subclass: c_int) -> c_int {
    unsafe { down_read(sem) }
}

#[inline]
pub unsafe fn down_write_nested(sem: *mut rw_semaphore, _subclass: c_int) -> c_int {
    unsafe { down_write(sem) }
}
