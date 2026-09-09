/* SPDX-License-Identifier: GPL-2.0 */

/* C dependencies: <linux/atomic.h>, <linux/cache.h>. */

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

extern "C" {
    pub static mut debug_locks: ::core::ffi::c_int;
    pub static mut debug_locks_silent: ::core::ffi::c_int;

    pub fn xchg(ptr: *mut ::core::ffi::c_int, value: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
    pub fn debug_locks_off() -> ::core::ffi::c_int;

    #[cfg(feature = "CONFIG_DEBUG_LOCKING_API_SELFTESTS")]
    pub fn locking_selftest();

    #[cfg(feature = "CONFIG_LOCKDEP")]
    pub fn debug_show_all_locks();
    #[cfg(feature = "CONFIG_LOCKDEP")]
    pub fn debug_show_held_locks(task: *mut task_struct);
    #[cfg(feature = "CONFIG_LOCKDEP")]
    pub fn debug_check_no_locks_freed(
        from: *const ::core::ffi::c_void,
        len: ::core::ffi::c_ulong,
    );
    #[cfg(feature = "CONFIG_LOCKDEP")]
    pub fn debug_check_no_locks_held();
}

/* These symbols are supplied by the surrounding kernel translation. */
extern "C" {
    pub static mut oops_in_progress: bool;
    pub fn instrumentation_begin();
    pub fn instrumentation_end();
    pub fn WARN(condition: ::core::ffi::c_int, format: *const ::core::ffi::c_char, ...);
}

#[inline(always)]
pub unsafe fn __debug_locks_off() -> ::core::ffi::c_int {
    xchg(&mut debug_locks, 0)
}

#[macro_export]
macro_rules! DEBUG_LOCKS_WARN_ON {
    ($c:expr) => {{
        let mut __ret: ::core::ffi::c_int = 0;
        if unsafe { !oops_in_progress } && ($c) {
            unsafe {
                instrumentation_begin();
                if debug_locks_off() != 0 && debug_locks_silent == 0 {
                    WARN(
                        1,
                        concat!("DEBUG_LOCKS_WARN_ON(", stringify!($c), ")\0")
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                instrumentation_end();
            }
            __ret = 1;
        }
        __ret
    }};
}

#[macro_export]
macro_rules! SMP_DEBUG_LOCKS_WARN_ON {
    ($c:expr) => {
        #[cfg(feature = "CONFIG_SMP")]
        {
            DEBUG_LOCKS_WARN_ON!($c)
        }
        #[cfg(not(feature = "CONFIG_SMP"))]
        {
            ()
        }
    };
}

#[macro_export]
macro_rules! locking_selftest {
    () => {
        #[cfg(feature = "CONFIG_DEBUG_LOCKING_API_SELFTESTS")]
        unsafe {
            locking_selftest();
        }
        #[cfg(not(feature = "CONFIG_DEBUG_LOCKING_API_SELFTESTS"))]
        {}
    };
}

#[cfg(not(feature = "CONFIG_LOCKDEP"))]
#[inline(always)]
pub unsafe fn debug_show_all_locks() {}

#[cfg(not(feature = "CONFIG_LOCKDEP"))]
#[inline(always)]
pub unsafe fn debug_show_held_locks(_task: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_LOCKDEP"))]
#[inline(always)]
pub unsafe fn debug_check_no_locks_freed(
    _from: *const ::core::ffi::c_void,
    _len: ::core::ffi::c_ulong,
) {
}

#[cfg(not(feature = "CONFIG_LOCKDEP"))]
#[inline(always)]
pub unsafe fn debug_check_no_locks_held() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
