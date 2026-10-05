// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (C) 2006 IBM Corporation
 *
 *  Author: Serge Hallyn <serue@us.ibm.com>
 *
 *  Jun 2006 - namespaces support
 *             OpenVZ, SWsoft Inc.
 *             Pavel Emelianov <xemul@openvz.org>
 */

//! Inactive namespace-transition owner using configured native boundaries.

// Existing Rust owner repaired against nsproxy.c at
// 126a30fae3bba11420ec2fcbde51a0a01bab1b5b. Source-only, unqualified.
#![no_std]
compile_error!("Lupos nsproxy source proposal is not build-admitted; qualification is incomplete");

use kernel::bindings::nsproxy_native as b;
use kernel::ffi::{c_int, c_uint, c_ulong};
use b::{cred, fs_struct, kmem_cache, ns_common, nsproxy, nsset, pid,
    pid_namespace, task_struct, user_namespace};
use b::{
    RUST_NSPROXY_CLONE_NS_ALL as CLONE_NS_ALL,
    RUST_NSPROXY_CLONE_NEWUSER as CLONE_NEWUSER,
    RUST_NSPROXY_CLONE_NEWPID as CLONE_NEWPID,
    RUST_NSPROXY_CLONE_NEWNS as CLONE_NEWNS,
    RUST_NSPROXY_CLONE_NEWUTS as CLONE_NEWUTS,
    RUST_NSPROXY_CLONE_NEWIPC as CLONE_NEWIPC,
    RUST_NSPROXY_CLONE_NEWCGROUP as CLONE_NEWCGROUP,
    RUST_NSPROXY_CLONE_NEWNET as CLONE_NEWNET,
    RUST_NSPROXY_CLONE_NEWTIME as CLONE_NEWTIME,
    RUST_NSPROXY_CLONE_VM as CLONE_VM,
    RUST_NSPROXY_CLONE_SYSVSEM as CLONE_SYSVSEM,
    RUST_NSPROXY_UNSHARE_EMPTY_MNTNS as UNSHARE_EMPTY_MNTNS,
    RUST_NSPROXY_CLONE_EMPTY_MNTNS as CLONE_EMPTY_MNTNS,
    RUST_NSPROXY_ENOMEM as ENOMEM, RUST_NSPROXY_EPERM as EPERM,
    RUST_NSPROXY_EINVAL as EINVAL, RUST_NSPROXY_ESRCH as ESRCH,
    RUST_NSPROXY_EBADF as EBADF, RUST_NSPROXY_CAP_SYS_ADMIN as CAP_SYS_ADMIN,
    RUST_NSPROXY_PTRACE_MODE_READ_REALCREDS as PTRACE_MODE_READ_REALCREDS,
    rust_nsproxy_current as current,
    rust_nsproxy_current_user_ns as current_user_ns,
    rust_nsproxy_current_cred as current_cred,
    rust_nsproxy_prepare_creds as prepare_creds,
    rust_nsproxy_put_cred as put_cred,
    rust_nsproxy_nsset_cred as nsset_cred,
    rust_nsproxy_ns_capable as ns_capable,
    rust_nsproxy_likely as likely,
    rust_nsproxy_copy_mnt_ns as copy_mnt_ns,
    rust_nsproxy_copy_utsname as copy_utsname,
    rust_nsproxy_copy_ipcs as copy_ipcs,
    rust_nsproxy_copy_pid_ns as copy_pid_ns,
    rust_nsproxy_copy_cgroup_ns as copy_cgroup_ns,
    rust_nsproxy_copy_net_ns as copy_net_ns,
    rust_nsproxy_copy_time_ns as copy_time_ns,
    rust_nsproxy_get_time_ns as get_time_ns,
    rust_nsproxy_put_mnt_ns as put_mnt_ns,
    rust_nsproxy_put_uts_ns as put_uts_ns,
    rust_nsproxy_put_ipc_ns as put_ipc_ns,
    rust_nsproxy_put_pid_ns as put_pid_ns,
    rust_nsproxy_put_time_ns as put_time_ns,
    rust_nsproxy_put_cgroup_ns as put_cgroup_ns,
    rust_nsproxy_put_net as put_net,
    rust_nsproxy_get_nsproxy as get_nsproxy,
    rust_nsproxy_put_nsproxy as put_nsproxy,
    rust_nsproxy_active_get as nsproxy_ns_active_get,
    rust_nsproxy_active_put as nsproxy_ns_active_put,
    rust_nsproxy_timens_on_fork as timens_on_fork,
    rust_nsproxy_might_sleep as might_sleep,
    rust_nsproxy_task_lock as task_lock,
    rust_nsproxy_task_unlock as task_unlock,
    rust_nsproxy_rcu_read_lock as rcu_read_lock,
    rust_nsproxy_rcu_read_unlock as rcu_read_unlock,
    rust_nsproxy_ptrace_may_access as ptrace_may_access,
    rust_nsproxy_copy_fs_struct as copy_fs_struct,
    rust_nsproxy_free_fs_struct as free_fs_struct,
    rust_nsproxy_set_fs_root as set_fs_root,
    rust_nsproxy_set_fs_pwd as set_fs_pwd,
    rust_nsproxy_fdget as fdget,
    rust_nsproxy_fd_empty as fd_empty,
    rust_nsproxy_fd_file as fd_file,
    rust_nsproxy_fdput as fdput,
    rust_nsproxy_proc_ns_file as proc_ns_file,
    rust_nsproxy_pidfd_pid as pidfd_pid,
    rust_nsproxy_perf_event_namespaces as perf_event_namespaces,
};

static mut NSPROXY_CACHEP: *mut kmem_cache = core::ptr::null_mut();

// The same init_nsproxy initializer is in nsproxy_native.c so REFCOUNT_INIT,
// native pointer storage and omitted zero fields use the configured C authority.
/// Initial namespace proxy, stored and initialized by the native owner.
pub use b::init_nsproxy;

unsafe fn create_nsproxy() -> *mut nsproxy {
    // SAFETY: The caller runs after serialized cache initialization; the native
    // allocator supplies writable nsproxy storage, checked before initialization.
    unsafe {
        let nsproxy = b::rust_nsproxy_alloc(NSPROXY_CACHEP);
        if !nsproxy.is_null() {
            b::rust_nsproxy_refcount_set(nsproxy, 1);
        }
        nsproxy
    }
}

unsafe fn nsproxy_free(ns: *mut nsproxy) {
    // SAFETY: The caller owns one fully initialized inactive nsproxy and its
    // constituent references; each is released exactly once in native order.
    unsafe {
        put_mnt_ns((*ns).mnt_ns);
        put_uts_ns((*ns).uts_ns);
        put_ipc_ns((*ns).ipc_ns);
        put_pid_ns((*ns).pid_ns_for_children);
        put_time_ns((*ns).time_ns);
        put_time_ns((*ns).time_ns_for_children);
        put_cgroup_ns((*ns).cgroup_ns);
        put_net((*ns).net_ns);
        b::rust_nsproxy_free(NSPROXY_CACHEP, ns);
    }
}

/// Deactivate and destroy the final active namespace proxy reference.
///
/// # Safety
/// The caller must own the final proxy reference and its active namespaces,
/// with no further users of the live, fully initialized proxy.
#[no_mangle]
pub unsafe extern "C" fn deactivate_nsproxy(ns: *mut nsproxy) {
    // SAFETY: The caller supplies final ownership under the native contract.
    unsafe {
        nsproxy_ns_active_put(ns);
        nsproxy_free(ns);
    }
}

/* Create new nsproxy and all of its associated namespaces. */
unsafe fn create_new_namespaces(
    flags: u64,
    tsk: *mut task_struct,
    user_ns: *mut user_namespace,
    new_fs: *mut fs_struct,
) -> *mut nsproxy {
    // SAFETY: The caller pins tsk, its source nsproxy, user_ns and new_fs under
    // the native copy contract. Every successful acquisition has the matching
    // failure release; the failed ERR_PTR field is never released.
    unsafe {
        let new_nsp = create_nsproxy();
        if new_nsp.is_null() { return b::rust_nsproxy_error(-ENOMEM); }

        (*new_nsp).mnt_ns = copy_mnt_ns(flags, (*b::rust_nsproxy_task_nsproxy(tsk)).mnt_ns, user_ns, new_fs);
        if b::rust_nsproxy_is_err_mnt((*new_nsp).mnt_ns) { let err = b::rust_nsproxy_error_mnt((*new_nsp).mnt_ns); b::rust_nsproxy_free(NSPROXY_CACHEP, new_nsp); return b::rust_nsproxy_error(err); }
        (*new_nsp).uts_ns = copy_utsname(flags, user_ns, (*b::rust_nsproxy_task_nsproxy(tsk)).uts_ns);
        if b::rust_nsproxy_is_err_uts((*new_nsp).uts_ns) { let err = b::rust_nsproxy_error_uts((*new_nsp).uts_ns); put_mnt_ns((*new_nsp).mnt_ns); b::rust_nsproxy_free(NSPROXY_CACHEP, new_nsp); return b::rust_nsproxy_error(err); }
        (*new_nsp).ipc_ns = copy_ipcs(flags, user_ns, (*b::rust_nsproxy_task_nsproxy(tsk)).ipc_ns);
        if b::rust_nsproxy_is_err_ipc((*new_nsp).ipc_ns) { let err = b::rust_nsproxy_error_ipc((*new_nsp).ipc_ns); put_uts_ns((*new_nsp).uts_ns); put_mnt_ns((*new_nsp).mnt_ns); b::rust_nsproxy_free(NSPROXY_CACHEP, new_nsp); return b::rust_nsproxy_error(err); }
        (*new_nsp).pid_ns_for_children = copy_pid_ns(flags, user_ns, (*b::rust_nsproxy_task_nsproxy(tsk)).pid_ns_for_children);
        if b::rust_nsproxy_is_err_pid_ns((*new_nsp).pid_ns_for_children) { let err = b::rust_nsproxy_error_pid_ns((*new_nsp).pid_ns_for_children); put_ipc_ns((*new_nsp).ipc_ns); put_uts_ns((*new_nsp).uts_ns); put_mnt_ns((*new_nsp).mnt_ns); b::rust_nsproxy_free(NSPROXY_CACHEP, new_nsp); return b::rust_nsproxy_error(err); }
        (*new_nsp).cgroup_ns = copy_cgroup_ns(flags, user_ns, (*b::rust_nsproxy_task_nsproxy(tsk)).cgroup_ns);
        if b::rust_nsproxy_is_err_cgroup((*new_nsp).cgroup_ns) { let err = b::rust_nsproxy_error_cgroup((*new_nsp).cgroup_ns); put_pid_ns((*new_nsp).pid_ns_for_children); put_ipc_ns((*new_nsp).ipc_ns); put_uts_ns((*new_nsp).uts_ns); put_mnt_ns((*new_nsp).mnt_ns); b::rust_nsproxy_free(NSPROXY_CACHEP, new_nsp); return b::rust_nsproxy_error(err); }
        (*new_nsp).net_ns = copy_net_ns(flags, user_ns, (*b::rust_nsproxy_task_nsproxy(tsk)).net_ns);
        if b::rust_nsproxy_is_err_net((*new_nsp).net_ns) { let err = b::rust_nsproxy_error_net((*new_nsp).net_ns); put_cgroup_ns((*new_nsp).cgroup_ns); put_pid_ns((*new_nsp).pid_ns_for_children); put_ipc_ns((*new_nsp).ipc_ns); put_uts_ns((*new_nsp).uts_ns); put_mnt_ns((*new_nsp).mnt_ns); b::rust_nsproxy_free(NSPROXY_CACHEP, new_nsp); return b::rust_nsproxy_error(err); }
        (*new_nsp).time_ns_for_children = copy_time_ns(flags, user_ns, (*b::rust_nsproxy_task_nsproxy(tsk)).time_ns_for_children);
        if b::rust_nsproxy_is_err_time((*new_nsp).time_ns_for_children) { let err = b::rust_nsproxy_error_time((*new_nsp).time_ns_for_children); put_net((*new_nsp).net_ns); put_cgroup_ns((*new_nsp).cgroup_ns); put_pid_ns((*new_nsp).pid_ns_for_children); put_ipc_ns((*new_nsp).ipc_ns); put_uts_ns((*new_nsp).uts_ns); put_mnt_ns((*new_nsp).mnt_ns); b::rust_nsproxy_free(NSPROXY_CACHEP, new_nsp); return b::rust_nsproxy_error(err); }
        (*new_nsp).time_ns = get_time_ns((*b::rust_nsproxy_task_nsproxy(tsk)).time_ns);
        new_nsp
    }
}

/// Copy namespace ownership for a task being created.
///
/// # Safety
/// The caller must satisfy native clone setup and exclusive task-publication
/// rules, with a live source nsproxy, credentials and fs on tsk.
#[no_mangle]
pub unsafe extern "C" fn copy_namespaces(flags: u64, tsk: *mut task_struct) -> c_int {
    // SAFETY: Clone setup supplies the live source and exclusive destination.
    unsafe {
        let old_ns = b::rust_nsproxy_task_nsproxy(tsk);
        let user_ns = b::rust_nsproxy_task_cred_user_ns(tsk);
        if likely((flags & (CLONE_NS_ALL & !CLONE_NEWUSER)) == 0) {
            if (flags & CLONE_VM) != 0 || likely((*old_ns).time_ns_for_children == (*old_ns).time_ns) { get_nsproxy(old_ns); return 0; }
        } else if !ns_capable(user_ns, CAP_SYS_ADMIN) { return -EPERM; }
        if (flags & (CLONE_NEWIPC | CLONE_SYSVSEM)) == (CLONE_NEWIPC | CLONE_SYSVSEM) { return -EINVAL; }
        let new_ns = create_new_namespaces(flags, tsk, user_ns, b::rust_nsproxy_task_fs(tsk));
        if b::rust_nsproxy_is_err_nsproxy(new_ns) { return b::rust_nsproxy_error_nsproxy(new_ns); }
        if (flags & CLONE_VM) == 0 { timens_on_fork(new_ns, tsk); }
        nsproxy_ns_active_get(new_ns); b::rust_nsproxy_store_task_nsproxy(tsk, new_ns); 0
    }
}

/// Prepare an inactive namespace proxy for native unshare.
///
/// # Safety
/// The caller must provide a writable new_nsp output and keep any supplied
/// new_cred and new_fs live, under the current task's native unshare contract.
#[no_mangle]
pub unsafe extern "C" fn unshare_nsproxy_namespaces(unshare_flags: c_ulong, new_nsp: *mut *mut nsproxy, new_cred: *mut cred, new_fs: *mut fs_struct) -> c_int {
    // SAFETY: The caller provides the output and pins the optional new objects;
    // otherwise the current task owns the borrowed credential and fs state.
    unsafe {
        let mut flags = unshare_flags as u64;
        if (flags & (CLONE_NS_ALL & !CLONE_NEWUSER)) == 0 { return 0; }
        let user_ns = if !new_cred.is_null() { b::rust_nsproxy_cred_user_ns(new_cred) } else { current_user_ns() };
        if !ns_capable(user_ns, CAP_SYS_ADMIN) { return -EPERM; }
        if (flags & UNSHARE_EMPTY_MNTNS) != 0 { flags &= !(UNSHARE_EMPTY_MNTNS as u64); flags |= CLONE_EMPTY_MNTNS; }
        *new_nsp = create_new_namespaces(flags, current(), user_ns, if !new_fs.is_null() { new_fs } else { b::rust_nsproxy_task_fs(current()) });
        if b::rust_nsproxy_is_err_nsproxy(*new_nsp) { return b::rust_nsproxy_error_nsproxy(*new_nsp); } 0
    }
}

/// Publish a replacement namespace proxy, including a NULL exit value.
///
/// # Safety
/// The caller must be allowed to change p's namespaces and may sleep. A
/// non-NULL new proxy must be fully initialized and its reference transferred.
#[no_mangle]
pub unsafe extern "C" fn switch_task_namespaces(p: *mut task_struct, new: *mut nsproxy) {
    // SAFETY: The caller owns the replacement; task_lock protects publication,
    // and the detached old reference is released only after unlocking.
    unsafe { might_sleep(); if !new.is_null() { nsproxy_ns_active_get(new); } task_lock(p); let ns = b::rust_nsproxy_task_nsproxy(p); b::rust_nsproxy_store_task_nsproxy(p, new); task_unlock(p); if !ns.is_null() { put_nsproxy(ns); } }
}

/// Detach a task's namespace proxy during exit.
///
/// # Safety
/// The caller must own namespace exit for the live task p and may sleep.
#[no_mangle]
pub unsafe extern "C" fn exit_nsproxy_namespaces(p: *mut task_struct) {
    // SAFETY: Exit grants the caller authority to detach this live task.
    unsafe { switch_task_namespaces(p, core::ptr::null_mut()); }
}

/// Transfer active user-namespace ownership during credential replacement.
///
/// # Safety
/// Both credential pointers must remain live and the caller must own the
/// native credential transition represented by old and new.
#[no_mangle]
pub unsafe extern "C" fn switch_cred_namespaces(old: *const cred, new: *const cred) {
    // SAFETY: The native credential transition pins both immutable credentials.
    unsafe { b::rust_nsproxy_user_active_get(b::rust_nsproxy_cred_user_ns(new)); b::rust_nsproxy_user_active_put(b::rust_nsproxy_cred_user_ns(old)); }
}

/// Acquire the active user namespace of a task's objective credentials.
///
/// # Safety
/// Native task setup must pin tsk and its real_cred and require this acquisition.
#[no_mangle]
pub unsafe extern "C" fn get_cred_namespaces(tsk: *mut task_struct) {
    // SAFETY: Task setup pins the objective credentials for this acquisition.
    unsafe { b::rust_nsproxy_user_active_get(b::rust_nsproxy_task_real_cred_user_ns(tsk)); }
}

/// Release the active user namespace of a task's objective credentials.
///
/// # Safety
/// Native task teardown must pin tsk and real_cred and own the active reference.
#[no_mangle]
pub unsafe extern "C" fn exit_cred_namespaces(tsk: *mut task_struct) {
    // SAFETY: Task teardown owns the active objective-credential reference.
    unsafe { b::rust_nsproxy_user_active_put(b::rust_nsproxy_task_real_cred_user_ns(tsk)); }
}

/// Reconcile the current task's active and child time namespaces on exec.
///
/// # Safety
/// The caller must run in the current task's native exec transition context.
#[no_mangle]
pub unsafe extern "C" fn exec_task_namespaces() -> c_int {
    // SAFETY: Exec pins current's namespace, credentials and fs; switching uses
    // its native lock and transfers the newly created proxy on success.
    unsafe {
        let tsk = current();
        if (*b::rust_nsproxy_task_nsproxy(tsk)).time_ns_for_children == (*b::rust_nsproxy_task_nsproxy(tsk)).time_ns { return 0; }
        let new = create_new_namespaces(0, tsk, current_user_ns(), b::rust_nsproxy_task_fs(tsk));
        if b::rust_nsproxy_is_err_nsproxy(new) { return b::rust_nsproxy_error_nsproxy(new); }
        timens_on_fork(new, tsk); switch_task_namespaces(tsk, new); 0
    }
}

fn check_setns_flags(flags: c_ulong) -> c_int {
    let flags = flags as u64;
    if flags == 0 || (flags & !CLONE_NS_ALL) != 0 { return -EINVAL; }
    #[cfg(not(CONFIG_USER_NS))]
    if flags & CLONE_NEWUSER != 0 { return -EINVAL; }
    #[cfg(not(CONFIG_PID_NS))]
    if flags & CLONE_NEWPID != 0 { return -EINVAL; }
    #[cfg(not(CONFIG_UTS_NS))]
    if flags & CLONE_NEWUTS != 0 { return -EINVAL; }
    #[cfg(not(CONFIG_IPC_NS))]
    if flags & CLONE_NEWIPC != 0 { return -EINVAL; }
    #[cfg(not(CONFIG_CGROUPS))]
    if flags & CLONE_NEWCGROUP != 0 { return -EINVAL; }
    #[cfg(not(CONFIG_NET_NS))]
    if flags & CLONE_NEWNET != 0 { return -EINVAL; }
    #[cfg(not(CONFIG_TIME_NS))]
    if flags & CLONE_NEWTIME != 0 { return -EINVAL; }
    0
}

unsafe fn put_nsset(nsset: *mut nsset) {
    // SAFETY: The caller supplies initialized native nsset storage and invokes
    // this once for its owned state. Published flags govern the native cleanup.
    unsafe {
        let flags = (*nsset).flags as u64;
        if flags & CLONE_NEWUSER != 0 { put_cred(nsset_cred(nsset)); }
        if !(*nsset).fs.is_null() && flags & CLONE_NEWNS != 0 && flags & !CLONE_NEWNS != 0 { free_fs_struct((*nsset).fs); }
        if !(*nsset).nsproxy.is_null() { nsproxy_free((*nsset).nsproxy); }
    }
}

unsafe fn prepare_nsset(flags: c_uint, nsset: *mut nsset) -> c_int {
    // SAFETY: The caller supplies exclusively owned zeroed nsset storage in
    // current's setns context; this function owns partial preparation cleanup.
    unsafe {
        let me = current();
        (*nsset).nsproxy = create_new_namespaces(0, me, current_user_ns(), b::rust_nsproxy_task_fs(me));
        // Allocation failure owns no cleanup. In particular, never free ERR_PTR.
        if b::rust_nsproxy_is_err_nsproxy((*nsset).nsproxy) {
            return b::rust_nsproxy_error_nsproxy((*nsset).nsproxy);
        }
        (*nsset).cred = if flags as u64 & CLONE_NEWUSER != 0 { prepare_creds() } else { current_cred() };
        if (*nsset).cred.is_null() { put_nsset(nsset); return -ENOMEM; }
        if flags as u64 == CLONE_NEWNS {
            (*nsset).fs = b::rust_nsproxy_task_fs(me);
        } else if flags as u64 & CLONE_NEWNS != 0 {
            (*nsset).fs = copy_fs_struct(b::rust_nsproxy_task_fs(me));
            if (*nsset).fs.is_null() { put_nsset(nsset); return -ENOMEM; }
        }
        // Preserve the C oracle's publication point, including partial-state unwind.
        (*nsset).flags = flags;
        0
    }
}

unsafe fn validate_ns(nsset: *mut nsset, ns: *mut ns_common) -> c_int {
    // Native leaf preserves the randomized ops field and C indirect-call type.
    // SAFETY: The caller pins ns and its operations and owns a prepared nsset;
    // the native callback performs installation under those same contracts.
    unsafe { b::rust_nsproxy_install(nsset, ns) }
}

unsafe fn validate_nsset(nsset: *mut nsset, pid: *mut pid) -> c_int {
    // SAFETY: The fd pins pid, and the caller owns a successfully prepared
    // nsset. RCU and task_lock protect the snapshot until each reference is
    // acquired; all acquired references reach the single release sequence.
    unsafe {
        let flags = (*nsset).flags as u64;
        #[cfg(CONFIG_USER_NS)]
        let mut user_ns: *mut user_namespace = core::ptr::null_mut();
        #[cfg(not(CONFIG_USER_NS))]
        let user_ns: *mut user_namespace = core::ptr::null_mut();
        #[cfg(CONFIG_PID_NS)]
        let mut pid_ns: *mut pid_namespace = core::ptr::null_mut();
        #[cfg(not(CONFIG_PID_NS))]
        let pid_ns: *mut pid_namespace = core::ptr::null_mut();

        rcu_read_lock();
        let tsk = b::rust_nsproxy_pid_task(pid);
        if tsk.is_null() { rcu_read_unlock(); return -ESRCH; }
        if !ptrace_may_access(tsk, PTRACE_MODE_READ_REALCREDS) { rcu_read_unlock(); return -EPERM; }
        task_lock(tsk);
        let nsp = b::rust_nsproxy_task_nsproxy(tsk);
        if !nsp.is_null() { get_nsproxy(nsp); }
        task_unlock(tsk);
        if nsp.is_null() { rcu_read_unlock(); return -ESRCH; }

        // This block's exits all join the original out: reference-release order.
        let ret = 'install: {
            #[cfg(CONFIG_PID_NS)]
            if flags & CLONE_NEWPID != 0 {
                pid_ns = b::rust_nsproxy_task_active_pid_ns(tsk);
                if b::rust_nsproxy_unlikely(pid_ns.is_null()) {
                    rcu_read_unlock();
                    break 'install -ESRCH;
                }
                b::rust_nsproxy_get_pid_ns(pid_ns);
            }
            #[cfg(CONFIG_USER_NS)]
            if flags & CLONE_NEWUSER != 0 {
                user_ns = b::rust_nsproxy_get_user_ns(b::rust_nsproxy_task_user_ns_rcu(tsk));
            }
            rcu_read_unlock();

            #[cfg(CONFIG_USER_NS)]
            if flags & CLONE_NEWUSER != 0 {
                let ret = validate_ns(nsset, b::rust_nsproxy_from_user_ns(user_ns));
                if ret != 0 { break 'install ret; }
            }
            if flags & CLONE_NEWNS != 0 {
                let ret = validate_ns(nsset, b::rust_nsproxy_from_mnt_ns((*nsp).mnt_ns));
                if ret != 0 { break 'install ret; }
            }
            #[cfg(CONFIG_UTS_NS)]
            if flags & CLONE_NEWUTS != 0 {
                let ret = validate_ns(nsset, b::rust_nsproxy_from_uts_ns((*nsp).uts_ns));
                if ret != 0 { break 'install ret; }
            }
            #[cfg(CONFIG_IPC_NS)]
            if flags & CLONE_NEWIPC != 0 {
                let ret = validate_ns(nsset, b::rust_nsproxy_from_ipc_ns((*nsp).ipc_ns));
                if ret != 0 { break 'install ret; }
            }
            #[cfg(CONFIG_PID_NS)]
            if flags & CLONE_NEWPID != 0 {
                let ret = validate_ns(nsset, b::rust_nsproxy_from_pid_ns(pid_ns));
                if ret != 0 { break 'install ret; }
            }
            #[cfg(CONFIG_CGROUPS)]
            if flags & CLONE_NEWCGROUP != 0 {
                let ret = validate_ns(nsset, b::rust_nsproxy_from_cgroup_ns((*nsp).cgroup_ns));
                if ret != 0 { break 'install ret; }
            }
            #[cfg(CONFIG_NET_NS)]
            if flags & CLONE_NEWNET != 0 {
                let ret = validate_ns(nsset, b::rust_nsproxy_from_net_ns((*nsp).net_ns));
                if ret != 0 { break 'install ret; }
            }
            #[cfg(CONFIG_TIME_NS)]
            if flags & CLONE_NEWTIME != 0 {
                let ret = validate_ns(nsset, b::rust_nsproxy_from_time_ns((*nsp).time_ns));
                if ret != 0 { break 'install ret; }
            }
            0
        };

        if !pid_ns.is_null() { put_pid_ns(pid_ns); }
        if !nsp.is_null() { put_nsproxy(nsp); }
        b::rust_nsproxy_put_user_ns(user_ns);
        ret
    }
}

unsafe fn commit_nsset(nsset: *mut nsset) {
    // SAFETY: The caller owns a successfully validated nsset in current's
    // setns context. Credential and proxy slots are cleared after transfer,
    // before the caller performs the one remaining nsset cleanup.
    unsafe {
        let me = current();
        let flags = (*nsset).flags as u64;
        #[cfg(CONFIG_USER_NS)]
        if flags & CLONE_NEWUSER != 0 {
            b::rust_nsproxy_commit_creds(nsset_cred(nsset));
            (*nsset).cred = core::ptr::null();
        }
        if flags & CLONE_NEWNS != 0 && flags & !CLONE_NEWNS != 0 {
            set_fs_root(b::rust_nsproxy_task_fs(me), (*nsset).fs);
            set_fs_pwd(b::rust_nsproxy_task_fs(me), (*nsset).fs);
        }
        #[cfg(CONFIG_IPC_NS)]
        if flags & CLONE_NEWIPC != 0 { b::rust_nsproxy_exit_sem(me); }
        #[cfg(CONFIG_TIME_NS)]
        if flags & CLONE_NEWTIME != 0 { b::rust_nsproxy_timens_commit(me, (*(*nsset).nsproxy).time_ns); }
        switch_task_namespaces(me, (*nsset).nsproxy);
        (*nsset).nsproxy = core::ptr::null_mut();
    }
}

/// Execute setns decisions behind the native syscall metadata wrapper.
///
/// # Safety
/// The caller must enter from the native syscall context with current's
/// credentials, fs and namespace state subject to the original setns rules.
#[export_name = "rust_nsproxy_setns"]
pub unsafe extern "C" fn setns(fd: c_int, mut flags: c_int) -> c_int {
    // SAFETY: Native syscall entry supplies the execution context. Zeroed
    // nsset has only integer/pointer fields; the native fd remains pinned
    // through validation, commit and cleanup, and is put on each return path.
    unsafe {
        let mut ns: *mut ns_common = core::ptr::null_mut();
        let mut nsset: nsset = core::mem::zeroed();
        let mut err = 0;
        let f = fdget(fd);
        if fd_empty(&f) {
            fdput(&f);
            return -EBADF;
        }
        if proc_ns_file(fd_file(&f)) {
            ns = b::rust_nsproxy_proc_ns(fd_file(&f));
            if flags != 0 && b::rust_nsproxy_ns_type(ns) != flags as u32 { err = -EINVAL; }
            flags = b::rust_nsproxy_ns_type(ns) as c_int;
        } else if !b::rust_nsproxy_is_err_pid(pidfd_pid(fd_file(&f))) {
            err = check_setns_flags(flags as c_ulong);
        } else {
            err = -EINVAL;
        }
        if err != 0 {
            fdput(&f);
            return err;
        }
        err = prepare_nsset(flags as c_uint, &mut nsset);
        if err != 0 {
            // prepare_nsset owns its error cleanup; the caller must not repeat it.
            fdput(&f);
            return err;
        }
        err = if proc_ns_file(fd_file(&f)) {
            validate_ns(&mut nsset, ns)
        } else {
            validate_nsset(&mut nsset, pidfd_pid(fd_file(&f)))
        };
        if err == 0 {
            commit_nsset(&mut nsset);
            perf_event_namespaces(current());
        }
        put_nsset(&mut nsset);
        fdput(&f);
        err
    }
}

/// Initialize proxy storage through the native init-metadata entry.
///
/// # Safety
/// The native init entry must call this once, serially during boot, before any
/// proxy allocation and before init text is discarded.
#[export_name = "rust_nsproxy_cache_init"]
#[link_section = ".init.text"]
#[cold]
pub unsafe extern "C" fn nsproxy_cache_init() -> c_int {
    // SAFETY: Native boot ordering serializes the sole cache-pointer write.
    unsafe { NSPROXY_CACHEP = b::rust_nsproxy_cache_create(); }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
