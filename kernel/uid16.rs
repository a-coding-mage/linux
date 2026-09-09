// SPDX-License-Identifier: GPL-2.0
/*
 * Wrapper functions for 16bit uid back compatibility. All nicely tied
 * together in the faint hope we can take the out in five years time.
 *
 * Declarations supplied by the Linux kernel headers are intentionally left as
 * external Rust symbols.
 */

extern "C" {
    fn ksys_chown(filename: *const core::ffi::c_char, user: uid_t, group: gid_t) -> long;
    fn ksys_lchown(filename: *const core::ffi::c_char, user: uid_t, group: gid_t) -> long;
    fn ksys_fchown(fd: core::ffi::c_uint, user: uid_t, group: gid_t) -> long;
    fn __sys_setregid(rgid: gid_t, egid: gid_t) -> long;
    fn __sys_setgid(gid: gid_t) -> long;
    fn __sys_setreuid(ruid: uid_t, euid: uid_t) -> long;
    fn __sys_setuid(uid: uid_t) -> long;
    fn __sys_setresuid(ruid: uid_t, euid: uid_t, suid: uid_t) -> long;
    fn __sys_setresgid(rgid: gid_t, egid: gid_t, sgid: gid_t) -> long;
    fn __sys_setfsuid(uid: uid_t) -> long;
    fn __sys_setfsgid(gid: gid_t) -> long;
    fn current_cred() -> *const cred;
    fn current_user_ns() -> *mut user_namespace;
    fn current_uid() -> kuid_t;
    fn current_euid() -> kuid_t;
    fn current_gid() -> kgid_t;
    fn current_egid() -> kgid_t;
    fn from_kuid_munged(ns: *mut user_namespace, uid: kuid_t) -> uid_t;
    fn from_kgid_munged(ns: *mut user_namespace, gid: kgid_t) -> gid_t;
    fn high2lowuid(uid: uid_t) -> old_uid_t;
    fn high2lowgid(gid: gid_t) -> old_gid_t;
    fn low2highuid(uid: old_uid_t) -> uid_t;
    fn low2highgid(gid: old_gid_t) -> gid_t;
    fn put_user<T>(value: T, ptr: *mut T) -> int;
    fn get_user<T>(value: *mut T, ptr: *const T) -> int;
    fn current_user_ns() -> *mut user_namespace;
    fn make_kgid(ns: *mut user_namespace, gid: gid_t) -> kgid_t;
    fn gid_valid(gid: kgid_t) -> bool;
    fn may_setgroups() -> bool;
    fn groups_alloc(size: int) -> *mut group_info;
    fn put_group_info(info: *mut group_info);
    fn groups_sort(info: *mut group_info);
    fn set_current_groups(info: *mut group_info) -> int;
}

#[repr(C)]
pub struct cred {
    pub user_ns: *mut user_namespace,
    pub uid: kuid_t,
    pub euid: kuid_t,
    pub suid: kuid_t,
    pub gid: kgid_t,
    pub egid: kgid_t,
    pub sgid: kgid_t,
    pub group_info: *mut group_info,
}

#[repr(C)]
pub struct group_info {
    pub ngroups: int,
    pub gid: *mut kgid_t,
}

pub enum user_namespace {}
pub type old_uid_t = u16;
pub type old_gid_t = u16;
pub type uid_t = u32;
pub type gid_t = u32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = i32;
pub type long = isize;

const EFAULT: int = 14;
const EINVAL: int = 22;
const EPERM: int = 1;
const ENOMEM: int = 12;
const NGROUPS_MAX: u32 = 65536;

pub unsafe fn chown16(filename: *const core::ffi::c_char, user: old_uid_t, group: old_gid_t) -> long {
    ksys_chown(filename, low2highuid(user), low2highgid(group))
}

pub unsafe fn lchown16(filename: *const core::ffi::c_char, user: old_uid_t, group: old_gid_t) -> long {
    ksys_lchown(filename, low2highuid(user), low2highgid(group))
}

pub unsafe fn fchown16(fd: core::ffi::c_uint, user: old_uid_t, group: old_gid_t) -> long {
    ksys_fchown(fd, low2highuid(user), low2highgid(group))
}

pub unsafe fn setregid16(rgid: old_gid_t, egid: old_gid_t) -> long { __sys_setregid(low2highgid(rgid), low2highgid(egid)) }
pub unsafe fn setgid16(gid: old_gid_t) -> long { __sys_setgid(low2highgid(gid)) }
pub unsafe fn setreuid16(ruid: old_uid_t, euid: old_uid_t) -> long { __sys_setreuid(low2highuid(ruid), low2highuid(euid)) }
pub unsafe fn setuid16(uid: old_uid_t) -> long { __sys_setuid(low2highuid(uid)) }
pub unsafe fn setresuid16(ruid: old_uid_t, euid: old_uid_t, suid: old_uid_t) -> long { __sys_setresuid(low2highuid(ruid), low2highuid(euid), low2highuid(suid)) }

pub unsafe fn getresuid16(ruidp: *mut old_uid_t, euidp: *mut old_uid_t, suidp: *mut old_uid_t) -> long {
    let cred = &*current_cred();
    let ruid = high2lowuid(from_kuid_munged(cred.user_ns, cred.uid));
    let euid = high2lowuid(from_kuid_munged(cred.user_ns, cred.euid));
    let suid = high2lowuid(from_kuid_munged(cred.user_ns, cred.suid));
    let mut retval = put_user(ruid, ruidp);
    if retval == 0 { retval = put_user(euid, euidp); }
    if retval == 0 { retval = put_user(suid, suidp); }
    retval as long
}

pub unsafe fn setresgid16(rgid: old_gid_t, egid: old_gid_t, sgid: old_gid_t) -> long { __sys_setresgid(low2highgid(rgid), low2highgid(egid), low2highgid(sgid)) }

pub unsafe fn getresgid16(rgidp: *mut old_gid_t, egidp: *mut old_gid_t, sgidp: *mut old_gid_t) -> long {
    let cred = &*current_cred();
    let rgid = high2lowgid(from_kgid_munged(cred.user_ns, cred.gid));
    let egid = high2lowgid(from_kgid_munged(cred.user_ns, cred.egid));
    let sgid = high2lowgid(from_kgid_munged(cred.user_ns, cred.sgid));
    let mut retval = put_user(rgid, rgidp);
    if retval == 0 { retval = put_user(egid, egidp); }
    if retval == 0 { retval = put_user(sgid, sgidp); }
    retval as long
}

pub unsafe fn setfsuid16(uid: old_uid_t) -> long { __sys_setfsuid(low2highuid(uid)) }
pub unsafe fn setfsgid16(gid: old_gid_t) -> long { __sys_setfsgid(low2highgid(gid)) }

unsafe fn groups16_to_user(grouplist: *mut old_gid_t, group_info: *mut group_info) -> int {
    let user_ns = current_user_ns();
    for i in 0..(*group_info).ngroups {
        let kgid = *(*group_info).gid.add(i as usize);
        let group = high2lowgid(from_kgid_munged(user_ns, kgid));
        if put_user(group, grouplist.add(i as usize)) != 0 { return -EFAULT; }
    }
    0
}

unsafe fn groups16_from_user(group_info: *mut group_info, grouplist: *const old_gid_t) -> int {
    let user_ns = current_user_ns();
    for i in 0..(*group_info).ngroups {
        let mut group: old_gid_t = 0;
        if get_user(&mut group, grouplist.add(i as usize)) != 0 { return -EFAULT; }
        let kgid = make_kgid(user_ns, low2highgid(group));
        if !gid_valid(kgid) { return -EINVAL; }
        *(*group_info).gid.add(i as usize) = kgid;
    }
    0
}

pub unsafe fn getgroups16(gidsetsize: int, grouplist: *mut old_gid_t) -> int {
    let cred = &*current_cred();
    if gidsetsize < 0 { return -EINVAL; }
    let mut i = (*cred.group_info).ngroups;
    if gidsetsize != 0 {
        if i > gidsetsize { i = -EINVAL; }
        else if groups16_to_user(grouplist, cred.group_info) != 0 { i = -EFAULT; }
    }
    i
}

pub unsafe fn setgroups16(gidsetsize: int, grouplist: *const old_gid_t) -> int {
    if !may_setgroups() { return -EPERM; }
    if gidsetsize as u32 > NGROUPS_MAX { return -EINVAL; }
    let group_info = groups_alloc(gidsetsize);
    if group_info.is_null() { return -ENOMEM; }
    let retval = groups16_from_user(group_info, grouplist);
    if retval != 0 { put_group_info(group_info); return retval; }
    groups_sort(group_info);
    let retval = set_current_groups(group_info);
    put_group_info(group_info);
    retval
}

pub unsafe fn getuid16() -> old_uid_t { high2lowuid(from_kuid_munged(current_user_ns(), current_uid())) }
pub unsafe fn geteuid16() -> old_uid_t { high2lowuid(from_kuid_munged(current_user_ns(), current_euid())) }
pub unsafe fn getgid16() -> old_gid_t { high2lowgid(from_kgid_munged(current_user_ns(), current_gid())) }
pub unsafe fn getegid16() -> old_gid_t { high2lowgid(from_kgid_munged(current_user_ns(), current_egid())) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
