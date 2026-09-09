// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright (C) 2007
 *
 *  Author: Eric Biederman <ebiederm@xmision.com>
 */

// Kernel dependencies supplied by other translation units.

unsafe fn proc_ipc_dointvec_minmax_orphans(
    table: *const ctl_table, write: c_int, buffer: *mut c_void,
    lenp: *mut usize, ppos: *mut loff_t,
) -> c_int {
    let ns = container_of((*table).data, ipc_namespace, shm_rmid_forced);
    let mut err = proc_dointvec_minmax(table, write, buffer, lenp, ppos);
    if err < 0 { return err; }
    if write != 0 && (*ns).shm_rmid_forced != 0 { shm_destroy_orphaned(ns); }
    err
}

unsafe fn proc_ipc_auto_msgmni(
    table: *const ctl_table, write: c_int, buffer: *mut c_void,
    lenp: *mut usize, ppos: *mut loff_t,
) -> c_int {
    let mut ipc_table = *table;
    let mut dummy: c_int = 0;
    ipc_table.data = &mut dummy as *mut c_int as *mut c_void;
    if write != 0 { pr_info_once!("writing to auto_msgmni has no effect"); }
    proc_dointvec_minmax(&ipc_table, write, buffer, lenp, ppos)
}

unsafe fn proc_ipc_sem_dointvec(
    table: *const ctl_table, write: c_int, buffer: *mut c_void,
    lenp: *mut usize, ppos: *mut loff_t,
) -> c_int {
    let ns = container_of((*table).data, ipc_namespace, sem_ctls);
    let semmni = (*ns).sem_ctls[3];
    let mut ret = proc_dointvec(table, write, buffer, lenp, ppos);
    if ret == 0 { ret = sem_check_semmni(ns); }
    /* Reset the semmni value if an error happens. */
    if ret != 0 { (*ns).sem_ctls[3] = semmni; }
    ret
}

pub static mut ipc_mni: c_int = IPCMNI;
pub static mut ipc_mni_shift: c_int = IPCMNI_SHIFT;
pub static mut ipc_min_cycle: c_int = RADIX_TREE_MAP_SIZE;

static mut ipc_sysctls: [ctl_table; 9] = [
    ctl_table { procname: c_str!("shmmax"), data: unsafe { &mut init_ipc_ns.shm_ctlmax as *mut _ as *mut c_void }, maxlen: size_of::<_>(), mode: 0o644, proc_handler: Some(proc_doulongvec_minmax), extra1: SYSCTL_ZERO, extra2: core::ptr::null_mut() },
    ctl_table { procname: c_str!("shmall"), data: unsafe { &mut init_ipc_ns.shm_ctlall as *mut _ as *mut c_void }, maxlen: size_of::<_>(), mode: 0o644, proc_handler: Some(proc_doulongvec_minmax), extra1: SYSCTL_ZERO, extra2: core::ptr::null_mut() },
    ctl_table { procname: c_str!("shmmni"), data: unsafe { &mut init_ipc_ns.shm_ctlmni as *mut _ as *mut c_void }, maxlen: size_of::<_>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: SYSCTL_ZERO, extra2: unsafe { &mut ipc_mni as *mut _ as *mut c_void } },
    ctl_table { procname: c_str!("shm_rmid_forced"), data: unsafe { &mut init_ipc_ns.shm_rmid_forced as *mut _ as *mut c_void }, maxlen: size_of::<_>(), mode: 0o644, proc_handler: Some(proc_ipc_dointvec_minmax_orphans), extra1: SYSCTL_ZERO, extra2: SYSCTL_ONE },
    ctl_table { procname: c_str!("msgmax"), data: unsafe { &mut init_ipc_ns.msg_ctlmax as *mut _ as *mut c_void }, maxlen: size_of::<_>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: SYSCTL_ZERO, extra2: SYSCTL_INT_MAX },
    ctl_table { procname: c_str!("msgmni"), data: unsafe { &mut init_ipc_ns.msg_ctlmni as *mut _ as *mut c_void }, maxlen: size_of::<_>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: SYSCTL_ZERO, extra2: unsafe { &mut ipc_mni as *mut _ as *mut c_void } },
    ctl_table { procname: c_str!("auto_msgmni"), data: core::ptr::null_mut(), maxlen: size_of::<c_int>(), mode: 0o644, proc_handler: Some(proc_ipc_auto_msgmni), extra1: SYSCTL_ZERO, extra2: SYSCTL_ONE },
    ctl_table { procname: c_str!("msgmnb"), data: unsafe { &mut init_ipc_ns.msg_ctlmnb as *mut _ as *mut c_void }, maxlen: size_of::<_>(), mode: 0o644, proc_handler: Some(proc_dointvec_minmax), extra1: SYSCTL_ZERO, extra2: SYSCTL_INT_MAX },
    ctl_table { procname: c_str!("sem"), data: unsafe { &mut init_ipc_ns.sem_ctls as *mut _ as *mut c_void }, maxlen: 4 * size_of::<c_int>(), mode: 0o644, proc_handler: Some(proc_ipc_sem_dointvec), extra1: core::ptr::null_mut(), extra2: core::ptr::null_mut() },
];

unsafe fn set_lookup(_root: *mut ctl_table_root) -> *mut ctl_table_set { &mut (*current).nsproxy.ipc_ns.ipc_set }
unsafe fn set_is_seen(set: *mut ctl_table_set) -> c_int { (set == &mut (*current).nsproxy.ipc_ns.ipc_set) as c_int }

unsafe fn ipc_set_ownership(head: *mut ctl_table_header, uid: *mut kuid_t, gid: *mut kgid_t) {
    let ns = container_of((*head).set, ipc_namespace, ipc_set);
    let ns_root_uid = make_kuid((*ns).user_ns, 0);
    let ns_root_gid = make_kgid((*ns).user_ns, 0);
    *uid = if uid_valid(ns_root_uid) { ns_root_uid } else { GLOBAL_ROOT_UID };
    *gid = if gid_valid(ns_root_gid) { ns_root_gid } else { GLOBAL_ROOT_GID };
}

unsafe fn ipc_permissions(head: *mut ctl_table_header, table: *const ctl_table) -> c_int {
    let mut mode = (*table).mode;
    let mut ns_root_uid = core::mem::zeroed();
    let mut ns_root_gid = core::mem::zeroed();
    ipc_set_ownership(head, &mut ns_root_uid, &mut ns_root_gid);
    if uid_eq(current_euid(), ns_root_uid) { mode >>= 6; }
    else if in_egroup_p(ns_root_gid) { mode >>= 3; }
    mode &= 7;
    (mode << 6) | (mode << 3) | mode
}

static mut set_root: ctl_table_root = ctl_table_root { lookup: Some(set_lookup), permissions: Some(ipc_permissions), set_ownership: Some(ipc_set_ownership) };

pub unsafe fn setup_ipc_sysctls(ns: *mut ipc_namespace) -> bool {
    setup_sysctl_set(&mut (*ns).ipc_set, &mut set_root, Some(set_is_seen));
    let tbl = kmemdup(ipc_sysctls.as_ptr() as *const c_void, size_of_val(&ipc_sysctls), GFP_KERNEL) as *mut ctl_table;
    if !tbl.is_null() {
        for i in 0..ipc_sysctls.len() {
            let data = (*tbl.add(i)).data;
            (*tbl.add(i)).data = if data == (&mut init_ipc_ns.shm_ctlmax as *mut _ as *mut c_void) { &mut (*ns).shm_ctlmax as *mut _ as *mut c_void }
            else if data == (&mut init_ipc_ns.shm_ctlall as *mut _ as *mut c_void) { &mut (*ns).shm_ctlall as *mut _ as *mut c_void }
            else if data == (&mut init_ipc_ns.shm_ctlmni as *mut _ as *mut c_void) { &mut (*ns).shm_ctlmni as *mut _ as *mut c_void }
            else if data == (&mut init_ipc_ns.shm_rmid_forced as *mut _ as *mut c_void) { &mut (*ns).shm_rmid_forced as *mut _ as *mut c_void }
            else if data == (&mut init_ipc_ns.msg_ctlmax as *mut _ as *mut c_void) { &mut (*ns).msg_ctlmax as *mut _ as *mut c_void }
            else if data == (&mut init_ipc_ns.msg_ctlmni as *mut _ as *mut c_void) { &mut (*ns).msg_ctlmni as *mut _ as *mut c_void }
            else if data == (&mut init_ipc_ns.msg_ctlmnb as *mut _ as *mut c_void) { &mut (*ns).msg_ctlmnb as *mut _ as *mut c_void }
            else if data == (&mut init_ipc_ns.sem_ctls as *mut _ as *mut c_void) { &mut (*ns).sem_ctls as *mut _ as *mut c_void }
            else { core::ptr::null_mut() };
        }
        (*ns).ipc_sysctls = __register_sysctl_table(&mut (*ns).ipc_set, c_str!("kernel"), tbl, ipc_sysctls.len());
    }
    if (*ns).ipc_sysctls.is_null() { kfree(tbl as *mut c_void); retire_sysctl_set(&mut (*ns).ipc_set); return false; }
    true
}

pub unsafe fn retire_ipc_sysctls(ns: *mut ipc_namespace) {
    let tbl = (*ns).ipc_sysctls.ctl_table_arg;
    unregister_sysctl_table((*ns).ipc_sysctls);
    retire_sysctl_set(&mut (*ns).ipc_set);
    kfree(tbl as *mut c_void);
}

unsafe fn ipc_sysctl_init() -> c_int {
    if !setup_ipc_sysctls(&mut init_ipc_ns) { pr_warn!("ipc sysctl registration failed\n"); return -ENOMEM; }
    0
}

device_initcall!(ipc_sysctl_init);

unsafe fn ipc_mni_extend(_str: *mut c_char) -> c_int {
    ipc_mni = IPCMNI_EXTEND;
    ipc_mni_shift = IPCMNI_EXTEND_SHIFT;
    ipc_min_cycle = IPCMNI_EXTEND_MIN_CYCLE;
    pr_info!("IPCMNI extended to %d.\n", ipc_mni);
    0
}

early_param!(c_str!("ipcmni_extend"), ipc_mni_extend);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
