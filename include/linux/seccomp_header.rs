/* SPDX-License-Identifier: GPL-2.0 */

// Translated from <uapi/linux/seccomp.h> and <linux/seccomp_types.h>.

pub const SECCOMP_FILTER_FLAG_MASK: u32 = SECCOMP_FILTER_FLAG_TSYNC
    | SECCOMP_FILTER_FLAG_LOG
    | SECCOMP_FILTER_FLAG_SPEC_ALLOW
    | SECCOMP_FILTER_FLAG_NEW_LISTENER
    | SECCOMP_FILTER_FLAG_TSYNC_ESRCH
    | SECCOMP_FILTER_FLAG_WAIT_KILLABLE_RECV;

// sizeof() the first published struct seccomp_notif_addfd
pub const SECCOMP_NOTIFY_ADDFD_SIZE_VER0: usize = 24;
pub const SECCOMP_NOTIFY_ADDFD_SIZE_LATEST: usize = SECCOMP_NOTIFY_ADDFD_SIZE_VER0;

#[cfg(feature = "CONFIG_SECCOMP")]
extern "C" {
    pub fn __seccomp_permit_syscall() -> bool;

    #[cfg(not(feature = "CONFIG_HAVE_ARCH_SECCOMP_FILTER"))]
    pub fn secure_computing_strict(this_syscall: i32);

    pub fn prctl_get_seccomp() -> ::core::ffi::c_long;
    pub fn prctl_set_seccomp(arg2: ::core::ffi::c_ulong, arg3: *mut ::core::ffi::c_void)
        -> ::core::ffi::c_long;
}

#[cfg(all(feature = "CONFIG_SECCOMP", feature = "CONFIG_HAVE_ARCH_SECCOMP_FILTER"))]
#[inline(always)]
pub unsafe fn seccomp_permit_syscall() -> bool {
    // `unlikely(test_syscall_work(SECCOMP))` is supplied by the architecture dependencies.
    if test_syscall_work(SECCOMP) {
        return __seccomp_permit_syscall();
    }
    true
}

#[cfg(feature = "CONFIG_SECCOMP")]
#[inline]
pub unsafe fn seccomp_mode(s: *const seccomp) -> i32 {
    (*s).mode
}

#[cfg(not(feature = "CONFIG_SECCOMP"))]
pub struct seccomp;

#[cfg(not(feature = "CONFIG_SECCOMP"))]
pub struct seccomp_data;

#[cfg(all(not(feature = "CONFIG_SECCOMP"), feature = "CONFIG_HAVE_ARCH_SECCOMP_FILTER"))]
#[inline]
pub fn seccomp_permit_syscall() -> bool { true }

#[cfg(all(not(feature = "CONFIG_SECCOMP"), not(feature = "CONFIG_HAVE_ARCH_SECCOMP_FILTER")))]
#[inline]
pub fn secure_computing_strict(_this_syscall: i32) {}

#[cfg(not(feature = "CONFIG_SECCOMP"))]
#[inline]
pub fn __seccomp_permit_syscall() -> bool { true }

#[cfg(not(feature = "CONFIG_SECCOMP"))]
#[inline]
pub fn prctl_get_seccomp() -> ::core::ffi::c_long { -EINVAL as ::core::ffi::c_long }

#[cfg(not(feature = "CONFIG_SECCOMP"))]
#[inline]
pub unsafe fn prctl_set_seccomp(
    _arg2: ::core::ffi::c_ulong,
    _arg3: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_long { -EINVAL as ::core::ffi::c_long }

#[cfg(not(feature = "CONFIG_SECCOMP"))]
#[inline]
pub unsafe fn seccomp_mode(_s: *mut seccomp) -> i32 { SECCOMP_MODE_DISABLED }

#[cfg(feature = "CONFIG_SECCOMP_FILTER")]
extern "C" {
    pub fn seccomp_filter_release(tsk: *mut task_struct);
    pub fn get_seccomp_filter(tsk: *mut task_struct);
}

#[cfg(not(feature = "CONFIG_SECCOMP_FILTER"))]
#[inline]
pub fn seccomp_filter_release(_tsk: *mut task_struct) {}

#[cfg(not(feature = "CONFIG_SECCOMP_FILTER"))]
#[inline]
pub fn get_seccomp_filter(_tsk: *mut task_struct) {}

#[cfg(all(feature = "CONFIG_SECCOMP_FILTER", feature = "CONFIG_CHECKPOINT_RESTORE"))]
extern "C" {
    pub fn seccomp_get_filter(
        task: *mut task_struct,
        filter_off: ::core::ffi::c_ulong,
        data: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_long;
    pub fn seccomp_get_metadata(
        task: *mut task_struct,
        filter_off: ::core::ffi::c_ulong,
        data: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_long;
}

#[cfg(not(all(feature = "CONFIG_SECCOMP_FILTER", feature = "CONFIG_CHECKPOINT_RESTORE")))]
#[inline]
pub fn seccomp_get_filter(
    _task: *mut task_struct,
    _n: ::core::ffi::c_ulong,
    _data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_long { -EINVAL as ::core::ffi::c_long }

#[cfg(not(all(feature = "CONFIG_SECCOMP_FILTER", feature = "CONFIG_CHECKPOINT_RESTORE")))]
#[inline]
pub fn seccomp_get_metadata(
    _task: *mut task_struct,
    _filter_off: ::core::ffi::c_ulong,
    _data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_long { -EINVAL as ::core::ffi::c_long }

#[cfg(all(feature = "CONFIG_SECCOMP_CACHE_DEBUG"))]
extern "C" {
    pub fn proc_pid_seccomp_cache(
        m: *mut seq_file,
        ns: *mut pid_namespace,
        pid: *mut pid,
        task: *mut task_struct,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
