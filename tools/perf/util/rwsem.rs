// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/rwsem.c.
// Original C dependencies: "util.h", "rwsem.h", and, when RWS_ERRORCHECK is
// enabled, "mutex.h".

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]
#![allow(unexpected_cfgs)]

use core::ffi::{c_int, c_void};

#[repr(C)]
pub struct pthread_rwlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rw_semaphore {
    #[cfg(RWS_ERRORCHECK)]
    pub mtx: mutex,
    #[cfg(not(RWS_ERRORCHECK))]
    pub lock: pthread_rwlock_t,
}

unsafe extern "C" {
    #[cfg(not(RWS_ERRORCHECK))]
    fn pthread_rwlock_init(lock: *mut pthread_rwlock_t, attr: *const c_void) -> c_int;
    #[cfg(not(RWS_ERRORCHECK))]
    fn pthread_rwlock_destroy(lock: *mut pthread_rwlock_t) -> c_int;
    #[cfg(not(RWS_ERRORCHECK))]
    fn pthread_rwlock_rdlock(lock: *mut pthread_rwlock_t) -> c_int;
    #[cfg(not(RWS_ERRORCHECK))]
    fn pthread_rwlock_wrlock(lock: *mut pthread_rwlock_t) -> c_int;
    #[cfg(not(RWS_ERRORCHECK))]
    fn pthread_rwlock_unlock(lock: *mut pthread_rwlock_t) -> c_int;

    #[cfg(RWS_ERRORCHECK)]
    fn mutex_init(mtx: *mut mutex);
    #[cfg(RWS_ERRORCHECK)]
    fn mutex_destroy(mtx: *mut mutex);
    #[cfg(RWS_ERRORCHECK)]
    fn mutex_lock(mtx: *mut mutex);
    #[cfg(RWS_ERRORCHECK)]
    fn mutex_unlock(mtx: *mut mutex);

    #[cfg(not(RWS_ERRORCHECK))]
    static mut perf_singlethreaded: c_int;
}

#[no_mangle]
pub unsafe extern "C" fn init_rwsem(sem: *mut rw_semaphore) -> c_int {
    #[cfg(RWS_ERRORCHECK)]
    {
        unsafe {
            mutex_init(core::ptr::addr_of_mut!((*sem).mtx));
        }
        0
    }

    #[cfg(not(RWS_ERRORCHECK))]
    {
        unsafe { pthread_rwlock_init(core::ptr::addr_of_mut!((*sem).lock), core::ptr::null()) }
    }
}

#[no_mangle]
pub unsafe extern "C" fn exit_rwsem(sem: *mut rw_semaphore) -> c_int {
    #[cfg(RWS_ERRORCHECK)]
    {
        unsafe {
            mutex_destroy(core::ptr::addr_of_mut!((*sem).mtx));
        }
        0
    }

    #[cfg(not(RWS_ERRORCHECK))]
    {
        unsafe { pthread_rwlock_destroy(core::ptr::addr_of_mut!((*sem).lock)) }
    }
}

#[no_mangle]
pub unsafe extern "C" fn down_read(sem: *mut rw_semaphore) -> c_int {
    // Original C function was annotated NO_THREAD_SAFETY_ANALYSIS.
    #[cfg(RWS_ERRORCHECK)]
    {
        unsafe {
            mutex_lock(core::ptr::addr_of_mut!((*sem).mtx));
        }
        0
    }

    #[cfg(not(RWS_ERRORCHECK))]
    {
        unsafe {
            if perf_singlethreaded != 0 {
                0
            } else {
                pthread_rwlock_rdlock(core::ptr::addr_of_mut!((*sem).lock))
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn up_read(sem: *mut rw_semaphore) -> c_int {
    // Original C function was annotated NO_THREAD_SAFETY_ANALYSIS.
    #[cfg(RWS_ERRORCHECK)]
    {
        unsafe {
            mutex_unlock(core::ptr::addr_of_mut!((*sem).mtx));
        }
        0
    }

    #[cfg(not(RWS_ERRORCHECK))]
    {
        unsafe {
            if perf_singlethreaded != 0 {
                0
            } else {
                pthread_rwlock_unlock(core::ptr::addr_of_mut!((*sem).lock))
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn down_write(sem: *mut rw_semaphore) -> c_int {
    // Original C function was annotated NO_THREAD_SAFETY_ANALYSIS.
    #[cfg(RWS_ERRORCHECK)]
    {
        unsafe {
            mutex_lock(core::ptr::addr_of_mut!((*sem).mtx));
        }
        0
    }

    #[cfg(not(RWS_ERRORCHECK))]
    {
        unsafe {
            if perf_singlethreaded != 0 {
                0
            } else {
                pthread_rwlock_wrlock(core::ptr::addr_of_mut!((*sem).lock))
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn up_write(sem: *mut rw_semaphore) -> c_int {
    // Original C function was annotated NO_THREAD_SAFETY_ANALYSIS.
    #[cfg(RWS_ERRORCHECK)]
    {
        unsafe {
            mutex_unlock(core::ptr::addr_of_mut!((*sem).mtx));
        }
        0
    }

    #[cfg(not(RWS_ERRORCHECK))]
    {
        unsafe {
            if perf_singlethreaded != 0 {
                0
            } else {
                pthread_rwlock_unlock(core::ptr::addr_of_mut!((*sem).lock))
            }
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
