/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding kernel headers:
// linux/refcount.h, linux/spinlock.h, linux/sched.h

use core::ffi::c_void;

#[repr(C)]
pub struct mnt_namespace {
    _private: [u8; 0],
}

#[repr(C)]
pub struct uts_namespace {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ipc_namespace {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pid_namespace {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cgroup_namespace {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fs_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct time_namespace {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cred {
    _private: [u8; 0],
}

#[repr(C)]
pub struct refcount_t {
    _private: [u8; 0],
}

extern "C" {
    pub static mut init_nsproxy: nsproxy;

    pub fn copy_namespaces(flags: u64, tsk: *mut task_struct) -> i32;
    pub fn switch_cred_namespaces(old: *const cred, new: *const cred);
    pub fn exit_nsproxy_namespaces(tsk: *mut task_struct);
    pub fn get_cred_namespaces(tsk: *mut task_struct);
    pub fn exit_cred_namespaces(tsk: *mut task_struct);
    pub fn switch_task_namespaces(tsk: *mut task_struct, new: *mut nsproxy);
    pub fn exec_task_namespaces() -> i32;
    pub fn deactivate_nsproxy(ns: *mut nsproxy);
    pub fn unshare_nsproxy_namespaces(
        flags: usize,
        ns: *mut *mut nsproxy,
        cred: *mut cred,
        fs: *mut fs_struct,
    ) -> i32;
    pub fn nsproxy_cache_init() -> i32;

    pub fn refcount_dec_and_test(r: *mut refcount_t) -> bool;
    pub fn refcount_inc(r: *mut refcount_t);
}

#[repr(C)]
pub struct nsproxy {
    pub count: refcount_t,
    pub uts_ns: *mut uts_namespace,
    pub ipc_ns: *mut ipc_namespace,
    pub mnt_ns: *mut mnt_namespace,
    pub pid_ns_for_children: *mut pid_namespace,
    pub net_ns: *mut net,
    pub time_ns: *mut time_namespace,
    pub time_ns_for_children: *mut time_namespace,
    pub cgroup_ns: *mut cgroup_namespace,
}

#[repr(C)]
pub struct nsset {
    pub flags: u32,
    pub nsproxy: *mut nsproxy,
    pub fs: *mut fs_struct,
    pub cred: *const cred,
}

// CLONE_NEWUSER is supplied by the corresponding kernel headers.
pub unsafe fn nsset_cred(set: *mut nsset) -> *mut cred {
    if (*set).flags & (CLONE_NEWUSER as u32) != 0 {
        return (*set).cred as *mut cred;
    }

    core::ptr::null_mut()
}

pub unsafe fn put_nsproxy(ns: *mut nsproxy) {
    if refcount_dec_and_test(&mut (*ns).count) {
        deactivate_nsproxy(ns);
    }
}

pub unsafe fn get_nsproxy(ns: *mut nsproxy) {
    refcount_inc(&mut (*ns).count);
}

// DEFINE_FREE(put_nsproxy, struct nsproxy *, if (_T) put_nsproxy(_T))
// c_void is retained for the macro's erased cleanup type when provided by the build.
#[allow(dead_code)]
type NsproxyCleanupType = *mut c_void;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
