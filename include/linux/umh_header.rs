// Translated from linux/umh.h.
// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced but not defined here.

// #include <linux/gfp.h>
// #include <linux/stddef.h>
// #include <linux/errno.h>
// #include <linux/compiler.h>
// #include <linux/workqueue.h>
// #include <linux/sysctl.h>

pub const UMH_NO_WAIT: i32 = 0x00; // don't wait at all
pub const UMH_WAIT_EXEC: i32 = 0x01; // wait for the exec, but not the process
pub const UMH_WAIT_PROC: i32 = 0x02; // wait for the process to complete
pub const UMH_KILLABLE: i32 = 0x04; // wait for EXEC/PROC killable
pub const UMH_FREEZABLE: i32 = 0x08; // wait for EXEC/PROC freezable

#[repr(C)]
pub struct subprocess_info {
    pub work: work_struct,
    pub complete: *mut completion,
    pub path: *const ::core::ffi::c_char,
    pub argv: *mut *mut ::core::ffi::c_char,
    pub envp: *mut *mut ::core::ffi::c_char,
    pub wait: ::core::ffi::c_int,
    pub retval: ::core::ffi::c_int,
    pub init: Option<unsafe extern "C" fn(
        info: *mut subprocess_info,
        new: *mut cred,
    ) -> ::core::ffi::c_int>,
    pub cleanup: Option<unsafe extern "C" fn(info: *mut subprocess_info)>,
    pub data: *mut ::core::ffi::c_void,
}

unsafe extern "C" {
    pub fn call_usermodehelper(
        path: *const ::core::ffi::c_char,
        argv: *mut *mut ::core::ffi::c_char,
        envp: *mut *mut ::core::ffi::c_char,
        wait: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub fn call_usermodehelper_setup(
        path: *const ::core::ffi::c_char,
        argv: *mut *mut ::core::ffi::c_char,
        envp: *mut *mut ::core::ffi::c_char,
        gfp_mask: gfp_t,
        init: Option<unsafe extern "C" fn(
            info: *mut subprocess_info,
            new: *mut cred,
        ) -> ::core::ffi::c_int>,
        cleanup: Option<unsafe extern "C" fn(info: *mut subprocess_info)>,
        data: *mut ::core::ffi::c_void,
    ) -> *mut subprocess_info;

    pub fn call_usermodehelper_exec(
        info: *mut subprocess_info,
        wait: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub fn __usermodehelper_disable(depth: umh_disable_depth) -> ::core::ffi::c_int;
    pub fn __usermodehelper_set_disable_depth(depth: umh_disable_depth);

    pub fn usermodehelper_read_trylock() -> ::core::ffi::c_int;
    pub fn usermodehelper_read_lock_wait(timeout: ::core::ffi::c_long) -> ::core::ffi::c_long;
    pub fn usermodehelper_read_unlock();
}

#[repr(C)]
pub struct cred;

#[repr(C)]
pub struct file;

#[repr(C)]
pub struct work_struct;

#[repr(C)]
pub struct completion;

pub type gfp_t = ::core::ffi::c_uint;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum umh_disable_depth {
    UMH_ENABLED = 0,
    UMH_FREEZING,
    UMH_DISABLED,
}

#[inline]
pub unsafe fn usermodehelper_disable() -> ::core::ffi::c_int {
    unsafe { __usermodehelper_disable(umh_disable_depth::UMH_DISABLED) }
}

#[inline]
pub unsafe fn usermodehelper_enable() {
    unsafe { __usermodehelper_set_disable_depth(umh_disable_depth::UMH_ENABLED) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
