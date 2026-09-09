// SPDX-License-Identifier: GPL-2.0-only

// Kernel dependencies supplied by the surrounding repository.

pub static mut init_ucounts: ucounts = ucounts {
    ns: unsafe { &mut init_user_ns },
    uid: GLOBAL_ROOT_UID,
    count: RCUREF_INIT(1),
};

pub const UCOUNTS_HASHTABLE_BITS: usize = 10;
pub const UCOUNTS_HASHTABLE_ENTRIES: usize = 1 << UCOUNTS_HASHTABLE_BITS;
static mut ucounts_hashtable: [hlist_nulls_head; UCOUNTS_HASHTABLE_ENTRIES] =
    [HLIST_NULLS_HEAD_INIT(0); UCOUNTS_HASHTABLE_ENTRIES];
static mut ucounts_lock: spinlock = DEFINE_SPINLOCK();

#[cfg(CONFIG_SYSCTL)]
unsafe fn set_lookup(_root: *mut ctl_table_root) -> *mut ctl_table_set {
    &mut (*current_user_ns()).set
}

#[cfg(CONFIG_SYSCTL)]
unsafe fn set_is_seen(set: *mut ctl_table_set) -> i32 {
    ((*current_user_ns()).set as *mut ctl_table_set == set) as i32
}

#[cfg(CONFIG_SYSCTL)]
unsafe fn set_permissions(head: *mut ctl_table_header, table: *const ctl_table) -> i32 {
    let user_ns = container_of((*head).set, user_namespace, set);
    let mode;
    // Allow users with CAP_SYS_RESOURCE unrestrained access
    if ns_capable_noaudit(user_ns, CAP_SYS_RESOURCE) {
        mode = ((*table).mode & S_IRWXU) >> 6;
    } else {
        // Allow all others at most read-only access
        mode = (*table).mode & S_IROTH;
    }
    (mode << 6) | (mode << 3) | mode
}

#[cfg(CONFIG_SYSCTL)]
static mut set_root: ctl_table_root = ctl_table_root {
    lookup: Some(set_lookup),
    permissions: Some(set_permissions),
};

#[cfg(CONFIG_SYSCTL)]
static mut ue_zero: libc::c_long = 0;
#[cfg(CONFIG_SYSCTL)]
static mut ue_int_max: libc::c_long = INT_MAX as libc::c_long;

#[cfg(CONFIG_SYSCTL)]
static user_table: &[ctl_table] = &[
    UCOUNT_ENTRY!("max_user_namespaces"),
    UCOUNT_ENTRY!("max_pid_namespaces"),
    UCOUNT_ENTRY!("max_uts_namespaces"),
    UCOUNT_ENTRY!("max_ipc_namespaces"),
    UCOUNT_ENTRY!("max_net_namespaces"),
    UCOUNT_ENTRY!("max_mnt_namespaces"),
    UCOUNT_ENTRY!("max_cgroup_namespaces"),
    UCOUNT_ENTRY!("max_time_namespaces"),
    #[cfg(CONFIG_INOTIFY_USER)] UCOUNT_ENTRY!("max_inotify_instances"),
    #[cfg(CONFIG_INOTIFY_USER)] UCOUNT_ENTRY!("max_inotify_watches"),
    #[cfg(CONFIG_FANOTIFY)] UCOUNT_ENTRY!("max_fanotify_groups"),
    #[cfg(CONFIG_FANOTIFY)] UCOUNT_ENTRY!("max_fanotify_marks"),
    #[cfg(CONFIG_BINFMT_MISC)] UCOUNT_ENTRY!("max_binfmt_misc_interpreters"),
];

pub unsafe fn setup_userns_sysctls(ns: *mut user_namespace) -> bool {
    #[cfg(CONFIG_SYSCTL)] {
        let mut tbl: *mut ctl_table;
        BUILD_BUG_ON!(user_table.len() != UCOUNT_COUNTS);
        setup_sysctl_set(&mut (*ns).set, &set_root, set_is_seen);
        tbl = kmemdup(user_table.as_ptr(), core::mem::size_of_val(user_table), GFP_KERNEL);
        if !tbl.is_null() {
            for i in 0..UCOUNT_COUNTS {
                (*tbl.add(i)).data = &mut (*ns).ucount_max[i] as *mut _;
            }
            (*ns).sysctls = __register_sysctl_table(&mut (*ns).set, b"user\0".as_ptr() as _, tbl, user_table.len());
        }
        if (*ns).sysctls.is_null() {
            kfree(tbl);
            retire_sysctl_set(&mut (*ns).set);
            return false;
        }
    }
    true
}

pub unsafe fn retire_userns_sysctls(ns: *mut user_namespace) {
    #[cfg(CONFIG_SYSCTL)] {
        let tbl = (*(*ns).sysctls).ctl_table_arg;
        unregister_sysctl_table((*ns).sysctls);
        retire_sysctl_set(&mut (*ns).set);
        kfree(tbl);
    }
}

unsafe fn find_ucounts(ns: *mut user_namespace, uid: kuid_t, hashent: *mut hlist_nulls_head) -> *mut ucounts {
    let mut ucount: *mut ucounts;
    let mut pos: *mut hlist_nulls_node;
    guard_rcu!();
    hlist_nulls_for_each_entry_rcu!(ucount, pos, hashent, node) {
        if uid_eq((*ucount).uid, uid) && (*ucount).ns == ns && rcuref_get(&mut (*ucount).count) {
            return ucount;
        }
    }
    core::ptr::null_mut()
}

unsafe fn hlist_add_ucounts(ucount: *mut ucounts) {
    let hashent = ucounts_hashentry((*ucount).ns, (*ucount).uid);
    spin_lock_irq(&mut ucounts_lock);
    hlist_nulls_add_head_rcu(&mut (*ucount).node, hashent);
    spin_unlock_irq(&mut ucounts_lock);
}

pub unsafe fn alloc_ucounts(ns: *mut user_namespace, uid: kuid_t) -> *mut ucounts {
    let hashent = ucounts_hashentry(ns, uid);
    let ucount = find_ucounts(ns, uid, hashent);
    if !ucount.is_null() { return ucount; }
    let new = kzalloc_obj!();
    if new.is_null() { return core::ptr::null_mut(); }
    (*new).ns = ns;
    (*new).uid = uid;
    rcuref_init(&mut (*new).count, 1);
    spin_lock_irq(&mut ucounts_lock);
    let existing = find_ucounts(ns, uid, hashent);
    if !existing.is_null() {
        spin_unlock_irq(&mut ucounts_lock);
        kfree(new);
        return existing;
    }
    hlist_nulls_add_head_rcu(&mut (*new).node, hashent);
    get_user_ns((*new).ns);
    spin_unlock_irq(&mut ucounts_lock);
    new
}

pub unsafe fn put_ucounts(ucount: *mut ucounts) {
    let mut flags = 0;
    if rcuref_put(&mut (*ucount).count) {
        spin_lock_irqsave(&mut ucounts_lock, &mut flags);
        hlist_nulls_del_rcu(&mut (*ucount).node);
        spin_unlock_irqrestore(&mut ucounts_lock, flags);
        put_user_ns((*ucount).ns);
        kfree_rcu!(ucount, rcu);
    }
}

unsafe fn atomic_long_inc_below(v: *mut atomic_long_t, u: libc::c_long) -> bool {
    let mut c = atomic_long_read(v);
    loop {
        if c >= u { return false; }
        if atomic_long_try_cmpxchg(v, &mut c, c.wrapping_add(1)) { break; }
    }
    true
}

pub unsafe fn inc_ucount(ns: *mut user_namespace, uid: kuid_t, type_: ucount_type) -> *mut ucounts {
    let ucounts = alloc_ucounts(ns, uid);
    let mut iter = ucounts;
    let mut bad;
    while !iter.is_null() {
        let tns = (*iter).ns;
        let max = READ_ONCE!((*tns).ucount_max[type_ as usize]);
        if !atomic_long_inc_below(&mut (*iter).ucount[type_ as usize], max) {
            bad = iter;
            let mut rollback = ucounts;
            while rollback != bad {
                atomic_long_dec(&mut (*rollback).ucount[type_ as usize]);
                rollback = (*(*rollback).ns).ucounts;
            }
            put_ucounts(ucounts);
            return core::ptr::null_mut();
        }
        iter = (*tns).ucounts;
    }
    ucounts
}

EXPORT_SYMBOL_FOR_MODULES!(inc_ucount, "binfmt_misc");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
