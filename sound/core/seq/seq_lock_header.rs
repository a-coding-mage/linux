/* SPDX-License-Identifier: GPL-2.0 */

// C dependency intent: #include <linux/sched.h>

pub type snd_use_lock_t = atomic_t;

/* initialize lock */
#[inline]
pub unsafe fn snd_use_lock_init(lockp: *mut snd_use_lock_t) {
    unsafe { atomic_set(lockp, 0) }
}

/* increment lock */
#[inline]
pub unsafe fn snd_use_lock_use(lockp: *mut snd_use_lock_t) {
    unsafe { atomic_inc(lockp) }
}

/* release lock */
#[inline]
pub unsafe fn snd_use_lock_free(lockp: *mut snd_use_lock_t) {
    unsafe { atomic_dec(lockp) }
}

/* wait until all locks are released */
unsafe extern "C" {
    pub fn snd_use_lock_sync_helper(lock: *mut snd_use_lock_t, file: *const ::core::ffi::c_char, line: ::core::ffi::c_int);
}

#[macro_export]
macro_rules! snd_use_lock_sync {
    ($lockp:expr) => {
        unsafe {
            snd_use_lock_sync_helper(
                $lockp,
                concat!(file!(), "\0").as_ptr() as *const ::core::ffi::c_char,
                line!() as ::core::ffi::c_int,
            )
        }
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
