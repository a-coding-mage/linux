// SPDX-License-Identifier: GPL-2.0-or-later
/* Task credentials management - see Documentation/security/credentials.rst
 *
 * Copyright (C) 2008 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Continued from Rust blob ba2263cd8df4792b6523e7ea196bc14e772d1ad8.
// Native oracle: cred.c blob 3df4e15bd67ffe4e63c0fd0ce31bdc5c10d20faf.
// Only configured native headers describe credentials, tasks and their fields.
// The owner-local interface is an inactive proposal; no source is selected.
#![no_std]
compile_error!("SOURCE ONLY HOLD: credential Rust owner is not admitted");

use kernel::bindings::cred_native::*;
use kernel::bindings::cred_native::{
    LUPOS_CRED_CLONE_NEWUSER as CLONE_NEWUSER,
    LUPOS_CRED_CLONE_THREAD as CLONE_THREAD,
    LUPOS_CRED_EAGAIN as EAGAIN,
    LUPOS_CRED_EINVAL as EINVAL,
    LUPOS_CRED_ENOMEM as ENOMEM,
    LUPOS_CRED_GFP_KERNEL as GFP_KERNEL,
    LUPOS_CRED_GFP_KERNEL_ACCOUNT as GFP_KERNEL_ACCOUNT,
    LUPOS_CRED_PROC_EVENT_GID as PROC_EVENT_GID,
    LUPOS_CRED_PROC_EVENT_UID as PROC_EVENT_UID,
    LUPOS_CRED_UCOUNT_RLIMIT_NPROC as UCOUNT_RLIMIT_NPROC,
    lupos_cred_uid_eq as uid_eq, lupos_cred_uid_lt as uid_lt,
    lupos_cred_uid_gt as uid_gt, lupos_cred_uid_valid as uid_valid,
    lupos_cred_gid_eq as gid_eq, lupos_cred_gid_lt as gid_lt,
    lupos_cred_gid_gt as gid_gt, lupos_cred_gid_valid as gid_valid,
    lupos_cred_get as get_cred, lupos_cred_get_many as get_cred_many,
    lupos_cred_put as put_cred, lupos_cred_put_many as put_cred_many,
    lupos_cred_get_uid as get_uid, lupos_cred_free_uid as free_uid,
    lupos_cred_get_user_ns as get_user_ns, lupos_cred_put_user_ns as put_user_ns,
    lupos_cred_get_group_info as get_group_info,
    lupos_cred_put_group_info as put_group_info,
    lupos_cred_get_ucounts as get_ucounts,
    lupos_cred_security_alloc_blank as security_cred_alloc_blank,
    lupos_cred_security_free as security_cred_free,
    lupos_cred_security_prepare as security_prepare_creds,
    lupos_cred_security_act_as as security_kernel_act_as,
    lupos_cred_security_create_files_as as security_kernel_create_files_as,
};
use kernel::ffi::c_int;

// The original #if 0 kdebug/no_printk branch never evaluates its arguments.
macro_rules! kdebug { ($($arg:tt)*) => {}; }

static mut CRED_JAR: *mut kmem_cache = core::ptr::null_mut();

/* The RCU callback to actually dispose of a set of credentials */
// SAFETY: Called only for the final native reference, either after the RCU
// grace period or with non_rcu set by the native lifetime protocol. No access
// follows the final slab free. Native atomic operations never form Rust refs.
// The native callback trampoline only calls this retained Rust body. Its
// signature contains no callback-valued parameter or Rust Option<fn>.
#[export_name = "lupos_cred_put_rcu_body"]
unsafe extern "C" fn put_cred_rcu(rcu: *mut callback_head) {
    // SAFETY: The caller contract above and native leaves govern these operations.
    unsafe {
        let cred = lupos_cred_from_rcu(rcu);

        kdebug!("put_cred_rcu(%p)", cred);
        if lupos_cred_usage_read(cred) != 0 {
            lupos_cred_bad_usage(cred);
        }

        security_cred_free(cred);
        // With KEYS=n, native key_put discards even the absent field arguments.
        #[cfg(CONFIG_KEYS)] {
            key_put(lupos_cred_session_keyring(cred));
            key_put(lupos_cred_process_keyring(cred));
            key_put(lupos_cred_thread_keyring(cred));
            key_put(lupos_cred_request_key_auth(cred));
        }
        if !lupos_cred_group_info(cred).is_null() { put_group_info(lupos_cred_group_info(cred)); }
        free_uid(lupos_cred_user(cred));
        if !lupos_cred_ucounts(cred).is_null() { put_ucounts(lupos_cred_ucounts(cred)); }
        put_user_ns(lupos_cred_user_ns(cred));
        lupos_cred_cache_free(CRED_JAR, cred);
    }
}

/// # Safety
/// Native put_cred must have surrendered the last reference. This object is no
/// longer installed on current; its non_rcu/RCU state follows native cred.h.
#[no_mangle]
pub unsafe extern "C" fn __put_cred(cred: *mut cred) {
    // SAFETY: The caller contract above and native leaves govern these operations.
    unsafe {
        kdebug!("__put_cred(%p{%ld})", cred, atomic_long_read(&(*cred).usage));
        lupos_cred_bug_put_usage(cred);
        lupos_cred_bug_put_current_cred(cred);
        lupos_cred_bug_put_current_real_cred(cred);
        if lupos_cred_non_rcu(cred) != 0 { put_cred_rcu(lupos_cred_rcu(cred)); }
        else { lupos_cred_call_rcu(lupos_cred_rcu(cred)); }
    }
}

/// # Safety
/// Caller owns the native task teardown or failed-fork cleanup sequence and
/// serializes these task-pointer stores. The credential objects may be shared.
#[no_mangle]
pub unsafe extern "C" fn exit_creds(tsk: *mut task_struct) {
    // SAFETY: The caller contract above and native leaves govern these operations.
    unsafe {
        let real_cred: *const cred;
        let cred: *const cred;
        kdebug!("exit_creds(%u,%p,%p,{%ld})", (*tsk).pid, (*tsk).real_cred, (*tsk).cred, atomic_long_read(&(*(*tsk).cred).usage));
        real_cred = lupos_cred_task_real_cred(tsk);
        lupos_cred_set_task_real_cred(tsk, core::ptr::null());
        cred = lupos_cred_task_cred(tsk);
        lupos_cred_set_task_cred(tsk, core::ptr::null());
        if real_cred == cred { put_cred_many(cred, 2); }
        else { put_cred(real_cred); put_cred(cred); }
        #[cfg(CONFIG_KEYS_REQUEST_CACHE)]
        { key_put(lupos_cred_cached_requested_key(tsk)); lupos_cred_clear_cached_requested_key(tsk); }
    }
}

/// # Safety
/// Caller pins task, or holds tasklist_lock to keep it linked. Credentials can
/// change concurrently; acquire a reference inside the RCU retry loop.
#[no_mangle]
pub unsafe extern "C" fn get_task_cred(task: *mut task_struct) -> *const cred {
    // SAFETY: The caller contract above and native leaves govern these operations.
    unsafe {
        let mut cred: *const cred;
        lupos_cred_rcu_read_lock();
        loop {
            cred = lupos_cred_task_rcu(task);
            lupos_cred_bug_task_cred_null(cred);
            if !lupos_cred_get_rcu(cred).is_null() { break; }
        }
        lupos_cred_rcu_read_unlock();
        cred
    }
}

/// # Safety
/// Native credential initialization has completed and the caller may sleep.
/// A successful result transfers one reference for construction or abort.
#[no_mangle]
pub unsafe extern "C" fn cred_alloc_blank() -> *mut cred {
    // SAFETY: The caller contract above and native leaves govern these operations.
    unsafe {
        let new = lupos_cred_cache_zalloc(CRED_JAR, GFP_KERNEL);
        if new.is_null() { return core::ptr::null_mut(); }
        lupos_cred_usage_set(new, 1);
        if security_cred_alloc_blank(new, GFP_KERNEL_ACCOUNT) < 0 { abort_creds(new); return core::ptr::null_mut(); }
        new
    }
}

/// # Safety
/// Caller may sleep and is current under the native credential protocol.
/// The copied source is live and may be shared; only new is newly allocated.
/// Native copying deliberately includes shared atomic bytes, as cred.c does.
#[no_mangle]
pub unsafe extern "C" fn prepare_creds() -> *mut cred {
    // SAFETY: The caller contract above and native leaves govern these operations.
    unsafe {
        let task = lupos_cred_current();
        let new = lupos_cred_cache_alloc(CRED_JAR, GFP_KERNEL);
        if new.is_null() { return core::ptr::null_mut(); }
        kdebug!("prepare_creds() alloc %p", new);
        let old = lupos_cred_task_cred(task);
        lupos_cred_copy(new, old);
        lupos_cred_set_non_rcu(new, 0);
        lupos_cred_usage_set(new, 1);
        get_group_info(lupos_cred_group_info(new)); get_uid(lupos_cred_user(new)); get_user_ns(lupos_cred_user_ns(new));
        #[cfg(CONFIG_KEYS)] {
            lupos_cred_key_get(lupos_cred_session_keyring(new));
            lupos_cred_key_get(lupos_cred_process_keyring(new));
            lupos_cred_key_get(lupos_cred_thread_keyring(new));
            lupos_cred_key_get(lupos_cred_request_key_auth(new));
        }
        #[cfg(CONFIG_SECURITY)] { lupos_cred_clear_security(new); }
        lupos_cred_set_ucounts(new, get_ucounts(lupos_cred_ucounts(new)));
        if lupos_cred_ucounts(new).is_null() || security_prepare_creds(new, old, GFP_KERNEL_ACCOUNT) < 0 { abort_creds(new); return core::ptr::null_mut(); }
        new
    }
}

/// # Safety
/// Caller holds current's cred_guard_mutex and satisfies prepare_creds' rules.
/// Caller must commit or abort the returned construction reference.
#[no_mangle]
pub unsafe extern "C" fn prepare_exec_creds() -> *mut cred {
    // SAFETY: The caller contract above and native leaves govern these operations.
    unsafe {
        let new = prepare_creds();
        if new.is_null() { return new; }
        #[cfg(CONFIG_KEYS)] {
            key_put(lupos_cred_thread_keyring(new));
            lupos_cred_set_thread_keyring(new, core::ptr::null_mut());
            key_put(lupos_cred_process_keyring(new));
            lupos_cred_set_process_keyring(new, core::ptr::null_mut());
        }
        // Native chained assignments evaluate each effective ID once, then store
        // the filesystem ID followed by the saved ID. The old fs IDs are discarded.
        let euid = lupos_cred_euid(new);
        lupos_cred_set_fsuid(new, euid); lupos_cred_set_suid(new, euid);
        let egid = lupos_cred_egid(new);
        lupos_cred_set_fsgid(new, egid); lupos_cred_set_sgid(new, egid);
        new
    }
}

/// # Safety
/// Called at the native copy_process stage: p is not published, its inherited
/// credential pointers remain pinned by current, and the caller may sleep.
/// A successful return installs the child's two references and namespace refs.
#[no_mangle]
pub unsafe extern "C" fn copy_creds(p: *mut task_struct, clone_flags: u64) -> c_int {
    // SAFETY: The caller contract above and native leaves govern these operations.
    unsafe {
        #[cfg(CONFIG_KEYS_REQUEST_CACHE)] { lupos_cred_clear_cached_requested_key(p); }
        // Keep C's left-to-right short circuit: with KEYS=y, inspect the parent's
        // thread keyring before evaluating CLONE_THREAD. KEYS=n has no field read.
        #[cfg(CONFIG_KEYS)]
        let share = lupos_cred_thread_keyring(lupos_cred_task_cred(p)).is_null()
            && (clone_flags & CLONE_THREAD) != 0;
        #[cfg(not(CONFIG_KEYS))]
        let share = (clone_flags & CLONE_THREAD) != 0;
        if share {
            lupos_cred_set_task_real_cred(p, get_cred_many(lupos_cred_task_cred(p), 2));
            inc_rlimit_ucounts(lupos_cred_task_ucounts(p), UCOUNT_RLIMIT_NPROC, 1);
            get_cred_namespaces(p); return 0;
        }
        let new = prepare_creds();
        if new.is_null() { return -ENOMEM; }
        if (clone_flags & CLONE_NEWUSER) != 0 { let ret = lupos_cred_create_user_ns(new); if ret < 0 { put_cred(new); return ret; } let ret = set_cred_ucounts(new); if ret < 0 { put_cred(new); return ret; } }
        #[cfg(CONFIG_KEYS)] {
            if !lupos_cred_thread_keyring(new).is_null() {
                key_put(lupos_cred_thread_keyring(new));
                lupos_cred_set_thread_keyring(new, core::ptr::null_mut());
                if (clone_flags & CLONE_THREAD) != 0 {
                    // Original cred.c intentionally does not propagate this result.
                    install_thread_keyring_to_cred(new);
                }
            }
            if (clone_flags & CLONE_THREAD) == 0 {
                key_put(lupos_cred_process_keyring(new));
                lupos_cred_set_process_keyring(new, core::ptr::null_mut());
            }
        }
        let published = get_cred(new);
        lupos_cred_set_task_real_cred(p, published);
        lupos_cred_set_task_cred(p, published);
        inc_rlimit_ucounts(lupos_cred_task_ucounts(p), UCOUNT_RLIMIT_NPROC, 1);
        get_cred_namespaces(p); 0
    }
}

// SAFETY: commit_creds pins both credentials. Each credential pins its user
// namespace, and those namespaces pin the parent chain while it is traversed.
unsafe fn cred_cap_issubset(set: *const cred, subset: *const cred) -> bool {
    // SAFETY: The caller contract above and native leaves govern these operations.
    unsafe {
        let set_ns = lupos_cred_user_ns(set); let mut subset_ns = lupos_cred_user_ns(subset);
        if set_ns == subset_ns { return lupos_cred_cap_issubset(lupos_cred_cap_permitted(subset), lupos_cred_cap_permitted(set)); }
        while subset_ns != lupos_cred_init_user_ns() {
            if set_ns == lupos_cred_ns_parent(subset_ns) && uid_eq(lupos_cred_ns_owner(subset_ns), lupos_cred_euid(set)) { return true; }
            subset_ns = lupos_cred_ns_parent(subset_ns);
        }
        false
    }
}

/// # Safety
/// Caller supplies a live, fully initialized credential set for installation
/// and transfers one reference. Current's subjective creds must not be
/// overridden. Other references may exist; no Rust exclusivity is asserted.
#[no_mangle]
pub unsafe extern "C" fn commit_creds(new: *mut cred) -> c_int {
    // SAFETY: The caller contract above and native leaves govern these operations.
    unsafe {
        let task = lupos_cred_current(); let old = lupos_cred_task_real_cred(task);
        lupos_cred_bug_commit_overridden(task, old); lupos_cred_bug_commit_usage(new); get_cred(new);
        if !uid_eq(lupos_cred_euid(old), lupos_cred_euid(new)) || !gid_eq(lupos_cred_egid(old), lupos_cred_egid(new)) || !uid_eq(lupos_cred_fsuid(old), lupos_cred_fsuid(new)) || !gid_eq(lupos_cred_fsgid(old), lupos_cred_fsgid(new)) || !cred_cap_issubset(old, new) {
            if lupos_cred_task_has_mm(task) { lupos_cred_set_dumpable(); }
            lupos_cred_clear_pdeath_signal(task); lupos_cred_smp_wmb();
        }
        if !uid_eq(lupos_cred_fsuid(new), lupos_cred_fsuid(old)) { lupos_cred_key_fsuid_changed(new); }
        if !gid_eq(lupos_cred_fsgid(new), lupos_cred_fsgid(old)) { lupos_cred_key_fsgid_changed(new); }
        if lupos_cred_user(new) != lupos_cred_user(old) || lupos_cred_user_ns(new) != lupos_cred_user_ns(old) { inc_rlimit_ucounts(lupos_cred_ucounts(new), UCOUNT_RLIMIT_NPROC, 1); }
        lupos_cred_publish_task_real_cred(task, new); lupos_cred_publish_task_cred(task, new);
        if lupos_cred_user(new) != lupos_cred_user(old) || lupos_cred_user_ns(new) != lupos_cred_user_ns(old) { dec_rlimit_ucounts(lupos_cred_ucounts(old), UCOUNT_RLIMIT_NPROC, 1); }
        if lupos_cred_user_ns(new) != lupos_cred_user_ns(old) { switch_cred_namespaces(old, new); }
        if !uid_eq(lupos_cred_uid(new), lupos_cred_uid(old)) || !uid_eq(lupos_cred_euid(new), lupos_cred_euid(old)) || !uid_eq(lupos_cred_suid(new), lupos_cred_suid(old)) || !uid_eq(lupos_cred_fsuid(new), lupos_cred_fsuid(old)) { lupos_cred_proc_id_connector(task, PROC_EVENT_UID); }
        if !gid_eq(lupos_cred_gid(new), lupos_cred_gid(old)) || !gid_eq(lupos_cred_egid(new), lupos_cred_egid(old)) || !gid_eq(lupos_cred_sgid(new), lupos_cred_sgid(old)) || !gid_eq(lupos_cred_fsgid(new), lupos_cred_fsgid(old)) { lupos_cred_proc_id_connector(task, PROC_EVENT_GID); }
        put_cred_many(old, 2); 0
    }
}

/// # Safety
/// Caller transfers a live construction reference which has not been consumed
/// by commit_creds. Native reference counting decides final destruction.
#[no_mangle]
pub unsafe extern "C" fn abort_creds(new: *mut cred) {
    // SAFETY: The caller contract above and native leaves govern these operations.
    unsafe {
        lupos_cred_bug_abort_usage(new); put_cred(new);
    }
}

/// # Safety
/// Both credentials and their group arrays remain live and stable under the
/// native reference/RCU protocol; each array has its native ngroups entries.
#[no_mangle]
pub unsafe extern "C" fn cred_fscmp(a: *const cred, b: *const cred) -> c_int {
    // SAFETY: The caller contract above and native leaves govern these operations.
    unsafe {
        if a == b { return 0; } if uid_lt(lupos_cred_fsuid(a), lupos_cred_fsuid(b)) { return -1; } if uid_gt(lupos_cred_fsuid(a), lupos_cred_fsuid(b)) { return 1; }
        if gid_lt(lupos_cred_fsgid(a), lupos_cred_fsgid(b)) { return -1; } if gid_gt(lupos_cred_fsgid(a), lupos_cred_fsgid(b)) { return 1; }
        let ga = lupos_cred_group_info(a); let gb = lupos_cred_group_info(b); if ga == gb { return 0; } if ga.is_null() { return -1; } if gb.is_null() { return 1; }
        if lupos_cred_ngroups(ga) < lupos_cred_ngroups(gb) { return -1; } if lupos_cred_ngroups(ga) > lupos_cred_ngroups(gb) { return 1; }
        let mut g: c_int = 0;
        while g < lupos_cred_ngroups(ga) {
            if gid_lt(lupos_cred_group_gid(ga, g), lupos_cred_group_gid(gb, g)) { return -1; }
            if gid_gt(lupos_cred_group_gid(ga, g), lupos_cred_group_gid(gb, g)) { return 1; }
            g += 1;
        } 0
    }
}

/// # Safety
/// Caller owns modification of a prepared credential with a live ucounts
/// reference and may sleep. The old reference remains until replacement.
#[no_mangle]
pub unsafe extern "C" fn set_cred_ucounts(new: *mut cred) -> c_int {
    // SAFETY: The caller contract above and native leaves govern these operations.
    unsafe {
        let old = lupos_cred_ucounts(new);
        if lupos_cred_ucounts_ns(old) == lupos_cred_user_ns(new) && uid_eq(lupos_cred_ucounts_uid(old), lupos_cred_uid(new)) { return 0; }
        let fresh = alloc_ucounts(lupos_cred_user_ns(new), lupos_cred_uid(new)); if fresh.is_null() { return -EAGAIN; }
        lupos_cred_set_ucounts(new, fresh); put_ucounts(old); 0
    }
}

/// # Safety
/// Native boot ordering calls this once, before credential allocation or use.
/// The native __init entry shell owns the public entry's configured metadata.
/// This body's emitted attributes and section lifetime remain unqualified.
#[export_name = "lupos_cred_init_body"]
#[link_section = ".init.text"]
unsafe extern "C" fn cred_init() {
    // SAFETY: The caller contract above and native leaves govern these operations.
    unsafe {
        CRED_JAR = lupos_cred_cache_create();
    }
}

/// # Safety
/// Caller may sleep and keeps a non-null daemon task alive as get_task_cred
/// requires. Its source credential is shared and pinned, never exclusive.
/// A null daemon keeps the original WARN_ON_ONCE and null-result behavior.
#[no_mangle]
pub unsafe extern "C" fn prepare_kernel_cred(daemon: *mut task_struct) -> *mut cred {
    // SAFETY: The caller contract above and native leaves govern these operations.
    unsafe {
        if lupos_cred_warn_null_daemon(daemon) { return core::ptr::null_mut(); }
        let new = lupos_cred_cache_alloc(CRED_JAR, GFP_KERNEL); if new.is_null() { return core::ptr::null_mut(); }
        let old = get_task_cred(daemon);
        lupos_cred_assign(new, old); lupos_cred_set_non_rcu(new, 0); lupos_cred_usage_set(new, 1);
        get_uid(lupos_cred_user(new)); get_user_ns(lupos_cred_user_ns(new)); get_group_info(lupos_cred_group_info(new));
        #[cfg(CONFIG_KEYS)] {
            lupos_cred_set_session_keyring(new, core::ptr::null_mut());
            lupos_cred_set_process_keyring(new, core::ptr::null_mut());
            lupos_cred_set_thread_keyring(new, core::ptr::null_mut());
            lupos_cred_set_request_key_auth(new, core::ptr::null_mut());
            lupos_cred_set_jit_keyring(new, LUPOS_CRED_KEY_REQKEY_DEFL_THREAD_KEYRING);
        }
        #[cfg(CONFIG_SECURITY)] { lupos_cred_clear_security(new); }
        lupos_cred_set_ucounts(new, get_ucounts(lupos_cred_ucounts(new)));
        if lupos_cred_ucounts(new).is_null() || security_prepare_creds(new, old, GFP_KERNEL_ACCOUNT) < 0 { put_cred(new); put_cred(old); return core::ptr::null_mut(); }
        put_cred(old); new
    }
}

/// # Safety
/// Caller owns modification of a prepared credential and satisfies the LSM
/// kernel_act_as hook's context requirements.
#[no_mangle]
pub unsafe extern "C" fn set_security_override(new: *mut cred, secid: u32) -> c_int {
    // SAFETY: The caller contract above and native leaves govern these operations.
    unsafe {
        security_kernel_act_as(new, secid)
    }
}

/// # Safety
/// Caller owns modification of a prepared credential and pins inode under the
/// native ID/LSM synchronization contract throughout the operation.
#[no_mangle]
pub unsafe extern "C" fn set_create_files_as(new: *mut cred, inode: *mut inode) -> c_int {
    // SAFETY: The caller contract above and native leaves govern these operations.
    unsafe {
        if !uid_valid(lupos_cred_inode_uid(inode)) || !gid_valid(lupos_cred_inode_gid(inode)) { return -EINVAL; }
        lupos_cred_set_fsuid(new, lupos_cred_inode_uid(inode));
        lupos_cred_set_fsgid(new, lupos_cred_inode_gid(inode)); security_kernel_create_files_as(new, inode)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
