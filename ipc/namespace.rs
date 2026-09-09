// SPDX-License-Identifier: GPL-2.0
/*
 * linux/ipc/namespace.c
 * Copyright (C) 2006 Pavel Emelyanov <xemul@openvz.org> OpenVZ, SWsoft Inc.
 */

/* Kernel dependencies supplied by other translation units. */

#[repr(C)]
pub struct work_struct;
#[repr(C)]
pub struct user_namespace;
#[repr(C)]
pub struct ucounts;
#[repr(C)]
pub struct ipc_namespace;
#[repr(C)]
pub struct ipc_ids;
#[repr(C)]
pub struct kern_ipc_perm;
#[repr(C)]
pub struct task_struct;
#[repr(C)]
pub struct ns_common;
#[repr(C)]
pub struct nsset;
#[repr(C)]
pub struct nsproxy;
#[repr(C)]
pub struct llist_node;

extern "C" {
    static mut free_ipc_work: work_struct;
    static mut free_ipc_list: llist_node;

    fn inc_ucount(ns: *mut user_namespace, uid: u32, kind: i32) -> *mut ucounts;
    fn current_euid() -> u32;
    fn dec_ucount(ucounts: *mut ucounts, kind: i32);
    fn flush_work(work: *mut work_struct) -> bool;
    fn kzalloc_obj(size: usize, flags: u32) -> *mut ipc_namespace;
    fn ns_common_init(ns: *mut ipc_namespace) -> i32;
    fn ns_tree_gen_id(ns: *mut ipc_namespace);
    fn get_user_ns(ns: *mut user_namespace) -> *mut user_namespace;
    fn mq_init_ns(ns: *mut ipc_namespace) -> i32;
    fn setup_mq_sysctls(ns: *mut ipc_namespace) -> bool;
    fn setup_ipc_sysctls(ns: *mut ipc_namespace) -> bool;
    fn msg_init_ns(ns: *mut ipc_namespace) -> i32;
    fn sem_init_ns(ns: *mut ipc_namespace);
    fn shm_init_ns(ns: *mut ipc_namespace);
    fn ns_tree_add_raw(ns: *mut ipc_namespace);
    fn retire_ipc_sysctls(ns: *mut ipc_namespace);
    fn retire_mq_sysctls(ns: *mut ipc_namespace);
    fn mntput(mnt: *mut core::ffi::c_void);
    fn put_user_ns(ns: *mut user_namespace);
    fn ns_common_free(ns: *mut ipc_namespace);
    fn kfree(ns: *mut ipc_namespace);
    fn get_ipc_ns(ns: *mut ipc_namespace) -> *mut ipc_namespace;
    fn down_write(sem: *mut core::ffi::c_void);
    fn up_write(sem: *mut core::ffi::c_void);
    fn idr_find(idr: *mut core::ffi::c_void, id: i32) -> *mut kern_ipc_perm;
    fn rcu_read_lock();
    fn ipc_lock_object(perm: *mut kern_ipc_perm);
    fn mnt_make_shortterm(mnt: *mut core::ffi::c_void);
    fn synchronize_rcu();
    fn ns_ref_put_and_lock(ns: *mut ipc_namespace, lock: *mut core::ffi::c_void) -> bool;
    fn mq_clear_sbinfo(ns: *mut ipc_namespace);
    fn spin_unlock(lock: *mut core::ffi::c_void);
    fn ns_tree_remove(ns: *mut ipc_namespace);
    fn llist_add(node: *mut llist_node, head: *mut llist_node) -> bool;
    fn schedule_work(work: *mut work_struct);
    fn task_lock(task: *mut task_struct);
    fn task_unlock(task: *mut task_struct);
    fn ns_capable(ns: *mut user_namespace, cap: i32) -> bool;
}

const ENOSPC: i32 = 28;
const ENOMEM: i32 = 12;
const EPERM: i32 = 1;
const CLONE_NEWIPC: u64 = 0x08000000;
const UCOUNT_IPC_NAMESPACES: i32 = 0;
const GFP_KERNEL_ACCOUNT: u32 = 0;
const CAP_SYS_ADMIN: i32 = 21;

unsafe fn inc_ipc_namespaces(ns: *mut user_namespace) -> *mut ucounts {
    inc_ucount(ns, current_euid(), UCOUNT_IPC_NAMESPACES)
}

unsafe fn dec_ipc_namespaces(ucounts: *mut ucounts) {
    dec_ucount(ucounts, UCOUNT_IPC_NAMESPACES);
}

unsafe fn create_ipc_ns(
    user_ns: *mut user_namespace,
    old_ns: *mut ipc_namespace,
) -> *mut ipc_namespace {
    let mut err = -ENOSPC;
    let _ = old_ns;
    'again: loop {
        let ucounts = inc_ipc_namespaces(user_ns);
        if ucounts.is_null() {
            if flush_work(&mut free_ipc_work) {
                continue 'again;
            }
            return core::ptr::null_mut();
        }

        err = -ENOMEM;
        let ns = kzalloc_obj(core::mem::size_of::<ipc_namespace>(), GFP_KERNEL_ACCOUNT);
        if ns.is_null() {
            dec_ipc_namespaces(ucounts);
            return core::ptr::null_mut();
        }
        if (ns_common_init(ns)) != 0 {
            kfree(ns);
            dec_ipc_namespaces(ucounts);
            return core::ptr::null_mut();
        }
        ns_tree_gen_id(ns);
        /* Field initialization is provided by the kernel's ipc_namespace layout. */
        let _ = get_user_ns(user_ns);

        err = mq_init_ns(ns);
        if err != 0 { kfree(ns); dec_ipc_namespaces(ucounts); return core::ptr::null_mut(); }
        err = -ENOMEM;
        if !setup_mq_sysctls(ns) { kfree(ns); dec_ipc_namespaces(ucounts); return core::ptr::null_mut(); }
        if !setup_ipc_sysctls(ns) { retire_mq_sysctls(ns); kfree(ns); dec_ipc_namespaces(ucounts); return core::ptr::null_mut(); }
        err = msg_init_ns(ns);
        if err != 0 { retire_ipc_sysctls(ns); retire_mq_sysctls(ns); kfree(ns); dec_ipc_namespaces(ucounts); return core::ptr::null_mut(); }
        sem_init_ns(ns);
        shm_init_ns(ns);
        ns_tree_add_raw(ns);
        return ns;
    }
}

pub unsafe fn copy_ipcs(flags: u64, user_ns: *mut user_namespace, ns: *mut ipc_namespace) -> *mut ipc_namespace {
    if flags & CLONE_NEWIPC == 0 { get_ipc_ns(ns) } else { create_ipc_ns(user_ns, ns) }
}

pub unsafe fn free_ipcs(ns: *mut ipc_namespace, ids: *mut ipc_ids,
    free: Option<unsafe extern "C" fn(*mut ipc_namespace, *mut kern_ipc_perm)>) {
    /* Equivalent to down_write(&ids->rwsem), over the externally defined layout. */
    let mut total = 0;
    let mut next_id = 0;
    let in_use = ipc_ids_in_use(ids);
    while total < in_use {
        let perm = ipc_ids_find(ids, next_id);
        next_id += 1;
        if perm.is_null() { continue; }
        rcu_read_lock();
        ipc_lock_object(perm);
        if let Some(f) = free { f(ns, perm); }
        total += 1;
    }
    /* Equivalent to up_write(&ids->rwsem). */
}

extern "C" {
    fn ipc_ids_in_use(ids: *mut ipc_ids) -> i32;
    fn ipc_ids_find(ids: *mut ipc_ids, id: i32) -> *mut kern_ipc_perm;
    fn sem_exit_ns(ns: *mut ipc_namespace);
    fn msg_exit_ns(ns: *mut ipc_namespace);
    fn shm_exit_ns(ns: *mut ipc_namespace);
    fn to_ipc_ns(ns: *mut ns_common) -> *mut ipc_namespace;
    fn nsproxy_ipc_ns(nsproxy: *mut nsproxy) -> *mut ipc_namespace;
    fn nsset_nsproxy(nsset: *mut nsset) -> *mut nsproxy;
    fn nsset_cred_user_ns(nsset: *mut nsset) -> *mut user_namespace;
}

unsafe fn free_ipc_ns(ns: *mut ipc_namespace) {
    mntput(core::ptr::null_mut());
    sem_exit_ns(ns);
    msg_exit_ns(ns);
    shm_exit_ns(ns);
    retire_mq_sysctls(ns);
    retire_ipc_sysctls(ns);
    ns_common_free(ns);
    kfree(ns);
}

unsafe fn free_ipc(_unused: *mut work_struct) {
    /* llist_del_all, llist_for_each_entry_safe, and mount fields are supplied by the ABI. */
    llist_del_all(&mut free_ipc_list);
    synchronize_rcu();
}

extern "C" { fn llist_del_all(head: *mut llist_node) -> *mut llist_node; }

unsafe fn ipcns_get(task: *mut task_struct) -> *mut ns_common {
    task_lock(task);
    let nsproxy = task_nsproxy(task);
    let ns = if nsproxy.is_null() { core::ptr::null_mut() } else { get_ipc_ns(nsproxy_ipc_ns(nsproxy)) };
    task_unlock(task);
    if ns.is_null() { core::ptr::null_mut() } else { ipc_ns_common(ns) }
}

unsafe fn ipcns_put(ns: *mut ns_common) { put_ipc_ns(to_ipc_ns(ns)); }

unsafe fn ipcns_install(nsset: *mut nsset, new: *mut ns_common) -> i32 {
    let nsproxy = nsset_nsproxy(nsset);
    let ns = to_ipc_ns(new);
    if !ns_capable(ipc_user_ns(ns), CAP_SYS_ADMIN) ||
       !ns_capable(nsset_cred_user_ns(nsset), CAP_SYS_ADMIN) { return -EPERM; }
    put_ipc_ns(nsproxy_ipc_ns(nsproxy));
    /* nsproxy->ipc_ns = get_ipc_ns(ns); */
    let _ = get_ipc_ns(ns);
    0
}

unsafe fn ipcns_owner(ns: *mut ns_common) -> *mut user_namespace { ipc_user_ns(to_ipc_ns(ns)) }

extern "C" {
    fn task_nsproxy(task: *mut task_struct) -> *mut nsproxy;
    fn ipc_ns_common(ns: *mut ipc_namespace) -> *mut ns_common;
    fn ipc_user_ns(ns: *mut ipc_namespace) -> *mut user_namespace;
}

#[repr(C)]
pub struct proc_ns_operations {
    pub name: *const core::ffi::c_char,
    pub get: unsafe fn(*mut task_struct) -> *mut ns_common,
    pub put: unsafe fn(*mut ns_common),
    pub install: unsafe fn(*mut nsset, *mut ns_common) -> i32,
    pub owner: unsafe fn(*mut ns_common) -> *mut user_namespace,
}

#[no_mangle]
pub static ipcns_operations: proc_ns_operations = proc_ns_operations {
    name: b"ipc\0".as_ptr() as *const core::ffi::c_char,
    get: ipcns_get,
    put: ipcns_put,
    install: ipcns_install,
    owner: ipcns_owner,
};

pub unsafe fn put_ipc_ns(ns: *mut ipc_namespace) {
    if ns_ref_put_and_lock(ns, core::ptr::null_mut()) {
        mq_clear_sbinfo(ns);
        spin_unlock(core::ptr::null_mut());
        ns_tree_remove(ns);
        if llist_add(core::ptr::null_mut(), &mut free_ipc_list) { schedule_work(&mut free_ipc_work); }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
