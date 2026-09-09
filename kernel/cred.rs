// SPDX-License-Identifier: GPL-2.0-or-later
/* Task credentials management - see Documentation/security/credentials.rst
 *
 * Copyright (C) 2008 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependency declarations and build-time configuration are supplied by the
// surrounding kernel translation.

static mut cred_jar: *mut kmem_cache = core::ptr::null_mut();

/* The RCU callback to actually dispose of a set of credentials */
unsafe fn put_cred_rcu(rcu: *mut rcu_head) {
    let cred: *mut cred = container_of!(rcu, cred, rcu);

    kdebug!("put_cred_rcu(%p)", cred);
    if atomic_long_read(&(*cred).usage) != 0 {
        panic!("CRED: put_cred_rcu() sees %p with usage %ld\\n", cred, atomic_long_read(&(*cred).usage));
    }

    security_cred_free(cred);
    key_put((*cred).session_keyring);
    key_put((*cred).process_keyring);
    key_put((*cred).thread_keyring);
    key_put((*cred).request_key_auth);
    if !(*cred).group_info.is_null() { put_group_info((*cred).group_info); }
    free_uid((*cred).user);
    if !(*cred).ucounts.is_null() { put_ucounts((*cred).ucounts); }
    put_user_ns((*cred).user_ns);
    kmem_cache_free(cred_jar, cred);
}

pub unsafe fn __put_cred(cred: *mut cred) {
    kdebug!("__put_cred(%p{%ld})", cred, atomic_long_read(&(*cred).usage));
    BUG_ON!(atomic_long_read(&(*cred).usage) != 0);
    BUG_ON!(cred == (*current).cred);
    BUG_ON!(cred == (*current).real_cred);
    if (*cred).non_rcu { put_cred_rcu(&mut (*cred).rcu); }
    else { call_rcu(&mut (*cred).rcu, put_cred_rcu); }
}

pub unsafe fn exit_creds(tsk: *mut task_struct) {
    let real_cred: *mut cred;
    let cred: *mut cred;
    kdebug!("exit_creds(%u,%p,%p,{%ld})", (*tsk).pid, (*tsk).real_cred, (*tsk).cred, atomic_long_read(&(*(*tsk).cred).usage));
    real_cred = (*tsk).real_cred;
    (*tsk).real_cred = core::ptr::null_mut();
    cred = (*tsk).cred;
    (*tsk).cred = core::ptr::null_mut();
    if real_cred == cred { put_cred_many(cred, 2); }
    else { put_cred(real_cred); put_cred(cred); }
    #[cfg(CONFIG_KEYS_REQUEST_CACHE)]
    { key_put((*tsk).cached_requested_key); (*tsk).cached_requested_key = core::ptr::null_mut(); }
}

pub unsafe fn get_task_cred(task: *mut task_struct) -> *const cred {
    let mut cred: *const cred;
    rcu_read_lock();
    loop {
        cred = __task_cred(task);
        BUG_ON!(cred.is_null());
        if get_cred_rcu(cred) { break; }
    }
    rcu_read_unlock();
    cred
}

pub unsafe fn cred_alloc_blank() -> *mut cred {
    let new = kmem_cache_zalloc(cred_jar, GFP_KERNEL);
    if new.is_null() { return core::ptr::null_mut(); }
    atomic_long_set(&mut (*new).usage, 1);
    if security_cred_alloc_blank(new, GFP_KERNEL_ACCOUNT) < 0 { abort_creds(new); return core::ptr::null_mut(); }
    new
}

pub unsafe fn prepare_creds() -> *mut cred {
    let task = current;
    let new = kmem_cache_alloc(cred_jar, GFP_KERNEL);
    if new.is_null() { return core::ptr::null_mut(); }
    kdebug!("prepare_creds() alloc %p", new);
    let old = (*task).cred;
    core::ptr::copy_nonoverlapping(old, new, 1);
    (*new).non_rcu = false;
    atomic_long_set(&mut (*new).usage, 1);
    get_group_info((*new).group_info); get_uid((*new).user); get_user_ns((*new).user_ns);
    #[cfg(CONFIG_KEYS)] { key_get((*new).session_keyring); key_get((*new).process_keyring); key_get((*new).thread_keyring); key_get((*new).request_key_auth); }
    #[cfg(CONFIG_SECURITY)] { (*new).security = core::ptr::null_mut(); }
    (*new).ucounts = get_ucounts((*new).ucounts);
    if (*new).ucounts.is_null() || security_prepare_creds(new, old, GFP_KERNEL_ACCOUNT) < 0 { abort_creds(new); return core::ptr::null_mut(); }
    new
}

pub unsafe fn prepare_exec_creds() -> *mut cred {
    let new = prepare_creds();
    if new.is_null() { return new; }
    #[cfg(CONFIG_KEYS)] { key_put((*new).thread_keyring); (*new).thread_keyring = core::ptr::null_mut(); key_put((*new).process_keyring); (*new).process_keyring = core::ptr::null_mut(); }
    (*new).suid = (*new).fsuid; (*new).fsuid = (*new).euid;
    (*new).sgid = (*new).fsgid; (*new).fsgid = (*new).egid;
    new
}

pub unsafe fn copy_creds(p: *mut task_struct, clone_flags: u64) -> i32 {
    #[cfg(CONFIG_KEYS_REQUEST_CACHE)] { (*p).cached_requested_key = core::ptr::null_mut(); }
    if (clone_flags & CLONE_THREAD) != 0 {
        (*p).real_cred = get_cred_many((*p).cred, 2); inc_rlimit_ucounts(task_ucounts(p), UCOUNT_RLIMIT_NPROC, 1); get_cred_namespaces(p); return 0;
    }
    let new = prepare_creds();
    if new.is_null() { return -ENOMEM; }
    if (clone_flags & CLONE_NEWUSER) != 0 { let ret = create_user_ns(new); if ret < 0 { put_cred(new); return ret; } let ret = set_cred_ucounts(new); if ret < 0 { put_cred(new); return ret; } }
    #[cfg(CONFIG_KEYS)] {
        if !(*new).thread_keyring.is_null() { key_put((*new).thread_keyring); (*new).thread_keyring = core::ptr::null_mut(); if (clone_flags & CLONE_THREAD) != 0 { install_thread_keyring_to_cred(new); } }
        if (clone_flags & CLONE_THREAD) == 0 { key_put((*new).process_keyring); (*new).process_keyring = core::ptr::null_mut(); }
    }
    (*p).cred = get_cred(new); (*p).real_cred = (*p).cred; inc_rlimit_ucounts(task_ucounts(p), UCOUNT_RLIMIT_NPROC, 1); get_cred_namespaces(p); 0
}

unsafe fn cred_cap_issubset(set: *const cred, subset: *const cred) -> bool {
    let set_ns = (*set).user_ns; let mut subset_ns = (*subset).user_ns;
    if set_ns == subset_ns { return cap_issubset((*subset).cap_permitted, (*set).cap_permitted); }
    while subset_ns != &mut init_user_ns { if set_ns == (*subset_ns).parent && uid_eq((*subset_ns).owner, (*set).euid) { return true; } subset_ns = (*subset_ns).parent; }
    false
}

pub unsafe fn commit_creds(new: *mut cred) -> i32 {
    let task = current; let old = (*task).real_cred;
    BUG_ON!((*task).cred != old); BUG_ON!(atomic_long_read(&(*new).usage) < 1); get_cred(new);
    if !uid_eq((*old).euid, (*new).euid) || !gid_eq((*old).egid, (*new).egid) || !uid_eq((*old).fsuid, (*new).fsuid) || !gid_eq((*old).fsgid, (*new).fsgid) || !cred_cap_issubset(old, new) { if !(*task).mm.is_null() { task_exec_state_set_dumpable(suid_dumpable); } (*task).pdeath_signal = 0; smp_wmb(); }
    if !uid_eq((*new).fsuid, (*old).fsuid) { key_fsuid_changed(new); } if !gid_eq((*new).fsgid, (*old).fsgid) { key_fsgid_changed(new); }
    if (*new).user != (*old).user || (*new).user_ns != (*old).user_ns { inc_rlimit_ucounts((*new).ucounts, UCOUNT_RLIMIT_NPROC, 1); }
    rcu_assign_pointer!((*task).real_cred, new); rcu_assign_pointer!((*task).cred, new);
    if (*new).user != (*old).user || (*new).user_ns != (*old).user_ns { dec_rlimit_ucounts((*old).ucounts, UCOUNT_RLIMIT_NPROC, 1); }
    if (*new).user_ns != (*old).user_ns { switch_cred_namespaces(old, new); }
    if !uid_eq((*new).uid, (*old).uid) || !uid_eq((*new).euid, (*old).euid) || !uid_eq((*new).suid, (*old).suid) || !uid_eq((*new).fsuid, (*old).fsuid) { proc_id_connector(task, PROC_EVENT_UID); }
    if !gid_eq((*new).gid, (*old).gid) || !gid_eq((*new).egid, (*old).egid) || !gid_eq((*new).sgid, (*old).sgid) || !gid_eq((*new).fsgid, (*old).fsgid) { proc_id_connector(task, PROC_EVENT_GID); }
    put_cred_many(old, 2); 0
}

pub unsafe fn abort_creds(new: *mut cred) { BUG_ON!(atomic_long_read(&(*new).usage) < 1); put_cred(new); }

pub unsafe fn cred_fscmp(a: *const cred, b: *const cred) -> i32 {
    if a == b { return 0; } if uid_lt((*a).fsuid, (*b).fsuid) { return -1; } if uid_gt((*a).fsuid, (*b).fsuid) { return 1; }
    if gid_lt((*a).fsgid, (*b).fsgid) { return -1; } if gid_gt((*a).fsgid, (*b).fsgid) { return 1; }
    let ga = (*a).group_info; let gb = (*b).group_info; if ga == gb { return 0; } if ga.is_null() { return -1; } if gb.is_null() { return 1; }
    if (*ga).ngroups < (*gb).ngroups { return -1; } if (*ga).ngroups > (*gb).ngroups { return 1; }
    let mut g = 0; while g < (*ga).ngroups { if gid_lt((*ga).gid[g], (*gb).gid[g]) { return -1; } if gid_gt((*ga).gid[g], (*gb).gid[g]) { return 1; } g += 1; } 0
}

pub unsafe fn set_cred_ucounts(new: *mut cred) -> i32 {
    let old = (*new).ucounts;
    if (*old).ns == (*new).user_ns && uid_eq((*old).uid, (*new).uid) { return 0; }
    let fresh = alloc_ucounts((*new).user_ns, (*new).uid); if fresh.is_null() { return -EAGAIN; }
    (*new).ucounts = fresh; put_ucounts(old); 0
}

pub unsafe fn cred_init() { cred_jar = KMEM_CACHE!(cred, SLAB_HWCACHE_ALIGN | SLAB_PANIC | SLAB_ACCOUNT); }

pub unsafe fn prepare_kernel_cred(daemon: *mut task_struct) -> *mut cred {
    if WARN_ON_ONCE!(daemon.is_null()) { return core::ptr::null_mut(); }
    let new = kmem_cache_alloc(cred_jar, GFP_KERNEL); if new.is_null() { return core::ptr::null_mut(); }
    let old = get_task_cred(daemon); core::ptr::copy_nonoverlapping(old, new, 1); (*new).non_rcu = false; atomic_long_set(&mut (*new).usage, 1); get_uid((*new).user); get_user_ns((*new).user_ns); get_group_info((*new).group_info);
    #[cfg(CONFIG_KEYS)] { (*new).session_keyring = core::ptr::null_mut(); (*new).process_keyring = core::ptr::null_mut(); (*new).thread_keyring = core::ptr::null_mut(); (*new).request_key_auth = core::ptr::null_mut(); (*new).jit_keyring = KEY_REQKEY_DEFL_THREAD_KEYRING; }
    #[cfg(CONFIG_SECURITY)] { (*new).security = core::ptr::null_mut(); }
    (*new).ucounts = get_ucounts((*new).ucounts); if (*new).ucounts.is_null() || security_prepare_creds(new, old, GFP_KERNEL_ACCOUNT) < 0 { put_cred(new); put_cred(old as *mut cred); return core::ptr::null_mut(); } put_cred(old as *mut cred); new
}

pub unsafe fn set_security_override(new: *mut cred, secid: u32) -> i32 { security_kernel_act_as(new, secid) }

pub unsafe fn set_create_files_as(new: *mut cred, inode: *mut inode) -> i32 {
    if !uid_valid((*inode).i_uid) || !gid_valid((*inode).i_gid) { return -EINVAL; }
    (*new).fsuid = (*inode).i_uid; (*new).fsgid = (*inode).i_gid; security_kernel_create_files_as(new, inode)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
