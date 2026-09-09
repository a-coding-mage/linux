// SPDX-License-Identifier: GPL-2.0
/*
 * Supplementary group IDs
 */

// Declarations supplied by the surrounding kernel translation unit.
extern "C" {
    fn kvmalloc_flex<T>(member: T, count: usize, flags: u32) -> *mut group_info;
    fn kvfree(ptr: *mut group_info);
    fn refcount_set(value: *mut refcount_t, count: i32);
    fn current_user_ns() -> *mut user_namespace;
    fn from_kgid_munged(ns: *mut user_namespace, gid: kgid_t) -> gid_t;
    fn put_user<T>(value: T, ptr: *mut T) -> i32;
    fn get_user<T>(value: *mut T, ptr: *const T) -> i32;
    fn make_kgid(ns: *mut user_namespace, gid: gid_t) -> kgid_t;
    fn gid_valid(gid: kgid_t) -> bool;
    fn gid_gt(a: kgid_t, b: kgid_t) -> bool;
    fn gid_lt(a: kgid_t, b: kgid_t) -> bool;
    fn sort<T>(base: *mut T, num: usize, size: usize,
               cmp: unsafe extern "C" fn(*const core::ffi::c_void,
                                          *const core::ffi::c_void) -> i32,
               priv_: *mut core::ffi::c_void);
    fn put_group_info(info: *mut group_info);
    fn get_group_info(info: *mut group_info);
    fn prepare_creds() -> *mut cred;
    fn current_cred() -> *const cred;
    fn security_task_fix_setgroups(new: *mut cred, old: *const cred) -> i32;
    fn commit_creds(new: *mut cred) -> i32;
    fn abort_creds(new: *mut cred);
    fn ns_capable_setid(ns: *mut user_namespace, cap: i32) -> bool;
    fn userns_may_setgroups(ns: *mut user_namespace) -> bool;
}

type gid_t = u32;
type kgid_t = u32;
type refcount_t = i32;

#[repr(C)]
pub struct group_info {
    pub usage: refcount_t,
    pub ngroups: u32,
    pub gid: [kgid_t; 0],
}

#[repr(C)]
pub struct user_namespace { _private: [u8; 0] }
#[repr(C)]
pub struct cred {
    pub group_info: *mut group_info,
    pub fsgid: kgid_t,
    pub egid: kgid_t,
}

const GFP_KERNEL_ACCOUNT: u32 = 0;
const EFAULT: i32 = 14;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const EPERM: i32 = 1;
const NGROUPS_MAX: u32 = 65536;
const CAP_SETGID: i32 = 6;

#[no_mangle]
pub unsafe extern "C" fn groups_alloc(gidsetsize: i32) -> *mut group_info {
    let gi = kvmalloc_flex(core::mem::MaybeUninit::<kgid_t>::uninit(),
                            gidsetsize as usize, GFP_KERNEL_ACCOUNT);
    if gi.is_null() { return core::ptr::null_mut(); }
    refcount_set(&mut (*gi).usage, 1);
    (*gi).ngroups = gidsetsize as u32;
    gi
}

#[no_mangle]
pub unsafe extern "C" fn groups_free(group_info: *mut group_info) {
    kvfree(group_info);
}

/* export the group_info to a user-space array */
unsafe fn groups_to_user(grouplist: *mut gid_t, group_info: *const group_info) -> i32 {
    let user_ns = current_user_ns();
    let count = (*group_info).ngroups;
    for i in 0..count {
        let gid = from_kgid_munged(user_ns, *(*group_info).gid.as_ptr().add(i as usize));
        if put_user(gid, grouplist.add(i as usize)) != 0 { return -EFAULT; }
    }
    0
}

/* fill a group_info from a user-space array - it must be allocated already */
unsafe fn groups_from_user(group_info: *mut group_info, grouplist: *mut gid_t) -> i32 {
    let user_ns = current_user_ns();
    let count = (*group_info).ngroups;
    for i in 0..count {
        let mut gid = 0;
        if get_user(&mut gid, grouplist.add(i as usize)) != 0 { return -EFAULT; }
        let kgid = make_kgid(user_ns, gid);
        if !gid_valid(kgid) { return -EINVAL; }
        *(*group_info).gid.as_mut_ptr().add(i as usize) = kgid;
    }
    0
}

unsafe extern "C" fn gid_cmp(a: *const core::ffi::c_void, b: *const core::ffi::c_void) -> i32 {
    let a = *(a as *const kgid_t);
    let b = *(b as *const kgid_t);
    gid_gt(a, b) as i32 - gid_lt(a, b) as i32
}

#[no_mangle]
pub unsafe extern "C" fn groups_sort(group_info: *mut group_info) {
    sort((*group_info).gid.as_mut_ptr(), (*group_info).ngroups as usize,
         core::mem::size_of::<kgid_t>(), gid_cmp, core::ptr::null_mut());
}

/* a simple bsearch */
#[no_mangle]
pub unsafe extern "C" fn groups_search(group_info: *const group_info, grp: kgid_t) -> i32 {
    if group_info.is_null() { return 0; }
    let mut left = 0u32;
    let mut right = (*group_info).ngroups;
    while left < right {
        let mid = (left + right) / 2;
        let item = *(*group_info).gid.as_ptr().add(mid as usize);
        if gid_gt(grp, item) { left = mid + 1; }
        else if gid_lt(grp, item) { right = mid; }
        else { return 1; }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn set_groups(new: *mut cred, group_info: *mut group_info) {
    put_group_info((*new).group_info);
    get_group_info(group_info);
    (*new).group_info = group_info;
}

#[no_mangle]
pub unsafe extern "C" fn set_current_groups(group_info: *mut group_info) -> i32 {
    let new = prepare_creds();
    if new.is_null() { return -ENOMEM; }
    let old = current_cred();
    set_groups(new, group_info);
    let retval = security_task_fix_setgroups(new, old);
    if retval < 0 { abort_creds(new); return retval; }
    commit_creds(new)
}

#[no_mangle]
pub unsafe extern "C" fn getgroups(gidsetsize: i32, grouplist: *mut gid_t) -> i32 {
    let cred = current_cred();
    if gidsetsize < 0 { return -EINVAL; }
    let mut i = (*(*cred).group_info).ngroups as i32;
    if gidsetsize != 0 {
        if i > gidsetsize { i = -EINVAL; }
        else if groups_to_user(grouplist, (*cred).group_info) != 0 { i = -EFAULT; }
    }
    i
}

#[no_mangle]
pub unsafe extern "C" fn may_setgroups() -> bool {
    let user_ns = current_user_ns();
    ns_capable_setid(user_ns, CAP_SETGID) && userns_may_setgroups(user_ns)
}

#[no_mangle]
pub unsafe extern "C" fn setgroups(gidsetsize: i32, grouplist: *mut gid_t) -> i32 {
    if !may_setgroups() { return -EPERM; }
    if (gidsetsize as u32) > NGROUPS_MAX { return -EINVAL; }
    let group_info = groups_alloc(gidsetsize);
    if group_info.is_null() { return -ENOMEM; }
    let retval = groups_from_user(group_info, grouplist);
    if retval != 0 { put_group_info(group_info); return retval; }
    groups_sort(group_info);
    let retval = set_current_groups(group_info);
    put_group_info(group_info);
    retval
}

#[no_mangle]
pub unsafe extern "C" fn in_group_p(grp: kgid_t) -> i32 {
    let cred = current_cred();
    if !gid_eq(grp, (*cred).fsgid) { return groups_search((*cred).group_info, grp); }
    1
}

#[no_mangle]
pub unsafe extern "C" fn in_egroup_p(grp: kgid_t) -> i32 {
    let cred = current_cred();
    if !gid_eq(grp, (*cred).egid) { return groups_search((*cred).group_info, grp); }
    1
}

extern "C" { fn gid_eq(a: kgid_t, b: kgid_t) -> bool; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
