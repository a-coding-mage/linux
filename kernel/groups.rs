// SPDX-License-Identifier: GPL-2.0
/*
 * Supplementary group IDs
 */

// Continued from Rust blob 659c84d650d23ebc73128b289028f6ee244df10c.
// Native oracle: groups.c blob b5e3be6a6b1f4acd624623fa49c1e7bfeb434e35.
// Configured native headers alone own types, layouts, constants and accessors.
// This source-only proposal does not select an object or replace groups.c.
#![no_std]
compile_error!("SOURCE ONLY HOLD: supplementary-group Rust owner is not admitted");

use kernel::bindings::groups_native::*;
use kernel::bindings::groups_native::{
    LUPOS_GROUPS_CAP_SETGID as CAP_SETGID,
    LUPOS_GROUPS_EFAULT as EFAULT,
    LUPOS_GROUPS_EINVAL as EINVAL,
    LUPOS_GROUPS_ENOMEM as ENOMEM,
    LUPOS_GROUPS_EPERM as EPERM,
    LUPOS_GROUPS_GFP_KERNEL_ACCOUNT as GFP_KERNEL_ACCOUNT,
    LUPOS_GROUPS_NGROUPS_MAX as NGROUPS_MAX,
    lupos_groups_current_cred as current_cred,
    lupos_groups_current_user_ns as current_user_ns,
    lupos_groups_from_kgid_munged as from_kgid_munged,
    lupos_groups_make_kgid as make_kgid,
    lupos_groups_gid_valid as gid_valid,
    lupos_groups_gid_gt as gid_gt,
    lupos_groups_gid_lt as gid_lt,
    lupos_groups_gid_eq as gid_eq,
    lupos_groups_get_group_info as get_group_info,
    lupos_groups_put_group_info as put_group_info,
    lupos_groups_security_task_fix_setgroups as security_task_fix_setgroups,
    lupos_groups_userns_may_setgroups as userns_may_setgroups,
};
use kernel::ffi::{c_int, c_long, c_uint, c_void};

/// # Safety
/// Caller may sleep and supplies the native int group count. A nonnull result
/// owns one reference and has an uninitialized gid array of that count.
#[no_mangle]
pub unsafe extern "C" fn groups_alloc(gidsetsize: c_int) -> *mut group_info {
    // SAFETY: The caller upholds the native lifetime and access contract above.
    unsafe {
        let gi = lupos_groups_alloc_flex(gidsetsize, GFP_KERNEL_ACCOUNT);
        if gi.is_null() { return core::ptr::null_mut(); }
        lupos_groups_usage_set(gi, 1);
        lupos_groups_set_ngroups(gi, gidsetsize);
        gi
    }
}

/// # Safety
/// The allocation is native and no remaining owner may access it after free.
#[no_mangle]
pub unsafe extern "C" fn groups_free(group_info: *mut group_info) {
    // SAFETY: The caller upholds the native lifetime and access contract above.
    unsafe {
        lupos_groups_kvfree(group_info);
    }
}

/* export the group_info to a user-space array */
// SAFETY: The current credential pins group_info. User pointers remain raw and
// are accessed only by the native __user put_user primitive, including indexing.
unsafe fn groups_to_user(grouplist: *mut gid_t, group_info: *const group_info) -> c_int {
    // SAFETY: The caller upholds the native lifetime and access contract above.
    unsafe {
        let user_ns = current_user_ns();
        let count = lupos_groups_ngroups(group_info) as c_uint;
        for i in 0..count {
            let gid = from_kgid_munged(user_ns, lupos_groups_gid(group_info, i));
            if lupos_groups_put_user_gid(gid, grouplist, i) != 0 { return -EFAULT; }
        }
        0
    }
}

/* fill a group_info from a user-space array - it must be allocated already */
// SAFETY: Caller exclusively constructs group_info and its native flex array.
// Every initialized value is mapped and validated before storage; partial
// failure leaves the object unpublished and the caller drops its reference.
unsafe fn groups_from_user(group_info: *mut group_info, grouplist: *mut gid_t) -> c_int {
    // SAFETY: The caller upholds the native lifetime and access contract above.
    unsafe {
        let user_ns = current_user_ns();
        let count = lupos_groups_ngroups(group_info) as c_uint;
        for i in 0..count {
            let mut gid: gid_t = 0;
            if lupos_groups_get_user_gid(&mut gid, grouplist, i) != 0 { return -EFAULT; }
            let kgid = make_kgid(user_ns, gid);
            if !gid_valid(kgid) { return -EINVAL; }
            lupos_groups_set_gid(group_info, i, kgid);
        }
        0
    }
}

/// # Safety
/// Only the native sort boundary passes pointers to initialized, aligned native
/// kgid_t elements. The native load preserves the original void-pointer ABI;
/// Rust never projects a fabricated kgid_t or credential layout.
#[no_mangle]
pub unsafe extern "C" fn lupos_groups_gid_cmp(a: *const c_void, b: *const c_void) -> c_int {
    // SAFETY: The caller upholds the native lifetime and access contract above.
    unsafe {
        let a = lupos_groups_sort_gid_value(a);
        let b = lupos_groups_sort_gid_value(b);
        gid_gt(a, b) as c_int - gid_lt(a, b) as c_int
    }
}

/// # Safety
/// Caller exclusively owns a fully initialized native group array until sorted.
#[no_mangle]
pub unsafe extern "C" fn groups_sort(group_info: *mut group_info) {
    // SAFETY: The caller upholds the native lifetime and access contract above.
    unsafe {
        lupos_groups_sort(group_info);
    }
}

/* a simple bsearch */
/// # Safety
/// A nonnull group_info is pinned and contains a sorted, initialized gid array.
#[no_mangle]
pub unsafe extern "C" fn groups_search(group_info: *const group_info, grp: kgid_t) -> c_int {
    // SAFETY: The caller upholds the native lifetime and access contract above.
    unsafe {
        if group_info.is_null() { return 0; }
        let mut left: c_uint = 0;
        let mut right = lupos_groups_ngroups(group_info) as c_uint;
        while left < right {
            let mid = left.wrapping_add(right) / 2;
            let item = lupos_groups_gid(group_info, mid);
            if gid_gt(grp, item) { left = mid + 1; }
            else if gid_lt(grp, item) { right = mid; }
            else { return 1; }
        }
        0
    }
}

/// # Safety
/// new is an exclusively mutable prepared native credential; group_info is
/// pinned by the caller. Drop old, take new, then store, as in native C.
#[no_mangle]
pub unsafe extern "C" fn set_groups(new: *mut cred, group_info: *mut group_info) {
    // SAFETY: The caller upholds the native lifetime and access contract above.
    unsafe {
        put_group_info(lupos_groups_cred_group_info(new));
        let _ = get_group_info(group_info);
        lupos_groups_cred_set_group_info(new, group_info);
    }
}

/// # Safety
/// Caller may prepare/commit current credentials and pins group_info. The LSM
/// receives the prepared credential and current old credential in native order.
#[no_mangle]
pub unsafe extern "C" fn set_current_groups(group_info: *mut group_info) -> c_int {
    // SAFETY: The caller upholds the native lifetime and access contract above.
    unsafe {
        let new = prepare_creds();
        if new.is_null() { return -ENOMEM; }
        let old = current_cred();
        set_groups(new, group_info);
        let retval = security_task_fix_setgroups(new, old);
        if retval < 0 { abort_creds(new); return retval; }
        commit_creds(new)
    }
}

// Native SYSCALL_DEFINE2 owns entry decoding, tracing metadata and long return.
/// # Safety
/// Called by the native getgroups syscall adapter in current's syscall context.
/// The raw userspace address is validated exclusively by the usercopy primitive.
#[no_mangle]
pub unsafe extern "C" fn lupos_groups_getgroups(gidsetsize: c_int, grouplist: *mut gid_t) -> c_long {
    // SAFETY: The caller upholds the native lifetime and access contract above.
    unsafe {
        let cred = current_cred();
        if gidsetsize < 0 { return (-EINVAL) as c_long; }
        let mut i = lupos_groups_ngroups(lupos_groups_cred_group_info(cred));
        if gidsetsize != 0 {
            if i > gidsetsize { i = -EINVAL; }
            else if groups_to_user(grouplist, lupos_groups_cred_group_info(cred)) != 0 { i = -EFAULT; }
        }
        i as c_long
    }
}

/// # Safety
/// Caller is current; namespace and capability policy are provided natively.
#[no_mangle]
pub unsafe extern "C" fn may_setgroups() -> bool {
    // SAFETY: The caller upholds the native lifetime and access contract above.
    unsafe {
        let user_ns = current_user_ns();
        ns_capable_setid(user_ns, CAP_SETGID) && userns_may_setgroups(user_ns)
    }
}

/// # Safety
/// Called by the native setgroups syscall adapter in current's syscall context.
/// Allocated groups remain private until sorting and the LSM/commit succeed.
#[no_mangle]
pub unsafe extern "C" fn lupos_groups_setgroups(gidsetsize: c_int, grouplist: *mut gid_t) -> c_long {
    // SAFETY: The caller upholds the native lifetime and access contract above.
    unsafe {
        if !may_setgroups() { return (-EPERM) as c_long; }
        if (gidsetsize as c_uint) > NGROUPS_MAX { return (-EINVAL) as c_long; }
        let group_info = groups_alloc(gidsetsize);
        if group_info.is_null() { return (-ENOMEM) as c_long; }
        let retval = groups_from_user(group_info, grouplist);
        if retval != 0 { put_group_info(group_info); return retval as c_long; }
        groups_sort(group_info);
        let retval = set_current_groups(group_info);
        put_group_info(group_info);
        retval as c_long
    }
}

/* Check whether we're fsgid/egid or in the supplemental group. */
/// # Safety
/// Called for current; its credential pins the group array throughout lookup.
#[no_mangle]
pub unsafe extern "C" fn in_group_p(grp: kgid_t) -> c_int {
    // SAFETY: The caller upholds the native lifetime and access contract above.
    unsafe {
        let cred = current_cred();
        if !gid_eq(grp, lupos_groups_cred_fsgid(cred)) {
            return groups_search(lupos_groups_cred_group_info(cred), grp);
        }
        1
    }
}

/// # Safety
/// Called for current; its credential pins the group array throughout lookup.
#[no_mangle]
pub unsafe extern "C" fn in_egroup_p(grp: kgid_t) -> c_int {
    // SAFETY: The caller upholds the native lifetime and access contract above.
    unsafe {
        let cred = current_cred();
        if !gid_eq(grp, lupos_groups_cred_egid(cred)) {
            return groups_search(lupos_groups_cred_group_info(cred), grp);
        }
        1
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
