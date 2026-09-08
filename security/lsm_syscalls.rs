// SPDX-License-Identifier: GPL-2.0-only
/*
 * System calls implementing the Linux Security Module API.
 *
 *  Copyright (C) 2022 Casey Schaufler <casey@schaufler-ca.com>
 *  Copyright (C) 2022 Intel Corporation
 */

// C dependencies translated as external declarations:
// asm/current.h, linux/compiler_types.h, linux/err.h, linux/errno.h,
// linux/security.h, linux/stddef.h, linux/syscalls.h, linux/types.h,
// linux/lsm_hooks.h, uapi/linux/lsm.h, and "lsm.h".

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

pub type u32 = core::ffi::c_uint;
pub type u64 = core::ffi::c_ulonglong;

pub const E2BIG: core::ffi::c_int = 7;
pub const EFAULT: core::ffi::c_int = 14;
pub const EINVAL: core::ffi::c_int = 22;

extern "C" {
    pub static mut current: *mut task_struct;
    pub static mut lsm_active_cnt: core::ffi::c_int;
    pub static mut lsm_idlist: *mut *mut lsm_id;

    pub static LSM_ATTR_CURRENT: u64;
    pub static LSM_ATTR_EXEC: u64;
    pub static LSM_ATTR_FSCREATE: u64;
    pub static LSM_ATTR_KEYCREATE: u64;
    pub static LSM_ATTR_PREV: u64;
    pub static LSM_ATTR_SOCKCREATE: u64;
    pub static LSM_ATTR_UNDEF: u64;

    pub fn strcmp(
        s1: *const core::ffi::c_char,
        s2: *const core::ffi::c_char,
    ) -> core::ffi::c_int;
    pub fn mutex_lock_interruptible(lock: *mut mutex) -> core::ffi::c_int;
    pub fn mutex_unlock(lock: *mut mutex);
    pub fn security_setselfattr(
        attr: core::ffi::c_uint,
        ctx: *mut lsm_ctx,
        size: u32,
        flags: u32,
    ) -> core::ffi::c_int;
    pub fn security_getselfattr(
        attr: core::ffi::c_uint,
        ctx: *mut lsm_ctx,
        size: *mut u32,
        flags: u32,
    ) -> core::ffi::c_int;
    pub fn get_user_u32(value: *mut u32, ptr: *const u32) -> core::ffi::c_int;
    pub fn put_user_u32(value: u32, ptr: *mut u32) -> core::ffi::c_int;
    pub fn put_user_u64(value: u64, ptr: *mut u64) -> core::ffi::c_int;
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct signal_struct {
    pub cred_guard_mutex: mutex,
}

#[repr(C)]
pub struct task_struct {
    pub signal: *mut signal_struct,
}

#[repr(C)]
pub struct lsm_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct lsm_id {
    pub id: u64,
}

/**
 * lsm_name_to_attr - map an LSM attribute name to its ID
 * @name: name of the attribute
 *
 * Returns the LSM attribute value associated with @name, or 0 if
 * there is no mapping.
 */
#[no_mangle]
pub unsafe extern "C" fn lsm_name_to_attr(name: *const core::ffi::c_char) -> u64 {
    if strcmp(name, b"current\0".as_ptr() as *const core::ffi::c_char) == 0 {
        return LSM_ATTR_CURRENT;
    }
    if strcmp(name, b"exec\0".as_ptr() as *const core::ffi::c_char) == 0 {
        return LSM_ATTR_EXEC;
    }
    if strcmp(name, b"fscreate\0".as_ptr() as *const core::ffi::c_char) == 0 {
        return LSM_ATTR_FSCREATE;
    }
    if strcmp(name, b"keycreate\0".as_ptr() as *const core::ffi::c_char) == 0 {
        return LSM_ATTR_KEYCREATE;
    }
    if strcmp(name, b"prev\0".as_ptr() as *const core::ffi::c_char) == 0 {
        return LSM_ATTR_PREV;
    }
    if strcmp(name, b"sockcreate\0".as_ptr() as *const core::ffi::c_char) == 0 {
        return LSM_ATTR_SOCKCREATE;
    }
    LSM_ATTR_UNDEF
}

/**
 * sys_lsm_set_self_attr - Set current task's security module attribute
 * @attr: which attribute to set
 * @ctx: the LSM contexts
 * @size: size of @ctx
 * @flags: reserved for future use
 *
 * Sets the calling task's LSM context. On success this function
 * returns 0. If the attribute specified cannot be set a negative
 * value indicating the reason for the error is returned.
 */
#[no_mangle]
pub unsafe extern "C" fn sys_lsm_set_self_attr(
    attr: core::ffi::c_uint,
    ctx: *mut lsm_ctx,
    size: u32,
    flags: u32,
) -> core::ffi::c_long {
    let mut rc: core::ffi::c_int;

    rc = mutex_lock_interruptible(&mut (*(*current).signal).cred_guard_mutex);
    if rc < 0 {
        return rc as core::ffi::c_long;
    }
    rc = security_setselfattr(attr, ctx, size, flags);
    mutex_unlock(&mut (*(*current).signal).cred_guard_mutex);
    rc as core::ffi::c_long
}

/**
 * sys_lsm_get_self_attr - Return current task's security module attributes
 * @attr: which attribute to return
 * @ctx: the user-space destination for the information, or NULL
 * @size: pointer to the size of space available to receive the data
 * @flags: special handling options. LSM_FLAG_SINGLE indicates that only
 * attributes associated with the LSM identified in the passed @ctx be
 * reported.
 *
 * Returns the calling task's LSM contexts. On success this
 * function returns the number of @ctx array elements. This value
 * may be zero if there are no LSM contexts assigned. If @size is
 * insufficient to contain the return data -E2BIG is returned and
 * @size is set to the minimum required size. In all other cases
 * a negative value indicating the error is returned.
 */
#[no_mangle]
pub unsafe extern "C" fn sys_lsm_get_self_attr(
    attr: core::ffi::c_uint,
    ctx: *mut lsm_ctx,
    size: *mut u32,
    flags: u32,
) -> core::ffi::c_long {
    security_getselfattr(attr, ctx, size, flags) as core::ffi::c_long
}

/**
 * sys_lsm_list_modules - Return a list of the active security modules
 * @ids: the LSM module ids
 * @size: pointer to size of @ids, updated on return
 * @flags: reserved for future use, must be zero
 *
 * Returns a list of the active LSM ids. On success this function
 * returns the number of @ids array elements. This value may be zero
 * if there are no LSMs active. If @size is insufficient to contain
 * the return data -E2BIG is returned and @size is set to the minimum
 * required size. In all other cases a negative value indicating the
 * error is returned.
 */
#[no_mangle]
pub unsafe extern "C" fn sys_lsm_list_modules(
    mut ids: *mut u64,
    size: *mut u32,
    flags: u32,
) -> core::ffi::c_long {
    let total_size: u32 =
        (lsm_active_cnt as u32).wrapping_mul(core::mem::size_of::<u64>() as u32);
    let mut usize: u32 = 0;
    let mut i: core::ffi::c_int;

    if flags != 0 {
        return -(EINVAL as core::ffi::c_long);
    }

    if get_user_u32(&mut usize, size as *const u32) != 0 {
        return -(EFAULT as core::ffi::c_long);
    }

    if put_user_u32(total_size, size) != 0 {
        return -(EFAULT as core::ffi::c_long);
    }

    if usize < total_size {
        return -(E2BIG as core::ffi::c_long);
    }

    i = 0;
    while i < lsm_active_cnt {
        if put_user_u64((*(*lsm_idlist.offset(i as isize))).id, ids) != 0 {
            return -(EFAULT as core::ffi::c_long);
        }
        ids = ids.offset(1);
        i += 1;
    }

    lsm_active_cnt as core::ffi::c_long
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
