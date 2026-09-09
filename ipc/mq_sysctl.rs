// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (C) 2007 IBM Corporation
 *
 *  Author: Cedric Le Goater <clg@fr.ibm.com>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/nsproxy.h, linux/ipc_namespace.h, linux/sysctl.h, linux/stat.h,
// linux/capability.h, linux/slab.h, linux/cred.h

static mut msg_max_limit_min: c_int = MIN_MSGMAX;
static mut msg_max_limit_max: c_int = HARD_MSGMAX;

static mut msg_maxsize_limit_min: c_int = MIN_MSGSIZEMAX;
static mut msg_maxsize_limit_max: c_int = HARD_MSGSIZEMAX;

static mut mq_sysctls: [ctl_table; 5] = [
    ctl_table {
        procname: b"queues_max\0".as_ptr() as *const c_char,
        data: unsafe { &raw mut (*(&raw mut init_ipc_ns)).mq_queues_max as *mut _ as *mut c_void },
        maxlen: core::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec),
        extra1: core::ptr::null_mut(),
        extra2: core::ptr::null_mut(),
    },
    ctl_table {
        procname: b"msg_max\0".as_ptr() as *const c_char,
        data: unsafe { &raw mut (*(&raw mut init_ipc_ns)).mq_msg_max as *mut _ as *mut c_void },
        maxlen: core::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax),
        extra1: unsafe { &raw mut msg_max_limit_min as *mut _ as *mut c_void },
        extra2: unsafe { &raw mut msg_max_limit_max as *mut _ as *mut c_void },
    },
    ctl_table {
        procname: b"msgsize_max\0".as_ptr() as *const c_char,
        data: unsafe { &raw mut (*(&raw mut init_ipc_ns)).mq_msgsize_max as *mut _ as *mut c_void },
        maxlen: core::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax),
        extra1: unsafe { &raw mut msg_maxsize_limit_min as *mut _ as *mut c_void },
        extra2: unsafe { &raw mut msg_maxsize_limit_max as *mut _ as *mut c_void },
    },
    ctl_table {
        procname: b"msg_default\0".as_ptr() as *const c_char,
        data: unsafe { &raw mut (*(&raw mut init_ipc_ns)).mq_msg_default as *mut _ as *mut c_void },
        maxlen: core::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax),
        extra1: unsafe { &raw mut msg_max_limit_min as *mut _ as *mut c_void },
        extra2: unsafe { &raw mut msg_max_limit_max as *mut _ as *mut c_void },
    },
    ctl_table {
        procname: b"msgsize_default\0".as_ptr() as *const c_char,
        data: unsafe { &raw mut (*(&raw mut init_ipc_ns)).mq_msgsize_default as *mut _ as *mut c_void },
        maxlen: core::mem::size_of::<c_int>(),
        mode: 0o644,
        proc_handler: Some(proc_dointvec_minmax),
        extra1: unsafe { &raw mut msg_maxsize_limit_min as *mut _ as *mut c_void },
        extra2: unsafe { &raw mut msg_maxsize_limit_max as *mut _ as *mut c_void },
    },
];

unsafe extern "C" fn set_lookup(_root: *mut ctl_table_root) -> *mut ctl_table_set {
    (*current).nsproxy.ipc_ns.mq_set.as_mut()
}

unsafe extern "C" fn set_is_seen(set: *mut ctl_table_set) -> c_int {
    if core::ptr::eq(&(*current).nsproxy.ipc_ns.mq_set, set) { 1 } else { 0 }
}

unsafe extern "C" fn mq_set_ownership(
    head: *mut ctl_table_header,
    uid: *mut kuid_t,
    gid: *mut kgid_t,
) {
    let ns = container_of!((*head).set, ipc_namespace, mq_set);
    let ns_root_uid = make_kuid((*ns).user_ns, 0);
    let ns_root_gid = make_kgid((*ns).user_ns, 0);
    *uid = if uid_valid(ns_root_uid) { ns_root_uid } else { GLOBAL_ROOT_UID };
    *gid = if gid_valid(ns_root_gid) { ns_root_gid } else { GLOBAL_ROOT_GID };
}

unsafe extern "C" fn mq_permissions(
    head: *mut ctl_table_header,
    table: *const ctl_table,
) -> c_int {
    let mut mode = (*table).mode;
    let mut ns_root_uid = core::mem::zeroed::<kuid_t>();
    let mut ns_root_gid = core::mem::zeroed::<kgid_t>();
    mq_set_ownership(head, &mut ns_root_uid, &mut ns_root_gid);
    if uid_eq(current_euid(), ns_root_uid) {
        mode >>= 6;
    } else if in_egroup_p(ns_root_gid) {
        mode >>= 3;
    }
    mode &= 7;
    (mode << 6) | (mode << 3) | mode
}

static mut set_root: ctl_table_root = ctl_table_root {
    lookup: Some(set_lookup),
    permissions: Some(mq_permissions),
    set_ownership: Some(mq_set_ownership),
};

pub unsafe extern "C" fn setup_mq_sysctls(ns: *mut ipc_namespace) -> bool {
    setup_sysctl_set(&mut (*ns).mq_set, &mut set_root, Some(set_is_seen));
    let mut tbl = kmemdup(
        mq_sysctls.as_ptr() as *const c_void,
        core::mem::size_of_val(&mq_sysctls),
        GFP_KERNEL,
    ) as *mut ctl_table;
    if !tbl.is_null() {
        for i in 0..mq_sysctls.len() {
            let entry = &mut *tbl.add(i);
            if entry.data == (&raw mut (*(&raw mut init_ipc_ns)).mq_queues_max as *mut _ as *mut c_void {
                entry.data = &raw mut (*ns).mq_queues_max as *mut _ as *mut c_void;
            } else if entry.data == (&raw mut (*(&raw mut init_ipc_ns)).mq_msg_max as *mut _ as *mut c_void {
                entry.data = &raw mut (*ns).mq_msg_max as *mut _ as *mut c_void;
            } else if entry.data == (&raw mut (*(&raw mut init_ipc_ns)).mq_msgsize_max as *mut _ as *mut c_void {
                entry.data = &raw mut (*ns).mq_msgsize_max as *mut _ as *mut c_void;
            } else if entry.data == (&raw mut (*(&raw mut init_ipc_ns)).mq_msg_default as *mut _ as *mut c_void {
                entry.data = &raw mut (*ns).mq_msg_default as *mut _ as *mut c_void;
            } else if entry.data == (&raw mut (*(&raw mut init_ipc_ns)).mq_msgsize_default as *mut _ as *mut c_void {
                entry.data = &raw mut (*ns).mq_msgsize_default as *mut _ as *mut c_void;
            } else {
                entry.data = core::ptr::null_mut();
            }
        }
        (*ns).mq_sysctls = __register_sysctl_table(
            &mut (*ns).mq_set,
            b"fs/mqueue\0".as_ptr() as *const c_char,
            tbl,
            mq_sysctls.len(),
        );
    }
    if (*ns).mq_sysctls.is_null() {
        kfree(tbl as *mut c_void);
        retire_sysctl_set(&mut (*ns).mq_set);
        return false;
    }
    true
}

pub unsafe extern "C" fn retire_mq_sysctls(ns: *mut ipc_namespace) {
    let tbl = (*(*ns).mq_sysctls).ctl_table_arg;
    unregister_sysctl_table((*ns).mq_sysctls);
    retire_sysctl_set(&mut (*ns).mq_set);
    kfree(tbl as *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
